## MODIFIED Requirements

### Requirement: A submission's outcome is collected at the settles that follow

Every settle of a place whose submission is open, flume having
submitted it and holding no outcome of it, SHALL collect the outcomes
pending for it through the merge queue seam's inbox and act on each
once, and SHALL drive its base one round through step, once per
repository and base in one command; a settle SHALL never submit the
place anew for an open submission (111, 124, 125, 133).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test, and the fake merge queue lands it before the next settle
- **THEN** that settle collects the landing, never submits anew, and tidies the place (S34)

#### Scenario: Two places on one base

- **WHEN** one settle of every place finds two places' submissions open on one base
- **THEN** the fake merge queue's log holds one step for that base

## ADDED Requirements

### Requirement: No inbox for a place with no submission open

A settle of a place flume has not submitted, or whose submission's
outcome it holds, SHALL ask the merge queue seam's inbox nothing, but
once as it submits the place anew after an outcome, taking nothing
pending then as the new submission's outcome (99, 101, 111, 133).

#### Scenario: The inbox is asked only while the submission is open

- **WHEN** a fix is settled before its submission, while its submission is open, while its fix round runs after the fake merge queue failed it with an outcome left pending for the place, and as its next delivery submits it anew
- **THEN** no inbox is asked before the submission or while the fix round runs, one is asked at each settle while it is open, the inbox is collected as it is submitted anew, the outcome left pending sends the place back no more, and the new submission lands (111, 133, 99, 101)

#### Scenario: A decided submission asks no inbox

- **WHEN** the fake merge queue withdraws a place's submission, or refuses it, and the place is settled again
- **THEN** the place waits for its caller and that settle asks the inbox nothing (112, 133)
