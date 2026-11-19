## MODIFIED Requirements

### Requirement: An effort is its program's own word

An agent's effort SHALL be its program's own word, kept as written, or
a table of programs, each entry its program's own word, kept as
written, taken whole as a model's table is; flume SHALL map no effort
from one program to another and SHALL define no scale of its own. A
table that names no program, a number or any other value SHALL be
refused naming its table and `effort` (172, 197, C.2).

#### Scenario: A word kept as written

- **WHEN** the kind's agents table gives `effort = "xhigh"` and a fix's agent runs the fake program `agent`
- **THEN** the agent's resolved settings give the effort `xhigh` as written (197)

#### Scenario: A table refused

- **WHEN** the kind's agents table gives `effort = {}`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 197)

#### Scenario: A table's entry passed

- **WHEN** the fake agent takes `low` and `high`, and the kind's agents table gives `effort = { agent = "high", other = "max" }`
- **THEN** the fix's agent is asked `high` and its command passes `high`, status giving the effort as the table was written from `kind` and the entry `agent` (56, 197)

## ADDED Requirements

### Requirement: An effort's table entry is the agent's program's, never a later layer's

The agent's entry of an effort's table SHALL be the entry for its
program's name. A table that names no entry for the agent's program
SHALL refuse the launch, or the stage at a later start, naming the
table as written, the program, the stage and the layer, and SHALL never
pass to a later layer's effort (4, 32, 169, 197).

#### Scenario: No entry for the agent's program

- **WHEN** the fake agent takes `low` and `high`, the kind's declaration gives `effort = "high"`, and the launch gives `--agent effort.other=low`
- **THEN** the launch is refused naming `{ other = "low" }`, the program `agent`, the stage and `launch`, the kind's `high` is not taken, and nothing is made (4, 197)
