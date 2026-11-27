# Design: flume tests its logic as units

## Context

flume's logic is in three places:

- the core, `flume-core`, which names no tool and reads each tool
  through a seam.
- the binary's commands, `crates/flume/src/commands/`, which parse a
  caller's words, ask the core and print its answer.
- the profiles, `crates/flume/src/profiles/`, which bind each seam to a
  tool.

The fakes of each seam are in `flume-fakes`. The conformance suite,
`flume-conformance`, runs 1,252 scenarios over the fakes in-process. It
holds the clause coverage. Its `World` builds a core over every fake,
with the fake host's clock.

Most tests under `crates/flume/tests/` do not use that world. They start
flume's binary, or the test binary as flume, for each command. Each such
command starts git, and a status starts `flume host`. The tools layer
starts herdr and Rex servers, sluice and a stand-in for Claude Code for
each test. Many of these tests prove again a behaviour of the core that
a conformance scenario already proves.

`command_line.rs` already shows the other way. It parses each command's
words as the binary parses them, and asks one core over the conformance
world, in the test's own process.

The rules that bind this change:

- `a-leaked-child-fails-its-test`: each end-to-end test ends each
  process that it starts.
- `a-tool-has-no-mode-that-only-its-tests-use`: an end-to-end test of
  status runs `flume host` with its watch. A unit test asks the core
  directly and never answers `flume status` without `flume host`.
- `the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock`:
  a unit test of a wait runs on a current-thread runtime with a paused
  clock, or on the fake host's clock.
- `a-gate-test-asserts-no-real-time-bound`: a unit test asserts no
  bound on real time.
- `a-rule-is-proven-by-the-tests-named-for-it`: a test that moves keeps
  its name where a rule's model names it.
- `no-task-reads-markdown`: no test reads `moves.md`.

## Decisions

### 1. Where a test of each kind of logic lives

| Logic | Home of its unit test | What the test drives |
|---|---|---|
| An act or a reading of the core | a scenario of `flume-conformance/src/engine/` | the core over the fakes, on the fake host's clock |
| A command's words, help, refusal of its words, and exit code | `crates/flume/src/commands/` | `commands::run` over a core of the conformance world |
| The words that flume prints for a person | `crates/flume/src/commands/print.rs` | `print::person` over an answer that the test builds |
| A profile's reading of files: a transcript, a hook's input, a kind's declaration, a lock, a source | the profile's own module | the profile over files in a scratch folder |
| A profile's calls to its tool | the profile's own module | the profile over a recording of the tool's answers |
| The host's configuration, its sources and its roles | `crates/flume/src/host/` | the reader over files in a scratch folder |

A unit test starts no process, no git and no network. A scratch folder
of files is not a process, so a unit test can read and write one. A
unit test of a wait uses the fake host's clock or tokio's paused clock.

### 2. A profile takes the call to its tool as a value

The herdr, Rex, sluice, openspec, Ghostty and Terminal profiles each
start their tool's program in one function today: for example,
`Herdr::run`. Each such profile takes that call as a value. The binary
gives it the call that starts the program, under the profile's bound.
A unit test gives it a call that answers from a recording of the pinned
tool's answers, or that does not answer.

This is a seam, not a mode. The binary has one path. No flag, variable
or command exists only for the tests. The recordings live beside the
profile. The end-to-end test of the seam runs the profile against the
pinned tool, and it proves each recording that the unit tests read.

### 3. The end-to-end set

An end-to-end test runs the built binary, or a profile against the
pinned tool. Each test owns each process that it starts and ends it at
its drop. The set starts a bounded number of processes at one time:
the `test` and `tools` tasks run the end-to-end tests with a bounded
count of test threads.

Each command's main path, each seam with a real tool, and each
walkthrough is proven by exactly one test of this set.

#### The commands, in `check-fast`, through the built binary on real git

| Command | Test |
|---|---|
| `launch`, `status`, `host` | `commands::the_same_answer_through_the_command_line_and_the_core` |
| `list` | `commands::the_list_through_the_command_line_and_the_core` |
| `settle` | `commands::a_watch_over_a_scratch_repository` |
| `drop`, `tidy` | `commands::flume_writes_neither_configuration_file` |
| `done` | `commands::a_change_is_declared_done_through_the_binary` |
| `inbox`, `confirm` | `commands::a_key_confirmed_through_the_binary` |
| `wait`, `needs` | `commands::a_wait_exits_by_its_outcome` |
| `wake` | `commands::a_wake_of_a_roles_address` |
| `kinds lock` | `commands::kinds_lock_two_writers_at_once_and_a_removal` |
| `open` | `commands::open_under_the_printed_command` |
| `mcp` | `commands::status_over_mcp` |
| `hold` | `commands::a_hold_after_the_launch_and_a_stall_bound` |
| `hibernate`, `resume` | `commands::hibernate_and_resume_through_the_binary` |
| `answer` | `commands::an_answer_naming_no_decision` |
| `prompt`, `priority`, `stop`, `finish`, `message` | `commands::a_callers_acts_reach_the_core_through_the_binary` |
| `ack` | `hooks_claude::the_started_and_received_acks` |
| `hook` | `hooks_claude::the_stop_asks_flume_host_and_leaves_no_process` |

`retry` and `relay` have their end-to-end tests in the walkthroughs.

#### The seams with a real tool, in `check-fast`

