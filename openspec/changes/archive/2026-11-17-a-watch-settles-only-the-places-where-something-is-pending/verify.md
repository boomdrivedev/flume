# Verify

The change `a-watch-settles-only-the-places-where-something-is-pending`
on `change/a-watch-settles-only-the-places-where-something-is-pending`
at 5164350, against its base `bolt/flume-stops-freezing` (ecdc817).
The work holds, so the change is archived.

## The whole checks

`devenv shell -- sluice exec bolt/flume-stops-freezing` was run once on
5164350. sluice ran the `check-fast` tasks the change affects, and every
task passed (exit 0, 1m 41s, 22 results recorded under their keys for
5164350): `root:format`, `root:changes`, `seams:format`, `seams:lint`,
`seams:test`, `core:format`, `core:lint`, `core:test`, `fakes:format`,
`fakes:lint`, `fakes:test`, `conformance:format`, `conformance:lint`,
`conformance:test`, `conformance:coverage`, `flume:format`,
`flume:lint`, `flume:test`, `nix:typecheck`, `nix:test`,
`plugin-claude:test` and `site:build`, with `site:install` beside them.
The log has no failed test and no panic. The base is a bolt, so
`check-tools` waits for the bolt's merge onto main (CLAUDE.md, Checks).

## Tasks

All 19 tasks of `tasks.md` are ticked, 1.1 to 5.1, and none is left
open. Their work landed on the branch in c53e5679 to 51643504.

## Requirements and scenarios

Each scenario is matched to the test that runs it: an engine scenario
of `crates/flume-conformance/src/engine/`, or a named test elsewhere.

### settling

- **Attended and watched**: Killed with nobody watching
  (`s27_killed_with_nobody_watching`), Crashed under a watch
  (`s28_crashed_under_a_watch`), A watched host with one stream
  unwatched (`a_watched_host_with_one_stream_unwatched`), A host with no
  runner (`s118_a_host_with_no_watch_runs_attended`). Every acting
  command reaches `settle_attended`, which runs `settle_pending` where
  `attended()` holds.
- **A caller keeps watch over the host**: A watch kept, then lapsed
  (`a_watch_kept_then_lapsed`), No terminal keeps watch
  (`no_terminal_keeps_watch`), A settle that runs past its bound
  (`a_settle_that_runs_past_its_bound`), A watch whose settle died
  (`a_watch_whose_settle_died`). `Watched` in
  `crates/flume-seams/src/store.rs` gains `by` and `ended`, both left out
  of the JSON while none. `kept` and `lapses` read the watch held while
  its command lives with no end, and otherwise until twice its bound
  after `ended`, or after `began`. `settle_every` writes `began`,
  `bound` and `by` first, and writes `ended` by compare-and-set only
  where the watch still names its own `began` and `by`. Round trip:
  `the_hosts_watch`. Fake store: `the_store_keeps_the_hosts_watch_and_the_clock_lets_it_lapse`.
- **A watch's settle moves every place, an agent in it or none**: A
  submitted chore with no agent
  (`s109_a_submitted_place_with_no_agent_moves_under_a_watch`), A queue
  that moves on its own (`a_queue_that_moves_on_its_own`), Two depths in
  one settle (`two_depths_in_one_settle`, through `settle_every`).
- **Each outcome is confirmed once acted on** (cited as 133): all three
  scenarios are run by `an_outcome_is_confirmed_once_acted_on` in
  `running_through.rs` and by `an_outcome_handed_again_holds_no_stage`.
- **A watch reads only the places where something is pending**: Places
  with nothing pending (`places_with_nothing_pending`), An unwatched
  drop (`an_unwatched_drop`). Both read the fake merge queue's,
  terminal's and agent program's logs.
- **A settle of the pending places moves each place in time**: A
  stall's time comes (`a_stalls_time_comes`), A hold's bound passes
  (`a_holds_bound_passes`), An end handed on once its recipient waits
  (`end_delivery.rs`), Room the host now has (`account_pools.rs`), A
  kept document let go (`tidying.rs`), A base place at each watch
  (`a_base_place_at_each_watch`).
- **A full settle reads every place**: A held submission landed by a
  hand (`a_held_submission_landed_by_a_hand`). `settle(None)` still
  settles every document the store holds.

### command-line

- **settle --every keeps the host's watch**: A watch over a scratch
  repository (`a_watch_over_a_scratch_repository` in
  `crates/flume/tests/commands.rs`, which reads `watch_ended` at or after
  `watch_settled`), A watch with a place named
  (`a_watch_with_a_place_named`, which reads the refusal's new words).

### end-delivery

- **A wait drives no settle while a caller keeps watch**: Under a watch
  (`a_wait_under_a_watch`), Attended under any terminal
  (`a_wait_attended`). `wait.rs` runs `settle_pending` before each
  unwatched reading.

### place-status

- **Status gives the host's watch**: A watched host and an unwatched one
  (`a_watched_host_and_an_unwatched_one` in `answers.rs`, and
  `a_host_under_a_watch` and `a_host_whose_watch_lapsed` in
  `commands::print`). `HostStatus.watch_ended` is filled from
  `watch_status`, and `watch_line` prints the three lines the spec and
  design 7 give.

### stream-terminals

- **No terminal decides whether flume runs attended**: One stream under
  another terminal and Two terminals and no watch
  (`stream_terminals.rs`).

### telemetry

- **Metrics by kind and stage, never by place**: The host's places at a
  settle of every place (`the_hosts_places` in `telemetry.rs`, and
  `the_hosts_reading_names_no_place` in `crates/flume-core`). `read_host`
  runs only in `settle_every`. A start's time by its kind and stage, and
  No metric names a place, are unchanged and pass.

### seam-contracts, seam-fakes, merge-queue-profile

These requirements only move the confirm's citation from 225 to 133.
The scenarios of `crates/flume-conformance/src/seams/merge_queue.rs`,
the fake merge queue's tests and `merge_queue_sluice.rs` cite 133, and
none cites 225. `conformance:coverage` passed. 225 is now cited only by
this change's scenarios, its code and core.62's detail.

## The statement and the rules

`crates/flume-conformance/src/clauses.rs` reads the statement at
boomdrivedev/blueprints 48a94da. At that revision clause 225 is in
Part A ("Something is pending on a place …"), and the confirm is part of
133 ("each then recorded by the merge queue as delivered to the place
once flume has read it"). Both were checked against the statement at
48a94da. `core-details.md` cites the confirm as 133 under core.6, and
core.62's detail names a hand's move read at a full settle.

`crates/flume-core/src/pending.rs` reads from held state, the host's
deliveries as `owing()` gives them once per settle, and the clock. It
asks no seam (core.62, core.1, core.2). Its time row runs `read_stall`
on a copy with no answerer asked, so it writes nothing (core.16). Design
1's rows are all present, along with the catch-all (`Pending::Nothing`):
a live place whose move waits on no caller and no room.

## Findings

None that holds the change back. Two notes for the record:

- The watch also lapses twice its bound after `began` when the command
  running its settle died with no end (design 4; the settling spec's "A
  watch whose settle died"). Clause 208 at 48a94da says only "while
  that settle runs, and until twice its bound has passed since it
  ended". A settle whose command died no longer runs, so this reads as
  that clause applied to a killed settle, not as a departure from it.
- The pending places are read once at a settle's start. A place made
  pending by another place's settle in the same pass is read at the next
  settle. A settle of every place reads each place once, in order, so
  this moves no place later than such a settle would (core.62).
