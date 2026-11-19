## MODIFIED Requirements

### Requirement: The agent seam says which parts its profile builds

The agent seam SHALL give, for its program, which of the thirteen
parts 171 names a profile may degrade its profile builds from the
program's own record, and for each other that it is degraded, with how
flume reads it; none built by default. Whether it offers send-back and
on-quiet, reports windows and reads its mode back SHALL be read from
them, and a part not built SHALL be answered the lesser way where it is
asked (93, 171, 174).

#### Scenario: A profile that builds no part

- **WHEN** an agent profile that keeps the seam's defaults is asked its parts
- **THEN** each of the thirteen is degraded, in 171's order, each with how flume reads it: received, asking, own work, ended and last words from the terminal, spent and windows unknown, send-back, guard, on-quiet and carry not offered, the mode recorded and not compared, and own settings neither reserved nor taken; and it offers neither send-back nor on-quiet, reports no windows and reads no mode back (174)

#### Scenario: A profile's answers agree with its parts

- **WHEN** the agent seam's scenarios run against a profile, the fake agent among them building every part, reading the five from its terminal, carrying none, taking no own settings, reporting no windows and offering no hooks
- **THEN** its parts name each of the thirteen once; send-back, on-quiet, windows and the mode's read-back are offered exactly where its parts build them; and each part it does not build answers the lesser way, received, asking, own work, ended, spent and windows unknown, last words none, guard and carry not offered, own settings given refused and no key reserved (93, 174)

### Requirement: The type of a part

The seams SHALL pass one type for a part a profile may degrade, which
SHALL serialize to its word, `received`, `asking`, `own_work`, `ended`,
`last_words`, `spent`, `windows`, `send_back`, `guard`, `on_quiet`,
`mode`, `carry` or `own_settings`, and back unchanged, a word that is
no part refused naming it; each part SHALL give its name as the
statement writes it, and how flume reads it where its profile does not
build it, naming no terminal (171, 174; core.1).

#### Scenario: Each part written and read

- **WHEN** each of the thirteen parts is written to JSON and read back, and the word `colour` is read as a part
- **THEN** each reads back unchanged from its word, and `colour` is refused naming it (174)

#### Scenario: A part's name and its lesser reading

- **WHEN** the name of `send_back`, of `own_work` and of `own_settings` is read, and how flume reads `carry` and `own_work` where its profile does not build them
- **THEN** the names are `send-back`, `own work` and `own settings`; carry reads as not offered, a stage whose agent stopped at its account's usage limit starting afresh, and own work as read from the terminal, none once two settles a minute or more apart read the agent waiting (171, 174)

## ADDED Requirements

### Requirement: The terminal seam sends and watches the turn

The terminal seam SHALL send text to an agent as its send does, its
held input cleared first, and answer whether it saw the turn the
sending began, the agent working or asking, within a bound it is given;
that it saw none, with its words; or a refusal. A terminal that reads
no turn SHALL refuse it, saying so, with nothing typed: the seam's
default (89, 95, 171, C.5).

#### Scenario: A terminal that keeps the seam's default reads no turn

- **WHEN** a terminal that answers only the seam's required operations is asked to send `go` to an agent and watch its turn
- **THEN** it refuses, saying it reads no turn a sending began (95, 171)

#### Scenario: A watched send's answers in JSON

- **WHEN** a turn seen, a turn not seen with words, and a refusal with words are each written to JSON and read back
- **THEN** each reads back unchanged (95)

### Requirement: A start holds when a settle read its agent waiting

A start's held record SHALL carry when a settle first read its agent
waiting and when one last did, none while none has or a settle read it
otherwise, left out of the JSON while none and none in a start written
before (31, 171; core.16).

#### Scenario: A start read waiting

- **WHEN** a start whose agent a settle first read waiting at 100 and last at 160 is written to JSON and read back
- **THEN** it reads back unchanged with both times (171)

#### Scenario: A start written before

- **WHEN** a start written before, with no field for its agent's waiting, is read
- **THEN** it reads with none, and writes back with no such field (171; core.16)
