# merge-queue-profile Specification

## Purpose
The merge-queue seam, Part B.6, through sluice: a place's submission
made with no wait on its verdict, its outcomes read from sluice's inbox
at the settles that follow, one round stepped at each settle that finds
it undecided, and where the base lives.

## Requirements

### Requirement: Submitted through sluice with no wait

The sluice profile SHALL submit by `sluice submit <base> --no-wait
--json` in the place, its environment stripped of every multiplexer's
session, `HERDR_*`, `REX_*`, `ZELLIJ*`, `TMUX` and `TMUX_PANE`, never
with `--check` and never through `wt submit`, answering with the entry
once sluice holds it, or the refusal with sluice's words (111, 113, 117,
C.6, core.6).

#### Scenario: A submission from a herdr pane's hook

- **WHEN** the profile submits from a process whose environment carries `HERDR_*` words
- **THEN** sluice's entry names the place's worktree as its submitter and no session, and the answer comes before any verdict

#### Scenario: A submission from a Rex block

- **WHEN** the profile submits from a process whose environment carries `REX_SERVER`, `REX_SESSION` and `REX_BLOCK`
- **THEN** sluice's entry names the place's worktree as its submitter and no session, and the answer comes before any verdict (C.6)

### Requirement: Outcomes collected from sluice's inbox