| Seam and tool | Test |
|---|---|
| History, on git | `host::the_history_scenarios_pass_against_git` |
| Place, on git worktrees | `place_git_worktree::the_place_profile_passes_the_place_scenarios` |
| Kinds, on declarations that git holds | `kinds_weir::the_kinds_profile_passes_the_kinds_scenarios` |
| Change, on the pinned openspec | `change_openspec::the_change_profile_passes_the_change_scenarios` |
| Store, on the host's files by two processes | `store_state_dir::two_processes_writing_at_one_version_leave_one_write` |
| The room lock, by two processes | `host::the_room_lock_is_exclusive_between_processes` |
| Telemetry, to a collector on the loopback | `telemetry::a_callers_collector_is_never_flumes` |

#### The seams with a pinned tool, in `check-tools`

| Seam and tool | Test |
|---|---|
| Merge queue, on the pinned sluice | `merge_queue_sluice::the_sluice_profile_passes_the_merge_queue_scenarios` |
| A base place, on the pinned sluice | `merge_queue_sluice::the_sluice_profile_passes_the_base_place_scenarios` |
| Terminal, on the pinned herdr | `terminal_herdr::the_herdr_profile_passes_the_terminal_scenarios` |
| Terminal, on the pinned Rex | `terminal_rex::the_rex_profile_passes_the_terminal_scenarios` |
| Agent, on the pinned Claude Code's program | `claude_words::the_pinned_release_holds_what_the_profile_reads` |
| Readiness, by the pinned sluice | `place_git_worktree::a_fresh_clones_pins_are_installed_by_the_pinned_sluice` |
| The stand-in for Claude Code | `stand_in::the_stand_in_drives_one_stage` |

#### The walkthroughs, in `check-tools`

| Path | Test |
|---|---|
| A fix from the `launch` command to its landing | `walkthroughs::a_fix_lands_through_flumes_own_submission` |
| A change through `construct`, `code` and `verify` | `walkthroughs::a_change_runs_construct_code_and_verify` |
| A failed submission back to code (`retry`) | `walkthroughs::a_failed_submission_goes_back_to_code_and_lands_on_the_second` |
| A watch that lands a chore with no agent | `merge_queue_sluice::s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone` |
| A standing agent takes a fix's end | `walkthroughs::a_standing_agent_under_herdr_takes_a_fixs_end_through_the_binary` |
| A fix on a stream under Rex | `walkthroughs::a_fix_on_a_stream_under_rex_lands_through_the_binary` |
| A person's relay to an agent (`relay`) | `walkthroughs::a_person_reaches_a_fixs_agent_through_a_relay` |
| A role's run woken for a question | `walkthroughs::s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run` |
| An agent kept open for its answer | `open_sessions::s4_an_answer_within_the_hour_by_a_retry_and_a_message_reaches_the_same_session` |
| A stream's view | `open::s90_a_streams_view_through_the_binary` |

The live walkthroughs, in `check-live`, stay as they are. No gate runs
them.

### 4. Exactly one proof for each behaviour

Before a test goes, the step finds the test that proves its behaviour
after the move. That test is one of these:

- a conformance scenario that already proves it.
- a unit test that the step writes, or a row of a table test that the
  step adds.
- an end-to-end test of the set.

Where a test proves several behaviours, each behaviour gets its row.
`moves.md` holds one row for each test that moves or goes. Each step
writes the test that proves it before the step lands.

Where many tests differ only in their data, one table test replaces
them, with `rstest` cases. Where a behaviour holds for each value of a
type, one property test replaces the copies, with `proptest`.

### 5. The steps

Each step moves the tests of one file, largest first, and lands alone.
A step that moves a profile's tests first gives the profile the call to
its tool as a value (decision 2). The order of the files that start a
process, by size:

1. `tools/walkthroughs.rs` (42 tests), with `tools/walk.rs`
2. `commands.rs` (72 tests)
3. `tools/terminal_herdr.rs` (62 tests)
4. `kinds_weir.rs` (78 tests)
5. `tools/terminal_rex.rs` (38 tests)
6. `agent_claude.rs` (45 tests)
7. `hooks_claude.rs` (44 tests)
8. `tools/merge_queue_sluice.rs` (20 tests)
9. `host.rs` (33 tests)
10. `place_git_worktree.rs` (24 tests)
11. the rest: `store_state_dir.rs`, `command_line.rs`, `telemetry.rs`,
    `built_in.rs`, `terminal_printed.rs`, `tools/open.rs`,
    `tools/stand_in.rs`, `tools/open_sessions.rs`,
    `change_openspec.rs`, `window_ghostty.rs`, `window_terminal.rs`,
    `tools/claude_words.rs`, `merge_queue_sluice_documents.rs`,
    `tools/place_git_worktree.rs`, `window_printed.rs`, `scratch.rs`

After each landing, the step's report gives the tests before and after,
the count of end-to-end tests, and the measured wall time and peak
processes.

## Risks

- **A recording falls behind its tool.** The end-to-end test of each
  seam runs the profile against the pinned tool. A pin that moves
  without new recordings fails that test.
- **A behaviour loses its only proof.** Each step writes the proving
  test before it removes a test. The coverage check still holds each
  clause to a conformance scenario.
- **The binary's assembly goes untested.** The end-to-end tests of the
  commands run the binary as the host's service manager and a caller
  run it.
