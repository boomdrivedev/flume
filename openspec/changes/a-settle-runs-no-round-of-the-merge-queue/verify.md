# Verify: a-settle-runs-no-round-of-the-merge-queue

Verified at `16a6796` on `change/a-settle-runs-no-round-of-the-merge-queue`,
cut from `bolt/flume-leaves-the-merge-queue-to-sluice`, whose tip,
`8cafda3`, the branch holds (`4ae34f7`). The whole checks pass, every
task is done, and every scenario of the delta is met. The change is not
archived: archiving it writes a merge-queue-profile spec that the gate's
`root:specs` fails, two of the requirements the delta modifies running
past 500 characters (see "The archive").

## The whole checks

`devenv shell -- sluice exec bolt/flume-leaves-the-merge-queue-to-sluice`
passed: 38 tasks completed, 37 passed, results recorded for `16a6796`,
exit 0, in 5 minutes 16 seconds. That is the bolt's `check-fast`. sluice
warned of no conflict with the bolt. moon's warnings that it could not
archive task outputs into `.moon/cache/outputs` fail no task.

Within it, the unit test task 2.1 adds,
`profiles::merge_queue::sluice::tests::a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own`,
passed in `flume:test`. The conformance suite's
`the_fake_merge_queue_passes_the_merge_queue_scenarios`, task 3.1's check,
passed in `conformance:test`. `root:adrs`, `root:specs`, `root:changes`
and `site:prose`, the checks of tasks 5.1 to 5.3, passed.

`root:toolchain` is not among the tasks the run chose. No toolchain file
(`devenv.nix`, `devenv.yaml`, `devenv.lock`, `.prototools`,
`rust-toolchain.toml`) has changed since the last verify at `e542164`,
where `sluice exec bolt/flume-leaves-the-merge-queue-to-sluice -- moon
run root:toolchain` passed. The pinned sluice answers `sluice 0.97.0`.

## The tools tests, under sluice's run

The bolt's gate does not run `check-tools`, and most of this change's
tasks are tools tests. So task 4.9's check was run as the task names it:
`sluice exec bolt/flume-leaves-the-merge-queue-to-sluice -- cargo test
-p flume --features tools --test tools -- merge_queue_sluice::
place_git_worktree:: walkthroughs::`. It passed: 60 tests, none
failed, in 262 seconds, exit 0. That covers the checks of tasks 4.1 to
4.9. These ran and passed:

- `merge_queue_sluice::the_sluice_profile_passes_the_merge_queue_scenarios`
- both base place tests
- `a_queue_the_settles_never_step_is_decided_by_sluices_host_process`
- the stand-in's settle
- the herdr pane's and the Rex block's submissions
- the close sluice refuses
- the branch whose records sluice cannot read
- the conflict at the submit
- `what_the_policy_says_of_a_place`
- both S109 tests
- both `place_git_worktree::` tests
- every walkthrough, among them those that land a place through sluice
  and `a_pull_request_is_followed_from_no_entry_to_its_entry_and_its_landing`

Under the gate's sandbox, a place's teardown that made its socket in
the host user's `/tmp/sluice-<uid>` would be refused, as the last
verify's run showed. These tests now pass under that sandbox, so the
readiness and teardown sluice reach each test's own state folder and
user scratch (core.51).

## Tasks

Every task is ticked. Read against the diff:

- 1.1: sluice is pinned at 0.97.0 with its three hashes in `devenv.nix`.
- 2.1: `step` runs nothing and answers `Stepped::MovesOnItsOwn`. The
  module doc says C.6's step row, names 0.97.0, and keeps #102's held
  `refused` record beside it. The unit test passes.
- 3.1: `unreadable_says_so` asks for anything but `Stepped::Stepped`,
  its words "an unreadable queue was stepped".
- 4.1: the harness's hold, `decide`, the short link of `user_scratch`
  and its removal are as the task says. `refuse_another` reads
  `sluice status bolt/x` until sluice refuses it, within 30 seconds.
- 4.2 to 4.8: as written, and their tests pass under sluice's run.
- 4.9: the walk's login shell puts `login_sluice` first on its `PATH`. It
  gives `XDG_STATE_HOME` and `SLUICE_USER_SCRATCH` where they are unset
  and runs the `sluice` the walk's `PATH` finds. The logins of
  `what_the_policy_says_of_a_place` and `readied` name both. Design
  section 5 says so and names the path not taken, carrying more of the
  login shell's environment into a place's steps.
- 5.1 to 5.3: record 0012's amendment, core.53's details and both
  guides say what the tasks ask. The site's rules page mirrors the
  rules' statements alone, and core.53's statement did not change.

## The specs

`openspec validate a-settle-runs-no-round-of-the-merge-queue --strict`
passes. The delta removes "One round at each undecided settle" and adds
"A settle runs no round of sluice's queue". It modifies "Read, base and
the place's data", "sluice takes a base place's submission by name from
the repository" and "A submitted place with no agent lands by a watch's
settles". Each scenario is met by a check that passed:

| Scenario | Check |
|---|---|
| A step over a stand-in sluice | `a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own` |
| A queue the settles never step | `a_queue_the_settles_never_step_is_decided_by_sluices_host_process` |
| A queue nothing drives | the same test, read-submission reading the entry open and then landed |
| A fetch that fails | `a_fetch_that_fails_is_answered_with_its_words` |
| What the policy says of a place | `what_the_policy_says_of_a_place` |
| The steps sluice equipped with | `the_steps_sluice_equipped_with` |
| An answer with no steps | `an_answer_with_no_steps` |
| A base place's branch over a scratch repository | `a_base_places_branch_over_a_scratch_repository` |
| Settled by a watch, no agent | `s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone` |
| A host process started again by a settle's read | `s109_a_host_process_ended_with_work_in_play_is_started_again_by_a_settles_read` |

## The archive

`openspec validate a-settle-runs-no-round-of-the-merge-queue --strict`
passes, but it does not catch what the archive writes. `openspec archive
a-settle-runs-no-round-of-the-merge-queue --yes` applied the delta to
`openspec/specs/merge-queue-profile/spec.md`: 1 added, 3 modified and 1
removed. `openspec validate --specs --strict` then failed on
merge-queue-profile, 50 passed and 1 failed, with these warnings:

```text
⚠ [WARNING] requirements[5]: Requirement text is very long (>500 characters). …
⚠ [WARNING] requirements[6]: Requirement text is very long (>500 characters). …
```

`--strict` fails a warning, so this is `root:specs`, a `check-fast` task.
The two requirements are the ones the delta modifies:

| Requirement | Before | After |
|---|---|---|
| sluice takes a base place's submission by name from the repository | 473 | 609 |
| A submitted place with no agent lands by a watch's settles | 361 | 508 |

The archive was undone in the working tree, and the change stands open.

## What the code stage must do

Bring each of the two requirements' text in the delta,
`specs/merge-queue-profile/spec.md`, to 500 characters or fewer. Move
detail into the scenarios, or state the host process's part in a
requirement of its own, as openspec's warning says. Then check that
`openspec archive` would leave `openspec validate --specs --strict`
passing.
