# Verify: the-common-profile-reads-an-agent-from-its-terminal

Verified on `change/the-common-profile-reads-an-agent-from-its-terminal`
at `f1bbd5be`, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`
(`019e6f9d`).

## Outcome

**Not archived.** The whole checks fail, so the change goes back to its
code stage. Every task is ticked and every scenario of the change's specs
has its test.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`,
run once: **exit 1**. 37 tasks completed and 2 failed,
`conformance:test` and `flume:test` (7m 43s). Everything else passed,
`site:build` among it.

### `conformance:test`: `tests/no_tool.rs`, `no_tool_is_named_outside_a_profile` (core.1)

Code outside a profile names `herdr` in four places, each added on this
branch:

- `crates/flume-conformance/src/engine/starts.rs:618`, the doc comment of
  the terminal-profiles scenario "Blocked for a profile that reads asking
  from its terminal" (`10ea7419`).
- `crates/flume-conformance/src/engine/terminal_readings.rs:8`, the
  module's doc comment, "herdr's watched send and states of C.5"
  (`10ea7419`).
- `crates/flume-conformance/src/engine/terminal_readings.rs:663`, the doc
  comment of "S73 over the fakes", "a program herdr starts" (`38f03bcd`).
- `crates/flume-seams/src/round_trip.rs:2257`,
  `for name in ["herdr", "Rex", "tmux"]` in a test of the parts' lesser
  readings (`bc65ce17`).

### `flume:test`: `tests/commands.rs`, 12 of 65 failed

`crates/flume/tests/commands.rs` is unchanged on this branch. Each failing
test drives a `stand-in` program through the binary under the `printed`
terminal profile (`"terminal":{"from":"neither","profile":"printed"}`).
The common profile binds that program, so the core now reads its received
and own work from the terminal. The failures read as follows:

- **Refused stop or done, `the agent … is not quiet; ask again forced to
  end it`:** `a_key_confirmed_through_the_binary` (4332),
  `a_place_made_in_the_repositorys_directory_is_read_and_tidied_where_it_stands`
  (2219), `a_readers_place_collected_by_its_inbox` (4255) and
  `flume_writes_neither_configuration_file` (3949).
- **Stage never begun, or the place not moving past `construct`:**
  - `a_session_for_every_stage_and_for_one_at_the_launch` (1478),
    `agent_settings_at_the_launch` (1408),
    `an_effort_for_every_stage_and_for_one_at_the_launch` (1537) and
    `host_settings_and_an_account_for_a_kind` (1960): the stage was
    `construct` where `code` was expected.
  - `a_wait_exits_by_its_outcome` (2601): the wait read `never_begun`.
  - `the_recipient_a_wait_and_an_inbox_default_to` (2812):
    `never_begun` where `delivered` was expected.
  - `needs_to_a_role_and_to_another_streams` (2601): refused, `the stage
    of … construct:1 is not running`.
  - `a_watch_over_a_scratch_repository` (1032): `the place was not
    submitted`.

These failures likely come from this change: the printed terminal keeps
the seam's default `send_watching`, which refuses with `NO_TURN`, and it
reads no state, so a common-profile program under it never takes its
prompt and is never read quiet. This was not confirmed against the base,
since the checks run only once.

## The change's tasks

All 25 tasks, 1.1 to 4.1, are ticked `[x]`. Each task has its commit on
this branch (17 commits since the base).

## The change's specs

Each scenario of each delta spec has a test, and each test read matches
its scenario:

| spec | scenarios | where |
|---|---|---|
| terminal-readings | 16 | 15 in `crates/flume-conformance/src/engine/terminal_readings.rs`, registered in `engine/mod.rs`; "amp under the pinned herdr" as `s73_amp_under_herdr_takes_its_prompt_goes_quiet_and_lands` in `crates/flume/tests/profiles/tools/walkthroughs.rs` |
| seam-contracts | 8 | `crates/flume-seams/src/round_trip.rs` (each part, name and lesser reading, no part built, watched send JSON, a start read waiting, a start written before); `crates/flume-seams/src/terminal.rs` (the seam's default reads no turn); `crates/flume-conformance/src/seams/agent.rs` `answers_agree_with_the_parts` under `FakeAgentHarness::reading_from_its_terminal` |
| seam-fakes | 3 | `crates/flume-fakes/tests/fakes.rs`: `a_new_fakes_parts`, `each_script_degrades_its_own_part`, `a_watched_sends_turn_seen_then_not` |
| agent-profile | 2 | `crates/flume/tests/profiles/agent_claude.rs`: `the_profile_builds_every_part` and the agent scenarios passed (`pass_all`) |
| agent-programs | 1 | `crates/flume/src/profiles/agent/common.rs`: `nothing_is_read_of_amp` |
| place-status | 3 | `crates/flume-conformance/src/engine/place_status.rs`: `a_profile_that_reads_the_five_from_its_terminal` beside the two existing scenarios; the person's form in `crates/flume/src/commands/print.rs` `an_agents_degraded_parts` |
| place-line | 3 | `crates/flume-conformance/src/engine/place_line.rs`, "An agent whose terminal reads it asking" added |
| terminal-profiles | 3 | `crates/flume-conformance/src/engine/starts.rs` (blocked read as asking); `crates/flume/tests/profiles/tools/terminal_herdr.rs` `a_watched_send_sees_the_turn_begin`, `a_watched_send_sees_no_turn` |
| clause-coverage | 3 | `crates/flume-conformance/src/clauses.rs`: `REVISION` `48a94da`, 171 and S73 out of `unbuilt`, 173, S71 and S72 under `PROFILES` |

The site's guide (`places-and-herdr.mdx`, "Programs herdr starts") and
its state reference (`reference/state.md`, the thirteen `degraded` words
and `asking` and `own_work` read from the terminal) say what tasks 3.4
and 3.5 ask.

## What the code stage must do

1. Keep the tool's name out of code outside a profile at the four places
   above, so `no_tool_is_named_outside_a_profile` passes (core.1).
2. Make the 12 `crates/flume/tests/commands.rs` tests pass again: either
   a `stand-in` under the printed terminal runs through as before, or the
   tests and the specs say how such a program is read now (171, C.4).
