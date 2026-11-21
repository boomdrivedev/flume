# Verify: a-settle-runs-no-round-of-the-merge-queue

Verified at `5b2c818` on `change/a-settle-runs-no-round-of-the-merge-queue`.
The branch was cut from `bolt/flume-leaves-the-merge-queue-to-sluice`, and
it holds the bolt's tip, `8cafda3`, as origin gives it today. The whole
checks pass. Every task is done, and every scenario of the delta is met.
Archiving the change leaves `openspec validate --specs --strict` passing
51 of 51. So the change is archived with this report.

## The whole checks

`devenv shell -- sluice exec bolt/flume-leaves-the-merge-queue-to-sluice`
ran the checks `5b2c818` affects against the bolt at `8cafda3`. It passed:
38 tasks completed and 37 passed, with results recorded for `5b2c818`,
exit 0, in 2 minutes 8 seconds. That is the bolt's `check-fast`. Its
tasks were these:

- `root:adrs`, `changes`, `format`, `hooks`, `marketplace` and `specs`
- `site:build`, `install` and `prose`
- `nix:test` and `typecheck`
- each crate's `lint` and `test`, and `conformance:coverage`
- `plugin-claude:engine-types`, `format`, `test` and `validate`
- `plugin-herdr:manifest` and `test`

sluice warned of no conflict with the bolt. moon's warnings that it
could not archive task outputs into `.moon/cache/outputs` fail no task.

Within the run, the unit test task 2.1 adds,
`profiles::merge_queue::sluice::tests::a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own`,
passed in `flume:test`. The conformance suite's
`the_fake_merge_queue_passes_the_merge_queue_scenarios`, task 3.1's
check, passed in `conformance:test`. `root:adrs`, `root:specs`,
`root:changes` and `site:prose`, the checks of tasks 5.1 to 5.4, passed.

`root:toolchain` is not among the tasks the run chose, since no
toolchain file has changed since `e542164`. There, `sluice exec
bolt/flume-leaves-the-merge-queue-to-sluice -- moon run root:toolchain`
passed, which is task 1.1's check. The pinned sluice answers
`sluice 0.97.0`.

## The tools tests

The bolt's gate does not run `check-tools`, but tasks 4.1 to 4.9 check
the change with tools tests. Their check, as task 4.9 names it, is
`sluice exec bolt/flume-leaves-the-merge-queue-to-sluice -- cargo test
-p flume --features tools --test tools -- merge_queue_sluice::
place_git_worktree:: walkthroughs::`. It passed at `16a6796`: 60 tests,
none failed, in 262 seconds, exit 0.

Since `16a6796` the branch has changed only files under
`openspec/changes/a-settle-runs-no-round-of-the-merge-queue/`, which are
the delta's text, task 5.4 and this report. `git diff 16a6796..HEAD --
':!openspec'` is empty, and the bolt's tip has not moved. The tools tests
therefore stand at `5b2c818` as they ran at `16a6796`, and they were not
run again. The tests that ran include the following:

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
- every walkthrough, including those that land a place through sluice
  and `a_pull_request_is_followed_from_no_entry_to_its_entry_and_its_landing`

## Tasks

Every task is ticked. Read against the diff:

- 1.1: sluice is pinned at 0.97.0 with its three hashes in `devenv.nix`.
- 2.1: `step` runs nothing and answers `Stepped::MovesOnItsOwn`. The
  module doc says C.6's step row, names 0.97.0, and keeps #102's held
  `refused` record beside it. The unit test passes.
- 3.1: `unreadable_says_so` asks that step is anything but
  `Stepped::Stepped`, and its words are "an unreadable queue was
  stepped".
- 4.1: the harness's hold, `decide`, the short link of `user_scratch`
  and its removal are as the task says. `refuse_another` reads
  `sluice status bolt/x` until sluice refuses it, within 30 seconds.
- 4.2 to 4.8: as written. Their tests pass under sluice's run.
- 4.9: the walk's login shell puts `login_sluice` first on its `PATH`.
  It gives `XDG_STATE_HOME` and `SLUICE_USER_SCRATCH` where they are
  unset and runs the `sluice` the walk's `PATH` finds. The logins of
  `what_the_policy_says_of_a_place` and `readied` name both. Design
  section 5 says so and names the path not taken.
- 5.1 to 5.3: record 0012's amendment, core.53's details and both
  guides say what the tasks ask. The site's rules page mirrors the
  rules' statements alone, and core.53's statement did not change.
- 5.4: "sluice takes a base place's submission by name from the
  repository" moves the pinned sluice's `--head` without `--check` and
  its recording no worktree into its scenario's WHEN. It says once that
  its queue moves by sluice's host process, which the submit starts.
  "A submitted place with no agent lands by a watch's settles" moves
  the submit's starting the host process into "Settled by a watch, no
  agent". Each keeps what it requires. `openspec validate
  a-settle-runs-no-round-of-the-merge-queue --strict` passes, and
  `openspec archive a-settle-runs-no-round-of-the-merge-queue --yes`
  over a copy of `openspec/` left `openspec validate --specs --strict`
  passing 51 of 51.

## The specs

The delta removes "One round at each undecided settle" and adds "A
settle runs no round of sluice's queue". It modifies "Read, base and
the place's data", "sluice takes a base place's submission by name from
the repository" and "A submitted place with no agent lands by a watch's
settles". Each scenario is met by a check that passed:

| Scenario | Check |
|---|---|
| A step over a stand-in sluice | `a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own` |
| A queue the settles never step | `a_queue_the_settles_never_step_is_decided_by_sluices_host_process` |
| A queue nothing drives | the same test, which reads the entry open through read-submission and then landed |
| A fetch that fails | `a_fetch_that_fails_is_answered_with_its_words` |
| What the policy says of a place | `what_the_policy_says_of_a_place` |
| The steps sluice equipped with | `the_steps_sluice_equipped_with` |
| An answer with no steps | `an_answer_with_no_steps` |
| A base place's branch over a scratch repository | `a_base_places_branch_over_a_scratch_repository` |
| Settled by a watch, no agent | `s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone` |
| A host process started again by a settle's read | `s109_a_host_process_ended_with_work_in_play_is_started_again_by_a_settles_read` |

One line of the watch's requirement, which begins "land by sluice's
host process and SHALL be read", runs past the spec's wrap. treefmt
excludes Markdown, so no check reads it, and it changes nothing the
requirement says.
