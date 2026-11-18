## ADDED Requirements

### Requirement: A delivery resumes a role whose place hibernates

A batch pending at a role's address whose place's current stage
hibernates SHALL resume it by `delivery`, written `delivery`: its start
SHALL claim the address's batch under the resume's identity, its prompt
the batch, then the way to stop short, or flume's bare resume's words
where none is pending then; recorded handed once received and let go
where never begun (146, 213, 214).

#### Scenario: Two messages and an end while asleep

- **WHEN** `east/ops`'s talk hibernates, and two messages and a fix's end come for `east/ops`
- **THEN** one resume starts, by `delivery`, its prompt the three as one batch, each with its confirm, then the way to stop short; each is recorded handed once the prompt is received, and status gives the resume by `delivery` (213, 214)

#### Scenario: A resume that never began

- **WHEN** that resume is read never begun
- **THEN** its claims are let go, nothing is recorded handed, and the start after it carries the same batch

#### Scenario: Handed meanwhile

- **WHEN** a delivery the resume was put waiting for is collected by the address's `flume inbox` before the resume's start fills its prompt, and nothing else is pending
- **THEN** the resume's prompt is flume's words for a bare resume, then the way to stop short

#### Scenario: A hibernated agent waiting on its answer

- **WHEN** `east/ops`'s talk ended short needing an answer and was hibernated at the end of its keep-open time, and a message comes for `east/ops`
- **THEN** nothing is resumed, and the message stays pending until the answer resumes the talk and its agent next waits (42, 214)

### Requirement: A role's place holding a batch is not hibernated on its idling

A role's place whose address holds a batch handed to none SHALL NOT be
hibernated on its idling while the batch stands, since a delivery waits
to be handed to it (212, 214).

#### Scenario: A batch the terminal could not hand

- **WHEN** `east/ops`'s talk is quiet past its hibernate time while the fake terminal refuses every send to `east/ops` and a message is pending there
- **THEN** no settle hibernates it, and once the send is taken and its agent quiet its hibernate time again, a settle does
