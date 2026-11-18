# Design

The statement is read at boomdrivedev/blueprints main 3a5c19d: C.6's
step row ("nothing: sluice's queue moves on its own once submitted, run
by sluice's host process, which the submit starts, and the settle's
read of the submission starts it again where work waits and none
runs"), 46, 124, 125, I8 and S109 as amended with it, and C.6's table
of what the profile reads, which holds no step row. sluice's side is
its record 0093 and the record's amendments of 2026-11-08 (one contract
with flume, four ways a queue runs) and 2026-11-13 (every read follows
the host process), and sluice clause 135. flume's is record 0012 as
sluice's record 0093 amended it, and rule core.6, which rules out a
`sluice step` from a settle. Where this design and the statement
disagree, the statement wins (core.19).

The rules relied on: core.2 (the profile implements the seam and
nothing of the core); core.6 (no step from a settle; the outcome read
from the merge queue's record); core.13 (no command of flume's waits on
the merge queue, and flume keeps no process); core.22 (the gate's sluice
is the version the tree pins); core.35 (the site speaks to any host's
operator); core.51 (a check finds room for its sockets in the scratch
the merge queue gives it); core.53 (a base place's submission and its
outcome). No area page exists, so core.md binds every path this change
touches.

## 1. The profile's step

`Sluice::step` runs no sluice command. It answers
`Stepped::MovesOnItsOwn` for every repository and base. The profile
cannot tell which of sluice's four ways runs a branch's queue: no
daemon, a daemon on this machine, a daemon on another machine, or a
GitHub Action. It has no need to. sluice keeps one contract with flume
whichever way runs it (sluice record 0093's amendment of 2026-11-08,
point 2), so the profile reads nothing to choose and runs the same
commands for every way.

Nothing the profile runs moves a queue. It runs no `sluice step`,
`sluice run`, `sluice daemon`, `sluice wait` or `sluice host`. What a
settle asks sluice of an open submission is what it asks today: the
place's inbox and read-submission's overview and status. Under the
pinned release each read reaches sluice's host process, or starts it
where none answers and work waits (sluice 135; record 0093's amendment
of 2026-11-13). That is the whole of "the settle's read of the
submission starts it again". flume adds no call for it.