The profile SHALL collect a place's pending outcomes by `sluice inbox
--json` in the place, never with `--confirm`: landed with its commit,
failed with the merge queue's words, in conflict with the files
contested, or held, its landing refused by the forge, each carrying
sluice's `notice` whole as its words. The profile SHALL read a
withdrawal from read-submission, never from the inbox (99, 101, 133, C.6
at boomdrivedev/blueprints `8b73d11` and `fc4323b`; sluice 51; core.58).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry collected once

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice
- **THEN** the first collect gives the failure with sluice's notice and confirms it by its key, and the second gives nothing (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

#### Scenario: A refused outcome

- **WHEN** a stand-in `sluice` answers the inbox with sluice `44e33d4`'s documents, a `refused` outcome under `7:refused` beside a `landed` one under `6:landed`
- **THEN** the inbox gives the landing and the outcome held with its `notice` whole, runs `sluice inbox --json` with no `--confirm`, and then one `sluice confirm` naming `6:landed` and `7:refused` (112, 133, C.6)

### Requirement: One round at each undecided settle

The profile SHALL drive one round by `sluice step <base>` at each settle
that finds the submission undecided, with no read of whether a `sluice
run` is live, since beside one the step finds the branch's lock held and
does nothing (124, 125; sluice clause 135).

#### Scenario: A step beside a live driver

- **WHEN** a `sluice run` drives the base and a settle steps it
- **THEN** the step does nothing and the settle goes on (124)

### Requirement: Read, base and the place's data

The profile SHALL read the newest entry of a branch from `sluice status
<base> --json`; answer the base at `origin/<base>` after one fetch
where `.config/sluice.toml` lands the base on a remote and the local
base otherwise; and give the place profile the readiness, `sluice equip
<base> --at <revision> --json` with the reading of its steps, the
teardown (`sluice release`) and the base's ref, reading no `[tests]`
key for a place (9, 11, 20, 126, C.1, C.6, core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on a base no `sluice run` drives and a settle steps it
- **THEN** one round runs and the entry is decided (124, 125)

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

### Requirement: A base sluice lands not

The profile's base SHALL answer that sluice lands no such base where
`.config/sluice.toml` names no table for the base, its own or a
pattern's, or no revision of the base holds a policy, so the core reads
the base at origin; and SHALL answer the local base where the base's
table names no remote, sluice landing it there (126, C.1, C.6).

#### Scenario: A base the policy names no table for

- **WHEN** base is asked of `trial`, a branch made from main whose policy names a table for main alone
- **THEN** it answers that sluice lands no such base, and main's at a revision (126, C.6)

### Requirement: sluice makes and removes a base place's branch

The sluice profile SHALL answer open by `sluice branch create <branch>
--from <base> --json` and close by `sluice branch delete <branch>
--into <base> --json`, each run in the repository with every
multiplexer's session stripped and `GIT_TERMINAL_PROMPT=0`: exit 0 its
answer, exit 1 a branch gone where sluice names that branch, exit 2 or
any other a refusal or a branch kept in its `error.message` (205, 206,
C.6; sluice 104, 105).

#### Scenario: A branch the policy names no table for

- **WHEN** flume opens `other/a` in a scratch repository whose policy names no table matching it
- **THEN** open answers refused with sluice's words, and no branch is made (205)

#### Scenario: A close sluice refuses

- **WHEN** flume closes `train/a` into `main` while a submission to `train/a` is still queued
- **THEN** close answers kept with sluice's words, and the branch stands (206)

### Requirement: sluice takes a base place's submission by name from the repository

The sluice profile SHALL submit a branch by name by `sluice submit
<base> --head <branch> --no-wait --json` run in the repository, read as
any submit's answer, and SHALL run no `sluice inbox` for it, reading its
outcome from read-submission's entry whose `from.head` is the branch,
with its words, files and `tested.on`, against the pinned sluice that
takes `--head` without `--check` and records no worktree as its
submitter (99, 117, 124, 125, 133, 203, C.6; sluice 124).

#### Scenario: A base place's branch over a scratch repository

- **WHEN** the pinned sluice lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix cut from `train/a` is submitted to it and settles step it until it lands, and the base place's submission of `train/a` by name is stepped until it lands on `main`
- **THEN** sluice made `train/a` at `main`'s revision on the stand-in origin, the fix landed on `train/a`, `train/a` landed on `main` with no worktree on it anywhere, flume read the landing from `sluice status` with no `sluice inbox` run for it, and closing it into `main` removed it there and on the host, no command of flume's pushing or moving a branch itself (14, 202 to 206, S110)

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, with no `sluice run` live, a place submitted
with no agent in it SHALL land by the steps of the settles a caller
keeping watch runs, `flume settle --every`, with no other caller's act
and no `sluice wait`; beside a live `sluice run`, those settles' steps
SHALL do nothing and the driver SHALL land it (124, 125, 208, C.6;
sluice 135).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, and the test then runs `flume settle --every 1m` until it lands
- **THEN** each settle ran one `sluice step main`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

### Requirement: sluice forecasts a base place's branch

The sluice profile SHALL answer forecast by `sluice submit <base>
--check --head <branch> --json` run in the repository, with every
multiplexer's session stripped and `GIT_TERMINAL_PROMPT=0`: exit 0 with
`conflict` null as clear, exit 2 with `conflict` as in conflict, and any
other answer as could not read with its `error.message` or its errors.
It SHALL pass `--check` to no submit, and the forecast SHALL leave no
entry in sluice's records (203, 210, C.6; sluice 64; core.6).

#### Scenario: A base place's branch forecast over a scratch repository

- **WHEN** the pinned sluice lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix lands a change to `README.md` on `train/a`, and flume forecasts `train/a` onto `main`; then a change to `README.md` lands on `main` and flume forecasts again
- **THEN** the first forecast answers clear with `main`'s revision; the second answers in conflict on `README.md` with `main`'s new revision and no entry ahead; and `sluice status main --all --json` gives no submission of `train/a` after either (203, 210; sluice 64)

### Requirement: sluice's forecast is read as a conflicting submission's words are

The head and the base's revision SHALL be the answer's `head` and
`onto`, the files contested its `conflict.paths`, and each entry ahead
it meets one of `conflict.with[]` named by its `id` and where it was
submitted from. The words SHALL be sluice's text form followed by the
files contested and the base's revision, as a conflicting submission's
words carry them (112, 210, C.6; sluice 65).

#### Scenario: A conflict's words

- **WHEN** the profile reads an answer with `conflict` naming `README.md` and no entry ahead, `head` `aaa111` and `onto` `c0ffee`
- **THEN** it gives the conflict with `README.md`, head `aaa111`, the base's revision `c0ffee` and no entry ahead, its words naming `README.md` and `c0ffee` in the lines a conflicting submission's words carry (112, 210)

### Requirement: A submit's decided answer is confirmed

Where sluice answers a submit with a decided entry's record, the profile
SHALL confirm it by `sluice confirm <key>` of the record's `delivery`
where it is owed, run where the submit ran, so no later inbox gives it
(101, 112, 133, C.6; sluice 51; sluice record 0102).

#### Scenario: A decided answer is confirmed

- **WHEN** sluice answers the place's submit with its entry's record in conflict with an entry queued ahead
- **THEN** submit answers the conflict, and the inbox collected after it gives nothing (112, 133)

### Requirement: Outcomes read are confirmed by key, and one unread is left owed

The profile SHALL confirm every outcome its inbox read by one `sluice
confirm` of each one's `delivery.key`, run in the place. An outcome in a
state the profile does not read, or whose outcome it cannot read, SHALL
be left owed, unconfirmed, and passed over by its branch with words
naming it (101, 133, C.6 at boomdrivedev/blueprints `fc4323b`; sluice
51; core.58).

#### Scenario: An outcome in a state flume does not read

- **WHEN** the stand-in's inbox holds a `landed` outcome under `6:landed` and one in the state `parked` under `8:parked` on `main`
- **THEN** the inbox gives the landing, passes `main` over with words naming #8 and `parked`, and confirms `6:landed` alone, leaving `8:parked` owed (133; core.58)

### Requirement: read-submission reads a refused record as held

read-submission SHALL read a `refused` record of `sluice status <base>
--all --json` as held, with its `refused.words`. One that carries no
`refused`, as sluice `44e33d4` prints it, SHALL still be held, its words
saying sluice's record gives none, and never unread. No words SHALL be
read through a wait (99, C.6 at `04e5a1a`; core.6, core.13, core.58).

#### Scenario: A refused record with its words

- **WHEN** a stand-in `sluice` answers `sluice status main --all --json` with a `refused` record of the place's branch carrying `refused {words, at}`, as sluice `44e33d4`'s `report` prints it
- **THEN** read-submission gives the submission held with those words, and the stand-in's log holds no `wait` and no `status` of a number (99, C.6)

#### Scenario: A refused record without its words

- **WHEN** the stand-in answers with a `refused` record of the place's branch and one of the base place's branch `train/x` by `from.head`, each carrying no `refused`, as sluice `44e33d4`'s `status --all` prints them
- **THEN** read-submission gives each held, with words saying sluice's record gives none of the forge's words, neither unread nor passed over, and the log holds no `wait` (99, 203, C.6; core.58)
