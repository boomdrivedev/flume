# Design

The statement is read at boomdrivedev/blueprints `8b73d11`, on the
branch `design/flume-reads-a-forge-refusal`: 99, 112, 124, 130 and 133,
and C.6's inbox and read-submission rows, the inbox row as `fc4323b`
amends it and the read-submission row as `04e5a1a` amends it. sluice is
read at `44e33d4` on `bolt/sluice-is-refactored-and-runs-on-pools`: its
statement 152, its record 0124 and its specs `refused-landing` and
`submission-wait`, and the documents its `crates/sluice/src/lib.rs`
prints. flume's record 0071 and its amendment of 2026-11-16 decide the
behaviour, and rule core.58 holds it. Where this design and the
statement disagree, the statement wins (core.19).

The rules relied on: core.1 and core.2 (sluice is named only in its
profile, reached through the merge-queue seam); core.6 and core.13 (no
wait on the merge queue, a held record's words among it); core.14 (the
answers only gain fields); core.15 (a submission's facts read where they
live at each ask); core.16 and core.17 (a write by compare-and-set,
safe to repeat); core.22 (the pin stays 0.93.2); core.24 (a wait says
which move and since when, and stalls); core.27 (the kind's bound
counts goings-back); core.35 (the site names sluice only as a product
flume integrates with); core.53 (a base place's fix); core.56 (no words
in telemetry); core.58 (the held submission). No area page exists, so
core.md binds every path this change touches.

## 1. Scope

This change builds flume's reading of a held submission over the
merge-queue seam and its sluice profile, and the core's wait on its
caller for it. It changes no kind and no command, and adds no end: a
caller learns of the hold from the place's wait and, after five minutes
under no hold's words, its stall (130; record 0071's consequences).
sluice's own fix, `refused {words, at}` on every record it prints, is
sluice's, asked of its planner by boomdrive-design; flume reads it once
a release carries it and reads the record without it until then.

## 2. The seam (99, 133)

`SubmissionState` gains `Held { words: String }`: a submission whose
landing the forge refused, which the merge queue holds, with the forge's
words, or words saying the merge queue's record gives none.
`SubmissionState::open()` stays `Waiting` and `UnderTest`: a round moves
nothing of a held landing (sluice record 0124), so the core steps no
round for it, while the submission is still the place's open one, its
verdict none (section 4).

`SubmissionOutcome` gains `Held { words: String }`, the merge queue's
words for it. It is an outcome the inbox hands over and no decided one:
it decides nothing, and the core takes it as section 4 says, never as a
verdict.

`Waits` gains `ForgeRefused`, serialized `forge_refused`: the place
waits on its caller because its submission's landing is held.
`SubmissionHeld` gains `landing_refused: bool`, serde default false and
skipped when false: the place's wait, in `waiting`, is a held
landing's. Each JSON form only gains a value or a field (core.14).

## 3. The sluice profile (C.6)

**The inbox.** The profile runs `sluice inbox --json` in the place, no
`--confirm`. Each record of `outcomes[]` is read by its `state`:

- `landed`, `failed` and `conflict` as today, a conflict's base its
  `tested.on` or the base's revision where it lives;
- `refused` as `SubmissionOutcome::Held`, its words its `notice` whole,
  or where it holds none the words `said` gives every record, as C.6's
  inbox row at `fc4323b` says;
- any other state, and a record whose outcome the profile cannot read,
  as a conflict whose base cannot be read, is left owed: it is not
  confirmed, and is passed over by its `branch` with the words
  `#<id>: sluice gave an outcome in the state "<state>"`, or the
  reading's own, so status names it as it names a branch the inbox
  passed over (133; core.58).

Then the profile runs one `sluice confirm <key>… --json` in the place
with the `delivery.key` of every record it read whose delivery is
`owed` or `expired`, as `owed` reads a submit's answer. A confirm that
fails leaves them owed: the next inbox hands them over again, and the
core acts on each once, as it does a submit's answer confirmed by key
(101, 133; sluice 51). The profile no longer passes `--confirm`, since
confirming on receipt would confirm a state it then drops (core.58).
The pinned sluice, 0.93.2, gives `delivery.key` on each outcome and
takes several keys in one `confirm`, so the tools layer checks this
path.

