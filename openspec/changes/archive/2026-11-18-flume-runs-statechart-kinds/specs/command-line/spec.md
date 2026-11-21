## ADDED Requirements

### Requirement: answer answers a statechart kind's decision

`flume answer <place> <region> <answer>` SHALL give the core the answer
to the decision standing at that region of the place's statechart, as
its caller, and print the core's answer: the answer taken, or the
refusal naming the region or the answer taken before (176, section 9).

#### Scenario: An answer over a scratch repository

- **WHEN** a statechart kind is launched on a scratch repository and `flume answer` names a region whose state carries no decision
- **THEN** it exits non-zero naming the region, and status of the place gives no answer (176)
