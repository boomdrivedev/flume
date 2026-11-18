## REMOVED Requirements

### Requirement: One round at each undecided settle

**Reason**: sluice's host process moves a base's queue once a submit
starts it, whichever of sluice's four ways runs the queue, and a
settle's read of the submission starts it again where work waits and
none runs. A settle drives no round (124, 125, C.6's step row; sluice
135; sluice record 0093 as amended; record 0012 as amended; core.6).

**Migration**: "A settle runs no round of sluice's queue" below.

## ADDED Requirements

### Requirement: A settle runs no round of sluice's queue

The sluice profile's step SHALL run no sluice command and SHALL answer
that the base's queue moves on its own, whichever of sluice's ways runs
it. The profile SHALL ask sluice of an open submission only its reads,
the inbox and read-submission, which start sluice's host process where
work waits and none runs (124, 125, C.6; sluice 135; sluice record 0093
as amended; core.6).

#### Scenario: A step over a stand-in sluice

- **WHEN** the profile's step is asked of `main` with a stand-in `sluice` first on its `PATH` that leaves a mark whenever it runs
- **THEN** it answers that `main`'s queue moves on its own, and the stand-in left no mark (125, C.6)

#### Scenario: A queue the settles never step

- **WHEN** a place's entry is submitted to `main` of a scratch repository through the pinned sluice, no `sluice run` or `sluice daemon` is live, and the profile's step, read-submission and inbox are asked as a settle asks them until the entry is decided
- **THEN** sluice's host process, which the submit started, lands the entry, each step answers that the queue moves on its own, and every sluice command the profile ran after the submit was `inbox`, `confirm`, `overview` or `status` (124, 125, C.6; sluice 135)

## MODIFIED Requirements

### Requirement: Read, base and the place's data

The profile SHALL read the newest entry of a branch from `sluice status
<base> --json`; answer the base at `origin/<base>` after one fetch
where `.config/sluice.toml` lands the base on a remote and the local
base otherwise; and give the place profile the readiness, `sluice equip
<base> --at <revision> --json` with the reading of its steps, the
teardown (`sluice release`) and the base's ref, reading no `[tests]`
key for a place (9, 11, 20, 126, C.1, C.6, core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on `main` of a scratch repository with no `sluice run` or `sluice daemon` live, and read-submission reads it with no sluice command of the test's or flume's stepping `main`
- **THEN** it reads the entry open until sluice's host process, which the submit started, decides it, and then reads the entry landed (99, 124, 125; sluice 135)

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

### Requirement: sluice takes a base place's submission by name from the repository

The sluice profile SHALL submit a branch by name by `sluice submit
<base> --head <branch> --no-wait --json` run in the repository, read as
any submit's answer, and SHALL run no `sluice inbox` for it, reading its
outcome from read-submission's entry whose `from.head` is the branch,
with its words, files and `tested.on`. Its queue SHALL move by
sluice's host process, which the submit starts, with no round run by
flume (99, 117, 124, 125, 133, 203, C.6; sluice 124, 135).

#### Scenario: A base place's branch over a scratch repository

- **WHEN** the pinned sluice, which takes `--head` without `--check` and records no worktree as the submitter of a branch submitted by name, lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix cut from `train/a` is submitted to it and read until it is decided, and the base place's submission of `train/a` by name is read until it is decided, no sluice command of the test's or flume's stepping either queue
- **THEN** sluice made `train/a` at `main`'s revision on the stand-in origin, sluice's host process landed the fix on `train/a` and `train/a` on `main` with no worktree on it anywhere, flume read the landing from `sluice status` with no `sluice inbox` run for it, and closing it into `main` removed it there and on the host, no command of flume's pushing or moving a branch itself (14, 202 to 206, S110)

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, a place submitted with no agent in it SHALL
land by sluice's host process and SHALL be read and tidied by the settles a caller keeping watch runs, `flume
settle --every`, with no other caller's act, no `sluice wait` and no
sluice command of any settle's that moves a queue. Where that host
process ended before the submission was decided, a settle's read SHALL
start it again, and the place SHALL land the same (46, 124, 125, 208,
C.6, S109; sluice 135).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, whose host process the submit starts, its check held until the watch's first settle has read it open, and the test then runs `flume settle --every 1m` until it lands
- **THEN** no settle ran a `sluice step`, `run`, `daemon` or `wait`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

#### Scenario: A host process started again by a settle's read

- **WHEN** a chore is submitted to the pinned sluice with its check held, sluice's host process is ended while the check waits, and the test then releases the check and runs `flume settle --every 1m` until the chore lands
- **THEN** a settle's read started a host process with another pid, the chore landed and was tidied, and no settle ran a `sluice step` (124, 208, S109; sluice 135)
