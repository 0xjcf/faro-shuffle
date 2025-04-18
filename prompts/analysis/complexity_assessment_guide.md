# Complexity Assessment Guide

## Purpose
This guide provides a structured approach to evaluate task complexity across multiple dimensions, enabling more accurate effort estimation, risk assessment, and resource allocation.

## Complexity Dimensions

### Technical Complexity

| Score | Level | Indicators |
|-------|-------|------------|
| 1 | Low | - Well-understood technology<br>- Simple implementations<br>- Established patterns<br>- Minimal integration points |
| 2 | Moderate | - Partial team familiarity with technology<br>- Multiple integration points<br>- Some custom development<br>- Moderate testing requirements |
| 3 | High | - New technology adoption<br>- Complex algorithms/logic<br>- Performance-critical components<br>- Significant integration challenges |
| 4 | Very High | - Experimental technologies<br>- Highly customized solutions<br>- Advanced technical requirements<br>- Many complex integration points |
| 5 | Extreme | - Cutting-edge research/technology<br>- No existing patterns/examples<br>- Extreme performance/scale requirements<br>- Multi-system complex integrations |

**Technical Complexity Questions:**
1. How familiar is the team with the technologies involved?
2. How many integration points exist between components?
3. Are there performance or scalability concerns?
4. How much custom development is required?
5. Are there complex algorithms or data structures involved?

### Domain/Business Complexity

| Score | Level | Indicators |
|-------|-------|------------|
| 1 | Low | - Simple business rules<br>- Well-documented domain<br>- Minimal stakeholders<br>- Clear acceptance criteria |
| 2 | Moderate | - Multiple business rules<br>- Some domain expertise required<br>- Several stakeholders<br>- Some ambiguity in requirements |
| 3 | High | - Complex business logic<br>- Specialized domain knowledge needed<br>- Multiple stakeholder groups<br>- Nuanced requirements |
| 4 | Very High | - Intricate business processes<br>- Deep domain expertise essential<br>- Cross-functional stakeholders<br>- Ambiguous or changing requirements |
| 5 | Extreme | - Highly specialized domain<br>- Regulatory/compliance factors<br>- Multiple competing stakeholder interests<br>- Evolving business context |

**Domain Complexity Questions:**
1. How well-documented are the business rules?
2. How many domain experts need to be consulted?
3. Are there regulatory or compliance considerations?
4. How many stakeholders are involved in requirements/approvals?
5. How much domain-specific knowledge is required?

### Operational Complexity

| Score | Level | Indicators |
|-------|-------|------------|
| 1 | Low | - Simple deployment<br>- Minimal operational changes<br>- Low impact on existing processes<br>- Standard monitoring |
| 2 | Moderate | - Coordinated deployment<br>- Some operational adjustments<br>- Moderate user impact<br>- Custom monitoring needs |
| 3 | High | - Multi-stage deployment<br>- Significant operational changes<br>- Training requirements<br>- Custom operational tooling |
| 4 | Very High | - Complex rollout strategy<br>- New operational processes<br>- Organizational change management<br>- Advanced monitoring solutions |
| 5 | Extreme | - Mission-critical deployment<br>- Major organizational process changes<br>- Extensive training programs<br>- Custom operational infrastructure |

**Operational Complexity Questions:**
1. What deployment challenges might arise?
2. How will existing operations be affected?
3. Is user training or change management required?
4. Are there special monitoring or support requirements?
5. Will operational procedures need updating?

### Uncertainty/Risk Factors

| Score | Level | Indicators |
|-------|-------|------------|
| 1 | Low | - Clear requirements<br>- Proven approach<br>- Minimal dependencies<br>- Low consequence of failure |
| 2 | Moderate | - Some requirement gaps<br>- Partial precedent exists<br>- Few external dependencies<br>- Moderate failure impact |
| 3 | High | - Significant requirement unknowns<br>- Limited precedent<br>- Several external dependencies<br>- Significant failure consequences |
| 4 | Very High | - Major requirement ambiguities<br>- No direct precedent<br>- Critical external dependencies<br>- High-impact failure scenarios |
| 5 | Extreme | - Highly speculative requirements<br>- Groundbreaking approach<br>- Uncontrollable dependencies<br>- Catastrophic failure potential |

**Uncertainty/Risk Questions:**
1. How complete and stable are the requirements?
2. Have similar tasks been completed before?
3. How many external dependencies impact this task?
4. What is the impact if the task fails or is delayed?
5. Are there unknown factors that could emerge?

## Overall Complexity Assessment

### Calculating Complexity Score
1. Rate each dimension from 1-5
2. Calculate weighted score (if needed):
   - Technical Complexity × [weight]
   - Domain Complexity × [weight]
   - Operational Complexity × [weight]
   - Uncertainty/Risk × [weight]
3. Sum the scores to get total complexity rating

### Complexity Tiers

| Total Score | Complexity Tier | Recommended Approach |
|------------|-----------------|----------------------|
| 4-8 | Tier 1 - Simple | Standard estimation and tracking<br>Limited decomposition needed<br>Single owner typically sufficient |
| 9-12 | Tier 2 - Moderate | Formal planning recommended<br>Task decomposition beneficial<br>Consider risks and dependencies |
| 13-16 | Tier 3 - Complex | Detailed planning required<br>Structured decomposition essential<br>Risk mitigation planning<br>Multiple specialists likely needed |
| 17-20 | Tier 4 - Very Complex | Comprehensive planning mandatory<br>Phased approach recommended<br>Formal risk management<br>Dedicated team with specialized expertise |

## Estimation Adjustment Factors

Adjust time/effort estimates based on final complexity assessment:

| Complexity Tier | Estimation Adjustment Factor |
|-----------------|------------------------------|
| Tier 1 - Simple | 1.0-1.2× |
| Tier 2 - Moderate | 1.3-1.5× |
| Tier 3 - Complex | 1.6-2.0× |
| Tier 4 - Very Complex | 2.0-3.0× |

**Special Considerations:**
- Add 20-30% buffer for dimensions scoring 4-5
- Consider breaking down tasks with multiple dimensions scoring 4+
- For total scores > 16, consider pre-work research/discovery phase

## Application Process

1. **Initial Assessment:**
   - Rate each dimension individually
   - Calculate overall complexity score
   - Determine complexity tier

2. **Planning Implications:**
   - Apply appropriate estimation adjustments
   - Determine decomposition strategy
   - Identify necessary expertise
   - Plan risk mitigation approaches

3. **Communication:**
   - Share complexity assessment with stakeholders
   - Explain key complexity factors
   - Set appropriate expectations
   - Document assumptions and unknowns 