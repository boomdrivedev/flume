## ADDED Requirements

### Requirement: launched takes the start from its environment

`flume ack launched -- <program> [<argument>…]` SHALL take the start it
runs for from `FLUME_AGENT` and `FLUME_SECRET`, refused when either is
missing or not the current start's, running nothing and exiting
non-zero; its help SHALL say it is run by a terminal flume opened, in
the program's place, and becomes the program (41, 171, C.4; core.35).

#### Scenario: launched from outside an agent

- **WHEN** `flume ack launched -- /bin/echo hello` runs with no `FLUME_AGENT`
- **THEN** it is refused naming `FLUME_AGENT`, nothing prints `hello`, and the exit is non-zero (41, C.4)

#### Scenario: launched's help

- **WHEN** `flume ack --help` runs
- **THEN** it lists `launched` beside `started` and `received`, saying a terminal flume opened runs it in the program's place and that it becomes the program (C.4; core.35)
