#[cfg(feature = "llm")]
use std::env;
#[cfg(feature = "llm")]
use std::time::Duration;

#[cfg(feature = "llm")]
use libafl::prelude::*;
#[cfg(feature = "llm")]
use libafl_bolts::prelude::SerdeAny;
#[cfg(feature = "llm")]
use reqwest::blocking::Client;
#[cfg(feature = "llm")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "llm")]
use super::url_parser::ParsedURL;

#[cfg(feature = "llm")]
const DEFAULT_API_URL: &str = "http://127.0.0.1:11434/api/generate";
#[cfg(feature = "llm")]
const SYSTEM_PROMPT: &str =
    "You are an expert fuzzing engine. Generate mutations designed to increase code coverage and find bugs. Return ONLY the raw string.";

#[cfg(feature = "llm")]
#[derive(Debug, Clone, Serialize, Deserialize, SerdeAny)]
pub struct LLMHistoryMetadata {
    pub last_prompt: String,
    pub generation: u32,
}

#[cfg(feature = "llm")]
impl SerdeAny for LLMHistoryMetadata {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any_boxed(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[cfg(feature = "llm")]
#[derive(Debug, Clone)]
pub struct LLMMutator {
    api_url: String,
    client: Client,
}

#[cfg(feature = "llm")]
impl LLMMutator {
    pub fn new() -> Self {
        let api_url = env::var("IFB_LLM_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());

        let client = Client::builder()
            .timeout(Duration::from_millis(10000))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { api_url, client }
    }
}

#[cfg(feature = "llm")]
impl Default for LLMMutator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "llm")]
impl Named for LLMMutator {
    fn name(&self) -> &std::borrow::Cow<'static, str> {
        &std::borrow::Cow::Borrowed("LLMMutator")
    }
}

#[cfg(feature = "llm")]
impl<S> Mutator<ValueInput<Vec<u8>>, S> for LLMMutator
where
    S: HasRand,
{
    fn mutate(
        &mut self,
        state: &mut S,
        input: &mut ValueInput<Vec<u8>>,
    ) -> Result<MutationResult, Error> {
        // Adaptive probability: 
        // - 30% base chance (more aggressive than 10%)
        // - LLM helps when corpus stagnates by generating more creative mutations
        // - The structure-aware mutator handles most mutations, LLM provides "escape" when stuck
        if state.rand_mut().below(std::num::NonZeroUsize::new(10).unwrap()) >= 3 {
            return Ok(MutationResult::Skipped);
        }

        let bytes = input.sub_bytes(..);
        let input_str = match std::str::from_utf8(bytes.as_slice()) {
            Ok(value) => value,
            Err(_) => return Ok(MutationResult::Skipped),
        };

        // Parse URL to understand structure
        let parsed_url = super::url_parser::ParsedURL::parse(input_str);
        let url_context = if let Some(ref parsed) = parsed_url {
            format!(
                "URL Structure: scheme='{}', host='{}', port={:?}, path='{}', query='{}', fragment='{}'",
                parsed.scheme, parsed.host, parsed.port, parsed.path, parsed.query, parsed.fragment
            )
        } else {
            format!("Raw URL (could not parse): '{}'", input_str)
        };

        // Get cURL knowledge base context
        let curl_knowledge = super::curl_knowledge::get_curl_knowledge_context();

        // Note: Metadata is stored in Testcase, not in Input
        // For now, we'll use the input itself as context
        let metadata_context = format!(
            "This input is in the corpus, meaning it increased code coverage. Generate a mutation that explores deeper paths."
        );

        // Build intelligent prompt with full context
        let user_prompt = format!(
            r#"You are an expert fuzzing engine for cURL/libcurl. Your goal is to generate URL mutations that increase code coverage and find security vulnerabilities.

CONTEXT:
{}

CURL KNOWLEDGE BASE:
{}

PREVIOUS MUTATION INFO:
{}

CURRENT INPUT:
'{}'

TASK:
Generate a mutated URL that:
1. Is syntactically valid (or close to valid) for cURL
2. Explores edge cases and potential vulnerabilities
3. Uses knowledge of cURL's supported protocols, headers, and URL schemes
4. If previous mutation worked on a specific component, focus on that component
5. Incorporates patterns from the knowledge base (malformed paths, special characters, etc.)

Return ONLY the mutated URL string, nothing else."#,
            url_context,
            curl_knowledge,
            metadata_context,
            input_str
        );

        // Ollama API request
        let request = OllamaRequest {
            model: "llama2:7b".to_string(),
            prompt: user_prompt,
            stream: false,
        };

        let response = match self.client.post(&self.api_url).json(&request).send() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("[NEURO] ❌ HTTP Error: {}", e);
                return Ok(MutationResult::Skipped);
            }
        };

        if !response.status().is_success() {
            return Ok(MutationResult::Skipped);
        }

        let completion = match response.json::<OllamaResponse>() {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("[NEURO] ❌ JSON Error: {}", e);
                return Ok(MutationResult::Skipped);
            }
        };

        let mutated = completion.response.trim();
        if mutated.is_empty() {
            return Ok(MutationResult::Skipped);
        }

        // Apply the mutation
        *input = ValueInput::new(mutated.as_bytes().to_vec());

        println!("[NEURO] 🧬 Coverage-guided LLM Mutation Applied: {} bytes", bytes.len());
        Ok(MutationResult::Mutated)
    }
}

#[cfg(feature = "llm")]
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[cfg(feature = "llm")]
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}
