# faro-shuffle Project Workflow Guide

This document provides guidelines, templates, and reference information for the faro-shuffle development process. It complements the V1 phase tracking in `V1_PROGRESS.md` and the assessments in `PHASE_ASSESSMENT.md`.

## V1 Phase Dependency Diagram

The following diagram illustrates the dependencies between the V1 project phases:

```mermaid
graph TD
    P1[Phase 1: Core Analysis (V1)] --> P2[Phase 2: CLI Interface (V1)];
    P1 --> P3[Phase 3: Basic Output (V1)];
    P2 --> P3;

    subgraph "V1 Scope"
        P1
        P2
        P3
    end
```

*(Note: Full project dependencies including post-V1 phases like Project Context (P4), MCP (P5), and Advanced Features (P6) are omitted here for V1 clarity)*

## Pre-Flight Checklist (V1)

Before beginning work on Phase 1, verify that the following prerequisites are met:

- [ ] `dev-setup` repository is cloned and available at `../dev-setup` or configured path
- [ ] `just` is installed and functional (`just --version`)
- [ ] `direnv` is installed and configured (`direnv --version`)
- [ ] Rust toolchain is installed (`rustc --version`, `cargo --version`)
- [ ] Cargo extensions installed (`cargo-edit`, `cargo-nextest`)
- [ ] LLM API access configured (Ollama setup verified)
- [ ] Development environment properly configured for Rust
- [ ] Access to simple test tasks (e.g., Markdown files) for analysis validation
- [ ] Documentation templates available

## Development Workflow

### Branch and Commit Strategy

- Create feature branches named `phase-XX-brief-description` (e.g., `phase-01-llm-client`)
- Commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/) style:
  - `feat(core): implement task complexity analyzer`
  - `fix(cli): correct argument parsing bug`
  - `docs(api): add documentation for public functions`
  - `chore(deps): update dependencies`
- Create PRs with references to the phase number being implemented
- Each phase should have its own PR(s) unless there's a compelling reason to combine phases

### Code Reviews

- PRs should be reviewed by at least one other team member
- Focus on:
  - Meeting the success criteria defined in the phase prompt
  - Code quality and maintainability
  - Test coverage for critical functionality
  - Documentation for public APIs and user-facing components

### Testing Strategy

- Unit tests for all core functionality
- Integration tests for components interactions
- End-to-end tests for CLI and MCP interfaces
- Test against real project data for validation
- Performance testing for large project analysis

## Effort Estimation Guidelines

When adding effort estimates to the `V1_PROGRESS.md` file, use the following guidelines:

- **X-Small (1-8 hours)**: Minimal changes, configuration, or simple output formatting.
- **Small (8-16 hours)**: Simple integration or configuration with minimal new code
- **Medium (16-32 hours)**: Features requiring moderate implementation or integration with external systems
- **Large (32-80 hours)**: Complex features requiring significant implementation or research
- **X-Large (80+ hours)**: Major system components requiring extensive development and testing

These estimates assume the use of AI pair programming tools (Cursor, Copilot, etc.). Without AI assistance, expect estimates to be approximately 1.5-2x higher.

Factors that may increase complexity:
- LLM integration and prompt engineering
- File system traversal and parsing
- Cross-platform compatibility
- Performance optimization requirements
- Error handling and edge cases
- Limited documentation or examples for the technologies used

## Phase Completion Report Template

When marking a phase complete in `V1_PROGRESS.md`, provide the following information in the Notes column:

```
Completed on YYYY-MM-DD
PR: #XX
Commits: abcd123, efgh456
Deviations: [Any deviations from original plan]
Lessons: [Key lessons learned]
Next: [Recommendations for subsequent phases]
```

Example:
```
Completed on 2023-10-25
PR: #42
Commits: 7a8b9c0, 1d2e3f4
Deviations: Used reqwest instead of ureq for HTTP client
Lessons: LLM prompt formatting is critical for consistent results
Next: Consider implementing retry logic for LLM API calls
```

## Issue Management (V1 Focus)

- Create issues for each V1 phase (01, 02, 03) with a checklist of tasks based on the phase prompt
- Link PRs to issues using GitHub keywords (Fixes #XX, Closes #XX)
- Use labels to categorize issues:
  - `phase-01`, `phase-02`, `phase-03` for phase-specific issues
  - `v1` to denote relevance to the initial release
  - `bug`, `enhancement`, `documentation`, etc. for type
  - `high`, `medium`, `low` for priority

## Output Format Standards

*(Note: The full JSON schema below represents the target for later phases. V1 will only implement basic console text output.)*

The faro-shuffle system should adhere to these output standards:

### JSON Output Schema
```json
{
  "metadata": {
    "projectName": "string",
    "taskId": "string",
    "taskTitle": "string",
    "originalEstimate": "string",
    "analyzedAt": "ISO string",
    "llmModel": "string"
  },
  "analysis": {
    "complexityScore": "number",
    "recommendedSubtasks": "number",
    "riskAreas": ["string"],
    "summary": "string"
  },
  "subtasks": [
    {
      "id": "string",
      "title": "string",
      "description": "string",
      "estimatedHours": "number",
      "complexity": "string",
      "dependencies": ["string"]
    }
  ]
}
```

### LLM Prompt Structure

Prompts to LLM services should follow a consistent structure:

1. **System Context:** Define the role and capabilities
2. **Task Description:** Provide the task to analyze
3. **Project Context:** Include relevant project details
4. **Output Instructions:** Specify the required format
5. **Reasoning Guidance:** Direct the LLM's analytical approach

## Additional Resources

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Just Command Runner Documentation](https://just.systems/man/en/)
- [Rust Documentation Guidelines](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
- ~[MCP Protocol Documentation](https://github.com/your-org/vim-mcp-server)~ *(Relevant post-V1)* 