use std::env;
use std::time::Duration;

use libafl::inputs::{BytesInput, HasBytesVec};
use libafl::mutators::{MutationResult, Mutator};
use libafl::prelude::{HasRand, Named};
use libafl::Error;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_API_URL: &str = "http://127.0.0.1:8000/v1/chat/completions";
const SYSTEM_PROMPT: &str =
    "You are a fuzzing mutation engine. Mutate this input to cause edge cases. Return ONLY the raw string.";

#[derive(Debug, Clone)]
pub struct LLMMutator {
    api_url: String,
    client: Client,
}

impl LLMMutator {
    pub fn new() -> Self {
        let api_url = env::var("IFB_LLM_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());

        let client = Client::builder()
            .timeout(Duration::from_millis(500))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { api_url, client }
    }
}

impl Default for LLMMutator {
    fn default() -> Self {
        Self::new()
    }
}

impl Named for LLMMutator {
    fn name(&self) -> &str {
        "LLMMutator"
    }
}

impl<S> Mutator<BytesInput, S> for LLMMutator
where
    S: HasRand,
{
    fn mutate(
        &mut self,
        state: &mut S,
        input: &mut BytesInput,
        _stage_idx: i32,
    ) -> Result<MutationResult, Error> {
        if state.rand_mut().below(100) != 0 {
            return Ok(MutationResult::Skipped);
        }

        let input_str = match std::str::from_utf8(input.bytes()) {
            Ok(value) => value,
            Err(_) => return Ok(MutationResult::Skipped),
        };

        let request = ChatCompletionRequest {
            model: "local-model".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input_str.to_string(),
                },
            ],
        };

        let response = match self.client.post(&self.api_url).json(&request).send() {
            Ok(resp) => resp,
            Err(_) => return Ok(MutationResult::Skipped),
        };

        if !response.status().is_success() {
            return Ok(MutationResult::Skipped);
        }

        let completion = match response.json::<ChatCompletionResponse>() {
            Ok(parsed) => parsed,
            Err(_) => return Ok(MutationResult::Skipped),
        };

        let Some(choice) = completion.choices.into_iter().next() else {
            return Ok(MutationResult::Skipped);
        };

        let mutated = choice.message.content;
        if mutated.is_empty() {
            return Ok(MutationResult::Skipped);
        }

        let bytes = input.bytes_mut();
        bytes.clear();
        bytes.extend_from_slice(mutated.as_bytes());

        Ok(MutationResult::Mutated)
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}
