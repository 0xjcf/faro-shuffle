mod task_parser;
mod llm_integration;
mod complexity_scoring;
mod error;
mod project_context;

pub use crate::error::Error;
pub use crate::task_parser::{Task, TaskParser};
pub use crate::llm_integration::{ComplexityResponse, LlmClient, OllamaClient};
pub use crate::complexity_scoring::{ComplexityScore, TaskAnalyzer, create_default_analyzer};
pub use crate::project_context::{analyze_project_context, ProjectContext};

pub type Result<T> = std::result::Result<T, Error>;

// Public API function for analyzing a task file
pub async fn analyze_task_file<P: AsRef<std::path::Path>>(
    task_file_path: P,
    project_dir: Option<P>,
) -> Result<ComplexityScore> {
    let analyzer = create_default_analyzer();
    analyzer.analyze_file(task_file_path, project_dir).await
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
