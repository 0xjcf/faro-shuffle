use crate::Result;
use crate::llm_integration::{ComplexityResponse, LlmClient, OllamaClient};
use crate::task_parser::{Task, TaskParser};
use std::path::Path;

/// Represents a complexity score with rationale
#[derive(Debug, Clone)]
pub struct ComplexityScore {
    /// Score on a scale of 1-10
    pub score: u8,
    /// Explanation for the score
    pub rationale: String,
}

impl From<ComplexityResponse> for ComplexityScore {
    fn from(response: ComplexityResponse) -> Self {
        Self {
            score: response.score,
            rationale: response.rationale,
        }
    }
}

/// Main analyzer for task complexity
pub struct TaskAnalyzer<C: LlmClient> {
    parser: TaskParser,
    client: C,
}

impl<C: LlmClient> TaskAnalyzer<C> {
    /// Create a new task analyzer with the given LLM client
    pub fn new(client: C) -> Self {
        Self {
            parser: TaskParser::new(),
            client,
        }
    }
    
    /// Analyze a task file for complexity
    pub async fn analyze_file<P: AsRef<Path>>(&self, path: P) -> Result<ComplexityScore> {
        let task = self.parser.parse_file(path)?;
        self.analyze_task(&task).await
    }
    
    /// Analyze a task for complexity
    pub async fn analyze_task(&self, task: &Task) -> Result<ComplexityScore> {
        let response = self.client.analyze_complexity(&task.description).await?;
        Ok(response.into())
    }
}

/// Convenience function to create a default task analyzer with Ollama
pub fn create_default_analyzer() -> TaskAnalyzer<OllamaClient> {
    TaskAnalyzer::new(OllamaClient::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_integration::ComplexityResponse;
    use async_trait::async_trait;
    
    // Mock LLM client for testing
    struct MockLlmClient;
    
    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn analyze_complexity(&self, task_description: &str) -> Result<ComplexityResponse> {
            // Return a mock response based on task length
            let score = if task_description.len() < 20 { 3 } else { 7 };
            Ok(ComplexityResponse {
                score,
                rationale: format!("Mock rationale for: {}", task_description),
            })
        }
    }
    
    #[tokio::test]
    async fn test_analyze_task() {
        let analyzer = TaskAnalyzer::new(MockLlmClient);
        let task = Task {
            description: "Build a complex system".to_string(),
        };
        
        let result = analyzer.analyze_task(&task).await.unwrap();
        assert_eq!(result.score, 7);
        assert!(result.rationale.contains("Build a complex system"));
    }
} 