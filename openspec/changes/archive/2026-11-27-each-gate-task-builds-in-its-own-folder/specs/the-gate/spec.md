## ADDED Requirements

### Requirement: Each task that builds a binary builds it in its own folder

Each task that builds a binary of the workspace SHALL build it in a
target folder that no other such task uses. Thus no build of one task
writes the binary that the tests of another task run. `root:targets`
SHALL fail where two tasks build one binary in one
folder, and SHALL name the tasks and the folder (core.21).

#### Scenario: Two tasks build the flume binary in one folder

- **WHEN** `root:targets` reads a tree where `flume:test` and `flume:tools` set no `CARGO_TARGET_DIR`
- **THEN** it fails and names `flume:test`, `flume:tools` and the folder `target`

#### Scenario: A script that a task runs builds the binary

- **WHEN** `root:targets` reads a tree where a task runs a script that runs `cargo build -p flume`, and that task and `flume:test` set no `CARGO_TARGET_DIR`
- **THEN** it fails and names the two tasks

#### Scenario: The tree as it is

- **WHEN** `root:targets` reads the tree
- **THEN** it passes, and `flume:tools`, `flume:live`, `plugin-claude:test` and `conformance:coverage` each build in a folder of their own under `target/`
