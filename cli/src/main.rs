use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use faro_shuffle_core as core;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "faro-shuffle")]
#[command(author = "Jose Flores")]
#[command(version = "0.1.0")]
#[command(about = "Know instantly if your project is a 2-hour tweak or a 2-week trap—before you waste time.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a task description file for complexity
    Analyze {
        /// Path to the task description markdown file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Optional path to the project directory for context analysis
        #[arg(long)]
        project_dir: Option<PathBuf>,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup error handling
    color_eyre::install()?;
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Handle commands
    match cli.command {
        Commands::Analyze { file, project_dir, format } => {
            println!("Analyzing task file: {}", file.display());
            if let Some(dir) = &project_dir {
                println!("Using project directory for context: {}", dir.display());
            }
            
            // Ensure the file exists
            if !file.exists() {
                eprintln!("Error: File not found: {}", file.display());
                std::process::exit(1);
            }
            
            // Call the core library to analyze the task
            let result = core::analyze_task_file(&file, project_dir.as_ref()).await
                .map_err(|e| color_eyre::eyre::eyre!("Analysis error: {}", e))?;
            
            // Display results based on format
            match format {
                OutputFormat::Text => {
                    // Current text output logic, now with subtasks
                    println!("\n📊 Task Complexity Analysis");
                    println!("---------------------------");
                    println!("🔢 Complexity Score: {} / 10", result.score);
                    println!("💡 Rationale: {}", result.rationale);
                    
                    if !result.subtasks.is_empty() {
                        println!("\n📋 Suggested Subtasks:");
                        for subtask in &result.subtasks {
                            println!("  - {}. {}", subtask.id, subtask.title);
                        }
                    }

                    println!("\n📝 Recommendation:");
                    match result.score {
                        1..=3 => println!("  This is a relatively simple task. Good for a quick win!"),
                        4..=6 => println!("  This is a moderately complex task. Plan for a reasonable effort."),
                        7..=8 => println!("  This is a complex task. Allow for unexpected challenges."),
                        9..=10 => println!("  This is an extremely complex task. Consider breaking it down further."),
                        _ => unreachable!(), 
                    }
                    println!("\n🔮 Coming soon: faro-shuffle Pro");
                    println!("Get full subtask decomposition, project-wide scoring, and exportable reports for teams.");
                    println!("Join the early access list: https://example.com/waitlist");
                }
                OutputFormat::Json => {
                    // JSON output now includes subtasks automatically due to Serialize derive
                    let json_output = serde_json::to_string_pretty(&result)
                        .map_err(|e| color_eyre::eyre::eyre!("Failed to serialize result to JSON: {}", e))?;
                    println!("{}", json_output);
                }
                OutputFormat::Markdown => {
                    // Markdown output logic, now with subtasks
                    println!("# 📊 Task Complexity Analysis");
                    println!("- **Score**: {} / 10", result.score);
                    println!("- **Rationale**: {}", result.rationale);
                    
                    if !result.subtasks.is_empty() {
                        println!("
## 📋 Suggested Subtasks");
                        for subtask in &result.subtasks {
                            println!("- {}. {}", subtask.id, subtask.title);
                        }
                    }

                    println!("
## Recommendation");
                    let recommendation = match result.score {
                        1..=3 => "This is a relatively simple task. Good for a quick win!",
                        4..=6 => "This is a moderately complex task. Plan for a reasonable effort.",
                        7..=8 => "This is a complex task. Allow for unexpected challenges.",
                        9..=10 => "This is an extremely complex task. Consider breaking it down further.",
                        _ => "Unknown complexity level.", 
                    };
                    println!("{}", recommendation);
                }
            }
        }
    }
    
    Ok(())
}
