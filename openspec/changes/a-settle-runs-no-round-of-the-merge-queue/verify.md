# Verify: a-settle-runs-no-round-of-the-merge-queue

Verified at `e542164` on `change/a-settle-runs-no-round-of-the-merge-queue`,
cut from `bolt/flume-leaves-the-merge-queue-to-sluice`. Not archived: the
walkthroughs that land through the pinned sluice fail under sluice's own
run, and the branch would conflict with the bolt's tip.

## The whole checks

`devenv shell -- sluice exec bolt/flume-leaves-the-merge-queue-to-sluice`
passed: 38 tasks completed, 37 passed, results recorded for `e542164`,
exit 0. That is the bolt's `check-fast`. The unit test task 2.1 adds,
`profiles::merge_queue::sluice::tests::a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own`,
ran in `flume:test` and passed. `root:toolchain` was not among the tasks
the run chose, so task 1.1's check was run alone,
`sluice exec bolt/flume-leaves-the-merge-queue-to-sluice -- moon run root:toolchain`,
and passed. The pinned sluice answers `sluice 0.97.0`.

## The tools tests, under sluice's run

The bolt's gate does not run `check-tools`, and most of this change's
tasks are tools tests, so they were run under sluice as main's gate runs
them: `sluice exec bolt/flume-leaves-the-merge-queue-to-sluice -- cargo
test -p flume --features tools --test tools -- merge_queue_sluice::
walkthroughs::`.

- Every `merge_queue_sluice::` test that drives the profile through its
  harness passed, among them the merge-queue and base-place scenarios,
  `a_queue_the_settles_never_step_is_decided_by_sluices_host_process`, both
  base place tests, the stand-in's settle, the herdr pane's and the Rex
  block's submissions, the close sluice refuses, and the unreadable branch.
- Both S109 tests failed, and every walkthrough that lands a place through
  sluice failed: `a_change_of_three_task_groups_takes_three_agents_in_turn`,
  `a_change_runs_construct_code_and_verify`,
  `a_conflict_sluice_answers_at_the_submit_goes_back_to_code_and_lands`,
  `a_conflicting_submission_merges_the_base_and_lands`, both failed-submission
  walkthroughs, `a_fix_lands_through_flumes_own_submission`,
  `a_fix_agents_line_is_status_line_and_the_fix_lands`,
  `a_fix_lands_when_its_prompt_is_submitted_inside_paste_tags`,
  `a_fix_on_a_stream_under_rex_lands_through_the_binary`,
  `a_fixs_code_finding_work_waits_on_an_answer_and_its_retry_lands` and
  `a_forks_attribution_is_the_hosts_a_repositorys_its_own_and_another_its_accounts`.
  The run then hit the branch's 10-minute bound, so the rest of the
  walkthroughs never reported.

Run alone,
`s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone`
passes under `devenv shell -- cargo test` in 14 seconds and fails under
`sluice exec` after 15 settles. In the failing run sluice landed the
chore, and flume read the landing. The tidy was then kept, because its
teardown failed:

```text
"teardown": {"step": "sluice release", "words": "could not release
…/home/data/flume/places/c7339efb50b3/chore--one: creating
/private/tmp/sluice-501/5a91e59e5c: Operation not permitted (os error 1)
…
sluice: warning: sluice's host process could not be reached, so nothing
moves the queue until a command of sluice's starts it: sluice's host
process did not answer within 10 seconds of its start; see
…/flume-test-watch-chore-…/home/.local/state/sluice/host/log"}
```

The place's teardown, `sluice release`, runs in the login environment.
That environment gives the walk's `HOME` but neither the walk's
`XDG_STATE_HOME` nor its `SLUICE_USER_SCRATCH`. Under 0.97.0 every sluice
command reaches or starts the host process of its state folder. So this
release looks for a host process under `home/.local/state/sluice`, a
second state folder that is not the walk's. It then tries to make its
socket's folder in the host user's default short directory,
`/private/tmp/sluice-501/<digest>`, which lies outside the scratch
sluice's run gives the check, and the run refuses it. Under 0.93.2 a
release started no host process and made nothing there, so the pin bump
of task 1.1 brings this in.

