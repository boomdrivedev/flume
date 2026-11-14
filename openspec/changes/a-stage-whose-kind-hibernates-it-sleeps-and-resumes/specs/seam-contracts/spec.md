## ADDED Requirements

### Requirement: An identity names its resume

An agent's identity SHALL name, beside its repository, place, stage and
start, the resume it is of that start, none for a start; it SHALL be
written `flume:<repository>:<place>:<stage>:<start>` for a start and
`flume:<repository>:<place>:<stage>:<start>.<resume>` for a resume, and
an identity written before SHALL read as a start's (30, 33, 213;
core.10).

#### Scenario: A resume's identity round-trips

- **WHEN** the identity of the second resume of talk's first start is written, as JSON and as flume spells it, and read back
- **THEN** each equals the identity written, and its spelling ends `:talk:1.2` (33, 213)

#### Scenario: An identity written before

- **WHEN** an identity's JSON written before holds no resume
- **THEN** it reads as a start's, spelled as before (33, 74)

### Requirement: The types of a hibernation

The seams SHALL pass a kind's stage saying whether it hibernates; a
stage state `hibernating`; a start's hibernation, since when and why;
and each resume of a start, when, by what, its identity and its
conversation. Each SHALL serialize to JSON and back unchanged, and a
held state written before SHALL read with none of them (56, 61, 74,
212, 213).

#### Scenario: A hibernated start round-trips

- **WHEN** a held state whose talk start hibernated idle, was resumed once by a tell and carried its conversation, is written to JSON and read back
- **THEN** it equals the value written, and its JSON gives the stage's state `hibernating`, the hibernation's `why` `idle` and the resume's `by` `tell` (61)

### Requirement: The types around a hibernation

The seams SHALL pass a conversation carried from a hibernated start, an
agent kept open closed by its hibernation, the acts `hibernate` and
`resume`, a start's mark that a tell is being handed to it, and the
setting `hibernate_after`, each serialized to JSON and back unchanged
(54, 56, 61, 198, 212, 213).

#### Scenario: Each round-trips

- **WHEN** a conversation whose cause is `hibernated`, a close by `hibernated`, the acts `hibernate` and `resume`, a start marked handing, and settings giving `hibernate_after` `20m` are each written to JSON and read back
- **THEN** each equals the value written (61)

#### Scenario: A held state written before

- **WHEN** a held state written before this change is read
- **THEN** its stages hibernate none and its starts hold no hibernation and no resume (74)
