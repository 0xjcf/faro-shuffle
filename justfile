# faro-shuffle Project Tasks

# Bootstrap the core library using standard cargo
bootstrap-core:
    @echo "📦 Bootstrapping faro-shuffle-core library using cargo..."
    rm -rf ./core
    cargo new --lib core

# Bootstrap the CLI application using standard cargo
bootstrap-cli:
    @echo "📦 Bootstrapping faro-shuffle CLI application using cargo..."
    rm -rf ./cli
    cargo new cli

# Bootstrap both core and CLI components for V1
bootstrap: bootstrap-core bootstrap-cli
    @echo "✅ V1 Core and CLI bootstrapped using cargo new."

# Perform initial project setup (Bootstrap + Environment)
setup: bootstrap
    @echo "⚙️ Setting up environment..."
    @if [ ! -f .envrc ]; then echo "use rust\nlayout rust" > .envrc; fi
    @echo "ⓘ Bootstrap finished. Run 'direnv allow .' manually if needed."
    @echo "   You MUST add the core library dependency manually to ./cli/Cargo.toml:"
    @echo "   faro_shuffle_core = { path = \"../core\" }"

# Default task lists available tasks
default:
    @just --list

# --- Project Management Tasks (Post-Setup) ---

# Build the CLI application (which includes the core library)
build:
    @echo "📦 Building faro-shuffle CLI..."
    cd cli && cargo build

# Build in release mode
build-release:
    @echo "📦 Building faro-shuffle CLI (Release)..."
    cd cli && cargo build --release

# Run the CLI application with forwarded arguments
run *args:
    @echo "🚀 Running faro-shuffle CLI..."
    # Ensure it's built first
    just build
    ./cli/target/debug/faro-shuffle {{args}}

# Run tests for both core and cli crates
test:
    @echo "🧪 Running tests..."
    # Assumes core dependency is added to cli/Cargo.toml
    cargo test --all

# Run clippy checks
check:
    @echo " Linting with clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
    @echo " Formatting code..."
    cargo fmt --all

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Build the application in debug mode
build-debug:
    cargo build

# Build the application in release mode
release:
    cargo build --release

# Run the application with arguments
run *ARGS:
    cargo run -- {{ARGS}}

# Analyze a task file (shortcut)
analyze TASK_FILE:
    cargo run -- analyze {{TASK_FILE}}

# Install the application locally
install:
    cargo build --release
    @echo "Installing faro-shuffle to ~/.cargo/bin"
    cp target/release/faro_shuffle ~/.cargo/bin/faro-shuffle

# Check if Ollama is running
check-ollama:
    @echo "Checking if Ollama is running..."
    @if curl -s http://localhost:11434/api/version > /dev/null; then \
        echo "✅ Ollama is running"; \
    else \
        echo "❌ Ollama is not running. Please start it with 'ollama serve'"; \
        exit 1; \
    fi

# Pull the llama3 model if not already available
pull-llama3:
    @echo "Checking for llama3 model..."
    @if ollama list | grep -q "llama3"; then \
        echo "✅ llama3 model is already pulled"; \
    else \
        echo "Pulling llama3 model..."; \
        ollama pull llama3; \
    fi

# Run a quick demo with example tasks
demo: setup
    @echo "Running demo with simple task..."
    just analyze examples/task2.md
    @echo "\nRunning demo with complex task..."
    just analyze examples/task1.md

# Create a new task file
new-task FILENAME:
    @echo "# Task Description\n\nDescribe your task here. Include details like:\n- Feature requirements\n- Constraints\n- Dependencies\n- Expected outcome" > {{FILENAME}}
    @echo "Created new task file: {{FILENAME}}" 