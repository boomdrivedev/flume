# Verify: a-program-without-its-own-profile-starts-through-flumes-launcher

Verified on `change/a-program-without-its-own-profile-starts-through-flumes-launcher`
at `ba2d7169`, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`.

**Outcome: not archived.** The whole checks fail on core.1; every task is
ticked and every scenario has its test, but the change goes back to its
code stage to fix the failure below.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`,
run once against the base at `85c601e`: the base is a bolt, so it ran the
`check-fast` layer, and `check-tools`, the herdr profile's tests among it,
is left to the bolt's merge onto main.

**Failed: 22 tasks passed, 1 failed, exit 1.**

- `conformance:test` failed: `crates/flume-conformance/tests/no_tool.rs`
  `no_tool_is_named_outside_a_profile` (core.1):

  ```
  a tool is named outside a profile (core.1):
  crates/flume-seams/src/round_trip.rs:2375: herdr
  ```

  The seams crate's `the_processs_answers_round_trip`, added by task 1.2
  (`ee719241`), gives `ProgramProcess::Unknown` the words `"herdr did not
  answer within 5 seconds"`. Code outside a profile names no terminal
  (core.1); the words need to name none, as the seam's own
  `NO_PROCESS` does.
- Every other task passed: the format, lint and tests of `seams`, `core`,
  `fakes`, `conformance` (its `coverage` among them) and `flume`;
  `site:build`; `nix:typecheck`; `root:format` and `root:changes`.

The scenarios of `terminal_herdr.rs` (tasks 4.1 and 4.2) are under the
`tools` feature, which `check-fast` does not run; this stage ran them no
other way.

## Tasks

Every task of `tasks.md`, 1.1 to 5.1, is ticked done, each by its own
commit on the branch, `6897fd8a` to `1a6c5c02`, with record 0026's
amendment for the code stage's choices in `ba2d7169`.

## Requirements and scenarios

Each scenario of the change's specs is met by the test named, which the
whole checks ran.

### agent-launcher

| Requirement | Scenario | Test |
|---|---|---|
| The launcher becomes the program its start names | amp becomes the start's | `crates/flume/tests/profiles/launcher_common.rs` `amp_becomes_the_starts` |
| | A program found past the launcher itself | `crates/flume/src/profiles/agent/launcher.rs` `a_program_found_past_the_launcher_itself` |
| The launcher's word is written once | A second launcher under one start | `launcher_common.rs` `a_second_launcher_under_one_start`; the word's half, `launcher.rs` `the_word_is_written_once` |
| A start the launcher finds otherwise runs nothing | A variable with another value | `launcher_common.rs` `a_variable_with_another_value`, status reading `AMP_HOME` by name, no value in the word or status |
| | Another program | `launcher_common.rs` `another_program` |
| | No start under its identity and secret | `launcher_common.rs` `no_start_under_its_identity_and_secret` |
| A start through the launcher counts once its process is read | The word and its process read | `crates/flume-conformance/src/engine/agent_launcher.rs` `the_word_and_its_process_read` |
| | The word alone | `agent_launcher.rs` `the_word_alone` |
| | A terminal that reads no process | `agent_launcher.rs` `a_terminal_that_reads_no_process`, four settles before the stage's limit |
| A program in another process is not the start's agent | amp started again by hand | `agent_launcher.rs` `s74_amp_started_again_by_hand` |
| | Before its prompt | `agent_launcher.rs` `before_its_prompt` |

The core reads every start's started and began-otherwise through
`Engine::started` and `Engine::began_otherwise` in
`crates/flume-core/src/reading.rs`: `read_agent`, the tell's wait in
`acts.rs`, both readings in `deliver.rs`, the stop check in
`agent_acts.rs`, and the begun stage's end in `end.rs`, as tasks 2.1 and
2.2 name.

The engine's scenarios cite 35, and 161 with S74, as data; C.4, which
the spec's first scenario names beside 35, is named in that scenario's
doc comment, since the suite cites as data only Parts A and B, the
invariants and section 11's scenarios (`dc31f458`).

