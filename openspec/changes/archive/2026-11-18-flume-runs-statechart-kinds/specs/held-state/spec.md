## ADDED Requirements

### Requirement: A statechart place's engine state is in its one document

A place of a statechart kind SHALL keep, in its one document written by
compare-and-set, the engine's state of its statechart, each region's
state and when it entered it, its counters, the answers it applied and
what its states set aside; each answer given with who gave it and when;
and each stage's entry recorded and how many times it was sent back (74,
75, 103, 176, core.16).

#### Scenario: A restart between two settles

- **WHEN** a statechart place's stage has recorded its sessions and the host restarts before the next settle
- **THEN** the next settle reads the same engine state, starts no session twice and performs no effect already proven (74, 76, core.17)
