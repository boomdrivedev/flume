# Verify: the-common-profile-reads-an-agent-from-its-terminal

Verified on `change/the-common-profile-reads-an-agent-from-its-terminal`
at `335063a9`. The branch holds
`bolt/flume-runs-every-agent-its-terminal-can-start` at `019e6f9d`, the
bolt's tip and the branch's merge base with it, so the branch is the
bolt with this change on it.

## Outcome

**Archived.** The whole checks pass, all 23 tasks are ticked, and every
scenario of the change's specs has a test that reads as the scenario
says.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`
ran once and exited **0**. sluice ran the checks that `335063a` affects
against the bolt at `019e6f9`, waiting first for room under the host's
bound. All 23 tasks finished in 3m 4s. 22 passed and were recorded under
their keys for `335063a`. The 23rd, `site:install`, only installs the
site's packages.

The tasks run were the format, lint and test of `seams`, `core`,
`fakes`, `conformance` and `flume`; `conformance:coverage`;
`site:build`; `plugin-claude:test`; `nix:typecheck` and `nix:test`; and
`root:format` and `root:changes`. Among them:

- `conformance:test`: `the_engine_passes_its_scenarios` passed, and so
  did `tests/no_tool.rs`'s `no_tool_is_named_outside_a_profile`
  (core.1).
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
  S1 to S131 at `48a94da` is cited by the suite's 1171 scenarios. 173,
  S71 and S72 are still left to record 0026's change of a pool's
  program, and 171 and S73 are no longer on the unbuilt list.
- `root:changes`: one open change, valid under
  `openspec validate --strict`, and it archives to specs that pass.
- `flume:test`: `tests/commands.rs` 73 passed, 0 failed; the profiles'
  tests 246 passed, 0 failed.
- `seams:test` 148 passed, `fakes:test` 73 passed, `core:test` 75
  passed, each with 0 failed.

`check-tools` runs at the bolt's merge onto main, not on this branch.
It holds herdr's watched send and S73's walkthrough under the pinned
herdr.

## The change's tasks

All 23 tasks, 1.1 to 6.1, are ticked `[x]`, and none is open.

Task 6.1's merge of the bolt at `019e6f9` holds as it says. Each
contested file keeps both sides:

- `clauses.rs` lists 171 and S73 as built beside 212, 213, S124, S125
  and S127, and keeps 173, S71 and S72 under `PROFILES`.
- In `acts.rs`, `clear_waiting` stands beside `stop_hibernated` and
  `hand_by_take`.
- `lib.rs` has `OWN_WORK_AFTER` beside `MESSAGE_BOUND`.
- In `settle.rs`'s `step`, a claimed hibernation is carried on before
  `read_waiting` reads the agent (212).
- The site's state page has `hibernating` beside `asking` and
  `own_work` read from the terminal.

Asking is read through `read_asking` in `hand`, in `hand_by_take`, and
in the deliver part's take in `batch.rs`. The seams' tests give
identities `resume: 0`. The scenarios ask `Engine::message`, and no
`Engine::tell` is left.

`openspec/specs/` is byte for byte the bolt's at `019e6f9`, so the
archive applies this change's deltas onto the bolt's specs.

## The change's specs

`openspec validate the-common-profile-reads-an-agent-from-its-terminal --strict`:
valid. Each scenario of each delta spec has a test, and each test reads
as its scenario says:

| spec | scenarios | where |
|---|---|---|
| terminal-readings | 16 | 15 in `crates/flume-conformance/src/engine/terminal_readings.rs`, registered in `engine/mod.rs`; "amp under the pinned herdr" as `s73_amp_under_herdr_takes_its_prompt_goes_quiet_and_lands` in `crates/flume/tests/profiles/tools/walkthroughs.rs` |
| seam-contracts | 8 | `crates/flume-seams/src/round_trip.rs`: `each_part_written_and_read`, `a_parts_name_and_its_lesser_reading`, `a_profile_that_builds_no_part`, `a_watched_sends_answers_round_trip`, `a_start_read_waiting`, `a_start_written_before_reads_no_waiting`; `crates/flume-seams/src/terminal.rs`: `a_terminal_that_keeps_the_seams_default_reads_no_turn`; `crates/flume-conformance/src/seams/agent.rs`: `answers_agree_with_the_parts` under `FakeAgentHarness::reading_from_its_terminal` |
| seam-fakes | 3 | `crates/flume-fakes/tests/fakes.rs`: `a_new_fakes_parts`, `each_script_degrades_its_own_part`, `a_watched_sends_turn_seen_then_not` |
| agent-profile | 2 | `crates/flume/tests/profiles/agent_claude.rs`: `the_profile_builds_every_part`, and the seam's agent scenarios under Claude Code's profile |
| agent-programs | 1 | `crates/flume/src/profiles/agent/common.rs`: `nothing_is_read_of_amp` |
| place-status | 3 | `crates/flume-conformance/src/engine/place_status.rs`: "A profile that reads the five from its terminal" beside the two existing scenarios; its form for a person in `crates/flume/src/commands/print.rs`, `an_agents_degraded_parts` |
| place-line | 3 | `crates/flume-conformance/src/engine/place_line.rs`, with "An agent whose terminal reads it asking" |
| terminal-profiles | 3 | `crates/flume-conformance/src/engine/starts.rs`: "Blocked for a profile that reads asking from its terminal"; `crates/flume/tests/profiles/tools/terminal_herdr.rs`: `a_watched_send_sees_the_turn_begin`, `a_watched_send_sees_no_turn` |
| clause-coverage | 3 | `crates/flume-conformance/src/clauses.rs`: `REVISION` `48a94da`, 171 and S73 out of `unbuilt`, and 173, S71 and S72 under `PROFILES` |

The site's guide (`places-and-herdr.mdx`, "Programs herdr starts") says
what task 3.4 asks. Its state reference (`reference/state.md`) says what
task 3.5 asks: the thirteen `degraded` words in 171's order, with
`asking` and `own_work` read from the terminal.
