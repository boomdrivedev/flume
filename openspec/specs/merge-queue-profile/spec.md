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
with its words, files and `tested.on`. Its queue SHALL move by
sluice's host process, which the submit starts, with no round run by
flume (99, 117, 124, 125, 133, 203, C.6; sluice 124, 135).

#### Scenario: A base place's branch over a scratch repository

- **WHEN** the pinned sluice, which takes `--head` without `--check` and records no worktree as the submitter of a branch submitted by name, lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix cut from `train/a` is submitted to it and read until it is decided, and the base place's submission of `train/a` by name is read until it is decided, no sluice command of the test's or flume's stepping either queue
- **THEN** sluice made `train/a` at `main`'s revision on the stand-in origin, sluice's host process landed the fix on `train/a` and `train/a` on `main` with no worktree on it anywhere, flume read the landing from `sluice status` with no `sluice inbox` run for it, and closing it into `main` removed it there and on the host, no command of flume's pushing or moving a branch itself (14, 202 to 206, S110)

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, a place submitted with no agent in it SHALL
land by sluice's host process and SHALL be read and tidied by the
settles of a caller keeping watch, with no other caller's act, no
`sluice wait` and no sluice command that moves a queue. Where that
host process ended before the submission was decided, the settle's
inbox SHALL start it again, and the place SHALL land the same (46, 124,
125, 208, C.6, S109; sluice 135).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, whose host process runs as `sluice daemon`, its check held until the watch's first settle has read it open, and the test then runs `flume settle --every 1m` until it lands
- **THEN** no settle ran a `sluice step`, `run`, `daemon` or `wait`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

#### Scenario: A host process started again by a settle's read

- **WHEN** a chore is submitted to the pinned sluice with its check held, sluice's host process is ended while the check waits, and the test then releases the check and runs `flume settle --every 1m` until the chore lands
- **THEN** the settle's inbox, finding no sluice host on its socket, ran `sluice inbox --json`, which started a host process with another pid where no service manager owns it, the chore landed and was tidied, and no settle ran a `sluice step` (124, 208, S109; sluice 135; the-service-manager-starts-each-resident)

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
SHALL answer it with the key of the record's `delivery` where it is
owed, and SHALL confirm nothing itself: the core confirms it by that key
once it has acted on it, and until then an inbox gives it too (101, 112,
133, C.6; sluice 51; sluice record 0102).

#### Scenario: A decided answer is confirmed

- **WHEN** sluice answers the place's submit with its entry's record in conflict with an entry queued ahead
- **THEN** submit answers the conflict with the key `<id>:conflict`, the inbox collected after it gives that entry's conflict under that key, and once it is confirmed by that key the inbox gives nothing (112, 133)

### Requirement: Outcomes collected from sluice's inbox until confirmed

The profile SHALL collect a place's pending outcomes from sluice's
inbox, asked as the requirement "The inbox asked of sluice host" says,
never with `--confirm`, each with its entry's `id` and its `delivery`'s
key where owed, given again until confirmed: landed with its commit, or
failed or in conflict with sluice's `notice` as its words, a conflict's
base revision its `tested.on`; and SHALL read a withdrawal from
read-submission (99, 101, 133, C.6; sluice 51).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry given until it is confirmed

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice, the outcome confirmed by its key, and the inbox collected again
- **THEN** the first two collects give the failure with sluice's notice, its entry's id and the key `<id>:failed`, the confirm answers confirmed, the third collect gives nothing, and a second confirm of the key answers confirmed (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

### Requirement: An outcome confirmed by sluice confirm

The profile SHALL confirm an outcome by `sluice confirm <key> --json` in
the place, reading exit 0, confirmed now or before, and exit 1, nothing
owed under the key, as confirmed, and any other exit as sluice unread in
its words, the outcome left owed for the next inbox (101, 133, C.6;
sluice 51).

#### Scenario: A failed submission's outcome confirmed once flume acted on it

- **WHEN** the gate fails a fix's first submission through the pinned sluice, flume sends the place back to code, and the place is settled again
- **THEN** sluice reads the failed entry's delivery `done`, and the second settle sends nothing back again (112, 133)

### Requirement: A refused outcome is held

The profile SHALL read an inbox's `refused` record, a landing the forge
refused that sluice holds, as held with its `notice` whole as its words,
or what a record with none says, owed under its key until confirmed as
any outcome is (112, 133, C.6 at boomdrivedev/blueprints `8b73d11` and
`fc4323b`; sluice 152; core.58).

#### Scenario: A refused outcome

- **WHEN** a stand-in sluice host answers the inbox with sluice `44e33d4`'s documents, a `refused` outcome of #7 under `7:refused` beside a `landed` one of #6 under `6:landed`, and `7:refused` is then confirmed
- **THEN** the inbox gives the landing and the outcome held with its `notice` whole, each with its entry's id and key, asks the inbox with no `--confirm` and confirms nothing, and the confirm runs `sluice confirm 7:refused --json` (112, 133, C.6)

### Requirement: An outcome the profile cannot read is left owed

An outcome in a state the profile does not read, or whose outcome it
cannot read, SHALL be left owed, unconfirmed, and passed over by its
branch with words naming it (133, C.6 at boomdrivedev/blueprints
`fc4323b`; core.58).

#### Scenario: An outcome in a state flume does not read

- **WHEN** the stand-in's inbox holds a `landed` outcome under `6:landed` and one in the state `parked` under `8:parked` on `main`
- **THEN** the inbox gives the landing under `6:landed`, passes `main` over with words naming #8 and `parked`, and confirms nothing, leaving `8:parked` owed (133; core.58)

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

### Requirement: A settle runs no round of sluice's queue

The sluice profile's step SHALL run no sluice command and SHALL answer
that the base's queue moves on its own, whichever of sluice's ways runs
it. Of an open submission the profile SHALL ask sluice only its reads,
the inbox and read-submission, on sluice host's socket; only the
inbox's `sluice inbox --json`, run where no sluice host answers, starts
sluice's host process (124, 125, C.6; sluice 135; sluice record 0093 as
amended; core.6).

