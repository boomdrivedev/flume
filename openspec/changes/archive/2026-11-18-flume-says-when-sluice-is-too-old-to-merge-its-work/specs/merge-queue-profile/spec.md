## ADDED Requirements

### Requirement: No work is handed to a sluice too old to merge it by itself

Before each submit, a place's from its worktree and a base place's by
name, the sluice profile SHALL read the sluice it runs by `sluice
--version`. Where that names a release before 0.97.0, the first whose
host process moves its queues, or names no release, the profile SHALL
run no `sluice submit` and SHALL answer the submit refused, its words
naming what sluice said and the release flume needs (111, 112, 203,
C.6; sluice 135; record 0080).

#### Scenario: A sluice before its host process

- **WHEN** the profile submits a place from its worktree, and then a base place's branch by name, through a stand-in `sluice` first on its `PATH` that says `sluice 0.95.1` and logs every other command it is asked
- **THEN** each submit is refused, its words naming `sluice 0.95.1` and `0.97.0`, and the stand-in logged no `submit` (C.6; sluice 135)

#### Scenario: A sluice that names no release

- **WHEN** the profile submits a place through a stand-in `sluice` that says, in turn, `sluice 0.0.0`, `sluice 0.97.0-rc.1`, `sluice`, and nothing with exit 64
- **THEN** each submit is refused, its words naming what the stand-in said, or that it said nothing, and `0.97.0`, and the stand-in logged no `submit` (C.6)

#### Scenario: A sluice with its host process

- **WHEN** the profile submits a place through a stand-in `sluice` that says, in turn, `sluice 0.97.0`, `sluice 0.100.0` and `sluice 1.0.0`, and answers each submit with an entry
- **THEN** each submit runs `sluice submit main --no-wait --json` after the version read and answers the entry submitted (111, C.6)

#### Scenario: The pinned sluice is handed the work

- **WHEN** a base place's branch is submitted by name through a `sluice` that logs each command it is asked and runs the pinned sluice, over a scratch repository
- **THEN** the first command it was asked is `--version`, the next is `submit main --head train/a --no-wait --json`, and the branch lands (203, C.6)
