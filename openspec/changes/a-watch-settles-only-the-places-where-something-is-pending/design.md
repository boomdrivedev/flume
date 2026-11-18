# Design

## Context

See proposal.md, Why. The statement is boomdrivedev/blueprints
`design/flume/requirements.md` at 48a94da: 225, what is pending on a
place; 46, 119 and 208 as amended to read it; 208's watch held while
its settle runs, from b20e60b before it; 220's host reading at a
settle of the watch's. Record 0078 (accepted) and rule core.62 hold
the outcome; record 0051's watch reads as amended there.

As built on this base, `crates/flume-core/src/settle.rs`:

- `settle_every` writes the host's watch (`Watched { began, bound }`,
  `crates/flume-seams/src/store.rs`) and runs `settle(None)`, which
  calls `settle_place` for every document the store holds and then
  `read_host`.
- `settle_attended`, which every acting command calls first
  (`acts.rs`, `agent_acts.rs`, `launch.rs`, `tidy.rs`, `wake.rs`,
  `wait.rs`'s inbox), runs `settle_all` where no caller keeps watch:
  every place again. A wait's unwatched readings call `settle_all` too.
- `settle_place` delivers ends, takes up to 64 steps (`step`: the
  submission, which reads the merge queue's record of a landing place
  each time; the current stage's end, which reads the place's branch,
  its change and its agent), forecasts a base place, reads a stall,
  delivers again, wakes roles, shows the place's clues in its terminal
  and starts the stages waiting for room.
- `Watched::kept` reads the watch kept while its settle began less than
  twice its bound ago, so a pass longer than twice its bound reads the
  host unwatched while it still runs.

No area page of `docs/architecture/` exists; `core.md` binds the
change. Rules it relies on: core.13 (no process of flume's; whether
flume runs attended read from the host's watch alone), core.62 (the
outcome), core.14 (one form, only gaining fields), core.15 and core.52
(status and the list read where facts live), core.16 and core.17 (held
state by compare-and-set, safe to repeat), core.25 (an end handed over
once, delivered on its recipient's confirm), core.26 (the queue), core.6
(a submission read at the settles that follow it) and core.1 and core.2
(no tool named outside a profile; each seam through one interface).

## Goals / Non-Goals

**Goals:**

- A watch's settle, an acting command's settle where none keeps watch,
  and a wait's unwatched settle read only the places where something is
  pending, and move each place no later than a settle of every place
  would (225; core.62).
- The watch held while its settle runs, renewed by that settle alone
  (208).
- The clause list at 48a94da, and the confirm cited where that revision
  holds it (core.19).

**Non-Goals:**

- No seam gains an operation: what is pending is read from what flume
  already holds, and no profile changes.
- `flume settle` naming no place, a settle naming a place and an agent's
  on-quiet settle what they settle now (121, 225).
- The cost of a settle of one pending place is unchanged: a place that
  is read is read as it is now. Making a pending place's read cheaper,
  its clues or its change's facts among it, is no part of this change.
- The runner, sluice and herdr are untouched: `flume settle --every
  <bound>` keeps its form (record 0078).

## Decisions

### 1. What is pending, read from held state alone

A new module, `crates/flume-core/src/pending.rs`, reads whether
something is pending on a place from its held state, the host's
deliveries as `owing()` reads them once per settle, and the host's
clock, and asks no seam: no merge queue, terminal, agent program,
place or change (225: "flume alone reads what is pending"; core.62).
It gives why, one of a closed set, so a scenario can name it; no line
of flume's log names it, since the log hands each line on as a warning
and a watch reads every place at each bound (core.56):

| 225 | held state |
|---|---|
| it is being made | `Phase::Making` |
| its work is over and it is not yet tidied away | `Phase::Over` or `Phase::Gone`; a `Phase::Tidied` document whose recipients pending as `owing` reads them differ from those it holds, none among it, so it is to be rewritten or let go (62) |
| a stage of it starts or runs | a stage's start held `live`: its agent may have ended with no word, never received its prompt, worked past its limit, been kept open past its time or spent its account; or an agent kept open for its answer whose letting go, or its end, is not yet recorded (38, 45, 118, 146, 198) |
| its submission is in flight | `submission_open` (made, no outcome held), or a submission `due` with no hold on it and no live start (111, 124) |
| it is a base place | its kind lands and has no stages (202, 203) |
| a time it waits on has come | the move it waits on its caller for, read again at the clock as `refresh_waiting` reads it, under no hold's words and not yet stalled, waiting `stall_after` or longer; or a hold on that move whose bound has passed and that has not lapsed (130) |
| an end of it, or a wake for it, waits to be handed on | an end with recipients whose delivery the host's deliveries do not hold yet; a notice claim answered sent whose hand-over they lack; or an end other than a drop's or a done's pending for a recipient nothing handed it to, a role's address among them (43, 132) |
| it waits on nothing | a live place whose move, read again, waits on no caller, with no stage waiting for room: a step is the settle's to take |

A live place that waits on its caller, an answer, a retry, a done or a
drop, under none of the rows has nothing pending (42, 47, 49, 53). The
last row keeps a place whose held state names no wait pending, so a
state no row foresaw is read as before rather than left.

The rows mirror the steps that act on them: `step` for the phases, the
live start and the submission, `forecast_base` for a base place,
`read_stall` for the times, and `deliver_ends`, `record_claims` and
`wake_roles` for the hand-overs. Each condition is the one the step
tests before it acts, so a place a row leaves out is one the step would
leave as it is. The times are read by `read_stall` itself, on a copy of
the held state with no answerer asked, a time come being a stall it
records, a hold it lapses or an earlier stall it reads; a predicate
restating its tests was the path not taken, since it could drift from
the step.

Considered: a caller naming the places, as the runner reading flume's
deadlines and the merge queue (record 0078, the option not chosen); a
pending flag written by each step into held state, which a crash
between the step and its write would leave stale, where a reading of
held state at each settle cannot.

### 2. Room is started host-wide, once per settle

A stage waiting for room is no reason of its own to read its place:
`start_waiting`, host-wide, reads the queue from held state, the host's
configuration and its accounts, and starts what the host now has room
for (70, 146, 207). The settle of the pending places runs it once after
the pending places, whether or not any place was pending, so room freed
by an account's reset or by a place's tidy is taken within the watch's
bound with nothing pending (225: "it waits for room the host now has").
A start it makes is live at the next settle, so the place is read then.

### 3. A place waiting only on its caller is read at its caller's call

225 holds that what moves such a place calls flume. A move a hand makes
outside flume of such a place, its branch landed by a hand's submission
(S30) or a deliverable committed onto a stage read lost, is read at a
settle naming no place and stating no bound, or at its caller's next
act on the place, which settles it, and never by a watch. That is 225's
own definition: a hand acting outside flume is its caller's move, and
reading every waiting place's merge-queue record at each watch to find
one is what the user chose against (record 0078). The settling spec's
"A full settle reads every place" holds it, and core.62's detail names
it under "Not ruled out".

Considered: keeping every landing place that waits on its caller
pending, so the watch reads its merge queue's record each time. That is
the sweep's cost for the most common waiting place, and nothing in 225's
list asks it.

### 4. The watch: when it began, by whom, when it ended

`Watched` gains `by`, the command running the watch's settle as
`host.command()` names it and as a launch's making names its command
(2, 76), and `ended`, when that settle ended; both are optional, so a
`watch.json` written before reads as it did (core.16).

- `settle_every` writes `began`, `bound` and `by` before it settles, as
  `keep_watch` writes now, by compare-and-set, a watch that began later
  kept; it writes `ended` once its settle ends, by compare-and-set,
  only where the watch held still names its own `began` and `by`.
- A watch is kept while `ended` is none and `by` lives
  (`host.alive`); and otherwise until twice its bound after `ended`,
  or after `began` where its command died with no end, or where a
  document written before names no command (208). `kept` and `lapses`
  take the liveness as an argument, so `flume-seams` names no host.
- The watch is renewed by its settle alone: no other write of the host's
  watch is added (208; core.13).

Considered: a heartbeat or a lease apart from the settle, which would
read the host watched while a stuck runner settles nothing (record
0078); `ended` alone, with no command, which leaves a settle killed
mid-pass reading watched for ever.

### 5. Which settles read the pending places

- `settle_every` settles the pending places, then hands on the host's
  reading (220).
- `settle_attended` and a wait's unwatched readings settle the pending
  places (119), the reading not handed on.
- `settle(None)`, `flume settle` naming none and stating no bound,
  settles every place the store holds, as now, the reading not handed
  on (220 at 48a94da reads it at a settle of the watch's alone).
- `settle(Some(place))`, an act's own settle of its place and an
  agent's on-quiet are unchanged (121).

`settle_all` gives way to `settle_pending`: it reads every document
once, `owing()` once and the clock once, settles each pending place by
`settle_place` as now, then starts the waiting stages once and lets go
of the gone deliveries (62). `Answer::Settled` names the places it
settled.

### 6. A place read by no settle shows no clue anew

`show` runs in `settle_place`, so a place with nothing pending has its
clues shown anew in no terminal: the count of places on its base, which
another place's landing changes, stays as last shown until the place is
read again. A clue is display only and nothing reads one back (139), so
the cost of a terminal call per waiting place per watch buys nothing a
caller relies on.

### 7. Status, the person's words and the help

`HostStatus` gains `watch_ended`, none while the watch's settle runs
and where none is held (core.14); `watch_lapsed` reads `lapses`. The
person's form says `a caller keeps watch: the places where something is
pending settle every <bound>, last at <time>`, and `no caller keeps
watch[ since <time>]: the places where something is pending settle at
each command of flume's that acts` (core.35). `flume settle --every`'s
help and its refusal beside a place say it settles the places where
something is pending as a caller's watch.

### 8. The clause list at 48a94da

At 48a94da Part A holds 225, the pending places, and Part B no 225; the
other clauses, the gaps (100, 150, 194, 209) and S1 to S131 are as at
aa710c6. The merge queue's confirm, which this tree cites as 225 from
`design/flume-confirms-an-outcome-it-acted-on` at aa710c6, a branch that
never reached blueprints' main, is part of 133 at 48a94da ("each then
recorded by the merge queue as delivered to the place once flume has
read it"). So `clauses.rs` reads 48a94da, the confirm's scenarios and
comments cite 133, and the specs that cite the confirm move their
citation in this change's deltas. core.6's detail on `core-details.md`
cites 133 likewise. Records 0012 and 0072 keep their citations, each
read at the revision it names.

### 9. The site and the README

`reference/commands.md` (settling first; `flume settle --every`),
`reference/state.md` (`watch.json`'s fields; `watch`, `watch_ended`,
`watch_lapsed`), `guides/places-and-herdr.mdx` (keeping watch),
`guides/merge-queue.md`, `guides/callers-and-holds.md` and
`plugins/herdr/README.md` say a watch, and an unwatched command, settle
the places where something is pending, the watch held while its settle
runs and until twice its bound after it ended (core.35).

## Risks / Trade-offs

- [A row of decision 1 drifts from the step it mirrors, and a place a
  full settle would move is left] → each row has an engine scenario
  through a watch, the settling spec's "A settle of the pending places
  moves each place in time", and a place in no foreseen state is
  pending by the last row.
- [A hand's move of a waiting place waits for its caller's call] →
  decision 3; `flume settle` with no place moves it at once.
- [A recipient no pane holds keeps its place pending until its inbox
  hands the end over] → that place's settle is its deliveries' alone
  once its work is over, and the end is handed by the recipient's first
  inbox (43).
- [A runner's settle that hangs keeps the host watched] → every herdr
  and Rex call is bounded (core.29), and a killed settle lapses twice
  its bound after it began.

## Migration Plan

A `watch.json` written before has no `by` or `ended`, and lapses twice
its bound after it began, as now; the next watch's settle writes both.
Nothing else is migrated, and no host document or held state changes
form but the watch's, which only gains fields.
