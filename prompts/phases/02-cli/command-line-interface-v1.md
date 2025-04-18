# V1 Command-Line Interface Implementation

## Phase Overview (V1)
This phase focuses on developing the **minimal viable** command-line interface (CLI) for faro-shuffle V1. It will allow users to analyze a task defined in a file using a simple terminal command, building upon the V1 core analysis engine.

*Note: Task expansion commands, configuration management, multiple I/O formats, and advanced UX features are deferred post-V1.*

## V1 Goals
- Create a basic CLI for V1 task analysis (`faro-shuffle analyze <file>`)
- Implement argument parsing for the input file
- Integrate with the V1 Core Analysis Engine (Phase 01)
- Display the analysis results (score, rationale) to the console
- Provide helpful basic error messages

## V1 Implementation Tasks

### 1. CLI Framework Setup (V1)
- Select and implement Rust CLI framework (`clap` recommended)
- Design the basic command structure: `faro-shuffle analyze <TASK_FILE>`
- Implement basic help text for the `analyze` command

### 2. Command Implementation (V1)
- Implement the `analyze` command logic:
    - Parse the `<TASK_FILE>` argument.
    - Read the content of the specified file.
    - Call the V1 Core Analysis engine (Phase 01) with the file content.
    - Receive the score and rationale result.
    - Pass the result to the basic output formatter (Phase 03 V1).

### 3. Input/Output Management (V1)
- Support reading the task description **only from a specified file**.
- Implement output **only to stdout** (console) via Phase 03 V1.

### 4. Error Handling and User Experience (V1)
- Implement basic user-friendly error messages for:
    - File not found or unreadable.
    - Errors returned from the Core Analysis engine (e.g., Ollama connection issue).
- Ensure proper exit codes for success and failure.

## V1 Success Criteria
- CLI can be invoked with `faro-shuffle analyze <path/to/task.md>`
- Successfully reads the task file and passes content to the V1 Core Engine
- Receives score/rationale from the Core Engine
- Outputs the score and rationale clearly to the console (via Phase 03 V1)
- Provides helpful error messages for common issues (file errors, core engine errors)
- Exits with appropriate status codes

## V1 Dependencies
- Phase 01 V1: Core Analysis Engine
- Rust CLI framework (`clap`)
- Basic Console Output capability (Phase 03 V1)

## V1 Technical Considerations
- Focus on a simple and clear command structure for V1
- Ensure good basic error handling with clear messages
- Follow standard CLI conventions for argument parsing 