## ADDED Requirements

### Requirement: The place seam makes, moves and removes a standing role's checkouts

The place seam SHALL make a standing role's checkouts under a location,
each detached at its revision, refusing where anything stands there and
undoing every one made where one is refused; move a checkout to a
revision, keeping it where it stands with the profile's words where the
move would overwrite a change; and remove the checkouts and what stands
at the location, running no teardown (78, 80, 81, 83, 193, C.1).

#### Scenario: Made under the location

- **WHEN** the place seam is asked to make two checkouts of two repositories at their revisions under a free location
- **THEN** each stands at `<location>/<folder>`, its head the revision, on no branch, and read gives what it holds

#### Scenario: Something stands at the location

- **WHEN** something already stands at the location
- **THEN** make is refused as an existing location, and no checkout is made

#### Scenario: A move kept over a change

- **WHEN** a checkout holds an uncommitted change to a file the revision it is moved to changes
- **THEN** the move answers kept with the profile's words, and the checkout's head and its change are as they were

#### Scenario: Removed with no teardown

- **WHEN** the checkouts are removed
- **THEN** no checkout and nothing at the location stands, no teardown step ran, and no branch was removed

### Requirement: The types of a checkout

The seams SHALL pass a checkout to make as its repository, its folder
and its revision; a move's outcome as moved, kept with its words, or
gone; a standing role's place's held checkouts with its repository, its
address, its folder, its base, its binding, its revision, where its base
was read, when it was moved and its last move not made; and a resume by
`delivery` (56, 193, 213, 214).

#### Scenario: Round-trip

- **WHEN** a held state with two checkouts, one with a move not made, and a start with a resume by `delivery`, is written and read again
- **THEN** each reads as written

#### Scenario: A held state written before

- **WHEN** a held state written before a place kept checkouts is read
- **THEN** it reads with no checkouts, and its place is no standing role's
