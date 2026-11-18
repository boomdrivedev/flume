## ADDED Requirements

### Requirement: The fake kinds reads a held statechart kind

The fake kinds SHALL take a statechart kind scripted as the seam's kind
with its text, and SHALL read a held statechart kind's text and the
stage template's through weir's crate as the kinds profile does, so the
engine's scenarios tick the statechart a kind's text declares (106,
176).

#### Scenario: A scripted statechart kind read back

- **WHEN** the fake kinds is scripted with a statechart kind's text and asked to read it back with the stage template's
- **THEN** it gives the statechart that text declares and the template `stage` (106, 176)

### Requirement: The fake git gives a revision's files

The fake git SHALL give, at a revision, the path of every file a commit
at or before it on its branch touched (106, 178).

#### Scenario: Files at two revisions

- **WHEN** the fake git commits `personas/a.md`, then `personas/b.md`
- **THEN** the first revision's files name `personas/a.md` alone, and the second's name both (178)
