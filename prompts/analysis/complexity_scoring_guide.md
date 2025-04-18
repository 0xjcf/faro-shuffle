# Task Complexity Scoring Guide

## Introduction
This guide provides a standardized framework for scoring the complexity of tasks. The goal is to ensure consistent evaluation of task complexity across different teams and projects, enabling better planning, resource allocation, and decomposition decisions.

## Scoring Scale
All factors are scored on a scale from 0-10:
- **0-2**: Minimal complexity
- **3-4**: Low complexity
- **5-6**: Moderate complexity
- **7-8**: High complexity
- **9-10**: Extreme complexity

## Technical Complexity Factors

### Technology Stack Familiarity
| Score | Description |
|-------|-------------|
| 0-2 | Well-established technology stack with high team familiarity |
| 3-4 | Familiar stack with minor new elements |
| 5-6 | Mixed familiar and unfamiliar technologies |
| 7-8 | Mostly unfamiliar technologies requiring significant learning |
| 9-10 | Entirely new or cutting-edge technologies with no team experience |

### Architecture Complexity
| Score | Description |
|-------|-------------|
| 0-2 | Simple, straightforward architecture following established patterns |
| 3-4 | Standard architecture with minor customizations |
| 5-6 | Moderately complex architecture with multiple components |
| 7-8 | Complex architecture involving multiple systems and integration points |
| 9-10 | Highly complex, distributed architecture requiring innovative solutions |

### Technical Dependencies
| Score | Description |
|-------|-------------|
| 0-2 | Few or no dependencies, all well-documented and stable |
| 3-4 | Some dependencies, all reliable and well-understood |
| 5-6 | Multiple dependencies with varying reliability/documentation |
| 7-8 | Many dependencies, some with reliability concerns or poor documentation |
| 9-10 | Extensive dependencies including unstable or poorly documented systems |

### Performance Requirements
| Score | Description |
|-------|-------------|
| 0-2 | Minimal performance requirements with generous tolerances |
| 3-4 | Standard performance requirements easily met with conventional approaches |
| 5-6 | Moderate performance requirements needing some optimization |
| 7-8 | Stringent performance requirements demanding significant optimization |
| 9-10 | Extreme performance requirements pushing the limits of available technology |

### Security Considerations
| Score | Description |
|-------|-------------|
| 0-2 | Minimal security requirements, low-sensitivity data |
| 3-4 | Standard security practices sufficient |
| 5-6 | Moderate security needs requiring specific implementation considerations |
| 7-8 | High security requirements with complex implementation needs |
| 9-10 | Extreme security requirements (e.g., financial, health, national security) |

## Domain/Business Complexity Factors

### Business Rule Complexity
| Score | Description |
|-------|-------------|
| 0-2 | Few simple, well-defined business rules |
| 3-4 | Standard business rules with clear documentation |
| 5-6 | Moderate number of business rules with some complexity or exceptions |
| 7-8 | Many complex business rules with numerous exceptions |
| 9-10 | Extremely complex business rules, potentially contradictory or ambiguous |

### Domain Knowledge Requirements
| Score | Description |
|-------|-------------|
| 0-2 | Minimal domain knowledge required |
| 3-4 | Basic domain knowledge sufficient |
| 5-6 | Moderate domain expertise needed |
| 7-8 | Deep domain expertise required |
| 9-10 | Expert-level domain knowledge essential with specialized terminology |

### Stakeholder Requirements Clarity
| Score | Description |
|-------|-------------|
| 0-2 | Crystal clear requirements with excellent documentation |
| 3-4 | Clear requirements with minor ambiguities |
| 5-6 | Moderately clear requirements with some gaps or ambiguities |
| 7-8 | Unclear requirements with significant gaps |
| 9-10 | Highly ambiguous or constantly changing requirements |

### Business Process Integration
| Score | Description |
|-------|-------------|
| 0-2 | Minimal integration with existing business processes |
| 3-4 | Simple integration with well-documented processes |
| 5-6 | Moderate integration needs with multiple processes |
| 7-8 | Complex integration with numerous business processes |
| 9-10 | Extensive integration affecting critical business processes |

### Regulatory/Compliance Factors
| Score | Description |
|-------|-------------|
| 0-2 | No regulatory or compliance concerns |
| 3-4 | Basic compliance needs easily addressed |
| 5-6 | Moderate compliance requirements |
| 7-8 | Significant regulatory requirements affecting multiple aspects |
| 9-10 | Strict regulatory environment with heavy auditing requirements |

## Operational Complexity Factors