**read-submission.** `state` reads `refused` as `SubmissionState::Held`,
its words the record's `refused.words`, as `04e5a1a`'s row says. A
`refused` record that carries no `refused`, or no `words` under it, as
sluice `44e33d4` prints every record but `report`'s, is held with the
words `the forge refused its landing; sluice's record gives none of its
words`. It is never read as not understood, never passed over and never
read through `sluice wait` or a submit that waits (core.6, core.13;
record 0071's amendment). A submit's answer is never `refused`, since a
landing comes after the rounds a submit with `--no-wait` does not wait
on; were one so, `state` reads it held, and the submit answers it
submitted, open and undecided.

**The module doc** says the inbox and read-submission as above, citing
C.6's rows at `8b73d11`, `fc4323b` and `04e5a1a`.

## 4. The core (112, 124, 130, 133)

**Taking a hold.** `Engine::landing_held(id, words)` records the place's wait
on its caller for a held landing: `submission.waiting` is `the forge
refused the submission's landing: <words>` and `landing_refused` is
true, in one write by compare-and-set (core.16). It writes only while
the place waits on that submission alone: the place is live, flume
submitted it and nothing is due, no verdict was taken since that submit,
no stage is live, and the place's last stage is delivered or finished (a
base place, which has none, always). So a held entry read again after a
caller's retry, until the next submit withdraws it, raises no wait. It
writes nothing where `landing_refused` already holds: the wait's words
and its since stand as first written.

A settle reaches it two ways. `collect` hands `SubmissionOutcome::Held`
to `landing_held`, never to `outcome`, so no verdict is taken. `submission`
reads `SubmissionRead::Found` held for the place's own submission
(`is_ours`) and hands its words to `landing_held`, which is how a base place,
which has no inbox, and a request's entry, whose outcome sluice
delivers to the request, are held (99, 133, 203). The first to read it
writes the wait.

**Starting nothing.** A held landing starts no stage, launches no base
place's fix, counts no going-back and takes no verdict, so the kind's
bound is untouched and `submission_open` holds (112, 131, 203;
core.27, core.53, core.58). `submission` steps no round for it, since
`open()` is false.

**Ending the wait.** Each of these clears `landing_refused` with
`waiting`, in the write that acts:

- read-submission reads the place's submission waiting or under test,
  back in the queue once another submission the held landing carried
  left: the wait ends, and the base is stepped as an open submission's
  (112, 124);
- it reads it landed: the work is over, as today (113);
- it reads it withdrawn, by its submitter, its planner or the operator:
  a withdrawal, as today, the place waiting for its caller naming it
  (112). The withdrawn arm takes a held submission's withdrawal though
  `waiting` holds the held landing's words;
- the inbox gives it failed or in conflict after it went back to the
  queue: the outcome is acted on as any, starting the fix round;
- a caller's retry naming a stage clears it with `waiting`, as the
  retry act does today, and the submit after the stages withdraws the
  held one (sluice 67); a retry naming the submission submits again and
  so withdraws it too (49);
- a drop ends the place.

A held landing the operator pushes again and the forge refuses again
stays held with sluice's new words on its record, which status reads at
each ask; the wait's words stay those first written.

## 5. Status, the line and the list (56, 130)

`waiting_on` gives a place whose `landing_refused` holds a wait on its
caller to retry the kind's fix round stage, `fix_round_of`, as a place
past its bound waits, its why `ForgeRefused`, since `waiting_since`,
with the wait's words. `base_waiting_on` gives a base place's with no
stage. Status's `submission.read` is read-submission's at the ask, held
with the record's words (core.15). The line's state for `ForgeRefused`
is `landing refused`, after `submission refused` in the line's order;
the list gives it in the same words and waits on the caller (core.52).
The stall and its delivery are 130's as for any wait on the caller.

## 6. The trace (217)

`waits_of` gives no wait on the merge queue while `landing_refused`
holds: the merge queue's wait ends at the settle that reads the landing
held, and the caller's wait, `forge_refused`, begins there. Once the
submission goes back to the queue the caller's wait ends and a merge
queue's begins again. `outcome_of` names a merge queue's wait that
ended on a held landing `held`. No words of the forge's go on a span or
a record (core.56).

## 7. The fake merge queue