Run by hand, the S109 walk passes, but the teardown's sluice then reaches past
the test's scratch into the host's own short directory and starts a host
process of a state folder the walk does not name. Under the gate they
fail. That breaks core.51, under which a check finds room for what it
makes, a socket's path among them, in the scratch the merge queue gives
its place. It also breaks the harness's own claim that nothing reaches
the host's own sluice state. Design section 5 assumes that "each walk
already gives sluice its own `XDG_STATE_HOME` and `SLUICE_USER_SCRATCH`".
That holds for the sluice commands the walk runs and for flume's own, but
not for the place's teardown, which runs under the login environment. Task
1.1's check of "each place a test reaches into sluice's records or state"
did not catch it. Main's `check-tools` would fail on these walkthroughs.

## The base moved

`sluice exec` warned `would conflict:
crates/flume/src/profiles/merge_queue/sluice/mod.rs with
bolt/flume-leaves-the-merge-queue-to-sluice`. The bolt is at `8cafda3`,
past this branch's merge base `7005c04`, with #102
(a-submission-the-forge-refused-waits-on-its-caller) landed. Its commits
`98245df` and `47af0b4` change the sluice profile's module, so the branch
needs the bolt merged in before it can land.

## Tasks

Every task is ticked. Read against the diff:

- 1.1: sluice is pinned at 0.97.0 with its three hashes, and
  `root:toolchain` passes. The check of where tests reach sluice's state
  missed the teardown's environment, as above.
- 2.1: `step` runs nothing and answers `MovesOnItsOwn`. The module doc
  says C.6's step row and names 0.97.0. The unit test passes.
- 3.1: `unreadable_says_so` asks for anything but `Stepped`, with its
  words as written. The conformance suite passes.
- 4.1: the harness's hold, `decide`, `host_process` and the short link of
  `user_scratch` are as the task says. `refuse_another` reads
  `sluice status bolt/x` once and asserts the refusal. The task says to
  read it until sluice refuses it, within 30 seconds. The test passes, but
  it does not follow the task's wording.
- 4.2 to 4.4 and 4.7: as written, and passing under sluice's run.
- 4.5, 4.6 and 4.8: written as the tasks say, but their tests fail under
  sluice's run, as above. Only 4.5's test was also run by hand, where it
  passes.
- 5.1 to 5.3: record 0012's amendment, core.53's details and both guides
  say what the tasks ask. The site's rules page mirrors the rules'
  statements alone, and core.53's statement did not change, so there is
  nothing to mirror there. `root:adrs` and `site:prose` passed in the
  whole checks.

## The specs

`openspec validate a-settle-runs-no-round-of-the-merge-queue --strict`
passes. The delta's requirement names match the merge-queue-profile
spec's. These scenarios are met, the checks proving them passing under
sluice's run: "A step over a stand-in sluice", "A queue the settles never
step", "A queue nothing drives" and "A base place's branch over a scratch
repository". "Settled by a watch, no agent" and "A host process started
again by a settle's read" are not met under sluice's run: the chore lands
but is never tidied, because its teardown cannot run.

## What the code stage must do

- Have the sluice commands a place's readiness and teardown run in the
  walks and harnesses reach the test's own state folder and its
  `SLUICE_USER_SCRATCH`. Today the login environment gives them neither,
  so under 0.97.0 they start a second host process and make its socket
  outside the scratch sluice's run gives the check (core.51). Then run the
  walkthroughs and both S109 tests under `sluice exec` to their end.
- Merge the bolt's tip, `8cafda3`, into the branch, resolving
  `crates/flume/src/profiles/merge_queue/sluice/mod.rs` against #102.
- Optionally, have `refuse_another` read `sluice status bolt/x` until it is
  refused, within 30 seconds, as task 4.1 says. Otherwise reword the task
  to match the test.
