use clap::{Parser, Subcommand};
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

#[derive(Subcommand)]
enum Commands {
    /// Analyze a task description file for complexity
    Analyze {
        /// Path to the task description markdown file
        #[arg(value_name = "FILE")]
        file: PathBuf,
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
        Commands::Analyze { file } => {
            println!("Analyzing task file: {}", file.display());
            
            // Ensure the file exists
            if !file.exists() {
                eprintln!("Error: File not found: {}", file.display());
                std::process::exit(1);
            }
            
            // Call the core library to analyze the task
            let result = core::analyze_task_file(&file).await
                .map_err(|e| color_eyre::eyre::eyre!("Analysis error: {}", e))?;
            
            // Display results
            println!("\n📊 Task Complexity Analysis");
            println!("---------------------------");
            println!("🔢 Complexity Score: {} / 10", result.score);
            println!("💡 Rationale: {}", result.rationale);
            println!("\n📝 Recommendation:");
            
            match result.score {
                1..=3 => println!("  This is a relatively simple task. Good for a quick win!"),
                4..=6 => println!("  This is a moderately complex task. Plan for a reasonable effort."),
                7..=8 => println!("  This is a complex task. Allow for unexpected challenges."),
                9..=10 => println!("  This is an extremely complex task. Consider breaking it down further."),
                _ => unreachable!(), // We validate in the core library
            }
            
            // Coming soon teaser for pro features
            println!("\n🔮 Coming soon: faro-shuffle Pro");
            println!("Get full subtask decomposition, project-wide scoring, and exportable reports for teams.");
            println!("Join the early access list: https://example.com/waitlist");
        }
    }
    
    Ok(())
}
