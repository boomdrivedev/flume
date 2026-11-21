## ADDED Requirements

### Requirement: Status gives a statechart place's statechart

Status SHALL give a place of a statechart kind each region with the
state it holds, since when and whether it is final, its counters,
whether its run is over, each decision standing with its region, kind,
answers and shows, each answer given with who gave it, when and whether
it was applied, and each stage entered with its entry, join, send-backs
and sessions (56, 175, 176).

#### Scenario: Persona at its test stage

- **WHEN** persona's test stage runs two sessions, one delivered and one working
- **THEN** status gives region `stages` at `test` since its entry, region `stages.test.run` at `working`, the stage `stages.test` at entry 1 joined by all with its two sessions, each session as a stage with its state, and the run not over (56, 175)

#### Scenario: A decision standing

- **WHEN** persona's region `stages` reaches `stopped`, whose decision answers `retry` and `drop`
- **THEN** status gives the decision at region `stages` with its kind `stalled` and its two answers, and says the place waits on its caller for it, since when (56, 130, 176)

### Requirement: Status gives each live session's agent

Status SHALL give, for a place of a statechart kind, each live
session's agent as it gives a simple kind's current stage's agent, in a
list of the place's agents, and its JSON SHALL only gain fields (56,
core.14).

#### Scenario: Two agents

- **WHEN** two sessions of a statechart place run
- **THEN** status gives both agents, each with its identity naming its session, its account and its settings with where each came from, and a simple kind's place gives the same JSON it gave before with an empty list (56, core.14)
