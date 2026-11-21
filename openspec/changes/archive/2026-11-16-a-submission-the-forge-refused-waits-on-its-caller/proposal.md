# A submission the forge refused waits on its caller

## Why

sluice now holds a landing the forge refuses, as when push protection
finds what looks like a secret, and marks each submission it carried
`refused`, with the forge's words. sluice decides no outcome for any of
them, and the landing holds its branch until one of its submissions
leaves or the operator pushes it again (sluice 152; sluice record 0124).
flume reads three decided outcomes, landed, failed and in conflict. Its
inbox runs `sluice inbox --confirm --json`, which confirms everything
sluice hands over, and keeps only those three, so a refusal would be
confirmed and lost. Its read of a submission refuses every other state,
so a held one would read as not understood. The place would show as
submitted and undecided, with nothing to say why, while its landing held
the branch.

Record 0071 and rule core.58 decide it: a held submission is neither
failed nor landed. It starts no stage and no fix and counts toward no
bound. The place waits on its caller with the forge's words while its
submission stays open, and no outcome sluice hands over is confirmed
until flume has read it. The statement says so at
boomdrivedev/blueprints `8b73d11` (99, 112, 130, 133 and C.6's inbox and
read-submission rows), with the inbox row as `fc4323b` and the
read-submission row as `04e5a1a` amend them, on the branch
`design/flume-reads-a-forge-refusal`. The second amendment follows the
design's ruling on this change's construct: sluice `44e33d4` prints a
held record's `refused {words, at}` only through `sluice wait`, so a
held record that carries none is still held, with words saying so
(record 0071's amendment of 2026-11-16).

## What Changes

- **The seam (99, 133).** A submission's state gains held, with the
  forge's words, and an outcome gains held, with the merge queue's
  words. Neither is a decided outcome, and a held submission is not one
  a round moves.
- **The inbox confirms what it read (133, C.6; core.58).** The sluice
  profile collects by `sluice inbox --json`, reads `refused` as held
  with its `notice` whole as its words, and confirms every outcome it
  read with one `sluice confirm <key>…`. An outcome in a state it cannot
  read, or whose outcome it cannot read, is left owed, unconfirmed, and
  passed over by its branch with words naming it, which status gives.
- **read-submission reads held (99, C.6 at `04e5a1a`).** A `refused`
  record from `sluice status <base> --all --json` reads as held with its
  `refused.words`. One that carries no `refused`, as sluice `44e33d4`
  prints every record, is still held, with words saying sluice's record
  gives none. Nothing reads it through a wait.
- **The place waits on its caller (112, 130; core.58).** A settle that
  reads the place's submission held, from its inbox or from the merge
  queue's record, has the place wait on its caller to retry the kind's
  fix round, with the forge's words. It starts no stage, launches no
  base place's fix, counts no going-back and steps no round for it. The
  submission stays open: its landing, once pushed again, makes the work
  over (113); its return to the queue ends the wait (124); a withdrawal
  reads as one (112); and a retry naming a stage runs it, the submission
  after it withdrawing the held one (sluice 67).
- **Status, the line, the list and the trace (56, 130, 217).** Status
  gives the submission held with its words and the move waited on, its
  why `forge_refused`. A place's line says `landing refused`. The merge
  queue's wait ends while the landing is held and the caller's begins.
- **The fake merge queue** holds a landing the forge refused, with or
  without the words on its record, pushes it again, puts it back in the
  queue, and withdraws it at a later submit of its branch.
- **The record and the rules.** Record 0071's amendment of 2026-11-16
  quotes the design's two rulings, and core.58's details rule out a held
  record read as not understood for want of its words, or read through
  a wait. The site's guide gains the line's word, and its rules page
  mirrors core.58.

The pin stays sluice 0.93.2, which holds no landing: the profile's
reading of `refused` is checked against documents written in the shapes
sluice `44e33d4` prints.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: the inbox collects without confirming on
  receipt and confirms what it read by key, reads `refused` as held, and
  leaves an outcome it cannot read owed; read-submission reads a
  `refused` record as held, with its `refused.words` or words saying
  sluice's record gives none.
- `seam-contracts`: a submission's state and an outcome gain held.
- `seam-fakes`: the fake merge queue holds a landing the forge refused.
- `settling`: a submission whose landing the forge refused leaves the
  place waiting on its caller, starting nothing and counting nothing,
  until it lands, goes back to the queue, is withdrawn or the caller
  moves.
- `place-status`: status gives a held submission's words and the move
  its place waits on.
- `place-line`: a place's state reads `landing refused`.
- `telemetry`: the merge queue's wait ends while a landing is held, and
  the caller's wait names it.

## Impact

- `crates/flume-seams`: `SubmissionState::Held`, `SubmissionOutcome::Held`,
  `Waits::ForgeRefused`, `SubmissionHeld.landing_refused`.
- `crates/flume`: the sluice profile's inbox, read-submission and module
  doc; a fast-layer test module over a stand-in `sluice` and fixtures of
  sluice `44e33d4`'s documents.
- `crates/flume-core`: `submit.rs` takes a held submission and ends its
  wait; `status.rs`, `line.rs` and `telemetry.rs` say it.
- `crates/flume-fakes`: the fake merge queue's held landing.
- `crates/flume-conformance`: the engine's scenarios of a held
  submission, and the fake's test.
- `docs/`: record 0071's amendment and core.58's details, in this
  change's first commit.
- `site/`: the line's word, and core.58 on the rules page.
- No new crate, plugin or site part, so no new moon project; no new
  dependency or tool; the sluice pin unchanged.
