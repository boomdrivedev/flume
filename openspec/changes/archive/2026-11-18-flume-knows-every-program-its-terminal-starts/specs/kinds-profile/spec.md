## ADDED Requirements

### Requirement: A declaration's agents tables give a model

The kinds profile SHALL read `model` from every agents table a
declaration holds, a kind's, a stage's, the repository's for a kind it
does not declare and the repository's `[agents]` for every kind, from
the table weir's crate returns as written: a string, or a table of
programs whose entries are strings, each kept as written; any other, a
table that names no program among it, SHALL fail the declaration
naming the field (23, 158, 172, C.2; core.4).

#### Scenario: A model for every stage and a table for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] model = "sonnet"` and a stage's `agents = [{ model = { claude = "opus", codex = "gpt-5.5" } }]`
- **THEN** the kind's settings read `sonnet` for every stage and the table for that stage (23, 172, C.2)

#### Scenario: A model that is neither

- **WHEN** a declaration's agents table gives `model = 3`, `model = {}` or `model = { claude = 3 }`
- **THEN** the declaration cannot be read, naming the table's `model` (4, 172, C.2)

## MODIFIED Requirements

### Requirement: A declaration's agents tables give an effort

The kinds profile SHALL read `effort` from every agents table a
declaration holds, a kind's, a stage's, the repository's for a kind it
does not declare and the repository's `[agents]` for every kind, from
the table weir's crate returns as written: a string, or a table of
programs whose entries are strings, each kept as written; any other, a
table that names no program among it, SHALL fail the declaration
naming the field (23, 158, 172, 197, C.2; core.4).

#### Scenario: An effort for every stage and one for a stage

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] effort = "high"` and a stage's `agents = [{ effort = "xhigh" }]`
- **THEN** the kind's settings read `high` for every stage and `xhigh` for that stage (23, 197, C.2)

#### Scenario: An effort for every kind of a repository

- **WHEN** a scratch repository's `.config/flume.toml` gives `[agents] effort = "medium"` beside a `[kinds.fix.agents]` table giving `effort = { claude = "high" }`
- **THEN** the kinds read give the repository's settings for every kind with `medium`, and its settings for fix with the table (32, 197, C.2)

#### Scenario: An effort that is no word

- **WHEN** a declaration's agents table gives `effort = 3`, `effort = {}` or `effort = { claude = 3 }`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 197, C.2)
