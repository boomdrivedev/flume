# Design

The statement is read at boomdrivedev/blueprints 4de08fb, on the branch
`design/flume-a-change-that-skips-specs-is-planned` cut from main at
81288ba: C.3's read-change row, amended there so a change's planning
complete is OpenSpec's own reading of it whatever its schema, with the
clauses it cites (59, 86, 87, 127). The amendment states the decision of
this change's brief, from the signal
`afterthought/crew-state:signals/2026-11-10-agentplot-design-cd548984/01-a-change-that-skips-specs-is-planned-without-one.md`
(agentplot/main bfac2be) and `boomdrivedev/blueprints:design/principles.md`
14: a tool that needs what another program knows reads that program's
own source. Until that branch lands on blueprints' main, this design and
C.3 there are read together; where they disagree, C.3 wins (core.19).

The rules relied on: core.1, core.2 and core.3 (only the OpenSpec
profile names `openspec`, the core reaches it through the change seam
alone, and the profile is the one `[profiles] change` binds); core.15
(every fact of a change read where it lives at each ask, none kept
between asks, none disagreeing with the change's own reader); core.16
(the read writes nothing into the repository or the place: its checkout
lies in the temporary folder and goes with the read); core.19 (the
clause changed first); core.22 (the tests run the `openspec` the tree
pins, 1.14.3). No area page exists; core.md binds every path this change
touches.

## 1. What OpenSpec reads

The pinned OpenSpec 1.14.3 reads a change's `.openspec.yaml` as change
metadata; `skip_specs: true` declares that the change has no spec
deltas. `openspec validate --strict` then accepts zero deltas, and
`openspec status --change <name> --json` reads every artifact whose
`generates` path lies under `specs/` as `skipped`, so `isPlanningComplete`
is true once the proposal, the design and the tasks stand. Read on
deneb on 2026-11-14, on a scratch change holding those three and no
`specs/`:

| `.openspec.yaml` | `openspec status` |
|---|---|
| `schema: spec-driven`, `skip_specs: true` | `isPlanningComplete: true`, `specs` skipped |
| `schema: spec-driven` | `isPlanningComplete: false`, `specs` ready |
| `schema: spec-driven`, `skip_specs: yes` | exit 1, `change_error`, invalid metadata, no `isPlanningComplete` |

OpenSpec honours the marker only where the metadata is valid and names a
known schema, reads the key's presence rather than its value to refuse a
marker it cannot honour, and keys its skipping on an artifact's
`generates` path, which a custom schema inherits. Those are OpenSpec's
rules, and they move with its releases.

## 2. The read (59, 86, 87, C.3)

`read` in `crates/flume/src/profiles/change/openspec/mod.rs` keeps every
fact but one as it is: the change exists where
`openspec/changes/<name>/` holds a file at the revision; its tasks and
groups are `tasks.md`'s lines, read from the archive's folder where the
change no longer stands; it is archived where
`openspec/changes/archive/<date>-<name>/` exists.

Planning complete becomes:

- a change that neither stands nor is archived: not planned, with no
  `openspec` process, as today;
- a standing change, whatever its schema: `status(&git, revision,
  name)`, the pinned `openspec status --change <name> --json` in a
  checkout of the revision's `openspec/` removed after the read, its
  `Some(complete)` taken as given and `None`, OpenSpec reading no such
  change, taken as not planned;
- an archived change that no longer stands: planned, since OpenSpec
  reads only a change that stands and an archived change was planned
  before it was archived, as today for a change of another schema.

An error from `status`, `openspec` not running or answering no JSON, is
the read's could-not-read with its words, as it is for a change of
another schema today.

The schema no longer bears on the read, so `spec_driven_planned`,
`schema`, `schema_of`, `SPEC_DRIVEN` and `PROJECT` go with their unit
test, `a_schema_is_read_from_its_line`. The module's doc comment and the
test file's header say planning complete is read through `openspec` for
every schema and the other facts from the files.

The path not taken: keep the file read and teach it the marker, reading
`skip_specs` from `.openspec.yaml` and asking no `specs/**/*.md` where it
is true. It agrees with OpenSpec on the three rows above but copies
OpenSpec's rules into flume, metadata validity and a known schema and
the `generates` paths of a custom schema among them, which is the count
of files the brief rules out and a guess at another program's knowledge
that principles 14 rules out (59).

## 3. What a read now costs

Every read of a standing change makes a checkout of the revision's
`openspec/` and runs `openspec`, where a spec-driven change ran neither.
On deneb on 2026-11-14, on this repository's `openspec/` (4.8 MB): the
checkout took 0.15 s and `openspec status` 0.42 s. The core reads a
place's change at each settle that reads the place's facts, in status,
in `status --line` and the Claude Code plugin's line on its ten-second
clock, and at a repeating stage's start. A launch reads the base's
revision, where the change does not yet stand, and so runs nothing.

The cost is taken. A read's planning complete is wanted only by a
change-written deliverable (`reading.rs`'s `delivered`), but read-change
gives every fact of a change in one read (86), and a read that skipped
planning complete for other callers would change the seam the statement
gives. The path not taken: a seam that reads planning complete apart,
asked only for a change-written deliverable; it would change B.3 and is
left to a later change should the cost be measured to matter. A cache of
the answer by revision is ruled out by core.15.

## 4. Where `openspec` runs

A host that binds `change = "openspec"` now needs `openspec` on the PATH
flume runs with for every standing change, where before only a change
of another schema needed it. Where it cannot run, the read is
could-not-read, `openspec could not run: <error>`, which status gives
with the place (59), and a change-written deliverable reads not yet made
until it can. flume ships no `openspec` and pins none on a host: the
tree's pin is the gate's (core.22).

## 5. Tests (59; kit:testing.5)

`crates/flume/tests/profiles/change_openspec.rs` gains
`a_change_that_skips_specs_is_planned_as_openspec_reads_it`: a scratch
repository with `openspec/config.yaml` and `openspec/specs/.keep`, as
the agreement test writes them, and a change holding `proposal.md`,
`design.md` and `tasks.md` and no `specs/`, committed at three
revisions with the three `.openspec.yaml` rows of section 1. At each,
read-change gives the change standing, and its planning complete equal
to what `openspec_reads` gives from the pinned `openspec` with the
revision checked out: true, false and false in turn. On the tree before
this change the first revision reads not planned, so the test fails
there.

The seam's scenarios (`the_change_profile_passes_the_change_scenarios`)
and the agreement at five revisions keep passing with planning complete
read through `openspec`; `write_change` writes what OpenSpec reads as
planned where a scenario's facts say so, and adds what the pinned
`openspec` needs to read a scratch repository's change where it lacks
it.

## 6. The record and the rule

Record 0071, proposed, at the next free number: flume takes whether a
change is planned from OpenSpec's own reading of it, whatever its
schema, and counts none of its files for it; its sources the signal,
principles 14 and C.3 at 4de08fb; its considered options the read
through `openspec` (chosen), the file read taught the marker, and a seam
that reads planning complete apart (section 3); its consequences the
cost of section 3 and the host's `openspec` of section 4. Its decision
makers are the change's brief, which decided the direction, and the
construction agent, for the choices of sections 2 to 4. It stays
proposed until the operator's yes.

core.15's details gain, among what it rules out, a change's planning
complete counted from its files where its tool reads it otherwise, as
flume read a spec-driven change that declares `skip_specs` never planned
on 2026-11-10, citing record 0071; its sources gain record 0071.