### agent-programs

| Requirement | Scenario | Test |
|---|---|---|
| The common profile's command | amp's command | `crates/flume/src/profiles/agent/common.rs` `amps_command` |
| The common profile answers the seam's defaults | Nothing read of amp | `common.rs` `nothing_is_read_of_amp` |
| The common profile reads its start from the launcher's word | amp's start read from the launcher's word | `common.rs` `amps_start_is_read_from_the_launchers_word` |

### seam-contracts

| Requirement | Scenario | Test |
|---|---|---|
| The agent seam says whether a start runs through the launcher | A profile that keeps the seam's defaults names no launcher | `crates/flume-seams/src/round_trip.rs` `a_profile_that_keeps_the_seams_defaults_names_no_launcher` |
| An agent's command carries the launcher's words | A command with the launcher's words and one without | `round_trip.rs` `a_command_with_the_launchers_words_and_one_without` |
| The terminal seam reads the process an agent's program runs in | A terminal that keeps the seam's default reads no process | `crates/flume-seams/src/terminal.rs` `a_terminal_that_keeps_the_seams_default_reads_no_process` |
| | The process's answers in JSON | `round_trip.rs` `the_processs_answers_round_trip` |
| A start begun in another process says so | Begun in another process | `round_trip.rs` `a_start_begun_in_another_process` |
| | A word written before | `round_trip.rs` `a_word_written_before_has_no_process` |

### seam-fakes

| Requirement | Scenario | Test |
|---|---|---|
| The fake agent scripts a start through the launcher | A launcher's start scripted | `crates/flume-fakes/tests/fakes.rs` `the_fake_agent_scripts_a_start_through_the_launcher` |
| The fake terminal scripts the process it reads | A process read, then another | `fakes.rs` `the_fake_terminal_scripts_the_process_it_reads` |

### terminal-profiles

| Requirement | Scenario | Test |
|---|---|---|
| herdr runs the program the settings name | Startup files that put another program first; A program that runs another in its place | the existing herdr tests in `crates/flume/tests/profiles/tools/terminal_herdr.rs`, unchanged in their scenarios; the link to the launcher is the new requirement's |
| The printed command | The launcher's line | `crates/flume/tests/profiles/terminal_printed.rs` `the_launchers_line`; the tell's and the seam's scenarios as before |
| herdr runs the launcher in the program's place | A program started through the launcher | `terminal_herdr.rs` `a_program_started_through_the_launcher`, under the pinned herdr |
| herdr reads the process its agent's program runs in | The launcher's process, then one started by hand | `terminal_herdr.rs` `launched_process_then_one_started_by_hand` |
| | A pane that is gone | `terminal_herdr.rs` `launched_process_of_a_gone_pane_reads_nothing` |

### command-line

| Requirement | Scenario | Test |
|---|---|---|
| launched takes the start from its environment | launched from outside an agent | `crates/flume/tests/command_line.rs` `launched_from_outside_an_agent_runs_nothing` |
| | launched's help | `command_line.rs` `launcheds_help` |

### clause-coverage

| Requirement | Scenario | Test |
|---|---|---|
| The suite knows the statement's clauses | The clauses listed | `crates/flume-conformance/src/clauses.rs` `the_clauses_unbuilt_are_named_by_the_changes_that_build_them`: revision 48a94da, S74 built, 171, 173 and S71 to S73 under `PROFILES` |
| A clause cited by nothing fails the suite | The suite as this change leaves it | `conformance:coverage`, S74 cited by the engine's scenarios of a start through the launcher |

## The site and the records

The commands reference gives `flume ack launched` its row and the agent
commands' table line, and the state reference the word
`launched/<identity>.json` and status's `unsettled.process`; the
places-and-herdr guide says how a program with no profile of its own
starts through the launcher (core.35). Record 0026 holds the construct
and code stages' choices as amendments, each with the path not taken;
rules core.10 and core.60 hold the outcome, and no rule changes.
