use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to parse markdown: {0}")]
    MarkdownParse(String),
    
    #[error("Task description is empty or invalid")]
    InvalidTaskDescription,
    
    #[error("LLM API request failed: {0}")]
    LlmRequest(#[from] reqwest::Error),
    
    #[error("Failed to parse LLM response: {0}")]
    LlmResponseParse(#[from] serde_json::Error),
    
    #[error("LLM response did not contain expected fields")]
    LlmResponseFormat,
    
    #[error("Complexity score out of range (0-10): {0}")]
    ScoreOutOfRange(i32),
    
    #[error("Project path is not a directory: {0}")]
    ProjectPathNotDirectory(PathBuf),
    
    #[error("Unknown error")]
    Unknown,
} 