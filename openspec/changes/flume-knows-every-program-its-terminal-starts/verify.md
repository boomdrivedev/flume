# Verify: flume-knows-every-program-its-terminal-starts

Verified on `change/flume-knows-every-program-its-terminal-starts` at
`51d2e81`, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`.

**Outcome: not archived.** The whole checks fail on two tasks, so the
change goes back to its code stage. Every task is ticked, and every
scenario of the change's specs has a test or an engine scenario. The
failures are in tests and a test double, not in the behavior the specs
describe.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`,
run once. sluice ran the checks `51d2e81` affects against the base at
`a7cb4d3`: 23 tasks, 21 passed, 2 failed (`conformance:test`,
`flume:test`), exit 1. The tasks that passed include `root:changes`
(the change validates and archives cleanly), `conformance:coverage`,
`site:build` and every crate's format and lint.

### `conformance:test`: `no_tool_is_named_outside_a_profile` (core.1)

`crates/flume-conformance/tests/no_tool.rs` finds `claude` named
outside a profile, in this change's unit tests:

- `crates/flume/src/commands/launch.rs`, lines 503, 505, 511, 518, 539,
  543, 546, 547 and 555 (`a_model_and_an_effort_per_program`,
  `a_name_and_a_table_for_one_stage_are_refused`; commit `b8a4bd25`)
- `crates/flume/src/commands/print.rs`, lines 2723, 2725, 2732, 2734,
  2736 and 2741 (`an_agents_model_and_its_tables_entry`; `95593812`)
- `crates/flume/src/host/config.rs`, lines 1970, 1982, 2014, 2018 and
  2042 (the model's and effort's store-profile tests; `d9b5940f`)
- `crates/flume/src/host/roles.rs`, lines 508 and 509
  (`a_roles_run_gives_a_model_per_program`; `8a3c6a75`)

The same file's tests of a mode's table show the fix: they name neutral
programs (`code.mode.agent=careful`, `code.mode.other=ask`) where the
spec's scenario names `claude`. These tests can do the same with
`agent` and `other`, not `claude` and `codex`.

### `flume:test`: two tests in `crates/flume/tests/commands.rs`

- `agent_settings_at_the_launch` (line 1392): the launch is refused,
  "the stage `construct`'s model `opus`, from the layer `launch`, cannot
  run on `stand-in`: its agent profile names no way to give a model,
  `opus` among them".
- `host_settings_and_an_account_for_a_kind` (line 1945): the launch is
  refused, "the stage `code`'s model `haiku`, from the layer
  `host-kind`, cannot run on `stand-in`: its agent profile names no way
  to give a model, `haiku` among them".

`StandInAgent` (`crates/flume/tests/commands.rs:87`) implements
`effort` but not `model`, so it keeps the seam's new default from task
1.2 and refuses every name. These two tests, which existed before this
change, give the stand-in a model. Task 1.2 says to build every use of
a changed trait; this use was missed. The stand-in's profile needs a
`model` that takes every name, as the fake agent's does by default.

## Tasks

All 23 tasks, 1.1 to 7.1, are ticked. Each test or file a task names
exists where the task says:

- **The settings and the seams (1.1 to 1.5):** `PerProgram`, `Agent::model` and `Agent::kind`, `Terminal::starts`, the fakes' scripts, and Claude Code's `model` and `kind`.
- **The core (2.1 to 2.5):** `settings.rs` resolves both entries, and `kind_started` is called from `launch.rs` and `start.rs`. `engine/agent_model.rs` and `engine/agent_programs.rs` are registered. Status gains `ModelStatus` and the effort's entry, and the person's form of status prints them.
- **The readers of a model and an effort (3.1 to 3.5):** `per_program.rs`, the kinds profile, the host's tables, a role's entry, and `--agent`'s tables.
- **Every program the terminal starts (4.1 to 4.4):** `programs.rs` holds 24 entries in the order of herdr's `KINDS`, and the resolver binds them, with `common.rs` as the common profile.
- **herdr's integration (5.1, 5.2):** `INTEGRATED` and herdr's `starts`.
- **The statement's clauses (6.1):** the clause list is held at `48a94da`, with 170, 172 and S75 no longer listed as unbuilt.
- **The site (7.1):** seven site pages.

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
| kinds-profile | `a_model_for_every_stage_and_a_table_for_one`, `a_model_that_is_neither_fails_the_declaration_naming_the_field`, and the effort's three |
| store-profile | the model's two and the effort's three (`host/config.rs`) |
| terminal-profiles | `integration_status_lines_read`, `integration_not_asked_of_a_kind_herdr_reads_on_its_own`, `omp_starts_once_its_integration_is_installed` (check-tools) |
| seam-contracts | `a_model_and_an_effort_for_each_program`, `a_profile_that_keeps_the_seams_defaults_refuses_a_model`, `a_terminal_that_keeps_the_seams_default_starts_every_kind`, the agent seam's scenarios over the fake taking none |
| seam-fakes | `the_fake_terminal_scripts_a_kind_it_cannot_start`; the fake's models (`engine/agent_model.rs`) |
| clause-coverage | `the_clauses_listed`, `the_clauses_unbuilt_are_named_by_the_changes_that_build_them`; `conformance:coverage` passed in this run |

The work meets every requirement as written. It falls short only where
the gate fails: core.1 in the tests, and the stand-in's model.

## Beside the gate

sluice reported that `51d2e81` would conflict with the base at
`a7cb4d3`. The base has since taken `#131`,
`change/each-agent-profile-says-which-parts-it-builds` (174), and the
conflict touches 10 files:

- `crates/flume-conformance/src/clauses.rs`
- `crates/flume-conformance/src/engine/answers.rs`
- `crates/flume-conformance/src/engine/place_status.rs`
- `crates/flume-conformance/src/seams/agent.rs`
- `crates/flume-core/src/answer.rs`
- `crates/flume-core/src/status.rs`
- `crates/flume-fakes/src/agent.rs`
- `crates/flume/src/commands/print.rs`
- `crates/flume/src/profiles/agent/claude/mod.rs`
- `site/src/content/docs/reference/state.md`

This change's submission will conflict unless the branch takes the base
first. The common profile then builds no part under 174's seam, which
is the seam's default.
