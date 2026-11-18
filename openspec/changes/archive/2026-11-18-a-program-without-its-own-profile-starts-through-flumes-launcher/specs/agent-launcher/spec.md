## Purpose

flume's launcher, the start of a program whose profile reads no word of
the program's own: what the launcher compares, the word it writes once
and the program it becomes; and the core's reading of such a start,
counted once the terminal reads the launcher's process ready, and begun
otherwise where the program runs in another process (35, 161, 171,
C.4).

## ADDED Requirements

### Requirement: The launcher becomes the program its start names

`flume ack launched -- <program> [<argument>…]`, run with a start's
`FLUME_AGENT` and `FLUME_SECRET`, SHALL read that start's record from
the core, compare each variable the record names, by its digest keyed
by the secret, and the program it is to run with the record's, and,
where nothing differs, write its word naming its own process and
become the program in that process, its arguments passed whole (35,
161, C.4).

#### Scenario: amp becomes the start's

- **WHEN** a fix is launched with `--agent program=amp` under the printed command, and the launcher runs as its printed line gives it, with a scratch `amp` on `PATH` that writes its process id and its arguments, and `x y` after the program
- **THEN** amp runs with the arguments `x y` in the launcher's own process, and the start's word names that process and nothing differing (35, 161, C.4)

#### Scenario: A program found past the launcher itself

- **WHEN** the launcher is given `amp` by name and the first `amp` on its `PATH` is a link to the launcher's own binary
- **THEN** it runs the next `amp` on `PATH`, never itself (C.4, C.5)

### Requirement: The launcher's word is written once

The launcher SHALL write its word for a start once: a launcher that
finds a word already written for its start SHALL leave it as it stands,
run nothing, and exit non-zero, saying the process the start began in
(35, 161).

#### Scenario: A second launcher under one start

- **WHEN** the launcher runs a second time with amp's start's environment, after its first word was written
- **THEN** it runs nothing, exits non-zero naming the process the first became, and the word is the first's (35, 161)

### Requirement: A start the launcher finds otherwise runs nothing

Where a variable the start's record names is missing or holds another
value, or the program it is to run is another than the record's, the
launcher SHALL write its word naming each variable by its name alone
and whether the program differed, run nothing, start a settle of the
place, and exit non-zero saying the start began otherwise; with no
start under its identity and secret it SHALL write nothing and run
nothing (161, C.4; core.30).

#### Scenario: A variable with another value

- **WHEN** the launcher runs with amp's start's environment, `AMP_HOME` given another value than its account's
- **THEN** amp does not run, the word names `AMP_HOME` and no value, the launcher exits non-zero saying the start began with another `AMP_HOME`, and status reads the start begun otherwise (161; core.30)

#### Scenario: Another program

- **WHEN** the launcher is given `/scratch/other/amp`, and the start's record names `amp`, found first on `PATH` past the launcher at `/scratch/bin/amp`
- **THEN** neither runs, and the word says the program differed (161, C.4)

#### Scenario: No start under its identity and secret

- **WHEN** the launcher runs with amp's start's `FLUME_AGENT` and another secret
- **THEN** it writes no word, runs nothing and exits non-zero saying the secret is not the start's (41, 161)

### Requirement: A start through the launcher counts once its process is read

Where an agent's profile starts its program through flume's launcher,
the core SHALL take its start only once the launcher's word says
nothing differed, the agent's terminal reads the program running in the
process that word names, and reads the agent waiting; until then the
start SHALL stay starting, sent no prompt, and a terminal that cannot
read the process SHALL leave it unknown, never failed (35, 171).

#### Scenario: The word and its process read

- **WHEN** a fix is launched on a fake agent that starts through the launcher, its word names the process 41 with nothing differing, and the fake terminal reads the program running in 41 and the agent waiting
- **THEN** the stage starts and its prompt is sent (35, C.4)

#### Scenario: The word alone

- **WHEN** the word names the process 41, and the fake terminal reads nothing running the program, then reads it running in 41
- **THEN** the stage stays starting and nothing is sent at the first settle, and it starts and its prompt is sent at the next (35)

#### Scenario: A terminal that reads no process

- **WHEN** the word names the process 41, and the fake terminal reads no process, answering unknown
- **THEN** the stage stays starting with nothing sent, and no end is read failed at any settle before the stage's limit (35)

### Requirement: A program in another process is not the start's agent

Where the launcher's word names a process and the terminal reads the
agent's program running in another, the core SHALL read the start begun
otherwise: it SHALL send it no prompt, end it once quiet and start its
stage afresh, as for any start begun otherwise, and the end SHALL say it
began in another process than its launcher became (161, S74).

#### Scenario: amp started again by hand

- **WHEN** a fix's agent started through the launcher in the process 41 and took its prompt, and the fake terminal then reads its program running in the process 42, the agent quiet
- **THEN** flume ends it, starts the stage afresh, and the start's end says it began in another process than the one its start's launcher became (161, S74)

#### Scenario: Before its prompt

- **WHEN** the word names the process 41, and the fake terminal first reads the program running in 42
- **THEN** no prompt is sent, and the start is read begun otherwise, naming the process (161, S74)
