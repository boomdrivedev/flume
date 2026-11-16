# A change that skips specs is planned

## Why

A spec-driven change may declare `skip_specs: true` in its
`.openspec.yaml`, which the pinned OpenSpec 1.14.3 honours: `openspec
validate --strict` accepts it with no delta spec, and `openspec status
--change <name> --json` reads its `specs` artifact skipped and its
planning complete once its proposal, design and tasks stand. flume reads
such a change as never planned. Its OpenSpec profile decides a
spec-driven change's planning complete from a count of its files,
`spec_driven_planned`, which asks for a `specs/**/*.md`, so a construct
stage that writes one never delivers (signal
2026-11-10-agentplot-design-cd548984/01, on flume 0.20.0 and main;
still true on main at ae188b74, boomdrive-design, 2026-11-14).

That answer disagrees with OpenSpec's own at the same revision, which
clause 59 rules out, and it is flume guessing what another program
knows, which principles 14 rules out: a tool that needs what another
program knows reads that program's own source. OpenSpec's rules for the
marker are its own (the key's presence, metadata that names a known
schema, the artifacts whose `generates` path lies under `specs/`), and a
copy of them in flume would drift with each release.

## What Changes

- **Planning complete is OpenSpec's reading of the change (59, C.3).**
  The OpenSpec profile reads a standing change's planning complete
  through `openspec status --change <name> --json` whatever its schema,
  in the checkout of the revision's `openspec/` it already makes for a
  change of another schema, and counts none of the change's files for
  it. A spec-driven change that declares `skip_specs` reads planned once
  its proposal, design and tasks stand, as OpenSpec reads it.
- **A change OpenSpec cannot read is not planned.** Where `openspec
  status` reads no such change, as for a `.openspec.yaml` whose
  `skip_specs` is no boolean, the change reads not planned, as a change
  of another schema does today.
- **Its presence, its tasks and its archive stay read from its files**
  with git alone, and an archived change no longer standing reads
  planned, as it was before it was archived.
- **The schema is no longer read.** The profile's `spec_driven_planned`
  and its reading of a change's schema go.
- **Every read of a standing change runs `openspec`.** A host that binds
  the OpenSpec change profile needs `openspec` where flume runs for
  every change, where before only a change of another schema did; where
  it cannot run, the read is could-not-read naming it, as for another
  schema today.
- **Record and rule.** A record, proposed, and core.15's details.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `change-profile`: a change's planning complete is read through
  `openspec status` for every schema, so a spec-driven change that
  declares `skip_specs` is planned with no delta spec, and a change
  OpenSpec cannot read is not planned. The requirement that reads a
  change from its files keeps its existence, its tasks and its archive,
  and a requirement of its own reads its planning complete.

## Impact

- `crates/flume/src/profiles/change/openspec/mod.rs`: `read` takes
  planning complete from `status` for every standing change;
  `spec_driven_planned`, `schema`, `schema_of`, `SPEC_DRIVEN`, `PROJECT`
  and the unit test `a_schema_is_read_from_its_line` go.
- `crates/flume/tests/profiles/change_openspec.rs`: a change that skips
  specs read as the pinned `openspec` reads it, with and without its
  marker and with a marker OpenSpec cannot honour.
- `docs/adr/`: record 0075, proposed; `docs/architecture/core.md`:
  core.15's details.
- The statement: C.3's read-change row amended on boomdrivedev/blueprints
  branch `design/flume-a-change-that-skips-specs-is-planned` at 4de08fb.
- No seam, core, fake or conformance change; no field of the JSON form
  changes (61); no new crate, plugin, tool or file type.
