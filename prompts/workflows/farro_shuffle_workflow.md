# Task Expansion Workflow

This document outlines the process for analyzing and expanding tasks using the faro-shuffle system.

## Workflow Overview

1. **Input Task Collection** - Gather task details from project management system
2. **Task Analysis** - Analyze complexity and dependencies
3. **Task Expansion** - Break down into well-defined subtasks
4. **Report Generation** - Create structured output with visualizations
5. **Integration** - Push expanded tasks back to project management system

## Detailed Steps

### 1. Input Task Collection

Tasks should be submitted with the following minimum information:
- Task ID
- Title
- Description
- Original estimate (if available)
- Project context

Use the `task_input_template.md` to ensure all necessary information is provided.

### 2. Task Analysis

The AI will analyze the task using the following criteria:
- Technical complexity
- Domain knowledge requirements
- Cross-cutting concerns
- Integration points
- Unknown factors

This analysis uses the structure in `task_analysis_template.md`.

### 3. Task Expansion

Based on the analysis, the task will be broken down into:
- Logical subtasks with clear boundaries
- Specific acceptance criteria for each subtask
- Dependencies between subtasks
- Individual effort estimates

### 4. Report Generation

The system generates a comprehensive report following `task_report_template.md`, including:
- Complexity assessment
- Subtask breakdown
- Risk identification
- Dependency visualization
- Total effort calculation

### 5. Integration

The expanded tasks can be integrated with:
- Jira
- GitHub Issues
- Azure DevOps
- Linear
- Notion

Use the appropriate integration script from the `/integrations` directory.

## Usage Examples

### Command Line Usage

```bash
just expand-task TASK_ID
```

### API Usage

```bash
curl -X POST https://api.taskexpander.dev/analyze \
  -H "Content-Type: application/json" \
  -d '{"task_id": "PROJ-123", "description": "Implement user authentication"}'
```

## Tips for Effective Task Expansion

1. Provide detailed context about the project and technical environment
2. Include constraints and non-functional requirements
3. Specify any known dependencies or integration points
4. Be explicit about quality expectations and acceptance criteria
5. Indicate which aspects of the task are most uncertain 