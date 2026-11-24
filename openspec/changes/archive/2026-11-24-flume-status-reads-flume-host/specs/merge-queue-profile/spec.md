## MODIFIED Requirements

### Requirement: One read for every branch asked at once

The profile SHALL read the newest entry of each branch asked from sluice
host's socket, asking `overview` and `status <base> --all` for each base
the overview names, once for every branch of one repository asked at
once, and SHALL run no sluice command to read it; it SHALL keep each
base's records with the version the overview gave them, and ask that
base's status again only once the version moved (99, C.6;
flume-reads-sluices-records-on-sluice-hosts-socket).

#### Scenario: Every branch asked at once

- **WHEN** read-submission is asked at once of `fix/one` and `fix/two`, each submitted to the pinned sluice's main, and `fix/none`, submitted nowhere
- **THEN** the answer gives `fix/one` and `fix/two` each its own entry and `fix/none` none, and no sluice command ran for it (99, C.6)

#### Scenario: Records kept until their version moves

- **WHEN** a stand-in sluice host answers the overview and each base's status from fixtures, read-submission is asked twice, and then main's records' version moves and it is asked again
- **THEN** the overview is asked at each read, each base's status at the first, and main's alone at the third

### Requirement: read-submission reads a refused record as held

read-submission SHALL read a `refused` record of the status of a base
as held, with its `refused.words`. One that carries no `refused`, as
sluice `44e33d4` prints it, SHALL still be held, its words saying
sluice's record gives none, and never unread. No words SHALL be read
through a wait (99, C.6 at `04e5a1a`; core.6, core.13, core.58).

#### Scenario: A refused record with its words

- **WHEN** a stand-in sluice host answers the status of main with a `refused` record of the place's branch carrying `refused {words, at}`, as sluice `44e33d4`'s `report` prints it
- **THEN** read-submission gives the submission held with those words, and the stand-in's log holds no `wait` and no `status` of a number (99, C.6)

#### Scenario: A refused record without its words

- **WHEN** the stand-in answers with a `refused` record of the place's branch and one of the base place's branch `train/x` by `from.head`, each carrying no `refused`, as sluice `44e33d4`'s `status --all` prints them
- **THEN** read-submission gives each held, with words saying sluice's record gives none of the forge's words, neither unread nor passed over, and the log holds no `wait` (99, 203, C.6; core.58)

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, a place submitted with no agent in it SHALL
land by sluice's host process and SHALL be read and tidied by the
settles of a caller keeping watch, with no other caller's act, no
`sluice wait` and no sluice command that moves a queue. Where that
host process ended before the submission was decided, the settle's
inbox, a sluice command, SHALL start it again, and the place SHALL land
the same (46, 124, 125, 208, C.6, S109; sluice 135).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, whose host process runs as `sluice daemon`, its check held until the watch's first settle has read it open, and the test then runs `flume settle --every 1m` until it lands
- **THEN** no settle ran a `sluice step`, `run`, `daemon` or `wait`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

#### Scenario: A host process started again by a settle's read

- **WHEN** a chore is submitted to the pinned sluice with its check held, sluice's host process is ended while the check waits, and the test then releases the check and runs `flume settle --every 1m` until the chore lands
- **THEN** the settle's inbox started a host process with another pid, the chore landed and was tidied, and no settle ran a `sluice step` (124, 208, S109; sluice 135)

### Requirement: A settle runs no round of sluice's queue

The sluice profile's step SHALL run no sluice command and SHALL answer
that the base's queue moves on its own, whichever of sluice's ways runs
it. The profile SHALL ask sluice of an open submission only its reads:
the inbox, a sluice command, which starts sluice's host process where
work waits and none runs, and read-submission, which asks its socket and
starts none (124, 125, C.6; sluice 135; sluice record 0093 as amended;
core.6; flume-reads-sluices-records-on-sluice-hosts-socket).

#### Scenario: A step over a stand-in sluice

- **WHEN** the profile's step is asked of `main` with a stand-in `sluice` first on its `PATH` that leaves a mark whenever it runs
- **THEN** it answers that `main`'s queue moves on its own, and the stand-in left no mark (125, C.6)

#### Scenario: A queue the settles never step

- **WHEN** a place's entry is submitted to `main` of a scratch repository through the pinned sluice, sluice's host process runs as `sluice daemon` and no `sluice run` is live, and the profile's step, read-submission and inbox are asked as a settle asks them until the entry is decided
- **THEN** sluice's host process lands the entry, each step answers that the queue moves on its own, and every sluice command the profile ran after the submit was `inbox` or `confirm` (124, 125, C.6; sluice 135)

### Requirement: Read, base and the place's data

The profile SHALL read each branch's newest entry as "One read for every
branch asked at once" says; answer the base at `origin/<base>` after one
fetch where `.config/sluice.toml` lands the base on a remote and the
local base otherwise; and give the place profile the readiness, `sluice
equip <base> --at <revision> --json` with the reading of its steps, the
teardown (`sluice release`) and the base's ref, reading no `[tests]` key
for a place (9, 11, 20, 126, C.1, C.6, core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on `main` of a scratch repository with sluice's host process run as `sluice daemon` and no `sluice run` live, and read-submission reads it with no sluice command of the test's or flume's stepping `main`
- **THEN** it reads the entry open until sluice's host process decides it, and then reads the entry landed (99, 124, 125; sluice 135)

#### Scenario: A fetch that fails

- **WHEN** the base lands on a remote that refuses the fetch
- **THEN** base answers why, and the launch is refused naming it (9, core.7)

#### Scenario: What the policy says of a place

- **WHEN** the policy at main names the moon runner, the toolchain `devenv print-dev-env` and `origin` as main's remote, and a place on main is readied at the revision R its launch read
- **THEN** the place's data is one readiness step, `sluice equip main --at R --json`, with the profile's reading of it, `sluice release` as its teardown and `origin` as the base's remote, and names no toolchain and no `moon setup` (11, 20, C.1, C.6)

#### Scenario: The steps sluice equipped with

- **WHEN** `sluice equip` answers the steps `toolchain`, `proto install` and `moon --version` passed and `moon setup` failed with its words, exiting 1
- **THEN** the reading gives those four steps in that order, each with its outcome, the last with sluice's words cut as any step's are (12, 79, C.6)

#### Scenario: An answer with no steps

- **WHEN** `sluice equip` answers a refusal with its `error.message`, or nothing the profile can read, or cannot be run
- **THEN** the reading gives one failed step, `sluice equip`, with sluice's message, its errors, or why it could not run (12, C.6)
