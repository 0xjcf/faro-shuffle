use crate::Result;
use crate::llm_integration::{ComplexityResponse, LlmClient, OllamaClient};
use crate::task_parser::{Task, TaskParser};
use crate::project_context::{analyze_project_context, ProjectContext};
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Represents a complexity score with rationale
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    
    /// Analyze a task file for complexity, optionally considering project context
    pub async fn analyze_file<P: AsRef<Path>>(
        &self,
        task_file_path: P,
        project_dir: Option<P>,
    ) -> Result<ComplexityScore> {
        let task = self.parser.parse_file(task_file_path)?;
        
        let context = match project_dir {
            Some(dir) => Some(analyze_project_context(dir).await?),
            None => None,
        };
        
        self.analyze_task(&task, context.as_ref()).await
    }
    
    /// Analyze a task for complexity, using optional project context
    pub async fn analyze_task(
        &self,
        task: &Task,
        context: Option<&ProjectContext>,
    ) -> Result<ComplexityScore> {
        let response = self.client.analyze_complexity(&task.description, context).await?;
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
        async fn analyze_complexity(
            &self,
            task_description: &str,
            _context: Option<&ProjectContext>,
        ) -> Result<ComplexityResponse> {
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
        
        // Test without context
        let result_no_context = analyzer.analyze_task(&task, None).await.unwrap();
        assert_eq!(result_no_context.score, 7);
        assert!(result_no_context.rationale.contains("Build a complex system"));
        
        // Test with mock context (needs MockLlmClient update)
        // let mock_context = ProjectContext { ... }; 
        // let result_with_context = analyzer.analyze_task(&task, Some(&mock_context)).await.unwrap();
        // assert_eq!(result_with_context.score, expected_score_with_context);
    }
} 