## ADDED Requirements

### Requirement: status --binding names the repository

`flume status --binding` SHALL ask `flume host` for the binding of the
repository that the command runs in, or of the one that `--repository`
names, beside the places, and print it. It SHALL refuse `--binding`
with `--line`. `flume host` SHALL refuse an ask that names repositories
and asks lines.

#### Scenario: Asked in a repository

- **WHEN** `flume status --binding --json` runs in a repository that binds no merge queue, with `flume host` running
- **THEN** `status.repositories[0]` names the repository's folder, its merge queue is not ready, and flume/fix@1 lands and is refused

#### Scenario: With a line

- **WHEN** `flume status --binding --line` runs
- **THEN** it is refused
