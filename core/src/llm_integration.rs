use crate::error::Error;
use crate::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;

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
    async fn analyze_complexity(&self, task_description: &str) -> Result<ComplexityResponse>;
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
    
    /// Create the complexity analysis prompt for the given task description
    fn create_complexity_prompt(&self, task_description: &str) -> String {
        COMPLEXITY_PROMPT_TEMPLATE.replace("{task_description}", task_description)
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn analyze_complexity(&self, task_description: &str) -> Result<ComplexityResponse> {
        let prompt = self.create_complexity_prompt(task_description);
        
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
            let error_text = response.text().await?;
            return Err(Error::OllamaApi(format!("API error: {}", error_text)));
        }
        
        // Parse the response JSON
        let response_body: serde_json::Value = response.json().await?;
        
        // Extract the "response" field which contains the raw LLM output as a string
        let output = response_body["response"].as_str()
            .ok_or_else(|| Error::ResponseParse("Missing 'response' field in API response".into()))?;
            
        // Parse the JSON from the LLM output
        let result: ComplexityResponse = serde_json::from_str(output)
            .map_err(|e| Error::ResponseParse(format!("Failed to parse JSON response: {}", e)))?;
            
        // Basic validation
        if result.score < 1 || result.score > 10 {
            return Err(Error::ResponseParse(format!("Invalid score: {}", result.score)));
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_complexity_prompt() {
        let client = OllamaClient::new();
        let prompt = client.create_complexity_prompt("Build a website");
        assert!(prompt.contains("Build a website"));
        assert!(prompt.contains("scale of 1-10"));
        assert!(prompt.contains("JSON format"));
    }
} 