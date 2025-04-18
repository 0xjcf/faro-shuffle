# V1 Task Analysis Engine Implementation

## Phase Overview (V1)
This phase focuses on implementing the **minimal viable** core analysis engine for faro-shuffle V1. It will parse a simple task description from a Markdown file, integrate **only with a local Ollama instance**, and generate a basic complexity score with a brief rationale.

*Note: Subtask generation, multi-LLM support, advanced scoring, and JSON I/O are deferred post-V1.*

## V1 Goals
- Develop basic Markdown task parsing
- Implement LLM prompting strategy for complexity analysis (score + rationale) using Ollama
- Create fundamental data structures for V1 task representation
- Build the core analysis function call

## V1 Implementation Tasks

### 1. Task Parser Development (V1)
- Create structures to represent a simple task input (e.g., description text)
- Implement basic parsing logic for reading task description from a Markdown file
- Add validation for required input (presence of text)

### 2. LLM Integration (V1 - Ollama Only)
- Implement client for connecting to a local Ollama instance (e.g., using `reqwest`)
- Design V1 prompt template specifically for complexity scoring (score 1-10, brief rationale) based on input text
- Build response parsing logic to extract score and rationale from Ollama's response
- Handle basic Ollama API errors (e.g., connection refused)

### 3. Complexity Scoring (V1)
- Implement the core function that takes task text, sends it to Ollama via the integration layer, and returns the score/rationale
- Add basic validation for the score received from Ollama (e.g., is it a number?)
- Structure the output (score, rationale) for the CLI phase to consume

## V1 Success Criteria
- Can parse task description text from a specified Markdown file
- Successfully connects to a running local Ollama instance and sends the V1 prompt
- Receives and parses a complexity score (e.g., number) and rationale (text) from Ollama
- Returns the score and rationale in a structured way for the next phase

## V1 Dependencies
- Rust toolchain
- Access to a running local Ollama instance
- `reqwest` or similar HTTP client crate

## V1 Technical Considerations
- Ensure robust error handling for file reading and Ollama communication
- Design for clear separation between parsing and analysis components
- Implement basic logging for Ollama requests/responses 