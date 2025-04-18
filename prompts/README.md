# faro-shuffle Prompts Directory

This directory contains all prompts, templates, and references used in the faro-shuffle project. The structure is organized to make it easy to find specific types of documents.

## Directory Structure

```
prompts/
├── analysis/         # Task analysis templates and complexity assessment guides
├── decomposition/    # Task decomposition strategies and templates
├── estimation/       # Effort estimation frameworks and references
├── phases/           # Implementation phase-specific prompts
│   ├── 01-core/      # Core Analysis Engine implementation
│   ├── 02-cli/       # Command-Line Interface implementation
│   ├── 03-output/    # Output Formatters implementation
│   ├── 04-context/   # Project Context Parser implementation
│   ├── 05-mcp/       # MCP Module implementation
│   └── 06-advanced/  # Advanced Features implementation
├── project/          # Project management and workflow documents
├── templates/        # Input and output templates
└── workflows/        # Process documentation and workflows
```

## Key Files

### Project Management

- `project/PROJECT_WORKFLOW.md` - Development workflow guidelines and templates
- `project/PROGRESS.md` - Phase implementation progress tracking
- `project/PHASE_ASSESSMENT.md` - Assessment of phase readiness

### Analysis and Scoring

- `analysis/task_analysis_template.md` - Standard template for analyzing tasks
- `analysis/complexity_assessment_guide.md` - Guide for evaluating task complexity
- `analysis/complexity_scoring_guide.md` - Detailed scoring criteria for task complexity

### Task Decomposition

- `decomposition/task_decomposition_guide.md` - Comprehensive guide for breaking down tasks
- `decomposition/task_decomposition_strategy.md` - Strategic approaches to task decomposition

### Estimation and Planning

- `estimation/effort_estimation_framework.md` - Framework for estimating task effort
- `estimation/dependency_analysis_template.md` - Template for mapping task dependencies
- `estimation/risk_assessment_template.md` - Template for evaluating project risks

### Implementation Phases

Each phase directory contains detailed prompts for implementing that phase:

- `phases/01-core/task-analysis-engine.md` - Core analysis engine implementation
- `phases/02-cli/command-line-interface.md` - CLI implementation
- `phases/03-output/output-formatters.md` - Output formatters implementation
- `phases/04-context/project-context-parser.md` - Project context parser implementation
- `phases/05-mcp/mcp-module-implementation.md` - MCP module implementation
- `phases/06-advanced/advanced-features.md` - Advanced features implementation

## Using These Prompts

1. Start with the project management files to understand the overall workflow
2. Review the phase-specific prompts for implementation details
3. Use the templates and guides during development for consistent approaches
4. Reference the analysis and estimation frameworks when evaluating tasks

For adding new prompts, please follow the established directory structure and naming conventions. 