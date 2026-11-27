# Tasks

Each step moves the tests of one file and lands alone. A step writes
each proving test before it removes a test, and fills the "Proven by"
column of `moves.md` for its file. Each check runs as `sluice exec main
-- moon run <project>:<task>`, and only when the host is free.

## 1. Measure

- [ ] 1.1 On deneb, measure the wall time of `flume:test` and `flume:tools`, and the peak count of flume and git processes during each. Write the numbers in `proposal.md`. Check: the table "Measured on deneb" holds each number before the first move.

## 2. The walkthroughs

- [ ] 2.1 In `crates/flume/tests/profiles/tools/walkthroughs.rs`, keep the seven walkthroughs of the end-to-end set. For each other walkthrough, find or write the conformance scenario that proves its behaviour, then remove it. Remove the helpers of `walk.rs` that no test uses. Check: `moon run flume:tools` and `moon run conformance:test`.

## 3. The commands

- [ ] 3.1 In `crates/flume/src/commands/`, add the unit tests of each command's words, help, refusals and exit codes, over a core of the conformance world. Add each printed form to `print.rs`'s tests. Check: `moon run flume:test`.
- [ ] 3.2 In `crates/flume/tests/commands.rs`, keep the end-to-end tests of the commands, and write `a_change_is_declared_done_through_the_binary`, `hibernate_and_resume_through_the_binary` and `a_callers_acts_reach_the_core_through_the_binary`. Remove each other test once its proof exists. Check: `moon run flume:test`.

## 4. The herdr profile

- [ ] 4.1 In `crates/flume/src/profiles/terminal/herdr/`, the profile takes the call to herdr as a value, and the binary gives it the call that starts herdr. Check: `moon run flume:test`.
- [ ] 4.2 Record herdr's answers for each case of `terminal_herdr.rs` from the pinned herdr. Move each case to a unit test over its recording. Keep `the_herdr_profile_passes_the_terminal_scenarios`, and make it prove each recording. Check: `moon run flume:test` and `moon run flume:tools`.

## 5. The kinds profile

- [ ] 5.1 Move the tests of `kinds_weir.rs` that read declarations, prompts, locks and sources to unit tests in `crates/flume/src/profiles/kinds/weir/`, over a scratch folder. Replace the tests of each version's prompts with `rstest` tables. Keep `the_kinds_profile_passes_the_kinds_scenarios`. Check: `moon run flume:test`.

## 6. The Rex profile

- [ ] 6.1 The Rex profile takes the call to Rex as a value. Move each case of `terminal_rex.rs` to a unit test over a recording of the pinned Rex. Keep `the_rex_profile_passes_the_terminal_scenarios`, and make it prove each recording. Check: `moon run flume:test` and `moon run flume:tools`.

## 7. The Claude Code profile and its hooks

- [ ] 7.1 Move the tests of `agent_claude.rs` to unit tests in `crates/flume/src/profiles/agent/claude/`, over fixture files. Check: `moon run flume:test`.
- [ ] 7.2 Move the tests of `hooks_claude.rs` to unit tests of `hook.rs` over each hook's input. Keep the two end-to-end tests of `ack` and `hook`. Check: `moon run flume:test`.
- [ ] 7.3 Merge the tests of `tools/claude_words.rs` into `the_pinned_release_holds_what_the_profile_reads`. Check: `moon run flume:tools`.

## 8. The sluice profile

- [ ] 8.1 The sluice profile takes the call to sluice as a value. Move each case of `merge_queue_sluice.rs` and `merge_queue_sluice_documents.rs` to a unit test over a recording of the pinned sluice. Keep the three end-to-end tests. Check: `moon run flume:test` and `moon run flume:tools`.

## 9. The host

- [ ] 9.1 Move the tests of `host.rs` that read the host's configuration, sources and roles to unit tests in `crates/flume/src/host/`. Keep the two end-to-end tests. Check: `moon run flume:test`.

## 10. The place profile

- [ ] 10.1 Move the cases of `place_git_worktree.rs` into the place scenarios that `the_place_profile_passes_the_place_scenarios` runs against git, or remove each that a scenario already proves. Check: `moon run flume:test`.

## 11. The rest

- [ ] 11.1 Move or remove the tests of `store_state_dir.rs`, `command_line.rs`, `telemetry.rs`, `built_in.rs`, `terminal_printed.rs`, `tools/open.rs`, `tools/stand_in.rs`, `tools/open_sessions.rs`, `change_openspec.rs`, `window_ghostty.rs`, `window_terminal.rs`, `window_printed.rs`, `tools/place_git_worktree.rs` and `scratch.rs`, as `moves.md` says. Check: `moon run flume:test` and `moon run flume:tools`.
- [ ] 11.2 `flume:tools` runs at most four tests at one time. Check: `moon run flume:tools`.

## 12. Measure again

- [ ] 12.1 On deneb, measure again as in 1.1, and write the numbers in `proposal.md`. Check: each number after is lower than its number before.
