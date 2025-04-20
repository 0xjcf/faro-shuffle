use crate::error::Error;
use crate::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;
use crate::project_context::ProjectContext;

/// Template for the complexity analysis prompt
const COMPLEXITY_PROMPT_TEMPLATE: &str = r#"
You are a task complexity analyzer. 
Given a task description, analyze its complexity on a scale of 1-10 (where 1 is trivial and 10 is extremely complex).
Provide a brief explanation for your rating (1-3 sentences).

Task Description:
------------
{task_description}
------------

Return your response in JSON format with the following structure:
{
  "score": <number between 1 and 10>,
  "rationale": "<brief explanation for the score>"
}
"#;

/// Trait for LLM clients
#[async_trait]
pub trait LlmClient {
    /// Send a complexity analysis prompt to the LLM and get a score with rationale
    async fn analyze_complexity(
        &self,
        task_description: &str,
        context: Option<&ProjectContext>,
    ) -> Result<ComplexityResponse>;
}

/// Response from the complexity analysis
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComplexityResponse {
    /// Complexity score from 1-10
    pub score: u8,
    /// Brief explanation for the score
    pub rationale: String,
}

/// Ollama API request for model generation
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,
}

/// Client for the Ollama API
pub struct OllamaClient {
    client: Client,
    endpoint: String,
    model: String,
}

impl OllamaClient {
    /// Create a new Ollama client with default settings
    pub fn new() -> Self {
        Self::with_endpoint("http://localhost:11434", "llama3")
    }
    
    /// Create a new Ollama client with custom endpoint and model
    pub fn with_endpoint(endpoint: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: format!("{}/api/generate", endpoint),
            model: model.to_string(),
        }
    }
    
    /// Create the complexity analysis prompt for the given task description and optional context
    fn create_complexity_prompt(&self, task_description: &str, context: Option<&ProjectContext>) -> String {
        let mut prompt = COMPLEXITY_PROMPT_TEMPLATE.replace("{task_description}", task_description);
        
        if let Some(ctx) = context {
            let context_str = format!(
                "\n\nProject Context:\n------------\nDetected Technologies: {:?}\nFile Counts (Top 5): {}\n------------",
                ctx.technologies,
                ctx.file_counts.iter()
                   .take(5) // Limit to top 5 for brevity
                   .map(|(ext, count)| format!("{}: {}", ext, count))
                   .collect::<Vec<_>>().join(", ")
            );
            // Insert context before the JSON instruction part
            if let Some(json_instruction_pos) = prompt.find("Return your response in JSON format") {
                 prompt.insert_str(json_instruction_pos, &context_str);
            } else {
                // Fallback: append if specific marker not found
                prompt.push_str(&context_str);
            }
        }
        
        prompt
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn analyze_complexity(
        &self,
        task_description: &str,
        context: Option<&ProjectContext>,
    ) -> Result<ComplexityResponse> {
        let prompt = self.create_complexity_prompt(task_description, context);
        
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false, // We don't need streaming for this use case
            format: "json".to_string(), // Request JSON output
        };
        
        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let _error_text = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            // No direct equivalent for OllamaApi(String), map to Unknown or a more specific HTTP error if possible
            // For now, let's use Unknown as a placeholder
            return Err(Error::Unknown); 
        }
        
        // Parse the response JSON - this returns serde_json::Error via #[from]
        let response_body: serde_json::Value = response.json().await?;
        
        // Extract the "response" field which contains the raw LLM output as a string
        let output = response_body["response"].as_str()
            .ok_or_else(|| Error::LlmResponseFormat)?;
            
        // Parse the JSON from the LLM output - this returns serde_json::Error via #[from]
        let result: ComplexityResponse = serde_json::from_str(output)?;
            
        // Basic validation
        if result.score < 1 || result.score > 10 {
            // Use the dedicated ScoreOutOfRange error
            return Err(Error::ScoreOutOfRange(result.score as i32)); 
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_context::{DetectedTechnology, ProjectContext};
    use std::collections::HashMap;
    use std::path::PathBuf;
    
    #[test]
    fn test_create_complexity_prompt_without_context() {
        let client = OllamaClient::new();
        let prompt = client.create_complexity_prompt("Build a website", None);
        assert!(prompt.contains("Build a website"));
        assert!(!prompt.contains("Project Context:"));
        assert!(prompt.contains("scale of 1-10"));
        assert!(prompt.contains("JSON format"));
    }

    #[test]
    fn test_create_complexity_prompt_with_context() {
        let client = OllamaClient::new();
        let mut file_counts = HashMap::new();
        file_counts.insert("rs".to_string(), 100);
        file_counts.insert("toml".to_string(), 5);
        let context = ProjectContext {
            root_dir: PathBuf::from("."),
            technologies: vec![DetectedTechnology::Rust],
            file_counts,
        };
        let prompt = client.create_complexity_prompt("Add a feature", Some(&context));
        assert!(prompt.contains("Add a feature"));
        assert!(prompt.contains("Project Context:"));
        assert!(prompt.contains("Detected Technologies: [Rust]"));
        assert!(prompt.contains("File Counts (Top 5): rs: 100, toml: 5")); // Order might vary
        assert!(prompt.contains("Return your response in JSON format"));
    }
} 