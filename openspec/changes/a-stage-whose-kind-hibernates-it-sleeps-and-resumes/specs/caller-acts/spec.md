## MODIFIED Requirements

### Requirement: Retry

A retry SHALL end the place's agent, one that is not quiet only when
forced, then start the stage it names, the current one where it names
none, with a fresh agent, its prompt's words carrying the question of
the stop-short the stage ended with before the retry's words, the stages
after it then running again in order, the host's room allowing, but
where it answers an agent kept open, or, with words naming a stage
waiting on its answer whose agent hibernated, resumes that agent's
session with them as the answer. A retry of any other hibernating stage
SHALL start it afresh as any retry does, carrying no conversation. A
retry naming the submission SHALL submit the place's head again and
start no agent (42, 49, 50, 70, 198, 212, 213).

#### Scenario: A retry with words

- **WHEN** a construct ended short needing an answer, its agent let go and ended, and its caller retries it with the answer as its words
- **THEN** construct's second start is a fresh agent whose prompt carries the question, then the answer (S4, 42, 49)

#### Scenario: A retry with no words while the agent is kept open

- **WHEN** a construct ended short needing an answer, its agent kept open and quiet, and its caller retries it with no words
- **THEN** the asking agent is ended, and a fresh agent starts whose prompt carries the question alone (49, 50)

#### Scenario: A retry of the first stage after readiness failed

- **WHEN** a change's readiness failed and its caller retries construct
- **THEN** readiness runs again as its launch read it, and construct starts once it passes (12)

#### Scenario: A retry with words answers a hibernated agent

- **WHEN** a code stage of a kind that hibernates it ended short needing an answer and its agent hibernated at the end of its keep-open time, and its caller retries code with "notes.md"
- **THEN** code resumes its session, its starts unchanged, its prompt the question then "notes.md" (42, 49, 213)

#### Scenario: A retry of a hibernating conversation

- **WHEN** a caller retries a place whose talk conversation hibernates, with no words
- **THEN** talk's second start is a fresh agent that resumes no conversation, and its hibernation is over (49, 212)

### Requirement: Stop

A stop SHALL end the place's agent and nothing else: the stage neither
ends nor moves on, and waits for a retry or a drop. An agent that is not
quiet SHALL be stopped only when forced, and an end the agent did not
take SHALL be reported as a failure. A stop of a place whose stage
hibernates SHALL be taken with no agent to end: the hibernation is over,
the stage reads stopped, or, where it waits on its answer, stays ended
short with its close read stopped, so nothing resumes it after (31, 42,
51, 212).

#### Scenario: A working agent stopped

- **WHEN** a caller stops a working agent unforced, then forced
- **THEN** the first stop is refused, the forced one ends the agent, and the stage waits for a retry neither ended nor moved on (S20)

#### Scenario: A hibernating conversation stopped

- **WHEN** a caller stops a place whose talk hibernates, then tells it
- **THEN** the stop is taken and talk reads stopped, and the tell is refused naming the place's want of a live agent, nothing typed and nothing started (51, 53, 212)

### Requirement: Finish

A finish SHALL end a conversation's agent once it is quiet, one not
quiet only when forced, and the conversation SHALL then count as
delivered; a hibernating conversation SHALL be finished with no agent
to end (109, 212).

#### Scenario: Finish while the agent works

- **WHEN** a caller finishes a conversation whose agent is working, unforced
- **THEN** the finish is refused and the stage goes on

#### Scenario: A hibernating conversation finished

- **WHEN** a caller finishes a place whose talk conversation hibernates
- **THEN** talk reads delivered and the next stage starts as 40 and 47 allow, no agent closed through the terminal (40, 109, 212)

### Requirement: Tell

A tell SHALL hand its words to the current stage's agent as a prompt
once it is waiting, received or sent again as a prompt is, and SHALL be
refused with nothing typed when the place has no live agent and no
hibernating stage, its agent is asking, its stage ended short needing
an answer and its agent is neither kept open nor hibernated, or it is
not delivered within its bound, ten minutes where the caller gives
none. A tell to a place whose stage hibernates SHALL resume it, its
words the resumed agent's prompt, or, where the stage waits on its
answer, the question then its words as that answer. A tell SHALL never
start, end, hold or move a stage, but as an answer to an agent kept
open or as a resume (53 to 55, 198, 212, 213).

#### Scenario: A tell into a question, then into work

- **WHEN** a caller tells an agent that is asking, then tells it again while it works
- **THEN** the first tell is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)

#### Scenario: A tell once the agent is let go

- **WHEN** a caller tells a place whose code ended short needing an answer and whose agent was let go
- **THEN** the tell is refused with nothing typed, naming the retry that answers the stage (42, 53, S104)

#### Scenario: A tell to a hibernating stage

- **WHEN** a caller tells a place whose talk hibernates "go on"
- **THEN** the tell is taken at once, and talk resumes with "go on" as its prompt (53, 213)
