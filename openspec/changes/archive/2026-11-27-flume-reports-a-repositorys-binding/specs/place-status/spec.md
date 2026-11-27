## ADDED Requirements

### Requirement: Status gives each repository its ask names

A status ask SHALL be able to name repositories by their folders. For
each one, status SHALL give the binding at the default branch as the
repository holds it, and the merge queue that the binding names. It
SHALL give each kind that flume can run there, whether it lands, and the
words of the binding's refusal of it (20, 105, core.3). It SHALL give no
`repositories` field where the ask names no repository (core.14).

#### Scenario: A merge queue that lands the default branch

- **WHEN** status names a repository that binds a merge queue which lands its default branch, and a change profile
- **THEN** the merge queue is ready, and no kind is refused

#### Scenario: A kind that reads a change

- **WHEN** status names a repository that binds a ready merge queue and no change profile
- **THEN** each kind that reads a change is refused with words that say that the repository binds no change profile

#### Scenario: No repository named

- **WHEN** status is asked with no repository
- **THEN** its JSON has no `repositories` field

### Requirement: Status names what a repository lacks and fetches nothing for it

Where the merge queue of a repository is not ready, status SHALL give
words that name the file or the table that is missing. Status SHALL
fetch nothing to read a repository (core.15).

#### Scenario: No declaration

- **WHEN** status names a repository whose default branch holds no `.config/flume.toml`
- **THEN** its merge queue is not ready, and its words say that the branch holds no `.config/flume.toml`. Each of flume's own kinds lands, and the binding refuses it with those words

#### Scenario: A declaration that names no merge queue

- **WHEN** status names a repository whose `.config/flume.toml` has a `[profiles]` table with no `merge_queue`
- **THEN** the merge queue's words say that the declaration names no `merge_queue` in its `[profiles]`

#### Scenario: Nothing fetched

- **WHEN** status names a repository that binds a merge queue
- **THEN** the kinds seam's fetching read of the binding is not made, and the merge queue is asked only whether it lands the default branch
