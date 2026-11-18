# A settle runs no round of the merge queue

## Why

sluice's record 0093, accepted by the user on 2026-11-06 and amended on
2026-11-08 and 2026-11-13, moves a host's queues into sluice's own host
process. `sluice submit` adds its entry and returns at once, having
started whatever the way that runs the branch's queue needs. A queue
runs whole in one of four ways: with no daemon, by a daemon on this
machine, by a daemon on another machine, or by a GitHub Action. sluice
keeps one contract with flume whichever way runs it: flume reads the
outcome sluice reports, marks the place, and launches the kind's
landing fix on a kick-back.

The statement follows it. C.6's step row is "nothing", and S109 has
each settle read the submission, driving no round of the queue, the
next settle's read starting sluice's host process again where it ended
with work in play (boomdrivedev/blueprints main 3a5c19d). Record 0012
is amended so, and rule core.6 rules out a `sluice step` from a
settle.

The sluice profile still runs `sluice step <base>` at every settle that
finds a submission open, and the tests against the pinned sluice still
land their submissions by those steps. The pinned sluice, 0.93.2, has
no host process, so nothing else would move its queues.

## What Changes

- **The profile runs no round (125, C.6).** The sluice profile's step
  runs no sluice command and answers that the base's queue moves on its
  own, whichever of sluice's four ways runs it. The profile runs no
  `sluice step`, `sluice run`, `sluice daemon` or `sluice wait`. A
  settle asks sluice only to read: the inbox and read-submission, as
  today. Those reads start sluice's host process where work waits and
  none runs (sluice 135; sluice record 0093 as amended 2026-11-13).
- **The seam keeps step (125).** Part B still names a queue that moves
  only while something drives it (46, 124, 125, I8). So the seam, the
  core's one step per repository and base in a command, and the fake
  stand as they are. The core reads sluice's answer as it reads any
  queue that moves on its own.
- **The pinned sluice is the first release with the host process.**
  devenv.nix pins it, and every command and field C.6 says the profile
  reads is checked against it (core.22).
- **The seam's scenarios.** "A merge queue that cannot be read says so"
  asks that a step of an unreadable queue never says it drove a round.
  A profile whose queue moves on its own runs nothing at step, so it
  has nothing it could fail to read.
- **The tests against the pinned sluice run no step.** Each test reads
  its submissions until sluice's host process decides them. A
  submission a scenario must read open is held open by the gate's
  check until the test releases it. The test of a queue nothing drives
  and the test of a step beside a live `sluice run` give way to one: a
  queue the settles never step is decided by sluice's host process.
  S109 is walked as the statement now has it, a host process that
  ended with work in play among it.
- **Record 0012's amendment** gains this change's choice: keep the
  seam's step, with the profile answering that the queue moves on its
  own. **Rule core.53's details** stop naming the settles' steps. The
  site's merge-queue guide and walkthrough say flume reads the outcome
  and runs no round, and name the release flume needs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: the profile runs no round of sluice's queue;
  a base place's branch and a submitted place with no agent land by
  sluice's host process, read at the settles that follow.

## Impact

- `devenv.nix`: sluice's version and its three hashes.
- `crates/flume/src/profiles/merge_queue/sluice/mod.rs`: `step`, the
  module's doc, a unit test.
- `crates/flume-conformance/src/seams/merge_queue.rs`: the unreadable
  scenario's step check.
- `crates/flume/tests/profiles/tools/merge_queue_sluice.rs`: the
  harness's hold and its reads to a decision, the tests that stepped,
  and the tests the host process moves under them.
- `crates/flume/tests/profiles/tools/walk.rs`: the walk's hold, and a
  short link that gives the host process's socket room where the
  test's scratch is long.
- `crates/flume/tests/profiles/tools/walkthroughs.rs`: the two
  walkthroughs that read what the host process changes.
- `docs/adr/0012-…` (amended), `docs/architecture/core.md`,
  `site/src/content/docs/architecture/rules.md`,
  `site/src/content/docs/guides/merge-queue.md`,
  `site/src/content/docs/guides/walkthrough.md`.
- No change to the seam, the core, the fakes or the command line.
- It is built on sluice 0.97.0, the first release that carries
  one-sluice-process-runs-each-hosts-queues, sluice's host process.
