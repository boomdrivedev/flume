# agent-programs Specification

## Purpose
The programs flume runs: an entry for each kind its terminal starts,
its name and kind, bound by its own profile or the common profile; an
entry's ways to pass a model, an effort and a mode only from the
release the tree pins; the common profile's command and the seam's
defaults it answers; a launch naming any of them taken where its
terminal starts its kind, and what an entry cannot take refused naming
the program (170, 172, 197, C.4, C.5).

## Requirements

### Requirement: An entry for every program the terminal starts

flume's agent profiles SHALL hold one entry for each kind herdr's
pinned release starts, each giving its program's name and herdr's kind
for it, that name, and none for any other program (170, C.4, C.5).

#### Scenario: herdr's kinds, each with its entry

- **WHEN** the table is read beside herdr's pinned kinds
- **THEN** it holds one entry per kind, each named as its kind, and none for any other program (170, C.4)

### Requirement: The resolver binds every entry and nothing else

The resolver SHALL bind `claude` to Claude Code's profile and every
other entry's program to the common profile over its entry, by its
name or by a path whose file name is its name; a program no entry
names SHALL be bound to no profile, and a launch naming it refused
naming the program, the stage and the layer (4, 105, 170; core.3).

#### Scenario: Every entry bound

- **WHEN** the built-in resolver is asked for each of the 24 programs by name, and for `/opt/amp/bin/amp`
- **THEN** each gives a profile, Claude Code's for `claude` and the common profile for every other, whose kind is the program's own, and the path gives amp's (105, 170)

#### Scenario: A program with no entry

- **WHEN** a fix is launched with `--agent program=nonesuch`
- **THEN** the launch is refused naming `nonesuch`, the stage and `launch`, and nothing is made (4, 170)

### Requirement: An entry's ways only from the release the tree pins

An entry SHALL give a way to pass a model, an effort or a mode only
where that way is checked against the program's release the tree
pins; an entry that gives none SHALL refuse each value given, in words
naming its program. Claude Code's ways are its own profile's; every
other entry, for a program whose release the tree does not pin, SHALL
hold its name and kind alone (106, 170, C.4; core.22, core.39).

#### Scenario: An entry of name and kind

- **WHEN** the common profile over each entry but `claude`'s is asked of the model `opus`, the effort `high` and the mode `edits`
- **THEN** each is refused in words naming that entry's program (170, 172, 197)

### Requirement: A launch naming a program with an entry is taken

A launch whose stages' settings name a program with an entry, and give
it no model, effort, mode or settings of its own that its entry cannot
take, SHALL be taken, wherever the stream's terminal starts its kind;
one that gives any of them for that program SHALL be refused naming the
program, the stage and the layer, leaving nothing made (4, 8, 170, 172,
197).

#### Scenario: A program the common profile binds, given nothing

- **WHEN** `flume launch --kind fix --agent program=amp` runs over a scratch repository
- **THEN** the launch is taken and the place stands, its stage's program `amp` from `launch` (170)

#### Scenario: The same program given a model

- **WHEN** `flume launch --kind fix --agent program=amp --agent model=opus` runs
- **THEN** it is refused naming `amp`, the stage, `launch` and that its entry names no way to give a model, and nothing is made (4, 172, S73)

#### Scenario: Over the fakes, a profile with no ways

- **WHEN** the fake resolver binds `other` to a fake agent scripted to take no model, effort or mode, and a fix is launched with `--agent program=other`, then with each of `model=large`, `effort=high` and `mode=edits` beside it
- **THEN** the first is taken and its agent's command passes none of the three, and each other is refused naming `other`, the stage and `launch`, nothing made (4, 8, 170)

### Requirement: The common profile's command

The common profile SHALL run the program the settings name, by name
or path, in the place: with its entry's flag for each setting given
where the entry names one, the account's environment with the four
`FLUME_*` words over it, its entry's kind, and no prompt argument,
folder-trust answer or file. Its way to stop short SHALL name the
binary that started it by its absolute path (88, 92, 170, C.4;
core.60).

#### Scenario: amp's command

- **WHEN** the common profile over amp's entry gives the command for a start under an account giving `AMP_HOME`, with no model, effort or mode
- **THEN** it runs `amp` with no arguments, the kind `amp`, `AMP_HOME` and the four `FLUME_*` words, and its stop-short names the binary by its absolute path (88, 92, C.4)

### Requirement: The common profile answers the seam's defaults

Each part of the agent seam the common profile reads nothing of SHALL
answer as the seam's default: started, received, asking, own work,
ended, spent, windows and live unknown; last words and conversation
none; send-back, on-quiet, guard, carry and the mode read back not
offered; no reserved key; and own settings refused where any are given
(93, 170, 171, 195).

#### Scenario: Nothing read of amp

- **WHEN** the common profile over amp's entry is asked each reading of an agent it started
- **THEN** each answers the seam's default, and its own settings `{ "x": 1 }` are refused (93, 195)

### Requirement: The stream's terminal starts each stage's program

A launch SHALL ask the stream's terminal whether it starts each stage's
program's kind, before anything is made, and a refusal SHALL refuse the
launch in the terminal's words; each start SHALL ask the terminal its
start records, with the chosen account's environment, and a refusal
SHALL leave the stage refused in the same words, starting no agent,
until a retry (4, 8, 27, 32, C.5).

#### Scenario: A kind the terminal cannot start

- **WHEN** the fake terminal is scripted to refuse the kind `agent` with words naming an install, and a fix is launched; then the script is lifted and the fix launched again
- **THEN** the first launch is refused in those words and nothing is made, and the second is taken (4, 170, S75)

#### Scenario: A later start the terminal refuses

- **WHEN** a change is launched, and the fake terminal refuses the kind `agent` before its verify starts
- **THEN** verify is refused in the terminal's words, no agent starts, and a retry after the refusal is lifted starts it (27, 32)
