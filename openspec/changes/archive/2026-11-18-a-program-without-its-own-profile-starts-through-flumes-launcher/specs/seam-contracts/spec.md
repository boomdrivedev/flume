## ADDED Requirements

### Requirement: The agent seam says whether a start runs through the launcher

The agent seam SHALL say whether a profile starts its program through
flume's launcher, not by default, and SHALL give, read in the place's
own folder, the process its start's launcher wrote that it became, none
by default and none before the launcher's word (35, 171, C.4).

#### Scenario: A profile that keeps the seam's defaults names no launcher

- **WHEN** a profile that answers only the seam's required parts is asked whether it starts its program through the launcher, and the process of a start
- **THEN** it says it does not, and names no process (35)

### Requirement: An agent's command carries the launcher's words

An agent's command SHALL carry the words that run its program through
flume's launcher, the launcher's program and its arguments, after which
a terminal puts the program and its arguments; none by default, left
out of the JSON while none, and none in a command written before
(88, C.4, C.5).

#### Scenario: A command with the launcher's words and one without

- **WHEN** a command naming the launcher `/opt/flume/bin/flume` with `ack launched --`, and one naming none, are each written to JSON and read back, and a command written before is read
- **THEN** each reads back unchanged, the second's JSON holds no launcher, and the one written before reads with none (88, C.4)

### Requirement: The terminal seam reads the process an agent's program runs in

The terminal seam SHALL give the process the terminal reads running an
agent's program, by its id; that nothing runs it, where the agent's
part holds no such process or is gone; or unknown, in the terminal's
words. A terminal that reads no process SHALL answer unknown, saying
so, the seam's default (35, 171, C.5).

#### Scenario: A terminal that keeps the seam's default reads no process

- **WHEN** a terminal that answers only the seam's required operations is asked the process of an agent
- **THEN** it answers unknown, saying the terminal reads no process (35, C.5)

#### Scenario: The process's answers in JSON

- **WHEN** a process read running as 41, nothing running, and unknown with words are each written to JSON and read back
- **THEN** each reads back unchanged (C.5)

### Requirement: A start begun in another process says so

How a start began otherwise SHALL carry whether its program runs in
another process than the one its start's launcher became, false by
default, left out of the JSON while false and false in a word written
before, and its words SHALL say so, naming no process's id (161, S74).

#### Scenario: Begun in another process

- **WHEN** a start begun in another process is written to JSON and read back, and its words are read
- **THEN** it reads back unchanged, and its words say it began in another process than the one its start's launcher became (161, S74)

#### Scenario: A word written before

- **WHEN** a word of how a start began otherwise, written before, with no field for the process, is read
- **THEN** it reads with the process false, and writes back with no such field (161)
