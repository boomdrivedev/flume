# Design

## Context

See proposal.md for why. The statement is read at boomdrivedev/blueprints
`5efb363`, on the branch
`design/flume-says-when-sluice-is-too-old-to-merge-its-work`: C.6's
submit row and its version row, with 111, 112, 124 and 203. Record 0080
holds the decision and its paths not taken, and core.6's details in
`docs/architecture/core-details.md` rule out the submit it stops. The
only page of `docs/architecture/` the change touches is `core.md`'s,
through those details; there are no area pages.

The merge-queue seam's `submit` answers `Submitted::Refused { reason }`
where the merge queue refuses a submission (99, 111). The core already
holds a refused submit as the place waiting on its caller, its waiting
words `the merge queue refused the submission: <reason>`, until a retry
or a drop (112; `crates/flume-core/src/submit.rs`). A retry submits the
place again through the same `submit`. So the whole change is the
sluice profile's: the seam, the core and the fakes stay as they are
(core.1, core.2).

The profile's `submit` (`crates/flume/src/profiles/merge_queue/sluice/mod.rs`)
picks its directory and arguments by the head: a place's worktree with
`submit <base> --no-wait --json`, or the repository with `submit <base>
--head <branch> --no-wait --json` for a base place (203). It runs sluice
through `exited`, which strips every multiplexer's session and sets
`GIT_TERMINAL_PROMPT=0` (C.6).

sluice prints its version through clap as one line on stdout, `sluice
<version>`, exiting 0. A release's build says the release, and a build
of sluice's main says 0.0.0, which no release has (sluice record 0111).

## Goals / Non-Goals

**Goals:**

- No `sluice submit` reaches a sluice before 0.97.0, or one whose
  version names no release, and the place says which release flume
  needs.
- A sluice from 0.97.0 on is handed the work as before.

**Non-Goals:**

- A check at the launch, at a place's readiness, or at the forecast,
  open, close or any read: none hands sluice work to merge (record
  0080).
- A ceiling on sluice's release, or a check of the fields each command
  answers: the profile's commands and fields are checked against the
  pinned sluice at the build (core.22).
- Any change to the core, the seam, the fakes or the conformance suite.

## Decisions

### 1. The read runs inside `submit`, before the submit's command

Once `submit` has picked its directory and arguments, it runs `sluice
--version` through `exited` in that same directory, so the version read
runs in the environment the submit would. Where the answer is a release
from 0.97.0 on, `submit` goes on exactly as it does now. Otherwise it
returns `Submitted::Refused` with the words of section 3 and runs
nothing more. Both heads go through it, a base place's submit by name
among them (203).

It reads the version at every submit and keeps nothing between them, so
a retry after the host's sluice is updated reads the new one (core.17).
Alternative: read once per `Sluice` and keep it. Not taken: a command's
`Sluice` lives one command, so it would save one process a submit at
most, and a long-lived one would hand work to a sluice replaced under
it.

### 2. A release is read from the first line and compared part by part

The profile reads a release from the answer of an exit 0 alone: its
first line, trimmed, is exactly two words, `sluice` and a version, and
the version is exactly three parts joined by `.`, each of ASCII digits
alone, read as a whole number. 0.0.0 names no release. Anything else
names no release: a fourth part, a pre-release's `-rc.1`, a build's
`+meta`, a `v` before the version, one word alone, or no line.

A release is compared part by part as numbers, major, then minor, then
patch, so 0.100.0 is later than 0.97.0 and 1.0.0 later than both.

The release flume needs is one constant of the profile, `MERGES_BY_ITSELF`,
`(0, 97, 0)`, its doc naming it the first sluice release whose own host
process moves its queues (sluice 135; sluice record 0093; record 0080).
Alternative: the release `devenv.nix` pins. Not taken, as record 0080
says: the pin moves with every chore that updates it.

### 3. The words

A refusal's words are what sluice said, then `; flume needs sluice
0.97.0 or later, the first release whose own process moves its queues`,
the constant written as `<major>.<minor>.<patch>`. What sluice said is:

- for a release before 0.97.0: `sluice 0.95.1 is too old to merge this
  work by itself`, with the release it said;
- for an exit 0 whose first line names no release: `sluice --version
  said "sluice 0.0.0", which names no release`, with the first line as
  sluice printed it;
- for any other exit, or a sluice that could not be run: `sluice
  --version gave no release: ` and then what `refusal` reads of its
  output and errors, `it said nothing` where both are empty, or the
  run's error where it could not start.

So a place on a host whose sluice is 0.95.1 waits with `the merge queue
refused the submission: sluice 0.95.1 is too old to merge this work by
itself; flume needs sluice 0.97.0 or later, the first release whose own
process moves its queues`.

### 4. The unit tests stand a `sluice` in for each answer

The tests sit in the module's `tests`, beside
`a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own`, and build
their stand-in as it does: a script `sluice` in a scratch folder of the
test's own under `std::env::temp_dir()`, named for the test and the
process, put first on the seam's `PATH` by `with_env`. The stand-in
answers `--version` with the line and the exit the test gives it, and
for any other command appends `$*` to a `ran` file beside it and, for
`submit`, prints `{"id":7,"branch":"main","ahead":[]}` and exits 0. A
helper in `tests`, `stand_in(scratch, says, code)`, writes it and
returns the seam and the `ran` file. The place is a `PlaceRef` whose
repository and location are the scratch folder, branch `fix/one`, base
`main`; the base place's head is `SubmitHead::Branch` naming `train/a`.

### 5. The tests against the pinned sluice read `--version` first

Two tools tests in `crates/flume/tests/profiles/tools/merge_queue_sluice.rs`
read the first command the counting sluice was asked as the submit:
`a_base_places_branch_over_a_scratch_repository` and
`a_queue_the_settles_never_step_is_decided_by_sluices_host_process`.
Each now reads `--version` first and the submit next, so the pinned
release is shown read and handed the work (core.22). The other tools
tests' stand-ins, `stand_in_sluice` and `stand_in_pull_requests` in
`walk.rs`, pass `--version` to the pinned sluice already, and the walk
tests find the submit by its words wherever it stands, so they stay as
they are.

### 6. The site

`site/src/content/docs/guides/merge-queue.md`'s paragraph that says
flume needs sluice 0.97.0 or later gains that flume reads sluice's
version before each submit. From an older sluice, or one whose version
names no release, it submits nothing: the place waits on its caller
with words naming the release flume needs, and a retry once sluice is
updated submits it (`flume retry <place> --submission`). It names sluice
only as a product flume integrates with (core.35).

## Risks / Trade-offs

- [A host that builds sluice from its main says 0.0.0, and every submit
  there is refused] → The words name the release flume needs; the host
  installs a release. Record 0080 takes this over leaving work waiting
  on a build flume cannot date.
- [One more process per submit] → A submit runs once per delivery or
  retry, never at each settle; `sluice --version` exits during argument
  parsing, before sluice reads a repository.
- [A later sluice changes how it prints its version] → The submit is
  refused with sluice's line in the words, never handed on unread; the
  tests over the pinned sluice fail at the pin's update first (core.22).
