# Verify: flume-knows-every-program-its-terminal-starts

Verified on `change/flume-knows-every-program-its-terminal-starts` at
`ec964ca`, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`
and holding that base at `a7cb4d3`.

**Outcome: not archived.** The whole checks fail on one test, so the
change goes back to its code stage. Every task, 1.1 to 8.4, is ticked.
Every scenario of the change's specs has a test or an engine scenario.
The failure is one older test that still says `codex` is bound to no
profile. Task 4.3 binds it to the common profile.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`,
run once. sluice ran the checks `ec964ca` affects against the base at
`a7cb4d3`, with no conflict reported. 23 tasks ran: 22 passed and 1
failed (`flume:test`). The command exited 1.

The tasks that passed include:

- `seams:test`, `core:test`, `fakes:test` and `conformance:test`.
  `no_tool` passes, so core.1 holds.
- `conformance:coverage`, which names 171, 173 and S71 to S74 as still
  to build under record 0026's changes.
- `root:changes`: the change validates and archives cleanly.
- `site:build`, and every crate's format and lint.

### `flume:test`: `agent_claude::the_resolver_binds_the_program_by_name_or_path`

```
thread 'agent_claude::the_resolver_binds_the_program_by_name_or_path' panicked at crates/flume/tests/profiles/agent_claude.rs:1484:5:
assertion failed: resolver.agent("codex").is_none()
test result: FAILED. 229 passed; 1 failed
error: test failed, to rerun pass `-p flume --test profiles`
```

`crates/flume/tests/profiles/agent_claude.rs:1484` was written before
this change (`73cbcbd8`). It asserts that the built-in resolver binds
`codex` to no profile. Task 4.3, and the agent-programs requirement
"The resolver binds every entry and nothing else", now bind `codex` to
the common profile over its entry, so the assertion no longer holds.

The fix is to assert a program no entry names, as `built_in.rs:157`
does with `nonesuch`, or that `codex` gets the common profile. Task
4.3's own check, `--test profiles built_in`, filters to `built_in` and
so never ran this test. The earlier verify did not see it either:
`commands.rs` failed first there, and cargo stopped before the
`profiles` binary.

cargo also stops at a failing binary. After `profiles`, flume's
`scratch` and `telemetry` binaries and its doc-tests did not run under
the gate. Run directly afterwards, they pass: 2, 6 and 0 tests. That
run is not the gate's.

## Tasks

All 27 tasks are ticked. Each test, file or call a task names exists
where the task says:

- **The settings and the seams (1.1 to 1.5):** `PerProgram`;
  `Agent::model` and `Agent::kind`; `Terminal::starts`; the fakes'
  scripts; Claude Code's `model` and `kind`.
- **The core (2.1 to 2.5):** `engine/agent_model.rs` (six scenarios)
  and `engine/agent_programs.rs` (three), each registered in
  `engine/mod.rs`. `agent_effort.rs` has the table's entry and the
  missing entry. Status gives `ModelStatus` and the effort's entry, and
  the person's form of status prints both.
- **The readers (3.1 to 3.5):** `per_program.rs`, the kinds profile,
  the host's tables, a role's entry and `--agent`'s tables.
- **Every program (4.1 to 4.4):** `programs.rs`, `common.rs`, the
  resolver, and the three launches in `tests/commands.rs`.
- **herdr (5.1, 5.2):** `INTEGRATED`, herdr's `starts`, and
  `omp_starts_once_its_integration_is_installed` in check-tools.
- **The clauses (6.1):** the list is held at `48a94da`. 170, 172, S75
  and 174 are built. 171, 173 and S71 to S74 are named under
  `PROFILES`.
- **The site (7.1):** the seven pages. Among them, `commands.md` gives
  `model.<program>` and `effort.<program>`, `state.md` gives `model`
  beside `degraded`, and the guides name `herdr integration install`.
- **The verify's findings and the base (8.1 to 8.4):**
  - 8.1: the base is merged at `a7cb4d3`.
  - 8.2: the common profile answers send-back and on-quiet from its
    parts. `nothing_is_read_of_amp` reads `Part::ALL` degraded.
  - 8.3: the tests of a model's and an effort's table name `agent` and
    `other`, and `no_tool` passes.
  - 8.4: `StandInAgent::model` takes every name and passes it by
    `--model`. `agent_settings_at_the_launch` and
    `host_settings_and_an_account_for_a_kind` pass.

## Requirements and scenarios

Each scenario has a test or an engine scenario that cites it:

| Capability | Scenarios and where they are checked |
|---|---|
| agent-programs | herdr's kinds (`every_kind_herdr_starts_has_its_entry`); every entry bound (`every_entry_is_bound`); no entry, amp given nothing, amp given a model (`tests/commands.rs`); an entry of name and kind, amp's command, nothing read of amp (`common.rs`); over the fakes, a kind the terminal cannot start, a later start refused (`engine/agent_programs.rs`) |
| agent-model | all six (`engine/agent_model.rs`) |
| agent-effort | a word kept, a table refused, a table's entry passed, no entry for the program (`engine/agent_effort.rs`, kinds-profile tests) |
| agent-profile | `every_model_name_is_taken_and_the_entry_passed_by_model` |
| place-status | a model given whole, a table's entry, no model, an effort's table entry (`engine/place_status.rs`; `print.rs`) |
| command-line | `a_model_and_an_effort_per_program`, `a_name_and_a_table_for_one_stage_are_refused` |
| kinds-profile | `a_model_for_every_stage_and_a_table_for_one`, `a_model_that_is_neither`, and the effort's three |
| store-profile | the model's two and the effort's three (`host/config.rs`) |
| terminal-profiles | `integration_status_lines_read`, `integration_not_asked_of_a_kind_herdr_reads_on_its_own`, `omp_starts_once_its_integration_is_installed` (check-tools) |
| seam-contracts | `a_model_and_an_effort_for_each_program`, `a_profile_that_keeps_the_seams_defaults_refuses_a_model`, `a_terminal_that_keeps_the_seams_default_starts_every_kind`, and the agent seam's scenarios over the fake that takes no model |
| seam-fakes | `the_fake_terminal_scripts_a_kind_it_cannot_start`, and the fake's models (`engine/agent_model.rs`) |
| clause-coverage | `the_clauses_listed`, `the_clauses_unbuilt_are_named_by_the_changes_that_build_them`; `conformance:coverage` passed in this run |

The command-line scenarios name `claude` and `codex`. The unit tests
in `commands/launch.rs` name `agent` and `other` instead, because
core.1 binds code outside a profile. The tests of a mode's table do
the same. Each scenario's shape is checked as written.

The work meets every requirement as written. It falls short only where
the gate fails, on the one stale assertion above.
