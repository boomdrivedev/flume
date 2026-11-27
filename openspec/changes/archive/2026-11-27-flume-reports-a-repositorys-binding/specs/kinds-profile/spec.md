## ADDED Requirements

### Requirement: The binding as the repository holds it

The weir profile SHALL read the `[profiles]` table of `.config/flume.toml`
at the default branch with no fetch. It SHALL read it as `origin` was
last fetched, else at the repository's own branch. It SHALL give the
branch, the revision and where it read it. It SHALL give the
declaration's path where that revision holds one. Where it cannot read
the binding, it SHALL give why (20, 105, core.15).

#### Scenario: Origin moved after the last fetch

- **WHEN** origin's default branch binds a merge queue after the clone's last fetch, and the clone's origin cannot be reached
- **THEN** the binding is read at the revision of the last fetch, from `origin`, with no merge queue, and the tracking ref does not move

#### Scenario: No origin and no declaration

- **WHEN** a repository with no `origin` holds no `.config/flume.toml`
- **THEN** the binding is read at its own branch, from `local`, with no declaration and no profile
