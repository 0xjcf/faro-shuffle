# Quick Start Guide

This guide will help you get `faro-shuffle` up and running in under 5 minutes.

## Prerequisites

- **Rust toolchain**: Install via [rustup.rs](https://rustup.rs/)
- **Ollama**: Local AI model runner - [installation instructions](https://ollama.ai/download)

## 1. Installation

```bash
# Clone the repository
git clone https://github.com/your-username/faro-shuffle.git
cd faro-shuffle

# Build and install the CLI
just install  # This will build and copy to ~/.cargo/bin
```

Or build without installing:

```bash
cargo build --release
```

## 2. Setup Ollama

```bash
# Start Ollama service
ollama serve

# In a new terminal, pull the llama3 model
ollama pull llama3
```

## 3. Check Your Setup

```bash
# Verify everything is working
just setup
```

This checks that Ollama is running and the right model is available.

## 4. Analyze Your First Task

```bash
# Create a task description file
just new-task my-task.md
```

Edit the file to describe your task, then analyze it:

```bash
just analyze my-task.md
```

## 5. Try the Demo

```bash
# Run demo with example tasks
just demo
```

## Command Reference

| Command | Description |
|---------|-------------|
| `faro-shuffle analyze <file>` | Analyze a task file |
| `just analyze <file>` | Shortcut for the above |
| `just setup` | Check prerequisites |
| `just new-task <file>` | Create a new task file |
| `just demo` | Run demo with example tasks |

## Common Issues

- **Connection refused**: Make sure Ollama is running with `ollama serve`
- **Model not found**: Run `ollama pull llama3` to download the model

## Next Steps

Check out the README.md for more details on how to use faro-shuffle effectively or contribute to its development. 