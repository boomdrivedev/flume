# Verify

The verify stage of `a-change-that-skips-specs-is-planned`, on
`change/a-change-that-skips-specs-is-planned` at f3a219d, cut from
`bolt/flumes-places-need-no-hand` at 4e7fa04, the base now at 62fa2af,
on deneb on 2026-11-15.

## The whole checks: passed

`sluice exec bolt/flumes-places-need-no-hand`, run once, ran the checks
f3a219d affects against the base at 62fa2af and passed all 10:
`root:format`, `root:adrs`, `root:changes`, `flume:format`,
`flume:lint`, `flume:test`, `conformance:test`, `plugin-claude:test`,
`nix:typecheck` and `nix:test`. sluice recorded the 10 results under
their keys for f3a219d. `flume:test` ran every profile test, the six of
`change_openspec` among them, and `root:changes` passed both open
changes, this one and `a-hosts-full-status-read-takes-seconds`.

The first call, `devenv shell -- sluice exec bolt/flumes-places-need-no-hand`,
ran no check. The devenv shell puts the sluice the tree pins for the
merge-queue profile, 0.71.0 (`devenv.nix`'s `sluiceVersion`), ahead of
the host's, and 0.71.0 refused the host's configuration before running
anything, exit 3:

```text
sluice: reading /Users/chuck/.config/sluice/config.toml: TOML parse error at line 21, column 1
unknown field `credential`, expected one of `app_id`, `key`, `discovery`
```

Since that call ran no check, the checks ran once with the host's own
sluice, 0.93.2, the one that judges the submission. Neither the change
nor its code stage causes or can fix this. Until the tree's sluice pin
moves past the `[forge]` `credential` key, or the instructions name the
host's sluice for `sluice exec`, a `devenv shell -- sluice exec` on
deneb runs nothing.

`root:specs` was not among the affected checks, since the change stands
unarchived and `openspec/specs/` is the base's. The merge queue refused
an earlier archive of this change on `root:specs`, so the archive was
tried first: the base's `openspec/` at 62fa2af with this change's folder
over it, archived by the pinned `openspec` 1.14.3, `openspec archive
a-change-that-skips-specs-is-planned --yes` (one requirement added, one
modified in `change-profile`), then `openspec validate --specs --strict
--no-interactive`, `root:specs`' own command: 49 passed, 0 failed. The
base's `change-profile` spec has not moved since 4e7fa04.

## The specs

`openspec validate a-change-that-skips-specs-is-planned --strict`: valid.
`openspec status`: planning complete, every artifact done. The archive
warns, without blocking, that the proposal's Why passes 1000 characters.

The delta modifies `change-profile`'s requirement "A change read
through OpenSpec's files", whose name matches the main spec's, and adds
"Planning complete read through OpenSpec's status".

| Requirement or scenario | Where it is met |
|---|---|
| Existence, tasks and archive read from the files, writing nothing | `read` in `crates/flume/src/profiles/change/openspec/mod.rs`: `git.files`, `tasks.md` from the folder or the archive's, `archived`; `reading_writes_nothing`, passed |
| Scenario: Seam scenarios over the profile | `the_change_profile_passes_the_change_scenarios`, passed |
| Scenario: Agreement with OpenSpec's own reader | `the_file_reading_agrees_with_openspec_at_five_revisions`, passed |
| A standing change's planning complete through `openspec status`, whatever its schema, counting no files | `read`: `if exists { status(..)?.unwrap_or(false) }`; `spec_driven_planned` and the schema reading are gone |
| A change `openspec status` reads no such change of reads not planned | `status` gives `None` on a failing exit with no `isPlanningComplete`; `read` takes it as `false` |
| An archived change that no longer stands reads planned | `read`: `archived_as.is_some()` where the change does not stand |
| Scenario: A change that skips specs is planned | `a_change_that_skips_specs_is_planned_as_openspec_reads_it`, the `skip_specs: true` row: standing, planned, as `openspec` reads it; passed |
| Scenario: No delta spec and no marker is not planned | the same test, the row with no `skip_specs`: not planned, as `openspec` reads it |
| Scenario: A change OpenSpec cannot read is not planned | the same test, the `skip_specs: yes` row: read (not could-not-read), standing, not planned |

## The tasks

Every task is ticked, and each holds:

- 1.1: `read` takes planning complete from `status` for every standing
  change, and the new test passed in the gate's `flume:test`.
- 1.2: `spec_driven_planned`, `schema`, `schema_of`, `SPEC_DRIVEN`,
  `PROJECT` and `a_schema_is_read_from_its_line` are gone, none left in
  `crates/`; the module's doc comment and the test file's header say
  planning complete is read through `openspec` for every schema.
  `flume:lint` passed.
- 1.3: the seam's scenarios, the agreement at five revisions, reading
  writing nothing and the change of another schema passed under the new
  read. `write_change` needed no change: the pinned `openspec` reads
  its scratch changes as their facts say.
- 2.1: record 0071, proposed, is in `docs/adr/` with the options,
  consequences and decision makers design section 6 names, and core.15's
  details rule out a planning complete counted from a change's files,
  citing C.3 and record 0071. `root:adrs` passed.
- 3.1: `openspec/specs/change-profile/spec.md` is the base's, and the
  delta splits the requirement in two as the task says. The trial
  archive above passed 49 of 49, and `root:changes` passed.

## Noted, not blocking

- No test asserts on its own that an archived change that no longer
  stands reads planned. The seam's archived scenario checks only that
  it reads archived, and the agreement test compares planning complete
  only where `openspec` reads the change as standing. The read is one
  line, `archived_as.is_some()`.
- The main spec's Purpose still says the change is read from its files
  for the spec-driven schema and through `openspec status` for any
  other. An OpenSpec delta carries requirements only, so the archive
  leaves the Purpose as it stands, and it disagrees with the
  requirements the archive writes.
- C.3's amendment is on boomdrivedev/blueprints' branch
  `design/flume-a-change-that-skips-specs-is-planned` at 4de08fb. It must
  land on blueprints' main for the statement to say what this change
  cites.
- Record 0071 stays proposed until the operator's yes.

## Outcome

The whole checks pass and the work meets every requirement, scenario
and task. The change is archived.
