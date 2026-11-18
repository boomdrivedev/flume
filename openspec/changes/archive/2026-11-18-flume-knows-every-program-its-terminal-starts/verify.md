# Verify: flume-knows-every-program-its-terminal-starts

Verified on `change/flume-knows-every-program-its-terminal-starts` at
`dce0f54`, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`
and holding that base at `a7cb4d3`, the base's revision on the host and
on origin.

**Outcome: archived.** The whole checks pass, every task, 1.1 to 8.5, is
ticked, and every requirement and scenario of the change's specs is met.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`,
run once. sluice ran the checks `dce0f54` affects against the base at
`a7cb4d3` and recorded 22 results under their keys for `dce0f54`. The
command exited 0. 23 tasks ran, `site:install` among them as
`site:build`'s dependency, and every check passed:

- `seams`, `core`, `fakes` and `conformance`: format, lint and test.
  `conformance:test` runs `no_tool`, so core.1 holds.
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
  S1 to S131 at `48a94da` is cited by the suite's 1042 scenarios. 171,
  173 and S71 to S74 are named under record 0026's changes still to
  build, the common profile's readings and a pool's program.
- `flume`: format, lint and test. The `profiles` binary passes 230
  tests, `agent_claude::the_resolver_binds_the_program_by_name_or_path`
  among them. The `scratch` and `telemetry` binaries pass 2 and 6.
- `plugin-claude:test` (17 pass), `nix:typecheck` and `nix:test`.
- `root:changes`: the one open change validates under
  `openspec validate --strict` and archives cleanly. `root:format`
  passes too.
- `site:build`: 19 pages built.

moon warned that it could not archive task outputs into
`.moon/cache/outputs/`. Those are cache archives, not checks, and no
task failed for them.

## Tasks

All 28 tasks are ticked. Each test, file or call a task names exists
where the task says:

- **The settings and the seams (1.1 to 1.5):** `PerProgram`;
  `Agent::model` and `Agent::kind`; `Terminal::starts`; the fakes'
  scripts; Claude Code's `model` and `kind`.
- **The core (2.1 to 2.5):** `engine/agent_model.rs` and
  `engine/agent_programs.rs`, each registered in `engine/mod.rs`.
  `agent_effort.rs` has the table's entry and the missing entry. Status
  gives `ModelStatus` and the effort's entry, and the person's form of
  status prints both.
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
- **The base and the gate (8.1 to 8.5):**
  - 8.1: the base is merged at `a7cb4d3`.
  - 8.2: the common profile answers send-back and on-quiet from its
    parts. `nothing_is_read_of_amp` reads every part degraded.
  - 8.3: the tests of a model's and an effort's table name `agent` and
    `other`, and `no_tool` passes.
  - 8.4: `StandInAgent::model` takes every name and passes it by
    `--model`.
  - 8.5: `the_resolver_binds_the_program_by_name_or_path` reads
    `nonesuch` bound to no profile. Every entry's program, `codex`
    among them, is bound to the common profile, as task 4.3 binds it.
    No other test asserts an entry's program unbound.

`omp_starts_once_its_integration_is_installed` is in check-tools. That
layer runs at the bolt's merge onto main, not on this branch's gate.

## Requirements and scenarios

Each scenario has a test or an engine scenario that cites it:

| Capability | Scenarios and where they are checked |
|---|---|
| agent-programs | herdr's kinds (`every_kind_herdr_starts_has_its_entry`); every entry bound (`every_entry_is_bound`); a program with no entry (`a_program_launched_with_no_entry_is_refused`, and `nonesuch` in `built_in.rs` and `agent_claude.rs`); amp given nothing, amp given a model (`tests/commands.rs`); an entry of name and kind, amp's command, nothing read of amp (`common.rs`); over the fakes, a kind the terminal cannot start, a later start refused (`engine/agent_programs.rs`) |
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

The command-line scenarios name `claude` and `codex`. The unit tests in
`commands/launch.rs` name `agent` and `other` instead, because core.1
binds code outside a profile. The tests of a mode's table do the same.
Each scenario's shape is checked as written.

The work meets every requirement as written, and the change is
archived with this report.
