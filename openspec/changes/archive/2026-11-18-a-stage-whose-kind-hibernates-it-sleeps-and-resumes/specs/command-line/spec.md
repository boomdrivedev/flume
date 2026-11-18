## MODIFIED Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `list`, `prompt`, `message`,
`hibernate`, `resume`, `stop`, `retry`, `finish`, `drop`, `done`,
`tidy`, `settle`, `needs`, `priority` and `open`, each building the
core over the host's profiles and answering the core's answer, printed
for a person or as its JSON with `--json`, exiting non-zero on a
refusal; no command SHALL compute a stage's end, a status, a submission
or a tidy of its own (56 to 61, 186, 200, 212, 213, section 9,
core.14).

#### Scenario: The same answer through the command line and the core

- **WHEN** `flume status <place> --json` runs over a scratch repository and the core is asked the same in a test over the same state
- **THEN** the two answers are equal (S14, 60)

#### Scenario: A refusal

- **WHEN** `flume done` names a fix place
- **THEN** the JSON is a refusal naming the place and the exit is non-zero (S26, 61)

#### Scenario: The list through the command line and the core

- **WHEN** `flume list --json` runs over a scratch host and the core is asked the list in a test over the same state
- **THEN** the two answers are equal (60, 200)

## ADDED Requirements

### Requirement: hibernate and resume name a place

`flume hibernate <place> [--force]` SHALL ask the core to hibernate the
place's current stage, forced only with `--force`, and `flume resume
<place>` SHALL ask it to resume the place's hibernating stage with no
words, each place named as every command names one (31, 47, 212, 213).

#### Scenario: A hibernate refused through the command line

- **WHEN** `flume hibernate <place> --json` names a fix place on a scratch host
- **THEN** the JSON is a refusal naming the place and that its kind does not hibernate its stage, and the exit is non-zero (61, 212)

#### Scenario: A resume of nothing hibernating through the command line

- **WHEN** `flume resume <place> --json` names a place with no hibernating stage on a scratch host
- **THEN** the JSON is a refusal naming the place, and the exit is non-zero (61, 213)

### Requirement: launch gives hibernate_after by --agent

`flume launch --agent [<stage>.]hibernate_after=<time>` SHALL give the
setting in the forms `keep_open` takes, `<n>`, `<n>s`, `<n>m` or
`<n>h`, and SHALL refuse a value of any other form, or one given for a
program, naming the field and the forms (7, 32, 198, 212).

#### Scenario: A stage's time

- **WHEN** a caller launches with `--agent talk.hibernate_after=20m`
- **THEN** the launch's settings give talk `hibernate_after` `20m` (7, 32)

#### Scenario: A time in another form

- **WHEN** a caller launches with `--agent hibernate_after=later`
- **THEN** the launch is refused naming `hibernate_after` and the forms it takes, and nothing is made (7)
