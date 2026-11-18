## ADDED Requirements

### Requirement: Status gives a standing role's address and place

Status SHALL give every address whose standing role's place stands,
with that place and since when it hibernates where it does; and of a
standing role's place, its branch empty and each checkout with its
repository, folder, base, revision, when it was moved, where its base
was read, its last move not made with when and why, and what it holds,
read at the ask, in the JSON and the person's form (56, 80, 193, 214;
core.14, core.15).

#### Scenario: A hibernating standing role with nothing pending

- **WHEN** `east/ops`'s place hibernates and nothing is pending at `east/ops`
- **THEN** status gives `east/ops` among the host's addresses, with `standing_role` naming its place and since when it hibernates, and the person's form gives the same under the address

#### Scenario: Its checkouts

- **WHEN** status of `east/ops`'s place is asked, its `blueprints` checkout holding an untracked file and its last move not made
- **THEN** it gives two checkouts, `blueprints`'s holding the file and its move not made with when, the tip and the words, and the person's form gives no branch line for the place

#### Scenario: A place of a run

- **WHEN** status of a fix's place is asked
- **THEN** it gives no checkouts, and its JSON's other fields are as before
