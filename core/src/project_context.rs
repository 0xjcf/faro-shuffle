use crate::{Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectedTechnology {
    Rust,
    Node,
    Python,
    // Add more as needed
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub root_dir: PathBuf,
    pub technologies: Vec<DetectedTechnology>,
    pub file_counts: HashMap<String, usize>,
    // Add more fields later: dependencies, structure map, etc.
}

/// Analyzes a project directory to extract context.
pub async fn analyze_project_context<P: AsRef<Path>>(root_dir: P) -> Result<ProjectContext> {
    let root_path = root_dir.as_ref().to_path_buf();
    if !root_path.is_dir() {
        return Err(Error::ProjectPathNotDirectory(root_path));
    }

    let mut technologies = Vec::new();
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    let mut saw_cargo = false;
    let mut saw_package_json = false;
    let mut saw_pyproject = false;

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_entry(|e| !is_hidden_or_ignored(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str());
        let extension = path.extension().and_then(|e| e.to_str());

        match file_name {
            Some("Cargo.toml") => saw_cargo = true,
            Some("package.json") => saw_package_json = true,
            Some("pyproject.toml") | Some("requirements.txt") => saw_pyproject = true,
            _ => {}
        }

        if let Some(ext) = extension {
            let count = file_counts.entry(ext.to_lowercase()).or_insert(0);
            *count += 1;
        }
    }

    if saw_cargo {
        technologies.push(DetectedTechnology::Rust);
    }
    if saw_package_json {
        technologies.push(DetectedTechnology::Node);
    }
    if saw_pyproject {
        technologies.push(DetectedTechnology::Python);
    }

    if technologies.is_empty() {
        // Basic heuristic based on file extensions if no primary marker found
        let rust_count = file_counts.get("rs").unwrap_or(&0);
        let node_count = file_counts.get("js").unwrap_or(&0) + file_counts.get("ts").unwrap_or(&0);
        let python_count = file_counts.get("py").unwrap_or(&0);
        
        if rust_count > &node_count && rust_count > python_count {
            technologies.push(DetectedTechnology::Rust);
        } else if &node_count > rust_count && &node_count > python_count {
            technologies.push(DetectedTechnology::Node);
        } else if python_count > rust_count && python_count > &node_count {
             technologies.push(DetectedTechnology::Python);
        } else {
            technologies.push(DetectedTechnology::Unknown);
        }
    }

    Ok(ProjectContext {
        root_dir: root_path,
        technologies,
        file_counts,
    })
}

/// Helper function to filter out hidden files/dirs and common ignored dirs.
fn is_hidden_or_ignored(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.starts_with('.')
                || s == "node_modules"
                || s == "target"
                || s == "dist"
                || s == "build"
                || s == "__pycache__"
                || s == "venv"
                || s == ".venv"
                // Add more common ignored dirs
        })
        .unwrap_or(false)
}

// Add appropriate Error variants in error.rs
// pub enum Error {
//     ...
//     #[error("Project path is not a directory: {0}")]
//     ProjectPathNotDirectory(PathBuf),
//     ...
// } 