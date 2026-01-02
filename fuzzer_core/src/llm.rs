#[cfg(feature = "llm")]
use std::{
    collections::VecDeque,
    env,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

#[cfg(feature = "llm")]
use reqwest::blocking::Client;
#[cfg(feature = "llm")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "llm")]
use crate::mutators::{curl_knowledge, url_parser::ParsedURL};

#[cfg(feature = "llm")]
const DEFAULT_API_URL: &str = "http://127.0.0.1:11434/api/generate";
#[cfg(feature = "llm")]
const DEFAULT_MODEL: &str = "llama2:7b";

#[cfg(feature = "llm")]
#[derive(Debug, Clone)]
pub struct LLMSeedConfig {
    pub stagnation_duration: Duration,
    pub batch_size: usize,
    pub max_seed_len: usize,
    pub history_cap: usize,
    pub timeout: Duration,
    pub max_retries: usize,
    pub min_interval: Duration,
}

#[cfg(feature = "llm")]
impl Default for LLMSeedConfig {
    fn default() -> Self {
        Self {
            stagnation_duration: Duration::from_secs(30),
            batch_size: 4,
            max_seed_len: 2048,
            history_cap: 64,
            timeout: Duration::from_secs(10),
            max_retries: 2,
            min_interval: Duration::from_secs(30),
        }
    }
}

#[cfg(feature = "llm")]
#[derive(Debug)]
pub struct LLMSeedGenerator {
    config: LLMSeedConfig,
    model: String,
    api_url: String,
    enabled: bool,
    request_tx: mpsc::Sender<LLMSeedRequest>,
    response_rx: mpsc::Receiver<LLMSeedResponse>,
    history: Arc<Mutex<VecDeque<String>>>,
    last_trigger: Option<std::time::Instant>,
}

#[cfg(feature = "llm")]
impl LLMSeedGenerator {
    pub fn new(config: LLMSeedConfig) -> Self {
        let api_url = env::var("IFB_LLM_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());
        let model = env::var("IFB_LLM_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        let has_api_key = env::var("OPENAI_API_KEY").is_ok();
        let has_custom_url = env::var("IFB_LLM_URL").is_ok();
        let enabled = has_api_key || has_custom_url;

        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, response_rx) = mpsc::channel();
        let history = Arc::new(Mutex::new(VecDeque::new()));

        if enabled {
            let history_clone = Arc::clone(&history);
            let config_clone = config.clone();
            let api_url_clone = api_url.clone();
            let model_clone = model.clone();
            thread::spawn(move || {
                llm_worker(
                    request_rx,
                    response_tx,
                    history_clone,
                    &api_url_clone,
                    &model_clone,
                    &config_clone,
                );
            });
        }

        Self {
            config,
            model,
            api_url,
            enabled,
            request_tx,
            response_rx,
            history,
            last_trigger: None,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn config(&self) -> &LLMSeedConfig {
        &self.config
    }

    pub fn maybe_trigger(&mut self, now: std::time::Instant) -> bool {
        if !self.enabled {
            return false;
        }
        if let Some(last) = self.last_trigger {
            if now.duration_since(last) < self.config.min_interval {
                return false;
            }
        }

        let _ = self.request_tx.send(LLMSeedRequest {
            batch_size: self.config.batch_size,
        });
        self.last_trigger = Some(now);
        true
    }

    pub fn try_recv(&mut self) -> Option<LLMSeedResponse> {
        self.response_rx.try_recv().ok()
    }

    pub fn record_success(&mut self, seed: &str) {
        if let Ok(mut history) = self.history.lock() {
            history.push_front(seed.to_string());
            while history.len() > self.config.history_cap {
                history.pop_back();
            }
        }
    }

    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    pub fn model(&self) -> &str {
        &self.model
    }
}

#[cfg(feature = "llm")]
#[derive(Debug)]
pub struct LLMSeedRequest {
    pub batch_size: usize,
}

#[cfg(feature = "llm")]
#[derive(Debug)]
pub struct LLMSeedResponse {
    pub seeds: Vec<String>,
    pub rejected: usize,
}

#[cfg(feature = "llm")]
fn llm_worker(
    request_rx: mpsc::Receiver<LLMSeedRequest>,
    response_tx: mpsc::Sender<LLMSeedResponse>,
    history: Arc<Mutex<VecDeque<String>>>,
    api_url: &str,
    model: &str,
    config: &LLMSeedConfig,
) {
    let client = Client::builder()
        .timeout(config.timeout)
        .build()
        .unwrap_or_else(|_| Client::new());

    while let Ok(request) = request_rx.recv() {
        let mut seeds = Vec::new();
        let mut rejected = 0;

        for _ in 0..request.batch_size {
            let prompt = build_prompt(history.clone());
            let mut attempt = 0;
            let mut generated: Option<String> = None;

            while attempt <= config.max_retries {
                attempt += 1;
                match fetch_seed(&client, api_url, model, &prompt) {
                    Ok(seed) => {
                        generated = Some(seed);
                        break;
                    }
                    Err(err) => {
                        eprintln!(
                            "[IFB][LLM] ⚠️  LLM request failed (attempt {}): {}",
                            attempt, err
                        );
                        if attempt > config.max_retries {
                            break;
                        }
                    }
                }
            }

            match generated.and_then(|seed| sanitize_seed(&seed, config.max_seed_len)) {
                Some(clean) => seeds.push(clean),
                None => rejected += 1,
            }
        }

        let _ = response_tx.send(LLMSeedResponse { seeds, rejected });
    }
}

#[cfg(feature = "llm")]
fn build_prompt(history: Arc<Mutex<VecDeque<String>>>) -> String {
    let history_snapshot = history
        .lock()
        .map(|history| {
            history
                .iter()
                .take(8)
                .map(|seed| format!("- {}", seed))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let memory_section = if history_snapshot.is_empty() {
        "(no successful seeds yet)".to_string()
    } else {
        history_snapshot
    };

    let curl_knowledge = curl_knowledge::get_curl_knowledge_context();

    format!(
        r#"You are a fuzzing seed generator for libcurl URL parsing.

SUCCESSFUL SEEDS:
{}

CURL KNOWLEDGE BASE:
{}

TASK:
Generate ONE candidate URL seed that is likely to exercise new parsing logic or edge cases.
Return ONLY the raw URL string, no formatting, no commentary."#,
        memory_section, curl_knowledge
    )
}

#[cfg(feature = "llm")]
fn fetch_seed(client: &Client, api_url: &str, model: &str, prompt: &str) -> Result<String, String> {
    let request = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let response = client
        .post(api_url)
        .json(&request)
        .send()
        .map_err(|err| err.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let completion = response
        .json::<OllamaResponse>()
        .map_err(|err| err.to_string())?;
    Ok(completion.response)
}

#[cfg(feature = "llm")]
fn sanitize_seed(seed: &str, max_len: usize) -> Option<String> {
    let trimmed = seed.trim().trim_matches('"');
    if trimmed.is_empty() {
        return None;
    }

    let mut cleaned = String::with_capacity(trimmed.len());
    for ch in trimmed.chars() {
        if ch.is_control() {
            continue;
        }
        cleaned.push(ch);
        if cleaned.len() >= max_len {
            break;
        }
    }

    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
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

#[cfg(feature = "llm")]
#[allow(dead_code)]
fn parse_url(input: &str) -> Option<ParsedURL> {
    ParsedURL::parse(input)
}
