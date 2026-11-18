## ADDED Requirements

### Requirement: The fake agent scripts a start through the launcher

The fake agent SHALL script that it starts its program through flume's
launcher, not by default; its command SHALL then carry the launcher's
words; and, for a start a test names, it SHALL script the process its
launcher's word names, none until scripted, each read logged
`launched <identity>` (106, C.4).

#### Scenario: A launcher's start scripted

- **WHEN** the fake agent is scripted to start through the launcher and a start's word to name the process 41
- **THEN** it says it starts through the launcher, its command carries the launcher's words, and it names 41 for that start and no process for any other (C.4)

### Requirement: The fake terminal scripts the process it reads

The fake terminal SHALL answer, for an agent a test names, the process
its program is scripted to run in, that nothing runs it, or unknown
with words, unknown saying it reads no process where nothing is
scripted, each asking logged `process <identity>` (106, C.5).

#### Scenario: A process read, then another

- **WHEN** the fake terminal is scripted to read an agent's program running in 41, then in 42, and is asked between
- **THEN** it answers 41, then 42, and unknown for an agent nothing was scripted for, logging each asking (C.5)
