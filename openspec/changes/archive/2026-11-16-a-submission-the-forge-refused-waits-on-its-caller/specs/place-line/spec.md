## MODIFIED Requirements

### Requirement: A place's state in one word

The line's state SHALL be the first that holds of: being made; landed, dropped or done; stalled; held; waiting on an answer; asking; lost; never begun; not ready; refused; stopped; past its bound; submission refused; landing refused; waiting on done; waiting for room; starting; asking, where the live agent's profile reads it asking; stuck; in conversation; working; submitted; else the current stage's state in words (12, 27, 34, 38, 42, 45, 47, 48, 51, 63, 70, 90, 112, 114, 118, 130).

#### Scenario: A hold, then a stall

- **WHEN** a place's code stage is on hold under no words, and five minutes later is stalled
- **THEN** its line's state is `held`, and then `stalled` (47, 130)

#### Scenario: A stop-short that needs an answer

- **WHEN** a place's construct stage stopped short needing an answer
- **THEN** its line's state is `waiting on an answer` (42)

#### Scenario: An agent asking a person

- **WHEN** a place's code agent has started and its profile's asking mark stands
- **THEN** its line's state is `asking`, and once the mark is cleared, `working` (90)

#### Scenario: Starting and waiting for room

- **WHEN** one place's code agent is opened and has not said it started, and another place's first stage waits for room on the host
- **THEN** the first's line's state is `starting` and the second's `waiting for room` (35, 70)

#### Scenario: Submitted, then landed

- **WHEN** a fix place's last stage delivered and flume submitted it, and a later settle reads its landing
- **THEN** its line's state is `submitted`, and after that settle `landed` (48, 63)

#### Scenario: A landing the forge refused

- **WHEN** a place's submission is read held, the forge having refused its landing, and five minutes pass under no hold's words
- **THEN** its line's state is `landing refused`, and then `stalled` (112, 130)
