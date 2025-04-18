mod task_parser;
mod llm_integration;
mod complexity_scoring;
mod error;

pub use crate::error::Error;
pub use crate::task_parser::{Task, TaskParser};
pub use crate::llm_integration::{ComplexityResponse, LlmClient, OllamaClient};
pub use crate::complexity_scoring::{ComplexityScore, TaskAnalyzer, create_default_analyzer};

pub type Result<T> = std::result::Result<T, Error>;

// Public API function for analyzing a task file
pub async fn analyze_task_file<P: AsRef<std::path::Path>>(path: P) -> Result<ComplexityScore> {
    let analyzer = create_default_analyzer();
    analyzer.analyze_file(path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
