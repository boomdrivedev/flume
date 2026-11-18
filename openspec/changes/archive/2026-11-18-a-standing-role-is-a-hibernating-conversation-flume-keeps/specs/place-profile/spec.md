## ADDED Requirements

### Requirement: A standing role's checkouts are detached git worktrees

The git worktree profile SHALL make each checkout by `git worktree add
--detach` with no hook run, move one by `git checkout --detach` with no
`--force`, and remove each by `git worktree remove` with no `--force`
and no teardown, then what stands at the location and its group folder
once empty, a bare repository's checkout as a clone's (78, 81, 193,
C.1; core.18, core.40; record 0057).

#### Scenario: Checkouts of a bare repository and a clone

- **WHEN** the profile makes checkouts of a scratch bare repository and a scratch clone under a place's location
- **THEN** each is a worktree of its repository, `git status` in it reading `(detached)` at its revision, and no hook of either repository ran

#### Scenario: A move git refuses

- **WHEN** a scratch checkout holds an uncommitted change to a file its base's new head changes, and the profile moves it there
- **THEN** the move answers kept with git's words, and the file and the head are as they were

#### Scenario: A move git carries a change over

- **WHEN** the change is to a file the new head does not touch
- **THEN** the checkout is moved, its change still in it

#### Scenario: Removed to the group folder

- **WHEN** the profile removes the checkouts, the location being the only place under its `places/<id>`
- **THEN** git lists neither worktree, and neither the location nor `places/<id>` stands
