# Task Effort Estimation Reference

## Estimation Units

### Time-Based Units
- Hours: For granular tasks (<8 hours)
- Days: For medium-sized tasks (0.5-10 days)
- Weeks: For larger initiatives (2+ weeks)

### Complexity-Based (Story Points)
- Fibonacci sequence (1, 2, 3, 5, 8, 13, 21)
- Powers of 2 (1, 2, 4, 8, 16, 32)
- T-shirt sizes (XS, S, M, L, XL, XXL)

### Effort-Based
- Person-days: Effort required by one person working full-time
- Function points: Measuring functional complexity (for enterprise systems)

## Complexity Tiers

| Tier | Complexity | Story Points | Typical Time Range | Examples |
|------|------------|--------------|---------------------|----------|
| 1 | Trivial | 1-3 | 0.5-2 days | Bug fix, simple UI adjustment, documentation update |
| 2 | Simple | 3-8 | 2-5 days | New simple feature, integration with well-documented API, adding basic component |
| 3 | Moderate | 8-13 | 5-10 days | Complex feature, multi-step workflow, performance optimization |
| 4 | Complex | 13-21+ | 10-20+ days | System architecture changes, complex integrations, security overhaul |

## Adjustment Factors

| Factor | Description | Multiplier Range |
|--------|-------------|------------------|
| Team Experience | Familiarity with technology/domain | 0.7-1.5x |
| Technical Debt | Quality of existing codebase | 1.0-1.7x |
| Dependencies | External dependencies or approvals | 1.0-1.5x |
| Clarity | Requirements certainty | 1.0-1.8x |
| Testing Needs | Comprehensive testing requirements | 1.0-1.3x |

## Confidence Levels

| Level | Description | Margin of Error | When to Use |
|-------|-------------|-----------------|-------------|
| Low | Rough order of magnitude | ±50-100% | Early exploration, novel technology |
| Medium | Informed estimate | ±25-50% | After initial investigation, some unknowns remain |
| High | Confident estimate | ±10-25% | Well-understood tasks, team has done similar work |
| Very High | Precise estimate | ±5-10% | Well-defined, routine tasks with historical data |

## Estimation Process

### 1. Task Analysis
- Review requirements documentation
- Identify scope boundaries and dependencies
- Clarify ambiguities with stakeholders
- Break down complex tasks (see Task Decomposition Guide)

### 2. Base Estimation
1. Assign complexity tier (1-4)
2. Select base estimate from typical range
3. Document assumptions

### 3. Apply Adjustment Factors
1. Evaluate each factor's relevance
2. Assign appropriate multiplier
3. Apply combined adjustment

Example:
```
Base estimate: 5 days
Adjustments:
- Team Experience: New technology (1.3x)
- Dependencies: External API approval required (1.2x)
Adjusted estimate: 5 × 1.3 × 1.2 = 7.8 days ≈ 8 days
```

### 4. Assign Confidence Level
- Consider novelty and uncertainty
- Note confidence level alongside estimate
- For low confidence, consider range-based estimates

### 5. Validate
- Compare with similar historical tasks
- Get peer review from experienced team members
- Consider breaking down further if confidence is low

## Special Scenarios

### Research/Discovery Tasks
- Use timeboxed approach
- Estimate initial research phase only
- Plan re-estimation point after discovery
- Consider using confidence levels as exit criteria

### Ongoing Maintenance
- Use historical averages
- Consider percentage-based allocation (e.g., 10% of development time)
- Use rolling average of recent similar tasks

## Team-Based Estimation Techniques

### Planning Poker
1. Each estimator receives a deck of cards with values
2. Facilitator presents item to be estimated
3. Estimators privately select cards
4. Cards are revealed simultaneously
5. Discuss significant variations
6. Re-estimate until consensus is reached

### Three-Point Estimation
1. Generate three estimates:
   - Optimistic (O): Best-case scenario
   - Most likely (M): Expected scenario
   - Pessimistic (P): Worst-case scenario
2. Calculate weighted average: (O + 4M + P) ÷ 6
3. Calculate standard deviation: (P - O) ÷ 6

### Affinity Estimation
1. Write each item on a card
2. Place first item on table as reference
3. Place each subsequent item relative to others
4. Discuss and adjust positions
5. Assign final estimates based on positions

## Continuous Improvement

### Tracking Accuracy
- Record actual time spent
- Calculate estimation accuracy: (Actual ÷ Estimated) × 100%
- Target 80-120% accuracy range
- Analyze trends by task type, estimator, and project

### Calibration
- Review estimates quarterly
- Identify patterns in under/overestimation
- Adjust base estimates or adjustment factors
- Update historical references

## Communication Guidelines

### Presenting Estimates
- Always include confidence level
- For low confidence, provide ranges
- Explain key assumptions
- Highlight major risk factors

### Discussing with Stakeholders
- Explain estimation process
- Focus on relative sizing over absolute timing
- Discuss tradeoffs between scope and certainty
- Document agreed adjustments

## Estimation for New/Novel Projects

### When Historical Data is Limited
1. Start with high-level estimates
2. Use analogous estimating (compare to similar projects)
3. Create small proof-of-concept implementations
4. Re-estimate frequently as information emerges
5. Use wider confidence intervals

### Building Team Estimation Capability
1. Retrospectively estimate completed tasks
2. Compare estimates to actuals
3. Discuss reasons for discrepancies
4. Build team reference database
5. Conduct regular calibration exercises 