## MODIFIED Requirements

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

### Requirement: A message to a place's agent

A message naming a place SHALL hand its words to the current stage's
agent as a prompt once it waits, received or sent again as a prompt is,
and SHALL be refused with nothing typed when the place has no live
agent and no hibernating stage, its agent asks, its stage waits on an
answer its agent is neither kept open nor hibernated for, or the bound
passes, ten minutes where none is given. It SHALL move no stage, but as
an answer to an agent kept open or as a resume (53 to 55, 198, 213).

#### Scenario: A message into a question, then into work

- **WHEN** a caller messages an agent that is asking, then messages it again while it works
- **THEN** the first is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)

#### Scenario: A message once the agent is let go

- **WHEN** a caller messages a place whose code ended short needing an answer and whose agent was let go
- **THEN** the message is refused with nothing typed, naming the retry that answers the stage (42, 53, S104)

## ADDED Requirements

### Requirement: A retry of a hibernating stage

A retry with words naming a stage that waits on its answer and whose
agent hibernated SHALL resume that agent's session, the question then
the words its prompt, counting no start. A retry of any other
hibernating stage SHALL start it afresh as any retry does, its
hibernation over, carrying no conversation (42, 49, 212, 213).

#### Scenario: A retry with words answers a hibernated agent

- **WHEN** a code stage of a kind that hibernates it ended short needing an answer and its agent hibernated at the end of its keep-open time, and its caller retries code with "notes.md"
- **THEN** code resumes its session, its starts unchanged, its prompt the question then "notes.md" (42, 49, 213)

#### Scenario: A retry of a hibernating conversation

- **WHEN** a caller retries a place whose talk conversation hibernates, with no words
- **THEN** talk's second start is a fresh agent that resumes no conversation, and its hibernation is over (49, 212)

### Requirement: A stop of a hibernating stage

A stop of a place whose stage hibernates SHALL be taken with no agent
to end: the hibernation SHALL be over and the stage read stopped, or,
where it waits on its answer, stay ended short with its close read
stopped, so nothing resumes it after (42, 51, 212).

#### Scenario: A hibernating conversation stopped

- **WHEN** a caller stops a place whose talk hibernates, then messages it
- **THEN** the stop is taken and talk reads stopped, and the message is refused naming the place's want of a live agent, nothing typed and nothing started (51, 53, 212)

### Requirement: A message to a hibernating stage resumes it

A message naming a place whose stage hibernates SHALL be taken at once
and SHALL resume the stage, its words the resumed agent's prompt, or,
where the stage waits on its answer, the question then its words as
that answer (53, 198, 213).

#### Scenario: A message to a hibernating stage

- **WHEN** a caller messages a place whose talk hibernates "go on"
- **THEN** the message is taken at once, and talk resumes with "go on" as its prompt (53, 213)