#### Scenario: A step over a stand-in sluice

- **WHEN** the profile's step is asked of `main` with a stand-in `sluice` first on its `PATH` that leaves a mark whenever it runs
- **THEN** it answers that `main`'s queue moves on its own, and the stand-in left no mark (125, C.6)

#### Scenario: A queue the settles never step

- **WHEN** a place's entry is submitted to `main` of a scratch repository through the pinned sluice, sluice's host process runs as `sluice daemon` and no `sluice run` is live, and the profile's step, read-submission and inbox are asked as a settle asks them until the entry is decided
- **THEN** sluice's host process lands the entry, each step answers that the queue moves on its own, and every sluice command the profile ran after the submit was `inbox` or `confirm` (124, 125, C.6; sluice 135)

### Requirement: No work is handed to a sluice too old to merge it by itself

Before each submit, a place's from its worktree and a base place's by
name, the sluice profile SHALL read the sluice it runs by `sluice
--version`. Where that names a release before 0.97.0, the first whose
host process moves its queues, or names no release, the profile SHALL
run no `sluice submit` and SHALL answer the submit refused, its words
naming what sluice said and the release flume needs (111, 112, 203,
C.6; sluice 135; record 0080).

#### Scenario: A sluice before its host process

- **WHEN** the profile submits a place from its worktree, and then a base place's branch by name, through a stand-in `sluice` first on its `PATH` that says `sluice 0.95.1` and logs every other command it is asked
- **THEN** each submit is refused, its words naming `sluice 0.95.1` and `0.97.0`, and the stand-in logged no `submit` (C.6; sluice 135)

#### Scenario: A sluice that names no release

- **WHEN** the profile submits a place through a stand-in `sluice` that says, in turn, `sluice 0.0.0`, `sluice 0.97.0-rc.1`, `sluice`, and nothing with exit 64
- **THEN** each submit is refused, its words naming what the stand-in said, or that it said nothing, and `0.97.0`, and the stand-in logged no `submit` (C.6)

#### Scenario: A sluice with its host process

- **WHEN** the profile submits a place through a stand-in `sluice` that says, in turn, `sluice 0.97.0`, `sluice 0.100.0` and `sluice 1.0.0`, and answers each submit with an entry
- **THEN** each submit runs `sluice submit main --no-wait --json` after the version read and answers the entry submitted (111, C.6)

#### Scenario: The pinned sluice is handed the work

- **WHEN** a base place's branch is submitted by name through a `sluice` that logs each command it is asked and runs the pinned sluice, over a scratch repository
- **THEN** the first command it was asked is `--version`, the next is `submit main --head train/a --no-wait --json`, and the branch lands (203, C.6)

### Requirement: Whether sluice lands a base, read with no fetch

The sluice profile SHALL say whether sluice lands a base, with no fetch
and no sluice command. It SHALL read `.config/sluice.toml` at the base
and at each remote's copy of it. Where no revision holds the policy, its
words SHALL name `.config/sluice.toml`. Where the policy names no table
for the base under `[branches]`, its words SHALL say so (126, core.15).

#### Scenario: No policy, then a policy for another branch, then one for main

- **WHEN** status names a repository that binds sluice, first with no `.config/sluice.toml`, then with one for another branch only, then with one for `main`
- **THEN** the merge queue's words name `.config/sluice.toml` at `main`, then say that it names no table for `main`, and then the merge queue is ready

### Requirement: The inbox asked of sluice host

The profile SHALL read a place's inbox by asking sluice host's socket
the `inbox` read that `sluice inbox --json` in the place asks, its
folder and worktree the place's and its caller the `SLUICE_AGENT` the
command would carry, starting no sluice process; and SHALL run `sluice
inbox --json` in the place only where no sluice host answers the ask
(133, S109; sluice 135;
flume-reads-sluices-records-on-sluice-hosts-socket).

#### Scenario: The inbox asked of sluice host

- **WHEN** a stand-in sluice host on the profile's socket answers the inbox with a `landed` outcome of #6 under `6:landed`, and the seam's `sluice` is no program
- **THEN** the inbox gives the landing under `6:landed`, and the host was asked `inbox` with the place's folder as its folder and worktree, the JSON answer and the caller's `SLUICE_AGENT`, confirming nothing (133)

#### Scenario: No sluice host answers

- **WHEN** no sluice host answers on the profile's socket, and the seam's `sluice` is no program
- **THEN** the inbox is unread in the words of `sluice inbox --json`, which could not be run, and not of the socket: the command is what starts sluice host on a bare install (133, S109; the-service-manager-starts-each-resident)
