# Verify: a-submission-the-forge-refused-waits-on-its-caller

Verified on `change/a-submission-the-forge-refused-waits-on-its-caller`
at fe1f0c0c, cut from `bolt/flume-leaves-the-merge-queue-to-sluice`
(7005c04a).

**Outcome: the whole checks pass, every task is done, every requirement
and scenario of the change's specs is met, and the specs the archive
writes pass `openspec validate --specs --strict`. The change is
archived.**

## The whole checks

`devenv shell -- sluice exec bolt/flume-leaves-the-merge-queue-to-sluice`,
run once with sluice 0.93.2 (the devenv's pin): exit 0. sluice ran the
checks fe1f0c0 affects against the base at 7005c04, the bolt's required
`#check-fast`: 25 tasks completed, 24 results recorded under their keys
for fe1f0c0, none failed. They were `seams`, `core`, `fakes`,
`conformance` and `flume`'s `format`, `lint` and `test`;
`conformance:coverage`; `nix:typecheck` and `nix:test`; `site:install`,
`site:prose` and `site:build`; `plugin-claude:test`; and `root:format`,
`root:changes` and `root:adrs`. The Rust suites passed 911 tests with
none failed, and `plugin-claude:test` 10 with none failed. `site:prose`
gave suggestions only. moon warned it could not archive task outputs
into its cache, which fails nothing.

`openspec validate a-submission-the-forge-refused-waits-on-its-caller
--strict`: valid.

The change leaves `openspec/specs/` untouched until it is archived, so
the gate's `root:changes` validates its deltas and no task of the gate
reads the specs they write. Task 6.2's check was run for that: a copy
of `openspec/` archived with the pinned OpenSpec 1.14.3, `+ 6, ~ 5, -
0`, and `openspec validate --specs --strict` over the specs it wrote:
51 passed, none failed.

The bolt requires no `#check-tools`, which runs at the bolt's merge onto
main. So task 3.2's two tests against the pinned sluice were run apart
from the gate, by the names the task gives: `devenv shell -- cargo test
-p flume --features tools --test tools -- --exact
merge_queue_sluice::a_failed_entry_is_collected_once_with_sluices_notice
merge_queue_sluice::a_settle_beside_the_stand_ins_submit_collects_at_most_the_outcome_its_answer_gives`:
2 passed, none failed.

## Tasks

Every task of `tasks.md`, 1.1 to 6.2, is ticked. Every test a task names
exists under that name and passed: 1.1's and 6.2's
`round_trip::a_held_submission_and_its_outcome` in `seams:test`; 2.1's
`the_fake_merge_queue_holds_a_landing_the_forge_refused` and 4.1 to
4.5's `the_engine_passes_its_scenarios` in `conformance:test`; 3.1 and
3.3's four `merge_queue_sluice_documents::` tests in `flume:test`; 3.2's
two tools tests in the run above; 4.6's
`telemetry::tests::a_landing_the_forge_refused_ends_the_merge_queues_wait`
and `telemetry::tests::the_waits_a_state_reads` in `core:test`; 5.1's
`site:prose` and `site:build` in the gate; and 6.1's and 6.2's
validations above.

- 1.1: `SubmissionState::Held`, `SubmissionOutcome::Held`,
  `Waits::ForgeRefused` and `SubmissionHeld.landing_refused`, serde
  default and skipped while false, each documented; `open()` gives
  waiting and under test alone, so a held state is not open.
- 3.1: the fixtures under `crates/flume/tests/profiles/fixtures/sluice/`
  carry a `README.md` naming the code at `44e33d4` each follows; the
  inbox runs `sluice inbox --json` and then one `sluice confirm <key>…
  --json` of the outcomes it read, each `delivery.key` owed or expired.
  A confirm that fails is not acted on, as design section 3 says: the
  next inbox hands those outcomes over again and the core acts on each
  once.
- 3.3: read-submission reads `refused` as held with `refused.words`, else
  with `HELD_UNSAID`; the module doc's inbox and read-submission
  paragraphs cite C.6's rows at `8b73d11`, `fc4323b` and `04e5a1a`.
- 5.1: the rules page mirrors core.58 word for word as `core.md` writes
  it, in its section after core.53 where `core.md` places it rather than
  after core.57, a choice design.md records with the path not taken.
- 6.2: the three requirements the archive modifies, in
  `merge-queue-profile`, `seam-contracts` and `telemetry`, are each
  under 500 characters; confirming by key and an unread outcome left
  owed, and a held submission's standing, are requirements of their
  own, a choice design.md records with the path not taken. Each
  modified requirement keeps every scenario the base gives it.

## Requirements and scenarios

| spec | requirement | met by |
|---|---|---|
| merge-queue-profile | Outcomes collected from sluice's inbox | "A refused outcome": `a_refused_outcome_is_held_with_its_notice_and_confirmed_by_key`, asserting `inbox --json` then `confirm 6:landed 7:refused --json`; "A conflict's base revision", "A failed entry collected once" and "Seam scenarios against sluice": the tools tests, the first run above |
| merge-queue-profile | Outcomes read are confirmed by key, and one unread is left owed | "An outcome in a state flume does not read": `an_outcome_in_a_state_flume_does_not_read_is_left_owed_and_named`, `main` passed over naming #8 and `parked` and `6:landed` alone confirmed |
| merge-queue-profile | read-submission reads a refused record as held | "A refused record with its words": `a_refused_record_is_held_with_its_refused_words`, no `wait` and no `status` of a number in the log; "A refused record without its words": `a_refused_record_without_them_is_held_saying_sluices_record_gives_none`, the place's and `train/x`'s by `from.head`, each held with `HELD_UNSAID`, neither passed over |
| seam-contracts | The types of an agent, the terminal and a submission | "A held submission and its outcome round-trip": `a_held_submission_and_its_outcome`, the held state not open and a state written before reading `landing_refused` false and writing nothing for it |
| seam-contracts | A held submission is neither decided nor moved | "A round moves no held landing": `a_held_submission_and_its_outcome` asserts the held state not open, and its round trip reads it back as `Held`, a variant apart from landed, failed, in conflict and withdrawn; `SubmissionState::Held`'s doc states the requirement |
| seam-fakes | The fake merge queue holds a landing the forge refused | "Held, pushed again, dropped and replaced": `the_fake_merge_queue_holds_a_landing_the_forge_refused`, over `Plan::Held`, `push_again`, `drop_held` and a later submit's withdrawal |
| settling | A submission whose landing the forge refused leaves the place waiting on its caller | "Held, nothing started": `held_nothing_started`, the wait a retry of code for `forge_refused` with the forge's words since the settle after the round, no agent opened, no verdict, no send-back, no step, and the since unchanged at the third settle; "A base place's held submission": `a_base_places_held_submission`, waiting on its caller with `HELD_WITHOUT_WORDS`, no fix, no forecast; "A held landing stalls": `a_held_landing_stalls`, one stall on a retry of code delivered once to its recipient |
| settling | A held submission stays open until it lands, returns, leaves or is replaced | "Pushed again": `pushed_again`, the place tidied; "Back in the queue": `back_in_the_queue`, the wait ended and the base stepped, then landed; "A retry past a held landing": `a_retry_past_a_held_landing`, code with the caller's words, the held submission withdrawn by the next, its failure the one send-back with the merge queue's words; "A held landing withdrawn": `a_held_landing_withdrawn`, the withdrawal named in status and nothing started |
| place-status | Status says what a waiting place waits on | "A submission whose landing the forge refused": `a_submission_whose_landing_the_forge_refused` in `place_status.rs`, the submission held with the record's words and the move a retry of code for `forge_refused` since t, in the JSON form and the person's |
| place-line | A place's state in one word | "A landing the forge refused": `a_landing_the_forge_refused` in `place_line.rs`, `landing refused` and then `stalled`; `state_words` gives `ForgeRefused` after `Submission` in the requirement's order, and `line.rs`'s unit test checks it |
| telemetry | A wait names whom it waits on | "A landing the forge refused": `a_landing_the_forge_refused_ends_the_merge_queues_wait`, the merge queue's wait ended `held` from 1000 to 2000, the caller's for `forge_refused` from 2000 to 3000, and a merge queue's wait begun again at the return, no words of the forge's in a signal |

## The record and the rules

Record 0071 and core.58 stand on the base (6b2c6610), and the record's
amendment of 2026-11-16 and core.58's details in the change's first
commit (a214ac00); the site's guide gives `landing refused` after `past
its bound`, `submission refused`, and its rules page mirrors core.58.
No new crate, plugin or site part, so no new moon project; the sluice
pin stays 0.93.2.
