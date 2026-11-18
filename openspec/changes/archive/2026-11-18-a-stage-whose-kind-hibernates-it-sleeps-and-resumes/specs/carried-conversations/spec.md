## MODIFIED Requirements

### Requirement: Only a start after a spent agent or a lost conversation carries one

No start but the one that follows an agent stopped at its account's
usage limit, the one a caller's retry makes of a conversation's stage
reported lost, and a resume of a hibernating stage SHALL carry a
conversation, and each SHALL carry only that agent's. A retry of any
other stage, a hibernating one among them, a fix round, a start after
one begun without its settings and every start of another stage SHALL
carry none (30, 49, 146, 212, 213).

#### Scenario: A retry carries nothing

- **WHEN** a caller retries a stage whose agent stopped at its account's limit, before its stage starts again
- **THEN** the start the retry makes resumes no conversation, and its prompt is the stage's with the retry's words (30, 49)

#### Scenario: The next stage carries nothing

- **WHEN** a carried code start delivers and verify starts
- **THEN** verify's agent resumes no conversation (30)

#### Scenario: A lost code stage's retry carries nothing

- **WHEN** the host restarts while a code stage's agent works, the fake terminal brings the agent back bare, and once flume ends it the caller retries code with words
- **THEN** code's fresh agent resumes no conversation, its prompt is code's with the retry's words, and the fake agent's log shows no `carry` (39, 45, 49, S116)

#### Scenario: A stopped conversation's retry carries nothing

- **WHEN** a caller stops a talk stage, a conversation, and then retries it
- **THEN** talk's fresh agent resumes no conversation (49, 51)

#### Scenario: A resume carries its own stage's conversation

- **WHEN** talk hibernated after its first start, and its caller resumes the place
- **THEN** the resumed agent's record names the session of talk's first start, and the fake agent's log shows one `carry` of that start's identity (179, 213)

## ADDED Requirements

### Requirement: Status gives a resume's conversation

Status SHALL give, with the agent of a resume, the start whose
conversation it carries with that it hibernated, and whether the agent
carries its conversation or, where it does not, why, never a
configuration home's path (56, 179, 213; core.30).

#### Scenario: A resumed talk

- **WHEN** status is asked of a place whose talk start 1 was resumed once
- **THEN** the agent's conversation gives start 1, hibernated, and carried, and the person's form reads that it carries the conversation of start 1, hibernated (56, 213)
