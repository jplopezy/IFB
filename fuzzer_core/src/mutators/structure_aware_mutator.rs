//! Structure-Aware Mutator for URLs
//! Mutates URL components intelligently instead of raw bytes

use libafl::prelude::*;
use libafl_bolts::rands::Rand;
use std::num::NonZeroUsize;

use super::curl_knowledge::*;
use super::url_parser::{ParsedURL, URLComponent};

// Note: We removed StructureAwareMetadata because ValueInput doesn't support metadata directly.
// Metadata is stored in Testcase, not in Input. The coverage feedback will track which inputs are interesting.

/// Structure-Aware Mutator that parses URLs and mutates components
#[derive(Debug, Clone)]
pub struct StructureAwareMutator {
    /// Probability of using structure-aware mutation (0-100)
    structure_aware_prob: u8,
}

impl StructureAwareMutator {
    pub fn new() -> Self {
        Self {
            structure_aware_prob: 80, // 80% structure-aware, 20% raw bytes
        }
    }

    pub fn with_probability(prob: u8) -> Self {
        Self {
            structure_aware_prob: prob.min(100),
        }
    }

    /// Mutate a URL component using knowledge base
    fn mutate_component(component: URLComponent, value: &str, state: &mut impl HasRand) -> String {
        match component {
            URLComponent::Scheme => {
                // Mutate scheme from knowledge base
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 30
                {
                    // 30% chance to use a random scheme from knowledge base
                    let len = URL_SCHEMES.len();
                    if len > 0 {
                        URL_SCHEMES[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    // 70% chance to mutate existing scheme
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::Host => {
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 30
                {
                    let len = URL_HOSTS.len();
                    if len > 0 {
                        URL_HOSTS[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::Path => {
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 30
                {
                    let len = URL_PATHS.len();
                    if len > 0 {
                        URL_PATHS[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::Query => {
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 30
                {
                    let len = URL_QUERIES.len();
                    if len > 0 {
                        URL_QUERIES[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::Fragment => {
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 30
                {
                    let len = URL_FRAGMENTS.len();
                    if len > 0 {
                        URL_FRAGMENTS[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::Port => {
                if state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(100) })
                    < 50
                {
                    let len = URL_PORTS.len();
                    if len > 0 {
                        URL_PORTS[state
                            .rand_mut()
                            .below(unsafe { NonZeroUsize::new_unchecked(len) })]
                        .to_string()
                    } else {
                        Self::mutate_string(value, state)
                    }
                } else {
                    Self::mutate_string(value, state)
                }
            }
            URLComponent::UserInfo => Self::mutate_string(value, state),
        }
    }

    /// Simple string mutation (insert, delete, replace, flip)
    fn mutate_string(s: &str, state: &mut impl HasRand) -> String {
        let mut result = s.to_string();
        let mutation_type = state
            .rand_mut()
            .below(unsafe { NonZeroUsize::new_unchecked(4) });

        match mutation_type {
            0 => {
                // Insert random character
                let pos = if result.is_empty() {
                    0
                } else {
                    state
                        .rand_mut()
                        .below(unsafe { NonZeroUsize::new_unchecked(result.len()) })
                };
                let ch = (state
                    .rand_mut()
                    .below(unsafe { NonZeroUsize::new_unchecked(256) })
                    as u8) as char;
                result.insert(pos, ch);
            }
            1 => {
                // Delete character
                if !result.is_empty() {
                    let pos = state
                        .rand_mut()
                        .below(unsafe { NonZeroUsize::new_unchecked(result.len()) });
                    result.remove(pos);
                }
            }
            2 => {
                // Replace character
                if !result.is_empty() {
                    let pos = state
                        .rand_mut()
                        .below(unsafe { NonZeroUsize::new_unchecked(result.len()) });
                    let ch = (state
                        .rand_mut()
                        .below(unsafe { NonZeroUsize::new_unchecked(256) })
                        as u8) as char;
                    result.replace_range(pos..=pos, &ch.to_string());
                }
            }
            _ => {
                // Flip bit
                if !result.is_empty() {
                    let pos = state
                        .rand_mut()
                        .below(unsafe { NonZeroUsize::new_unchecked(result.len()) });
                    if let Some(ch) = result.chars().nth(pos) {
                        let mut bytes = ch.to_string().into_bytes();
                        if !bytes.is_empty() {
                            bytes[0] ^= 1
                                << state
                                    .rand_mut()
                                    .below(unsafe { NonZeroUsize::new_unchecked(8) });
                            if let Ok(new_str) = String::from_utf8(bytes) {
                                result.replace_range(pos..=pos, &new_str);
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

impl Default for StructureAwareMutator {
    fn default() -> Self {
        Self::new()
    }
}

impl libafl_bolts::Named for StructureAwareMutator {
    fn name(&self) -> &std::borrow::Cow<'static, str> {
        &std::borrow::Cow::Borrowed("StructureAwareMutator")
    }
}

impl<S> Mutator<ValueInput<Vec<u8>>, S> for StructureAwareMutator
where
    S: HasRand,
{
    fn mutate(
        &mut self,
        state: &mut S,
        input: &mut ValueInput<Vec<u8>>,
    ) -> Result<MutationResult, Error> {
        // Decide: structure-aware or raw bytes?
        if state
            .rand_mut()
            .below(unsafe { NonZeroUsize::new_unchecked(100) })
            < self.structure_aware_prob as usize
        {
            // Structure-aware mutation
            let bytes = input.sub_bytes(..);
            let input_str = match std::str::from_utf8(bytes.as_slice()) {
                Ok(s) => s,
                Err(_) => return Ok(MutationResult::Skipped),
            };

            // Parse URL
            let mut parsed = match ParsedURL::parse(input_str) {
                Some(p) => p,
                None => {
                    // If parsing fails, try to create a valid URL
                    ParsedURL {
                        scheme: "http".to_string(),
                        host: "localhost".to_string(),
                        port: None,
                        path: input_str.to_string(),
                        query: String::new(),
                        fragment: String::new(),
                        userinfo: String::new(),
                    }
                }
            };

            // Select component to mutate
            let components = [
                URLComponent::Scheme,
                URLComponent::Host,
                URLComponent::Path,
                URLComponent::Query,
                URLComponent::Fragment,
                URLComponent::Port,
            ];
            let component = components[state
                .rand_mut()
                .below(unsafe { NonZeroUsize::new_unchecked(components.len()) })];

            // Mutate selected component
            let current_value = parsed.get_component(component).to_string();
            let mutated_value = Self::mutate_component(component, &current_value, state);
            parsed.mutate_component(component, mutated_value);

            // Serialize back
            let new_url = parsed.to_string();
            *input = ValueInput::new(new_url.as_bytes().to_vec());

            // Update metadata - ValueInput doesn't support metadata directly
            // Metadata is stored in Testcase, not in Input
            // For now, we'll skip metadata tracking in the input itself
            // The coverage feedback will track which inputs are interesting

            Ok(MutationResult::Mutated)
        } else {
            // Raw bytes mutation (fallback to havoc)
            Ok(MutationResult::Skipped)
        }
    }

    fn post_exec(&mut self, _state: &mut S, _corpus_id: Option<CorpusId>) -> Result<(), Error> {
        Ok(())
    }
}
