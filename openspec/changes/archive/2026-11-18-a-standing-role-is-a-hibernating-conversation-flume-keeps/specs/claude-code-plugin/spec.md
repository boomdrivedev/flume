## MODIFIED Requirements

### Requirement: The MCP tools take the command line's words

`flume mcp`'s `message` SHALL take `target`, `words`, `within` and
`answers`; its `launch`, `kind`, `name`, `brief` and the command's other
words and switches by their long names, `base`, `recipient`, `hold`,
`hold_submission`, `mark`, `priority`, `stall_after`, `agent`, `stream`
and `intent`, arrays where the flag repeats; its `wake`, `address`; its
`hibernate`, `place` and `force`; its `resume`, `place`; and `status`
alone SHALL be read-only (7, 53, 60, 132, 212, 213).

#### Scenario: The tools listed

- **WHEN** a scripted MCP session lists `flume mcp`'s tools
- **THEN** it lists `status`, `message`, `launch`, `wake`, `hibernate` and `resume`, each with its input schema, `status` alone read-only

## ADDED Requirements

### Requirement: hibernate and resume over MCP

`flume mcp` SHALL serve `hibernate` and `resume`, each turned into the
command line's arguments and run by its parser and act, its result the
act's JSON and a refusal marked an error (60, 212, 213, C.2).

#### Scenario: A hibernate refused as the command line refuses it

- **WHEN** a scripted MCP session calls `hibernate` naming a fix's place
- **THEN** the tool's result is the refusal `flume hibernate` gives, naming the place, marked an error

#### Scenario: A resume of nothing hibernating

- **WHEN** a scripted MCP session calls `resume` naming a place whose stage does not hibernate
- **THEN** the tool's result is the refusal `flume resume` gives, marked an error, and nothing is started