`Plan::Held { words, recorded }`: the round that reaches the
submission holds it, its record `Held` with `words` where `recorded`,
else with `HELD_WITHOUT_WORDS`, words saying the merge queue's record
gives none of the forge's, and
leaves `SubmissionOutcome::Held` with `words` in its place's inbox, as
sluice's notice carries them. `push_again(repository, branch)` lands
the branch's held submission as the same landing, merging its head,
its outcome landed in the inbox. `drop_held(repository, branch)` puts it
back to waiting under its plan, as a held landing dropped when another
of its submissions left. A submit of a branch whose newest submission is
held withdraws it, as sluice 67 does. A held submission is not open, so
`run_round` leaves it.

## 8. Fixtures and tests

The pinned sluice holds no landing, so the profile's reading of
`refused` is checked over a stand-in `sluice` written under each test's
scratch folder. It logs each command it is asked, a line each, and
answers from fixtures under `crates/flume/tests/profiles/fixtures/sluice/`
that hold sluice `44e33d4`'s documents: `sluice overview --json` naming
`main` and `train/x`; `sluice status <base> --all --json` with a
`refused` record of the place's branch and one of the base place's
branch `train/x` by `from.head`, each once with `refused {words, at}` as
`report` prints it and once without, as `status_json` prints it today;
and `sluice inbox --json` with a `refused` outcome, its `notice` as
`sluice_core::notice` words it and its `delivery` owed under
`<id>:refused`, beside a `landed` one and one in a state flume does not
read. Each fixture is written from sluice's code at `44e33d4`, which it
names in a `README` beside them. treefmt already excludes
`crates/flume/tests/profiles/fixtures/**`.

The core is checked by the engine's scenarios over the fake merge queue
(section 7), the fake by a test of its own, the seam's types by a round
trip, the line and the trace by unit tests of `flume-core`, and the
inbox's confirm by key against the pinned sluice by the tools layer's
`a_failed_entry_is_collected_once_with_sluices_notice`.

## 9. The record, the rules and the site

Record 0071's amendment of 2026-11-16 and core.58's details are this
change's first commit, written by the construct when it heard the
rulings. The site's Claude Code guide gains `landing refused` in its
table of line states, and `site/src/content/docs/architecture/rules.md`
mirrors core.58 as `core.md` writes it, as it mirrors core.57 (core.35).

## 10. The construction agent's choices

Each made on the rules written and cheap to change, with the path not
taken:

- the wait is written once, by the first of the inbox and the record to
  give it, and not rewritten while it stands; not rewriting it at each
  read, which would move its since at each settle where the inbox's
  notice and the record's words differ (130);
- the wait is taken only while the place waits on that submission
  alone; not taking it whenever the place's own entry reads held, which
  would raise it again over a stage a caller's retry started;
- the move waited on is a retry of the kind's fix round, as a place past
  its bound waits, with a why of its own; not `Waits::Submission`,
  `submission refused`, which names the merge queue's refusal at submit;
- `landing refused` as the line's word; not `held`, which the line gives
  a caller's hold;
- no merge queue wait in the trace while held; not both waits at once,
  since the place's next move is its caller's (130, 217);
- no round stepped for a held submission; not stepping it as an open
  one, since sluice pushes a held landing again only on the operator's
  word;
- one `sluice confirm` with every key read; not a confirm a key;
- an unread outcome passed over by its branch; not a new field of the
  inbox's answer, since status already names what the inbox passed over;
- fixtures written from sluice's code at `44e33d4`; not captured from a
  run of a sluice built there, which the tree pins nowhere (core.22).
- `Engine::landing_held` as the core's name for taking a hold; not
  `Engine::held`, which already reads a place's held state;
- the fake's words for a held record with none name the merge queue;
  not sluice's words of section 3, since the fakes name no tool
  (core.1);
- the fake's held plan held once, the landing put back in the queue
  landing at its next round unless a test decides it otherwise, and
  `decide` reaching a held landing so a test withdraws it as a hand
  does; not a plan held again at every round, which would leave no
  landing to read back in the queue;
- core.58 on the rules page in its section after core.53, where
  `core.md` places it; not after core.57, which is another section's
  rule;
- the three requirements whose text the archive writes over openspec's
  500 characters brought under them by giving confirming by key and an
  unread outcome, and a held submission's standing, requirements of
  their own, and by leaving a conflict's `tested.on` and the merge
  queue's wait from a return to the scenarios that already say them;
  not by cutting the clauses' words, which would drop what they say
  (core.21).
