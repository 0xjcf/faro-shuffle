# Task Decomposition Strategy Guide

## Purpose
This guide helps identify the most effective strategy for breaking down complex tasks into manageable components, ensuring clarity, efficiency, and accuracy in project planning and execution.

## When to Use Task Decomposition
- When a task's estimated effort exceeds 1-2 days of work
- When a task involves multiple technical domains or skill sets
- When a task has complex dependencies or integration points
- When planning for parallel work across team members
- When task complexity is high or scope is ambiguous

## Primary Decomposition Strategies

### Functional Decomposition
**Best for:** Tasks that involve distinct functional components or modules

**Decomposition approach:**
- Break down by discrete functions/features
- Organize by user-facing capabilities
- Group related functionality together

**Example application:**
- Frontend feature development
- User flow implementation
- API feature development

**Key indicators:**
- Task involves multiple distinct user interactions
- Components can function independently
- Clear functional boundaries exist

### Technical Layer Decomposition
**Best for:** Tasks that span multiple technical layers

**Decomposition approach:**
- Separate by technical tier (frontend, backend, database)
- Divide by architectural layers
- Isolate infrastructure from business logic

**Example application:**
- Full-stack feature implementation
- System architecture changes
- Performance optimization work

**Key indicators:**
- Task crosses technical boundaries
- Different expertise required per component
- Testing needs vary by layer

### Sequential Workflow Decomposition
**Best for:** Tasks with clear procedural steps or dependencies

**Decomposition approach:**
- Organize by chronological sequence
- Define clear handoff points
- Structure based on natural dependencies

**Example application:**
- Data migration projects
- Multi-phase rollouts
- Process automation

**Key indicators:**
- Linear progression of work
- Output of one step feeds into another
- Natural blocking dependencies exist

### Domain-Based Decomposition
**Best for:** Tasks that span multiple business domains

**Decomposition approach:**
- Divide by business domain or context
- Separate by data ownership boundaries
- Group by domain expert requirements

**Example application:**
- Enterprise applications
- Cross-department features
- Systems with complex business rules

**Key indicators:**
- Multiple stakeholders from different domains
- Different business rule sets apply
- Domain knowledge separation is clear

### Risk-Based Decomposition
**Best for:** Tasks with significant unknowns or high-risk components

**Decomposition approach:**
- Isolate high-risk elements
- Prioritize uncertainty reduction
- Create safe fallback options

**Example application:**
- Integration with external systems
- Implementation of new technologies
- Performance-critical features

**Key indicators:**
- Significant technical unknowns exist
- Failure in one area shouldn't block all progress
- Research/prototyping needed for some elements

## Selection Matrix

| Factor | Functional | Technical | Sequential | Domain | Risk-Based |
|--------|------------|-----------|------------|--------|------------|
| Multiple features | ✓✓✓ | ✓ | ✓ | ✓✓ | ✓ |
| Cross-layer work | ✓ | ✓✓✓ | ✓✓ | ✓ | ✓ |
| Sequential dependencies | ✓ | ✓ | ✓✓✓ | ✓ | ✓ |
| Multiple domains | ✓ | ✓ | ✓ | ✓✓✓ | ✓ |
| Technical uncertainty | ✓ | ✓✓ | ✓ | ✓ | ✓✓✓ |
| Parallel team work | ✓✓ | ✓✓ | ✓ | ✓✓ | ✓✓ |
| Skills specialization | ✓ | ✓✓✓ | ✓ | ✓✓ | ✓✓ |

## Hybrid Approach Guidelines

Often, the most effective decomposition involves a combination of strategies:

1. **Start with primary strategy** based on dominant task characteristics
2. **Apply secondary strategy** within primary components if needed
3. **Consider team structure** and optimize for parallel work where possible
4. **Evaluate critical path** to ensure decomposition doesn't extend timeline
5. **Verify integration points** are clearly defined between components

## Implementation Guidelines

For each subtask created through decomposition:

1. **Clear scope definition** with explicit boundaries
2. **Independent acceptance criteria** where possible
3. **Explicit dependencies** and integration requirements
4. **Right-sized estimation** (ideally 1-8 hours per subtask)
5. **Assigned ownership** with clear responsibility

## Common Pitfalls

- **Over-decomposition:** Creating too many tiny tasks increases overhead
- **Under-decomposition:** Leaving tasks too large makes progress tracking difficult
- **Ignoring dependencies:** Failing to identify critical dependencies leads to blocking
- **Artificial separation:** Breaking logically connected work can increase integration issues
- **Improper sequencing:** Not considering the optimal order of subtasks 