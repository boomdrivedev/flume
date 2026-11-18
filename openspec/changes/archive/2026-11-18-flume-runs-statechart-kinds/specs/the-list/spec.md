## ADDED Requirements

### Requirement: The list names a statechart place's live stages

The list SHALL give a place of a statechart kind a line naming its
kind at its version, each stage whose sessions are live by its state's
path, and the place's state in the words status gives it, read from its
held state alone (200, core.52).

#### Scenario: Persona's line

- **WHEN** persona's test stage runs and the list is asked
- **THEN** its line names `persona@1`, `stages.test` and its state, and the list reads no worktree, branch or deliverable for it (200, core.52)
