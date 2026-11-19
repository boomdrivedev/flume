## ADDED Requirements

### Requirement: flume's default names a program only for an agent with no pool

flume's default layer SHALL give a stage's program only to an agent
with no pool; an agent with a pool whose layers above the default name
no program SHALL run the program of the account chosen for it, its
layer `account` (32, 144, 173).

#### Scenario: flume's default and a pool

- **WHEN** a fix's stage names no program in any layer, and it runs once on a stream with no pool and once on a stream whose pool holds other-1 of `other` alone
- **THEN** the first agent runs flume's default program, from `default`, and the second runs `other`, from `account`, under other-1 (32, 144, 173)
