## ADDED Requirements

### Requirement: A place whose agent runs in Rex is relayed as one under herdr is

The core SHALL relay a place whose current stage's agent runs in Rex
through the Rex profile's relay, the terminal its start recorded, with
the same lines, inputs, ends and nothing kept as under herdr (107, 190,
221, 222, C.5; core.49, core.57).

#### Scenario: Through the binary under Rex

- **WHEN** a fix is launched on a stream whose multiplexer is Rex, with the stand-in agent program echoing what is typed, `flume relay fix--one` runs once the fix's agent has said it started, the text `relay-marker` is written to its input and then the input is closed
- **THEN** its first line is a screen of the agent's block, a later line is a screen holding `relay-marker`, its last line is the relayed answer naming the agent of code's first start, ended by the caller, it exits 0, the place's held state is at the version it was before, and no file under the host's state directory holds `relay-marker` (107, 221, 222, C.5)
