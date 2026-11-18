# A watch settles only the places where something is pending

## Why

A watch's settle, and every acting command where no caller keeps
watch, settles every place on the host, reading each in full: its
merge queue, its terminal and its agent. On deneb a pass ran minutes,
outlived its own watch, and every command then settled every place
itself: the freezes of 190 to 300 seconds. Most places it read could
not have moved, since nothing but a call to flume moves a place that
waits only on its caller. The user chose on their card of 2026-11-17
that flume look only where something is pending (record 0078,
accepted; rule core.62).

## What Changes

- **A watch's settle reads only the places where something is pending
  (225, 46, 208).** Something is pending on a place while a change that
  calls flume nothing may have moved it: it is being made, or its work
  is over and it is not yet tidied away; a stage of it starts or runs;
  its submission is in flight; it is a base place; a time it waits on
  has come; or an end of it, or a wake for it, waits to be handed on.
  The settle then starts the stages waiting for room, host-wide, as
  each settle does. A place that waits only on an answer, its caller's
  word or a drop is read by no such settle: neither its merge queue,
  its terminal nor its agent.
- **flume alone reads what is pending (225, 208).** It reads it from
  each place's held state, the host's deliveries and the clock, asking
  no seam. `flume settle --every <bound>` keeps its form, and the
  caller names nothing of it.
- **A command where none keeps watch settles the pending places first
  (119).** Every acting command, and a wait before each reading, settles
  the same places a watch's settle would, rather than every place.
- **The watch holds while its settle runs (208).** The host's watch
  records when its newest settle began, the command running it and when
  it ended. A caller keeps watch while that settle runs, and until twice
  its bound has passed since it ended. A settle whose command died with
  no end lapses twice its bound after it began. The watch is renewed by
  that settle alone.
- **A full settle stays at hand (225).** `flume settle` naming no place
  and stating no bound still settles every place, at a caller's or a
  person's asking. A settle naming a place, and an agent's quiet, settle
  their own place, as before.
- **Status gives when the watch's settle ended (56, 208).** The host's
  part gains `watch_ended`, its JSON only gaining a field (core.14), and
  `watch_lapsed` reads twice the bound after that end. The person's form
  says the places where something is pending settle.
- **The host's reading goes at the watch's settle alone (220).** The
  places standing and each stream's agents are handed on at a settle of
  the watch's, and at no other settle.
- **The clause list reads the statement at 48a94da (core.19).** Clause
  225 is Part A's, the pending places. The merge queue's confirm, which
  this tree cited as 225 from a design branch never merged, is held by
  133 at 48a94da, and its citations move there.
- **The site, the herdr plugin's README and the command's help** say
  that a watch, and an unwatched command, settle the places where
  something is pending (core.35).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `settling`: a watch's settle, and an unwatched command's, reads only
  the places where something is pending, moving every place no later
  than a settle of every place would; the watch holds while its settle
  runs and until twice its bound after it ended; `flume settle` naming
  nothing still settles every place; a confirmed outcome cites 133.
- `command-line`: `flume settle --every` settles the places where
  something is pending and records when its settle ended.
- `place-status`: the host's watch gives `watch_ended`, lapses twice its
  bound after its settle ended, and says the pending places settle.
- `stream-terminals`: with no watch, an acting command settles the
  pending places first, under every terminal.
- `end-delivery`: with no watch, a wait settles the pending places
  before each reading.
- `telemetry`: the host's reading goes at a settle of the watch's.
- `seam-contracts`, `seam-fakes`, `merge-queue-profile`: the merge
  queue's confirm is cited as 133.

## Impact

- `crates/flume-seams/src/store.rs`: the host's watch document gains
  the command keeping it and when its settle ended, a document written
  before read as before.
- `crates/flume-core`: a reading of what is pending on a place
  (`pending.rs`); the watch's settle, the settle an acting command runs
  first and a wait's settle read it (`settle.rs`, `wait.rs`); the watch
  written at its settle's beginning and end; status's `watch_ended`
  (`answer.rs`, `room.rs`); the host's reading at the watch's settle
  alone (`telemetry.rs`).
- `crates/flume`: `flume settle --every`'s help and refusal, and the
  status line's words (`commands/mod.rs`, `commands/print.rs`).
- `crates/flume-conformance`: the engine's scenarios for the pending
  places and the watch; the clause list at 48a94da, the confirm's
  scenarios citing 133.
- `site/`, `plugins/herdr/README.md`, `docs/architecture/core-details.md`
  (core.6's citations of the confirm).
- No change to sluice, herdr or the runner: the runner's call is
  unchanged (record 0078).
