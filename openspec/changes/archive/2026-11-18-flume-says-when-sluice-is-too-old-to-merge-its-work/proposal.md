# flume says when sluice is too old to merge its work

## Why

flume runs no round of sluice's queue: from sluice 0.97.0, sluice's
own host process moves each queue, started by the submit (124, 125,
C.6; sluice 135). A sluice before 0.97.0 has no host process, so a
place flume submits to it waits for ever, shown submitted and
undecided, with nothing that says why. The launch's brief asks that
flume never leave work waiting on a sluice too old to merge it by
itself: it refuses to hand that sluice the work and says which sluice
version it needs.

The statement says so in C.6's submit row and its new version row,
amended on boomdrivedev/blueprints `5efb363`, on the branch
`design/flume-says-when-sluice-is-too-old-to-merge-its-work`. Record
0080 holds the decision, and rule core.6's details rule out the submit.

## What Changes

- **The sluice profile reads sluice's release before each submit
  (C.6).** It runs `sluice --version` in the directory and environment
  the submit runs in, for a place's submit from its worktree and a base
  place's by name alike (111, 203).
- **A sluice too old is handed no work (C.6; sluice 135).** Where
  sluice says a release before 0.97.0, the first whose host process
  moves its queues, or names no release, the profile runs no `sluice
  submit` and answers the submit refused. Its words name what sluice
  said and that flume needs sluice 0.97.0 or later. The core holds that
  as any refused submission: the place waits on its caller's retry or
  drop, status naming why (112).
- **A sluice from 0.97.0 on is handed the work as before**, whatever
  its major version.
- **The tests against the pinned sluice** read `--version` as the first
  command the profile asks before a submit.
- **The site's merge-queue guide** says flume reads sluice's version
  before each submit and refuses one too old, naming the release it
  needs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: the profile hands no work to a sluice too old
  to merge it by itself, refusing the submit with words naming the
  release flume needs.

## Impact

- `crates/flume/src/profiles/merge_queue/sluice/mod.rs`: `submit`, the
  release read and its words, the module's doc, unit tests.
- `crates/flume/tests/profiles/tools/merge_queue_sluice.rs`: the two
  tests that read the counting sluice's first command.
- `site/src/content/docs/guides/merge-queue.md`.
- No change to the seam, the core, the fakes, the conformance suite or
  the command line. `docs/adr/0080-…` and core.6's details in
  `docs/architecture/core-details.md` are written with this change's
  planning.
