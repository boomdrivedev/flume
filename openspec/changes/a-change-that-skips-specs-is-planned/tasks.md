# Tasks

The statement is C.3 at boomdrivedev/blueprints 4de08fb, on the branch
`design/flume-a-change-that-skips-specs-is-planned`; design sections are
this change's `design.md`.

## 1. The profile

- [x] 1.1 Have the OpenSpec profile's `read` take a standing change's planning complete from `status` whatever its schema, `None` read as not planned, an archived change that no longer stands read planned and a change neither standing nor archived read not planned with no `openspec` process (59, C.3; design 2); and add `a_change_that_skips_specs_is_planned_as_openspec_reads_it` to `crates/flume/tests/profiles/change_openspec.rs`, a change holding `proposal.md`, `design.md` and `tasks.md` and no `specs/` committed with `skip_specs: true`, with no `skip_specs` and with `skip_specs: yes`, each read standing with its planning complete as `openspec_reads` gives it at that revision, true, false and false (design 5). True when `devenv shell -- cargo test -p flume --test profiles change_openspec::a_change_that_skips_specs_is_planned_as_openspec_reads_it` passes.
- [x] 1.2 Remove what the profile no longer reads, `spec_driven_planned`, `schema`, `schema_of`, `SPEC_DRIVEN`, `PROJECT` and the unit test `a_schema_is_read_from_its_line`, and have the module's doc comment and `change_openspec.rs`'s header say planning complete is read through `openspec` for every schema and the other facts from the files (design 2). True when `sluice exec <base> -- moon run flume:lint` passes.
- [x] 1.3 Keep the profile's other tests passing with planning complete read through `openspec`: the seam's scenarios, the agreement at five revisions, reading writing nothing and a change of another schema, `write_change` writing what the pinned `openspec` reads as planned where a scenario's facts say so (86, 87, 106; design 5). True when `devenv shell -- cargo test -p flume --test profiles change_openspec::` passes.

## 2. The record and the rule

- [x] 2.1 Write record 0070, proposed, at the next free number in `docs/adr/` (0070 as this change is written): flume takes whether a change is planned from OpenSpec's own reading of it, whatever its schema, and counts none of its files for it, with the options, consequences and decision makers design section 6 names, citing the signal 2026-11-10-agentplot-design-cd548984/01, principles 14 and C.3 at blueprints 4de08fb; and add to core.15's details in `docs/architecture/core.md`, among what it rules out, a change's planning complete counted from its files where its tool reads it otherwise, as flume read a spec-driven change that declares `skip_specs` never planned on 2026-11-10, with record 0070 among its sources. True when `sluice exec <base> -- moon run root:adrs` passes.
