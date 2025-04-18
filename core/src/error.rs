use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to parse Markdown: {0}")]
    MarkdownParse(String),
    
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Failed to parse Ollama response: {0}")]
    ResponseParse(String),
    
    #[error("Ollama API error: {0}")]
    OllamaApi(String),
    
    #[error("Invalid task: {0}")]
    InvalidTask(String),
    
    #[error("Serialization/deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
} 