### Team Experience with Similar Tasks
| Score | Description |
|-------|-------------|
| 0-2 | Team has extensive experience with very similar tasks |
| 3-4 | Team has good experience with comparable tasks |
| 5-6 | Team has some experience with related tasks |
| 7-8 | Team has limited experience with similar tasks |
| 9-10 | Team has no experience with this type of task |

### Cross-team Dependencies
| Score | Description |
|-------|-------------|
| 0-2 | No dependencies on other teams |
| 3-4 | Minor dependencies on responsive, aligned teams |
| 5-6 | Moderate dependencies on multiple teams |
| 7-8 | Significant dependencies on multiple teams with different priorities |
| 9-10 | Critical dependencies on many teams with competing priorities |

### Deployment Complexity
| Score | Description |
|-------|-------------|
| 0-2 | Simple deployment with automated processes |
| 3-4 | Standard deployment with minimal manual steps |
| 5-6 | Moderately complex deployment with some coordination required |
| 7-8 | Complex deployment requiring significant coordination and planning |
| 9-10 | Extremely complex deployment with high risk and extensive coordination |

### Testing Requirements
| Score | Description |
|-------|-------------|
| 0-2 | Simple testing needs with high automation potential |
| 3-4 | Standard testing practices sufficient |
| 5-6 | Moderate testing complexity requiring specific test scenarios |
| 7-8 | Complex testing needs with extensive test case development |
| 9-10 | Extremely difficult-to-test scenarios requiring specialized techniques |

### Operational Support Needs
| Score | Description |
|-------|-------------|
| 0-2 | Minimal operational support needed, highly self-sustaining |
| 3-4 | Standard monitoring and maintenance sufficient |
| 5-6 | Moderate operational needs requiring specific monitoring |
| 7-8 | High operational overhead with specialized support procedures |
| 9-10 | Extensive ongoing operational support with 24/7 monitoring requirements |

## Uncertainty/Risk Factors

### Requirements Stability
| Score | Description |
|-------|-------------|
| 0-2 | Completely stable, unlikely to change requirements |
| 3-4 | Mostly stable with minor expected refinements |
| 5-6 | Moderately stable with some expected changes |
| 7-8 | Unstable with frequent expected changes |
| 9-10 | Highly volatile requirements, constantly evolving |

### Timeline Constraints
| Score | Description |
|-------|-------------|
| 0-2 | Generous timeline with substantial buffer |
| 3-4 | Comfortable timeline with adequate buffer |
| 5-6 | Moderate timeline pressure with minimal buffer |
| 7-8 | Aggressive timeline with no buffer |
| 9-10 | Extremely tight deadline with high pressure |

### External Dependencies
| Score | Description |
|-------|-------------|
| 0-2 | No external dependencies |
| 3-4 | Few external dependencies, all reliable |
| 5-6 | Some external dependencies with mostly reliable history |
| 7-8 | Multiple external dependencies with varying reliability |
| 9-10 | Critical dependencies on unreliable or uncontrollable external factors |

### Innovation Level Required
| Score | Description |
|-------|-------------|
| 0-2 | Established approaches sufficient, no innovation needed |
| 3-4 | Minor innovations or adaptations of existing solutions |
| 5-6 | Moderate innovation needed to adapt known approaches |
| 7-8 | Significant innovation required with few precedents |
| 9-10 | Groundbreaking innovation needed, no existing solutions |

### Potential Scope Expansion
| Score | Description |
|-------|-------------|
| 0-2 | Well-bounded scope with little risk of expansion |
| 3-4 | Clear scope with minor risk of small expansions |
| 5-6 | Moderate risk of scope expansion in identified areas |
| 7-8 | High risk of significant scope expansion |
| 9-10 | Almost certain major scope expansion during implementation |

## Calculating the Overall Complexity Score
1. Calculate the weighted average for each category using the weights specified in the template
2. Sum the weighted category scores to produce the final complexity score (0-10)
3. Classify the task according to the score ranges:
   - Simple: 0-2.5
   - Moderate: 2.6-5.0
   - Complex: 5.1-7.5
   - Very Complex: 7.6-10

## Using Complexity Scores
- **Simple tasks** generally don't require decomposition
- **Moderate tasks** may benefit from light decomposition into 2-3 subtasks
- **Complex tasks** should be decomposed into 4-7 subtasks
- **Very Complex tasks** require comprehensive decomposition into 8+ subtasks with potential further sub-decomposition

## Reviewing Complexity Scores
- Scores should be reviewed by at least one other team member
- If team members' scores differ significantly, discuss to understand different perspectives
- Document the reasoning behind scores to build organizational knowledge
- Review historical scores periodically to calibrate and improve the scoring process 