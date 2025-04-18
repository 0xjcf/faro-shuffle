use crate::error::Error;
use crate::Result;
use std::fs;
use std::path::Path;

/// Represents a task to be analyzed
#[derive(Debug, Clone)]
pub struct Task {
    /// The description text of the task
    pub description: String,
}

/// Parser for reading task information from files
pub struct TaskParser;

impl TaskParser {
    /// Create a new TaskParser
    pub fn new() -> Self {
        TaskParser
    }

    /// Parse a task from a markdown file
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Task> {
        let content = fs::read_to_string(path)?;
        self.parse_markdown(&content)
    }

    /// Parse a task from markdown content
    pub fn parse_markdown(&self, content: &str) -> Result<Task> {
        // For V1, we're just extracting text and doing minimal validation
        // Later versions can parse more structured formats
        if content.trim().is_empty() {
            return Err(Error::InvalidTask("Task description cannot be empty".into()));
        }

        // Use the markdown crate to parse the markdown, but we're just extracting text for now
        let parsed = markdown::to_html(content);
        
        // For V1, we're keeping this simple - just extract all text without HTML tags
        let description = html_to_text(&parsed);
        
        if description.trim().is_empty() {
            return Err(Error::InvalidTask("Task description cannot be empty after parsing".into()));
        }
        
        Ok(Task { description })
    }
}

/// Very basic HTML to text conversion - removes HTML tags
fn html_to_text(html: &str) -> String {
    // This is a very simplistic implementation that just strips HTML tags
    // We may want to use a proper HTML parser in the future
    let mut result = String::new();
    let mut in_tag = false;
    
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    
    // Replace multiple whitespace with a single space
    let result = result.split_whitespace().collect::<Vec<_>>().join(" ");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let parser = TaskParser::new();
        
        // Simple case
        let md = "# Test Task\nThis is a test task.";
        let task = parser.parse_markdown(md).unwrap();
        assert_eq!(task.description, "Test Task This is a test task.");
        
        // Empty case
        let md = "";
        assert!(parser.parse_markdown(md).is_err());
    }
    
    #[test]
    fn test_html_to_text() {
        assert_eq!(html_to_text("<p>Hello</p>"), "Hello");
        assert_eq!(html_to_text("<h1>Title</h1><p>Content</p>"), "Title Content");
    }
} 