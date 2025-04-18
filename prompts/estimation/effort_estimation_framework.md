# Effort Estimation Framework

## Purpose
This framework provides a structured approach to estimate effort for tasks after their complexity has been assessed, enabling more accurate planning, resource allocation, and timeline projections.

## Estimation Foundations

### Estimation Units
- **Time-Based**: Hours, days, weeks
- **Complexity-Based**: Story points, T-shirt sizes (S/M/L/XL)
- **Effort-Based**: Person-hours, person-days

### Team Velocity Calibration
Before applying estimation, establish baseline velocity:
1. Review historical data of similar completed tasks
2. Calculate average completion time per complexity tier
3. Adjust for current team composition and context

## Estimation Process

### 1. Baseline Estimation

| Complexity Tier | Baseline Range (Story Points) | Typical Time Range (Team-days) |
|-----------------|-------------------------------|--------------------------------|
| Tier 1 - Simple | 1-3 points | 0.5-2 days |
| Tier 2 - Moderate | 3-8 points | 2-5 days |
| Tier 3 - Complex | 8-13 points | 5-10 days |
| Tier 4 - Very Complex | 13-21+ points | 10-20+ days |

### 2. Adjustment Factors

Apply multipliers based on:

| Factor | Description | Multiplier Range |
|--------|-------------|------------------|
| Team Experience | Team's familiarity with similar tasks | 0.7 (expert) - 1.5 (novice) |
| Technical Debt | Impact of existing codebase quality | 1.0 (clean) - 1.7 (significant debt) |
| Dependencies | Number and nature of external dependencies | 1.0 (none) - 1.6 (many critical) |
| Clarity | Requirement completeness and stability | 1.0 (crystal clear) - 1.8 (highly ambiguous) |
| Testing Needs | Extent of testing required | 1.0 (minimal) - 1.5 (extensive) |

#### Calculation Example
```
Base Estimate: 5 days (Tier 2 task)
Adjustments:
- Team Experience: 1.2 (moderate expertise)
- Technical Debt: 1.3 (some debt in affected area)
- Dependencies: 1.1 (few dependencies)
- Clarity: 1.0 (clear requirements)
- Testing: 1.2 (moderate testing needs)

Calculation: 5 × 1.2 × 1.3 × 1.1 × 1.0 × 1.2 = 10.3 days
```

### 3. Confidence Levels

Express estimates with confidence ranges:

| Complexity | Confidence Range |
|------------|------------------|
| Tier 1 | ±10-20% |
| Tier 2 | ±20-30% |
| Tier 3 | ±30-50% |
| Tier 4 | ±50-100% |

**Example**: A Tier 3 task estimated at 8 days with ±40% confidence would be presented as 5-11 days.

## Decomposition-Based Estimation

For complex tasks (Tier 3-4), use decomposition for more accurate estimates:

1. Break down task into sub-tasks
2. Estimate each sub-task
3. Account for integration time between sub-tasks:
   - Add 10-20% for Tier 3 tasks
   - Add 20-30% for Tier 4 tasks
4. Sum the estimates plus integration time

## Special Scenarios

### Research/Discovery Tasks
For tasks with high uncertainty:
1. Time-box initial research phase (1-5 days based on complexity)
2. Re-estimate after research phase
3. Consider using techniques like spike solutions

### Ongoing Maintenance
For support or maintenance activities:
1. Use historical data to establish average effort per incident/request
2. Multiply by expected volume
3. Add buffer for unexpected issues (15-30%)

## Team-Based Estimation Techniques

### Planning Poker
1. Each estimator has cards with values (1, 2, 3, 5, 8, 13, 21, etc.)
2. Task is described to team
3. Each member silently selects an estimate card
4. Cards are revealed simultaneously
5. Discuss significant variances
6. Re-estimate until consensus is reached

### Three-Point Estimation
1. For each task, estimate:
   - Optimistic (best case scenario)
   - Most likely (typical scenario)
   - Pessimistic (worst case scenario)
2. Calculate weighted average:
   (Optimistic + 4×Most Likely + Pessimistic) ÷ 6

## Continuous Improvement

### Estimation Accuracy Tracking
1. Record initial estimates and actual time spent
2. Calculate accuracy ratio: Actual/Estimated
3. Identify patterns in under/over-estimation
4. Adjust estimation process based on findings

### Periodic Calibration
1. Review estimation accuracy quarterly
2. Adjust baseline estimates and multipliers
3. Update team velocity measurements
4. Refine estimation technique based on project phase

## Communication Guidelines

When presenting estimates:
1. Always include confidence levels or ranges
2. Explain major adjustment factors applied
3. Note key assumptions made
4. For high-complexity tasks, provide both overall estimate and breakdown
5. Highlight significant risks that could impact the estimate

## Estimation for New/Novel Projects

When limited or no historical data exists:
1. Use expert judgment as starting point
2. Break down into smallest possible components
3. Estimate components individually
4. Apply larger uncertainty ranges (add 50-100%)
5. Plan for early re-estimation after initial work phase 