---
number: 72
title: flume takes whether a change is planned from OpenSpec's own reading of it, whatever its schema, and counts none of its files for it
status: proposed
date: 2026-11-14
decision-makers:
- the brief of a-change-that-skips-specs-is-planned
- the construction agent of a-change-that-skips-specs-is-planned
---

# flume takes whether a change is planned from OpenSpec's own reading of it, whatever its schema, and counts none of its files for it

Status: proposed. The direction is the change's brief, from the signal `afterthought/crew-state:signals/2026-11-10-agentplot-design-cd548984/01-a-change-that-skips-specs-is-planned-without-one.md` (agentplot/main bfac2be), still true on flume's main at ae188b74 on 2026-11-14, and `boomdrivedev/blueprints:design/principles.md` 14: a tool that needs what another program knows reads that program's own source. The statement's C.3, amended on boomdrivedev/blueprints' branch `design/flume-a-change-that-skips-specs-is-planned` at 4de08fb, states it. What this record decides beyond them is the construction agent's, made on the rules written and cheap to change, and stays proposed until the operator's yes.

Deciders: the change's brief, which decided the direction; the construction agent of `a-change-that-skips-specs-is-planned`, for the choices of its design's sections 2 to 4.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 4de08fb, clauses 59, 86, 87 and 127 and C.3's read-change row; `boomdrivedev/blueprints:design/principles.md` 14; the signal above; the change's design, `openspec/changes/a-change-that-skips-specs-is-planned/design.md`; rules core.1, core.2, core.3, core.15, core.16, core.19 and core.22; OpenSpec 1.14.3 as the tree pins it, read on deneb on 2026-11-14 on a scratch change holding its proposal, design and tasks and no `specs/`.

## Context and problem statement

A spec-driven change may declare `skip_specs: true` in its `.openspec.yaml`. The pinned OpenSpec honours it: `openspec validate --strict` accepts the change with no delta spec, and `openspec status --change <name> --json` reads every artifact whose `generates` path lies under `specs/` as skipped, so its planning is complete once its proposal, design and tasks stand. flume's OpenSpec profile read a spec-driven change's planning complete from a count of its files that asked for a `specs/**/*.md`, so it read such a change never planned, and a construct stage that wrote one never delivered (signal 2026-11-10-agentplot-design-cd548984/01, on flume 0.20.0). That answer disagreed with OpenSpec's own at the same revision, which clause 59 rules out.

OpenSpec's rules for the marker are its own: it honours the key only where the change's metadata is valid and names a known schema, refuses a marker it cannot honour by the key's presence rather than its value, and keys its skipping on an artifact's `generates` path, which a custom schema inherits. Read on deneb on 2026-11-14:

| `.openspec.yaml` | `openspec status` |
|---|---|
| `schema: spec-driven`, `skip_specs: true` | `isPlanningComplete: true`, `specs` skipped |
| `schema: spec-driven` | `isPlanningComplete: false`, `specs` ready |
| `schema: spec-driven`, `skip_specs: yes` | exit 1, `change_error`, invalid metadata, no `isPlanningComplete` |

## Considered options

1. **Planning complete read through `openspec status` for every schema (chosen).** The profile runs the pinned `openspec status --change <name> --json` for every standing change, in the checkout of the revision's `openspec/` it already made for a change of another schema, and counts none of the change's files for it.
2. **The file read taught the marker.** The profile reads `skip_specs` from `.openspec.yaml` and asks no `specs/**/*.md` where it is true. It agrees with OpenSpec on the three rows above, but copies OpenSpec's rules into flume, the metadata's validity, a known schema and a custom schema's `generates` paths among them, which drift with each release: the count of files the brief rules out, and a guess at another program's knowledge that principles 14 rules out (59).
3. **A seam that reads planning complete apart**, asked only for a change-written deliverable, the one reader that wants it (`reading.rs`'s `delivered`). It would spare every other read its `openspec` run, but read-change gives every fact of a change in one read (86), and this would change the seam B.3 gives.

## Decision outcome

Option 1, as the brief decided.

1. **A standing change's planning complete is `openspec status`'s**, whatever its schema: its `isPlanningComplete` taken as given, and a change OpenSpec reads no such change of, as one whose `.openspec.yaml` gives `skip_specs` a value that is no boolean, read not planned (59, C.3).
2. **An archived change that no longer stands reads planned**, since OpenSpec reads only a change that stands and an archived change was planned before it was archived; **a change neither standing nor archived reads not planned** with no `openspec` run.
3. **Its presence, its tasks and its archive stay read from its files** with git alone (86, 87).
4. **The schema is no longer read**: the profile's count of a spec-driven change's files and its reading of a change's schema go.
5. **An error from `openspec`**, where it cannot run or answers no JSON, is the read's could-not-read in its words, as it was for a change of another schema; status gives it with the place (59), and a change-written deliverable reads not yet made until it can.

The construction agent's choices, each with the path not taken:

- **The cost of a read is taken** (design section 3): every read of a standing change makes a checkout of the revision's `openspec/` and runs `openspec`, 0.15 s and 0.42 s on this repository's 4.8 MB `openspec/` on deneb on 2026-11-14. Not taken: option 3, left to a later change should the cost be measured to matter; and a cache of the answer by revision, which core.15 rules out.
- **`openspec` runs from the PATH flume runs with** (design section 4), and flume ships and pins none on a host: the tree's pin is the gate's (core.22). Not taken: a path to `openspec` in the host's configuration, which no host has asked for.
- **The test holds the profile to the pinned `openspec` at each of the three rows**, rather than to the rows' answers alone, so a release that reads the marker otherwise fails the test where flume and OpenSpec part (design section 5).

### Consequences

- Good: flume and OpenSpec give one answer for a change's planning at a revision, whatever its schema or markers, and a release of OpenSpec that changes its rules changes flume's answer with it.
- Good: a spec-driven change that declares `skip_specs` delivers its plan stage once its proposal, design and tasks stand.
- Bad: every read of a standing change costs a checkout and an `openspec` run, at each settle that reads a place's facts, in status, in `status --line` and the Claude Code plugin's line on its ten-second clock, and at a repeating stage's start. A launch reads the base's revision, where the change does not yet stand, and runs nothing.
- Bad: a host that binds `change = "openspec"` needs `openspec` on flume's PATH for every standing change, where before only a change of another schema needed it; where it cannot run, the change reads could-not-read naming it.
- Rule: core.15's details rule out a change's planning complete counted from its files where its tool reads it otherwise.
