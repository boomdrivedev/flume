## MODIFIED Requirements

### Requirement: A submission's outcome is collected at the settles that follow

Every settle of a place flume submitted SHALL collect the outcomes
pending for it through the merge queue seam's inbox and act on each
once, and where its submission is open SHALL drive its base one round
through step, once per repository and base in one command; a settle
SHALL never submit the place anew for an open submission. An outcome
handed again, one already acted on, SHALL count as no step taken, so it
never keeps a settle from reading the place's stage (40, 46, 111, 124,
125, 133).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test, and the fake merge queue lands it before the next settle
- **THEN** that settle collects the landing, never submits anew, and tidies the place (S34)

#### Scenario: Two places on one base

- **WHEN** one settle of every place finds two places' submissions open on one base
- **THEN** the fake merge queue's log holds one step for that base

#### Scenario: An outcome handed again

- **WHEN** a change's submission fails, the merge queue hands that failure again at every inbox after, and code, sent back, ticks every task and goes quiet
- **THEN** the settle that sent code back reads its prompt received, the next settle ends code and starts verify, and the failure is acted on once: one submission and one going-back (40, 46, 74, 133)