A base place's submission by name, `sluice submit <base> --head
<branch> --no-wait --json`, starts the host process as any submit
does. Its outcome is read from read-submission as before (203, C.6).
The forecast's `sluice submit --check` stays as it is (210).

The module's doc says this in place of its sentence on `sluice step`:
step is nothing, as C.6's step row says, and the settles' reads start
sluice's host process where work waits and none runs. The doc names
the pinned release where it names 0.93.2.

## 2. The seam and the core stay

Clause 125 still defines step for a merge queue that moves only while
something drives it, and 46, 124 and I8 still have a settle drive such
a queue one round. Under core.2 every operation of Part B is reached
through the seam. So the seam keeps `step` and `Stepped`, the core
keeps `step_base`, asking each repository and base once in a command,
and the fake keeps its queue that moves only when stepped. The engine's
scenarios run against that fake. The core reads `MovesOnItsOwn` as
nothing more to do, as the settling spec's "A queue that moves on its
own" already says.

The path not taken: take step out of the seam, the core and the fake.
That changes no behaviour on a sluice host. It would leave 46, 124, 125
and I8 with no seam operation behind them, so the statement would have
to drop them first (core.19). Nothing asks for that, and a merge queue
other than sluice may move only while driven.

## 3. The seam's scenarios

`unreadable_says_so` asks that a step of an unreadable queue answer
`CouldNotRead`. A profile whose queue moves on its own runs nothing at
step, so it has nothing it could fail to read. The scenario now asks
that the step never answer `Stepped`: could not read, or moves on its
own. Its inbox and read-submission checks stay as they are, and they
are what reads sluice. The fake still answers `CouldNotRead`, so the
scenario passes over both. `step_drives_a_round` already takes either
answer.

The path not taken: a harness flag saying whether its queue moves on
its own, with the scenario asking `MovesOnItsOwn` of one and
`CouldNotRead` of the other. The flag would restate in the harness what
the profile's answer already says.

## 4. The pinned sluice

devenv.nix pins the first sluice release that carries
one-sluice-process-runs-each-hosts-queues, 0.97.0, with its three
hashes. Where the base
pins that release or a later one by the time this is built, the pin
stays. Every command and field C.6's table says the profile reads is
checked against the pinned release: submit's answer, the inbox, the
overview and status, `sluice confirm`, open, close, the forecast,
`sluice equip` and `sluice release`. So is each place a test reaches
into sluice's records or state, as the unreadable harness locks
`.git/sluice`. Where the release answers a command or a field
otherwise than C.6 says, the build stops short naming it, as found
work: C.6 changes first (core.19).

In 0.97.0 the host process keeps `version`, a JSON document of its
version, `pid` and start time, under `$XDG_STATE_HOME/sluice/host/`. It puts its socket in the user's short
directory, `SLUICE_USER_SCRATCH` in tests, and ends once nothing is in
play or its state folder is gone. The tests below lean on those
facts, read again against the pinned release.

## 5. The tests against the pinned sluice

**One state folder per test, and room for its socket.** Each harness
and each walk already gives sluice its own `XDG_STATE_HOME` and
`SLUICE_USER_SCRATCH` in the test's scratch. So each test has its own
host process, and that process's socket fits the scratch the merge
queue gives the place (core.51). A test whose host process cannot bind
fails naming what lacked room, never skipping.

**A held submission.** Under the host process a submission is decided
as soon as its check passes. Several scenarios must read one open:
`will_hold`, `open_submission` and the tests that read a submission
waiting. The harness's `check.sh` therefore waits, polling every 100
milliseconds for at most 120 seconds, while a file `hold` stands in the
test's scratch. It is written before the place is cut, so the place's
branch carries it. `will_hold` and `open_submission` write `hold`.
`until_decided`, `withdraw` and the harness's drop remove it. The
drop removes it before the scratch goes, so every run ends and the
host process of that state folder ends with nothing in play.

**Read to a decision, never stepped.** `Harness::decide` and
`stepped_until_decided` read `sluice status` until the newest entry of
the branch is decided, within 90 and 120 seconds as today, and run no
`sluice step`. `stepped_until_decided` becomes `read_until_decided`.
It asserts once that the profile's step answers `MovesOnItsOwn`, then
reads by read-submission alone. The stand-in beside the submit
(`a_settle_beside_the_stand_ins_submit_…`) collects the inbox beside
the waiting submit with no step.

**One test for a queue nothing steps.** The test of a queue nothing
drives and the test of a step beside a live `sluice run` go.
`a_queue_the_settles_never_step_is_decided_by_sluices_host_process`
takes their place. Through the counting sluice it submits the place by
the profile with its check held, and read-submission reads the entry
open. It then releases the hold and calls the profile's step,
read-submission and inbox, as a settle does, until the submission is
decided. The entry lands and is read landed, the step answers `MovesOnItsOwn` each time, and every counted
command after the submit is `inbox`, `confirm`, `overview` or
`status`. None is `step`, `run`, `daemon` or `wait`.

**S109 as the statement has it.** The walk's `check.sh` gets the same
hold, a `hold` file in the walk's scratch that `Walk::hold` writes and
`Walk::release` and the walk's drop remove. No walk but S109's writes
it. `s109_…_lands_by_a_watchs_settles_alone` holds the chore's
submission, so the watch's settles read it open.
After the watch's first settle it releases the hold. It then asserts
that no counted command of any settle is `step`, `run`, `daemon` or
`wait`, rather than one `step main` per settle. A second test,
`s109_a_host_process_ended_with_work_in_play_is_started_again_by_a_settles_read`,
holds the chore's submission and ends sluice's host process by the pid
its state folder records, waiting until that process is gone. It then
releases the hold and runs the watch's settles. The chore lands and is
tidied, the state folder records a host process with another pid, and
no settle ran a `step`. A run outlives the host process that started
it (sluice 135), so the run the hold kept waiting reports to the host
process the settle's read started.

**What stays.** The walkthroughs land through the pinned sluice by its
host process with no change of their own. Their `Walk` already gives
sluice a state folder and a socket's room of its own, and removing the
scratch ends its host process. The stand-in for a base that takes pull
requests passes every command but its own to the pinned sluice, so its
`take-in`'s submit starts the host process as any submit does.

## 6. Records, rules and the site

**Record 0012.** Its amendment line for sluice's record 0093 gains this
change's choice, as the construction agent's: the seam keeps step for a
merge queue that moves only while driven (125), and the sluice
profile's step runs no command and answers that the queue moves on its
own, whichever of the four ways runs it. The path not taken was taking
step out of the seam, which needs 46, 124, 125 and I8 out of the
statement first.

**core.53's details.** Two phrases still name the settles' steps.
"Rules out" names "a runner's, crew's or any caller's command driving a
queue the settles step, which is that tool's own to stop (record
0051)". It reads "a runner's, crew's or any caller's command driving a
queue that sluice's own process moves, which is that tool's own to stop
(record 0051; sluice record 0093)". "Not ruled out" names "a host's
`sluice run` driving the queue, the settles' steps doing nothing beside
it (sluice 135)". It reads "the base's queue run by whichever of
sluice's ways runs it, its host process, a daemon on this machine or
another, or a forge's jobs, the settles reading its outcome alike and
stepping none (C.6; sluice 135, 137; sluice record 0093 as amended
2026-11-08)". The site's rules page mirrors it. core.6 already says
this and stays. So does core.13's "a settle's one round of the merge
queue (125)", which names the seam's step for a queue that moves only
while driven.

**The site.** In `guides/merge-queue.md`, "Reading the outcome" drops
its item on `sluice step`. It says sluice's queue moves on its own once
submitted, whichever way runs it, and that flume only reads the
outcome. The paragraph on a caller keeping watch drops "and its base's
queue run one round". The pull-request section's last sentence drops
"each settle runs one round of the base's queue, which takes the
request in where this host takes them in". It says sluice takes the
request in wherever its queue runs. The release flume needs becomes the
first with the host process. In `guides/walkthrough.md`, section 5's
first sentence drops "and runs one round of sluice's queue where no
`sluice run` drives it". Each page names sluice only as the product
flume integrates with (core.35).
