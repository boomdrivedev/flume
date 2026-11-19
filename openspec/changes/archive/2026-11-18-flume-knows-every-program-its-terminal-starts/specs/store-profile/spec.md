## ADDED Requirements

### Requirement: A model in the host's agents tables

A model in an agents table of the host's configuration SHALL be read
as a string or a table of programs whose entries are strings, each
kept as written, in every agents table the host's configuration holds,
and as a key the local layer may set over the managed one; any other
value, a table that names no program among it, SHALL refuse the
configuration naming the table and `model` (32, 172, C.7).

#### Scenario: A model for a kind and a table for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] model = "sonnet"` and `[repos."/r".kinds.fix.agents.stages.code] model = { claude = "opus", codex = "gpt-5.5" }`
- **THEN** the host's settings for fix@1 in `/r` read `sonnet` for the kind at the host-kind layer and the table for code at the host-repository layer (32, 172, C.7)

#### Scenario: A model that is neither

- **WHEN** the configuration gives `[kinds.fix.agents] model = 4`, or `model = {}`, or `model = { claude = 4 }`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `model` (172, C.7)

## MODIFIED Requirements

### Requirement: An effort in the host's agents tables

An effort in an agents table of the host's configuration SHALL be read
as a string or a table of programs whose entries are strings, each
kept as written, in every agents table the host's configuration holds,
and as a key the local layer may set over the managed one; any other
value, a table that names no program among it, SHALL refuse the
configuration naming the table and `effort` (32, 197, C.7).

#### Scenario: An effort for a kind and one for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] effort = "high"` and `[repos."/r".kinds.fix.agents.stages.code] effort = { claude = "xhigh" }`
- **THEN** the host's settings for fix@1 in `/r` read `high` for the kind at the host-kind layer and the table for code at the host-repository layer (32, 197, C.7)

#### Scenario: An effort for every agent from the local layer

- **WHEN** the managed layer gives `[agents] effort = "medium"` and the local layer gives `[agents] effort = "high"`
- **THEN** the host's settings for every agent read `high` at the host layer, and status names the local layer as where it came from (32, 197, C.7)

#### Scenario: An effort that is no word

- **WHEN** the configuration gives `[kinds.fix.agents] effort = 3`, or `effort = {}`, or `effort = { agent = 3 }`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `effort` (197, C.7)
