# Verify: flume-says-when-sluice-is-too-old-to-merge-its-work

Verified at `22ad08f` on
`change/flume-says-when-sluice-is-too-old-to-merge-its-work`, cut from
`bolt/flume-leaves-the-merge-queue-to-sluice` at `36ee040`. The whole
checks pass, the two tools tests the change touches pass against the
pinned sluice, every task is done and every scenario of the delta is
met. So the change is archived with this report.

## The whole checks

`devenv shell -- sluice exec bolt/flume-leaves-the-merge-queue-to-sluice`
ran the checks `22ad08f` affects against the bolt at `36ee040`, the
bolt's `check-fast`. It passed: 11 tasks completed and 10 passed, their
results recorded under their keys for `22ad08f`, exit 0, in 2 minutes
40 seconds. Its tasks were these:

- `root:format` and `root:changes`
- `site:install` and `site:build`
- `nix:test` and `nix:typecheck`
- `flume:format`, `flume:lint` and `flume:test`
- `plugin-claude:test`
- `conformance:test`

Within `flume:test`, the three unit tests tasks 1.1 to 1.3 add passed:
`profiles::merge_queue::sluice::tests::a_sluice_before_its_host_process_is_handed_no_work`,
`a_sluice_that_names_no_release_is_handed_no_work` and
`a_sluice_with_its_host_process_is_handed_the_work`. `site:build`, task
3.1's check, passed. `root:changes` holds the open change to `openspec
validate --strict` and to archiving cleanly (core.59), and it passed.
moon's warning that it could not archive `flume:test`'s outputs into
`.moon/cache/outputs` fails no task.

## The tools tests

The bolt's gate does not run `check-tools`, but tasks 2.1 and 2.2 check
the change with tools tests against the pinned sluice, which answers
`sluice 0.97.0`. Both ran under sluice's run, `sluice exec
bolt/flume-leaves-the-merge-queue-to-sluice -- cargo test -p flume
--features tools --test tools --
merge_queue_sluice::a_base_places_branch_over_a_scratch_repository
merge_queue_sluice::a_queue_the_settles_never_step_is_decided_by_sluices_host_process`.
It passed 2 of 2 in 7.65 seconds.

## Tasks

Every task is ticked. Read against the diff:

- 1.1: `submit` picks its directory and arguments by its head, then runs
  `merges_by_itself` in that directory, which runs `sluice --version`
  through `exited`, for both heads. `MERGES_BY_ITSELF` is `(0, 97, 0)`.
  Its doc cites sluice 135, sluice record 0093 and record 0080.
  `release` reads exactly `sluice` and a version of three all-digit
  parts from the first line, and it reads 0.0.0 as no release. A release
  before the floor answers `Submitted::Refused` with design section 3's
  words and runs nothing more. The module doc says so, citing 111, 112,
  203 and C.6's submit and version rows. `stand_in(scratch, says, code)`
  and the unit test are as the task says.
- 1.2: an exit 0 naming no release says `sluice --version said
  "<line>", which names no release`. Any other exit says `sluice
  --version gave no release: ` and what `refusal` reads, or `it said
  nothing`. A sluice that cannot be run gives the run's error. Each ends
  with the release flume needs. The unit test covers `sluice 0.0.0`,
  `sluice 0.97.0-rc.1`, `sluice` and an empty answer with exit 64.
- 1.3: the unit test covers 0.97.0, 0.100.0 and 1.0.0, each handed the
  work with the entry `7` and `submit main --no-wait --json` as its one
  logged command. `submit` needed no change for it.
- 2.1: `a_base_places_branch_over_a_scratch_repository` reads `--version`
  first and `submit main --head train/a --no-wait --json` next. Its
  other assertions are kept.
- 2.2: `a_queue_the_settles_never_step_is_decided_by_sluices_host_process`
  reads `--version` first and the submit next. It keeps its check that
  every later command is a read.
- 3.1: the merge-queue guide says that flume reads `sluice --version`
  before each submit. From an older sluice, or one whose version names
  no release, it submits nothing, and the place waits with words naming
  what sluice said and the release flume needs. `flume retry <place>
  --submission` submits the place once sluice is updated, and that
  command is in the commands reference. The guide names sluice only as a
  product flume integrates with (core.35).

## The specs

The delta adds "No work is handed to a sluice too old to merge it by
itself" to `merge-queue-profile`. Each of its scenarios is met by a check
that passed:

| Scenario | Check |
|---|---|
| A sluice before its host process | `a_sluice_before_its_host_process_is_handed_no_work`, over a place's submit from its worktree and a base place's by name |
| A sluice that names no release | `a_sluice_that_names_no_release_is_handed_no_work` |
| A sluice with its host process | `a_sluice_with_its_host_process_is_handed_the_work` |
| The pinned sluice is handed the work | `a_base_places_branch_over_a_scratch_repository`, which also reads that `train/a`'s work landed on main |

The stand-in of "A sluice with its host process" logs every command but
`--version`, so the test reads the submit as the one logged command. It
does not read the order of the version read and the submit. `submit`
runs `merges_by_itself` before it runs the submit, and "The pinned
sluice is handed the work" reads that order against the pinned sluice.

Record 0080 and core.6's details, written at the change's planning, say
what the code does. The sentence that task 1.1 adds to the module doc
of `sluice/mod.rs` was not rewrapped. Its last line runs past the
module doc's width. rustfmt does not wrap comments, so no check reads
it, and the doc's meaning is the same.
