## ADDED Requirements

### Requirement: A pool names the program of a stage whose layers name none

Where no layer above flume's default names a stage's program and its
agent has a pool, the account chosen SHALL name the program, the
stage's usable accounts being those whose program can run it: a
profile binds the program, the stage's mode maps on it, its model is
given for it or none is given, its effort is taken or none is given,
and the own settings given for it are taken or none are given; the
start SHALL pass that program's entries (32, 144, 169, 172, 173, 196,
197).

#### Scenario: A mode on a pool of two programs

- **WHEN** a kind's `[kinds.review.agents]` gives `mode = "edits"` and names no program, its stream's fill-first pool holds agent-1 of `agent`, bounded at one agent, and other-1 of `other`, each fake profile mapping `edits` to a value of its own, and two places of the kind start review; then the host gives `[kinds.review.agents] mode = { agent = "own-auto" }`, which `agent`'s fake profile names among its own modes, and a third place starts review once agent-1 has room
- **THEN** the first review starts on agent-1 passed `agent`'s value for `edits` and the second on other-1 passed `other`'s, status naming `edits` from `kind` for both; the third starts on agent-1 passed `own-auto`, status naming `host-kind` and its `agent` entry, and other-1 is not usable for it, since the host's table is taken whole and names no entry for `other` (32, 144, 169, 173, S71)

#### Scenario: A model only one program takes

- **WHEN** a stage names no program and gives `model = "large"`, and its fill-first pool holds other-1 first, whose fake profile names no way to give a model, and agent-1, whose fake profile takes `large`
- **THEN** the stage starts on agent-1, passed `large`, other-1 not usable for it (172, 173)

#### Scenario: A program no profile binds

- **WHEN** a stage names no program and its fill-first pool holds nobody-1 of `nothing-here`, which no profile binds, first and agent-1 second
- **THEN** the stage starts on agent-1 and nobody-1 is not usable for it (105, 173)

#### Scenario: Effort and own settings only one program takes

- **WHEN** a stage names no program and gives `effort = "high"` and own settings for `agent` alone, and its fill-first pool holds other-1 first, whose fake profile takes no effort, and agent-1, whose fake profile takes `high` and the keys given
- **THEN** the stage starts on agent-1 passed `high` and the keys given, other-1 not usable for it (173, 196, 197)

#### Scenario: No pool

- **WHEN** a stage names no program and neither its stream nor the host names a pool
- **THEN** its agent runs flume's default program, from `default`, under no account (32, 144, 173)

### Requirement: A stage whose layers name no program waits for an account that can run it

A stage whose layers name no program and whose pool holds an account
that can run it, spent, past its threshold or full, and none usable
SHALL wait for room, its want naming the pool and the soonest time an
account that can run it is back; the accounts that cannot run it SHALL
not be counted (145, 173).

#### Scenario: The only account that can run it is spent

- **WHEN** a stage names no program and gives `mode = { agent = "edits" }`, and its pool holds agent-1 of `agent`, spent until 15:00, and other-1 of `other`, usable
- **THEN** the stage waits for room, nothing starts on other-1, and status gives the pool with 15:00 as the soonest time an account is back (145, 173)

### Requirement: A pool that holds no account that can run a stage refuses it

A launch SHALL be refused, with nothing made, where a stage whose
layers name no program has a pool that holds no account whose program
can run it, naming the stage, the pool and each of its programs with
why it cannot. At a later start, read against the host's configuration
as it stands, such a stage SHALL be refused so and start no agent
until a retry, even while it waits for room (4, 32, 145, 173; core.8,
core.32).

#### Scenario: A launch no account can run

- **WHEN** a kind's code stage names no program and gives `mode = { agent = "edits" }`, and a place of it is launched on a stream whose pool holds only other-1 of `other`
- **THEN** the launch is refused naming code, the pool and `other`, with the mode's table naming no entry for `other`, and nothing is made (4, 145, 173)

#### Scenario: The account that could run it dropped before a later start

- **WHEN** a place's review names no program and gives `mode = { agent = "edits" }`, its pool holds agent-1 and other-1 at its launch, and the host's configuration drops agent-1 from the pool while code works
- **THEN** once code delivers, review is refused naming the pool and `other` with why, no agent is opened, and status has the place waiting on its caller's retry; a retry once the pool holds agent-1 again starts review on it (32, 145, 173)

### Requirement: An intent keeps the program its last agent ran

flume SHALL keep, in each intent's affinity, the program its last agent
ran. A start of a stage whose layers name no program SHALL take the
account of the affinity for that program while it is usable and can
run the stage, and otherwise go where the strategy chooses among the
accounts that can run it, the affinity for the chosen account's
program written or moved as for a stage that names it (147, 173).

#### Scenario: A stage that names no program keeps to the last program

- **WHEN** a fill-first pool holds agent-1 of `agent` and then other-1 of `other`, and a place's code names `other` and its review names no program
- **THEN** code starts on other-1 and review on other-1 from the affinity, though fill-first's first account is agent-1, and no move is recorded (147, 173, S72)

#### Scenario: The last program cannot run the stage

- **WHEN** as above, but review gives `mode = { agent = "edits" }`
- **THEN** review starts on agent-1 from the strategy, no move is recorded, and the affinity keeps other-1 for `other` and agent-1 for `agent`, its last program `agent` (147, 173)

#### Scenario: The last program's account is spent

- **WHEN** a stage names no program, the intent's last program is `other` with other-1, and other-1 is spent while other-2 of `other` and agent-1 of `agent` are usable in a fill-first pool ordered agent-1, other-1, other-2
- **THEN** the stage starts on agent-1 from the strategy, the affinity keeping other-1 for `other`, recording no move for `other`, and writing agent-1 for `agent`, its last program `agent` (147, 173)

### Requirement: A conversation stays with its program

A start of a stage whose layers name no program that carries a lost or
hibernated conversation SHALL take the account its earlier start ran
under first while that account is usable and can run the stage; a
spent agent's stage that moves to an account of another program SHALL
start afresh, status saying its conversation stayed and why (145, 146,
179, 213).

#### Scenario: A spent agent's stage moves to another program

- **WHEN** a stage names no program, its agent on other-1 of `other` stops at its usage limit, and the pool's only usable account that can run it is agent-1 of `agent`
- **THEN** once quiet the agent is ended and the stage starts afresh on agent-1, its program `agent` from `account`, status saying it moved from other-1 to agent-1 because other-1 was spent and that its conversation stayed because agent-1 runs another program (146, 173, 179)

#### Scenario: A lost conversation's retry

- **WHEN** a stage names no program, its agent on other-1 of `other` is reported lost, and its caller retries it while other-1 is usable
- **THEN** the start takes other-1 for its conversation, its program `other` from `account`, and no affinity moves (145, 173, 179)

### Requirement: A stage's prompt is previewed on the account a start would choose

`flume prompt` of a stage whose layers name no program and whose agent
has a pool SHALL fill the prompt with the profile of the program of the
account a start would choose now, else of the first account in the
pool's order that can run the stage, and SHALL write nothing (29, 119,
173).

#### Scenario: The preview of a stage on the last program

- **WHEN** a stage names no program, its intent's last program is `other` with other-1 usable, `other`'s fake profile gives its own way to stop short, and `flume prompt` renders the stage
- **THEN** the prompt holds `other`'s way to stop short, and no held state, affinity or account document is written (29, 119, 173)
