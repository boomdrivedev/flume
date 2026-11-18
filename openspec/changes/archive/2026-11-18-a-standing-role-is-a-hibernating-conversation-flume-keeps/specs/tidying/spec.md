## ADDED Requirements

### Requirement: A standing role's place is tidied by its checkouts

A standing role's place whose work is over SHALL be tidied by reading
each checkout: one that cannot be read, or that holds a change not
committed, a file neither committed nor ignored, or a lock, SHALL keep
the place whole, named; otherwise every checkout and the location SHALL
be removed with no teardown, and no branch removed or kept for it. A
wake undone SHALL remove what it made the same way (2, 62, 65, 76;
core.18).

#### Scenario: Finished and removed

- **WHEN** `east/ops`'s talk is finished and its checkouts hold nothing
- **THEN** the tidy removes both checkouts and the location, removes no branch, and gives no branch kept

#### Scenario: A checkout holding a file

- **WHEN** `east/ops`'s place is done while its `blueprints` checkout holds an untracked file
- **THEN** the place is kept, neither checkout removed, and status names the file under `blueprints` (65)

#### Scenario: The waking command gone

- **WHEN** the command that woke `east/ops` is gone while its place was being made, one checkout made
- **THEN** the next settle removes that checkout and the location, and no held state of the place stands (2, 76)
