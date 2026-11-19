# Verify: the-common-profile-reads-an-agent-from-its-terminal

Verified on `change/the-common-profile-reads-an-agent-from-its-terminal`
at `5589aafb`. Its merge base with
`bolt/flume-runs-every-agent-its-terminal-can-start` is `2300916e`; the
bolt stands at `019e6f9d`.

## Outcome

**Archived.** The whole checks pass, every task is ticked, and every
scenario of the change's specs has a test that reads as the scenario
says.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`
ran once: **exit 0**. All 39 tasks completed in 1m 59s, and sluice
recorded 38 results under their keys for `5589aaf`. The 39th task,
`site:install`, only installs the site's packages. Every crate's format,
lint and test passed. So did the conformance suite's `coverage`,
`site:build`, both plugins' tasks, the flake's tasks, and the root's
`specs` (54 passed), `changes`, `changes-cases`, `hooks`, `layers`,
`toolchain` and `versions`. Among them:

- `conformance:test`, `tests/no_tool.rs`,
  `no_tool_is_named_outside_a_profile`: ok (core.1).
- `flume:test`, `tests/commands.rs`: 65 passed, 0 failed.

sluice forecast that the branch **would conflict** with the bolt as it
stands now, in `crates/flume-conformance/src/clauses.rs`,
`crates/flume-core/src/acts.rs`, `crates/flume-core/src/lib.rs`,
`crates/flume-core/src/settle.rs` and
`site/src/content/docs/reference/state.md`. The bolt has moved past the
branch's base. It took main at `21557e4` (`53c76b00`) and landed #148
(`019e6f9d`). The checks ran on the branch as it stands. The conflict is
the merge queue's to report at submission, and a conflicting submission
starts its fix as core.27 says.

## The change's tasks

All 22 tasks, 1.1 to 5.2, are ticked `[x]`, and none is open. Each has
its own commit on the branch, 21 commits since the merge base.

- 5.1: no terminal is named outside a profile.
  - The doc comments in `engine/starts.rs` and `engine/terminal_readings.rs`
    now say "a terminal's blocked, watched send and states of C.5".
  - `a_profile_that_builds_no_part` in `crates/flume-seams/src/round_trip.rs`
    spells the terminals' names in pieces.
  - `no_tool_is_named_outside_a_profile` passes.
- 5.2: the stand-in agent of `crates/flume/tests/commands.rs` builds
  received, asking, own work, ended and last words from its own marks.
  So the core reads none of them from the printed terminal, and all 65
  of the file's tests pass.

## The change's specs

`openspec validate the-common-profile-reads-an-agent-from-its-terminal --strict`:
valid. Each scenario of each delta spec has a test, and each test reads
as its scenario says:

| spec | scenarios | where |
|---|---|---|
| terminal-readings | 16 | 15 in `crates/flume-conformance/src/engine/terminal_readings.rs`, registered in `engine/mod.rs`; "amp under the pinned herdr" as `s73_amp_under_herdr_takes_its_prompt_goes_quiet_and_lands` in `crates/flume/tests/profiles/tools/walkthroughs.rs` |
| seam-contracts | 8 | `crates/flume-seams/src/round_trip.rs`: `each_part_written_and_read`, `a_parts_name_and_its_lesser_reading`, `a_profile_that_builds_no_part`, `a_watched_sends_answers_round_trip`, `a_start_read_waiting`, `a_start_written_before_reads_no_waiting`; `crates/flume-seams/src/terminal.rs`: `a_terminal_that_keeps_the_seams_default_reads_no_turn`; `crates/flume-conformance/src/seams/agent.rs`: `answers_agree_with_the_parts` under `FakeAgentHarness::reading_from_its_terminal` |
| seam-fakes | 3 | `crates/flume-fakes/tests/fakes.rs`: `a_new_fakes_parts`, `each_script_degrades_its_own_part`, `a_watched_sends_turn_seen_then_not` |
| agent-profile | 2 | `crates/flume/tests/profiles/agent_claude.rs`: `the_profile_builds_every_part` and the agent scenarios passed |
| agent-programs | 1 | `crates/flume/src/profiles/agent/common.rs`: `nothing_is_read_of_amp` |
| place-status | 3 | `crates/flume-conformance/src/engine/place_status.rs`: "A profile that reads the five from its terminal" beside the two existing scenarios; the person's form in `crates/flume/src/commands/print.rs`, `an_agents_degraded_parts` |
| place-line | 3 | `crates/flume-conformance/src/engine/place_line.rs`, "An agent whose terminal reads it asking" added |
| terminal-profiles | 3 | `crates/flume-conformance/src/engine/starts.rs`, "Blocked for a profile that reads asking from its terminal"; `crates/flume/tests/profiles/tools/terminal_herdr.rs`: `a_watched_send_sees_the_turn_begin`, `a_watched_send_sees_no_turn` |
| clause-coverage | 3 | `crates/flume-conformance/src/clauses.rs`: `REVISION` `48a94da`, 171 and S73 out of `unbuilt`, and 173, S71 and S72 under `PROFILES` |

The site's guide (`places-and-herdr.mdx`, "Programs herdr starts") says
what task 3.4 asks. Its state reference (`reference/state.md`) says what
task 3.5 asks: the thirteen `degraded` words, with `asking` and
`own_work` read from the terminal.

`check-tools` holds the walkthrough and herdr's watched send. It runs at
the bolt's merge onto main, not on this branch.
