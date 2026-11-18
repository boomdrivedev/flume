## ADDED Requirements

### Requirement: The kinds seam reads a held statechart kind

The kinds seam SHALL read a statechart kind's held text and the stage
template's text into the engine's statechart and templates, and SHALL
refuse, naming why, a text it cannot read; a kinds profile that reads no
statechart kind SHALL refuse every one (84, 85, 176, core.2).

#### Scenario: A kinds seam that reads none

- **WHEN** a kinds seam that gives no reading of its own is asked to read a held statechart kind
- **THEN** it refuses, naming the kind (84, core.2)

### Requirement: Git's history gives a revision's files

Git's history SHALL give the path of every file a revision of a
repository holds, the same revision always giving the same files, and
nothing for a revision it does not hold (176, 178, section 9).

#### Scenario: Files at two revisions

- **WHEN** a branch's first commit adds `personas/a.md` and its second adds `personas/b.md`
- **THEN** the first revision's files name `personas/a.md` alone, and the second's name both (178)

### Requirement: The types of a statechart kind and its place

The seams SHALL pass a kind's statechart, each stage parameter of it, a
session held as a stage of its place, and a statechart place's held
engine state, answers and stage entries as types that round-trip
through JSON, each new field left out while empty, so that a kind and a
held state written before read unchanged (21, 103, 177).

#### Scenario: Round trips

- **WHEN** a statechart kind, a held session and a statechart place's held state are written and read back, and a kind and a held state written before this change are read
- **THEN** each reads back equal, and the earlier two read with no statechart, no session and no engine state (21, 177)
