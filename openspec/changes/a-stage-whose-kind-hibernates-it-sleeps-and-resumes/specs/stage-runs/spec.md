## MODIFIED Requirements

### Requirement: An agent let go is ended

The core SHALL read an agent kept open let go at a settle once its time
has passed, it is read stopped or its session ended, or its account is
spent, recording when and why, but where its stage hibernates and its
time has passed first, SHALL record it hibernated instead; SHALL then
end the agent once it is quiet, never forced, freeing its room and its
account; and the stage SHALL keep waiting on its answer under the same
move and since, a later answer being a retry with a fresh agent, or,
for an agent hibernated, a resume of its session (31, 42, 70, 96, 130,
134, 146, 198, 212, 213).

#### Scenario: The time passing

- **WHEN** an hour passes on the fake clock after a code agent stopped short needing an answer, the agent quiet, with another place's stage waiting for room on a host at its bound, and a settle runs
- **THEN** the agent reads let go by its time and is ended through the terminal, code still reads ended short waiting on its answer since the stop-short, and the other place's stage starts (70, 130, 198, S104)

#### Scenario: Not quiet once its time has passed

- **WHEN** the asking agent works on once its time has passed
- **THEN** it reads let go, and it is not ended until a settle reads it quiet (31, 198)

#### Scenario: The agent dying within its time

- **WHEN** the asking agent's pane is gone within its hour
- **THEN** the next settle records it let go, ended, and the stage waits on its answer with no agent (96, 198, S105)

#### Scenario: The account spent within its time

- **WHEN** another agent of the asking agent's account reaches the account's usage limit within the hour
- **THEN** the next settle records the agent let go by its account spent, and ends it once it is quiet (146, 198, S105)

#### Scenario: Past its pool's threshold within its time

- **WHEN** the asking agent's account passes its pool's threshold within the hour, and is not spent
- **THEN** the agent stays kept open (185, 198)

#### Scenario: A late answer

- **WHEN** a code agent was let go by its time and ended, and its caller retries code with "notes.md"
- **THEN** code's second start is a fresh agent whose prompt carries the question, then the answer (42, 49, S104)

#### Scenario: Two settles at once

- **WHEN** two settles read an agent let go at once
- **THEN** it is recorded let go once, and its agent is closed once (74, 75)

#### Scenario: Hibernated at its time, not quiet

- **WHEN** a code agent of a stage that hibernates works on once its keep-open time has passed
- **THEN** its close reads hibernated, and it is ended, and its hibernation recorded, only once a settle reads it quiet (31, 198, 212)

### Requirement: A conversation ends on its caller's finish

A conversation SHALL have no quiet limit and no stop check, SHALL never
read stuck, and SHALL end only on its caller's finish, which ends its
agent once quiet, or, where it hibernates, ends none, and counts the
stage delivered (109, 212).

#### Scenario: An hour beside the operator

- **WHEN** a conversation's fake agent goes quiet many times over an hour, then its caller finishes it
- **THEN** it is never sent back nor reported stuck, and the finish ends the quiet agent and the stage reads delivered (S23)

## ADDED Requirements

### Requirement: A hibernating stage is never stuck, lost or past its limit

While a stage hibernates, no settle SHALL read it stuck, lost or its
agent working past its limit, nor read any agent of it, and a
hibernating stage SHALL hold no room and no account (44, 45, 70, 118,
145, 212).

#### Scenario: A hibernating conversation over a day

- **WHEN** talk hibernates, its pane gone through the fake terminal, and settles run over a day
- **THEN** talk reads `hibernating` at each, never lost or stuck, and the fake terminal is asked nothing of its agent (45, 212)

#### Scenario: A hibernating code stage

- **WHEN** a caller hibernates a place whose code stage of a kind that hibernates it runs, then settles run past code's quiet limit
- **THEN** code reads `hibernating` at each, never stuck, and another place's stage waiting for room on a host at its bound starts (44, 70, 212)
