## ADDED Requirements

### Requirement: The fake merge queue holds a landing the forge refused

The fake merge queue SHALL take a plan whose round holds the
submission, its record held with the forge's words or with none, and a
held outcome left in its place's inbox. It SHALL land a held submission
a test pushes again, queue one whose landing a test drops, and withdraw
one whose branch is submitted again (106, 112, 124; sluice 67, 152).

#### Scenario: Held, pushed again, dropped and replaced

- **WHEN** a test plans a held landing for a submission of `fix/x` and runs a round, collects the inbox, runs another round, pushes it again; then holds a second submission, drops its landing, holds a third, and submits `fix/x` again
- **THEN** the first reads held with the words and the inbox gives it held once, the second round leaves it held, and the push lands it; the second reads waiting once dropped; the third reads withdrawn once the branch is submitted again (112, 124)
