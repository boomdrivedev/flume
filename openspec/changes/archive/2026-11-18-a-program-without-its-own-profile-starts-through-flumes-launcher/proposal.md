# A program without its own profile starts through flume's launcher

## Why

flume takes a launch of any program herdr starts, each bound by its
own profile or the common profile (170), but a stage of a program the
common profile binds never begins: the profile reads no word of the
program's that it started, so its start stays starting until the
stage's limit. Claude Code says it started by its own hook; a program
with no hook flume can call says nothing. Record 0026, accepted by the
user on 2026-10-30, decided how such a start is read: a launcher of
flume's own runs in the pane under the start's identity and secret,
checks the start's record, records its word with its process and
becomes the program, and the start counts once the terminal reads that
same process running the program ready for input (35, 171 item 1,
C.4). A program started by hand in the pane runs in another process,
so it is not the stage's agent (S74).

The bolt `flume-runs-every-agent-its-terminal-can-start` builds record
0026 change by change (signal `2026-11-18-boomdrive-ops-cb94852a/01`).
This change is the start.

## What Changes

- **flume's launcher.** `flume ack launched -- <program> [<argument>…]`
  reads the start its environment names, `FLUME_AGENT` with the start's
  secret, from the core; compares each variable the start's record
  names, by its digest keyed by the secret, and the program it is to
  run with the program the record names; writes its word once in the
  place's own folder, its process id and whether anything differed;
  and, where nothing did, becomes the program in the same process, its
  arguments passed whole. Where anything differs, where no start is
  found, or where a word was already written for the start, it runs
  nothing, says why, and exits non-zero; a difference also starts a
  settle of the place, as Claude Code's hook does (161, C.4).
- **The common profile starts its program through the launcher.** Its
  command names the launcher, the binary that builds it by its absolute
  path with `ack launched --`; its word that it started is the
  launcher's word with nothing differing; how it began otherwise is
  that word's; and it names the process the launcher became (35, 171,
  C.4; core.60).
- **The terminal runs the launcher in the program's place.** herdr's
  link of the kind's name in the start's own folder goes to the
  launcher, which its agent start runs with the program's absolute path
  and its arguments after the launcher's words, and the pane's
  foreground must then run the program the settings name, as for any
  program; the printed command prints the launcher's line (C.5).
- **The terminal reads the process its agent's program runs in.** herdr
  reads the pane's foreground process running the program its open
  checked, by its id; the printed command reads none (C.5).
- **A start through the launcher counts once that process is read.**
  The core takes the start of an agent whose profile starts it through
  the launcher only once the launcher's word says nothing differed, the
  terminal reads the program running in the process that word names,
  and reads the agent waiting. Read running in another process, the
  start began otherwise: flume sends it no prompt, ends it once quiet
  and starts the stage afresh, and status names the process as what
  differed (35, 161, S74).
- **Outside this change.** The common profile's other readings from the
  terminal, received, asking, own work, ended and last words, so a
  stage of such a program starts but its prompt and its end wait on
  them (171 items 2 to 11, S73); the prompt's argument under the printed
  command (C.4); the program a pool's account chooses (173). Rex opens
  no program but `claude`, so it runs no launcher.

## Capabilities

### New Capabilities

- `agent-launcher`: flume's launcher, what it compares, the word it
  writes once and the program it becomes; and the core's reading of a
  start through it, started once the terminal reads its process ready,
  and begun otherwise where the program runs in another process.

### Modified Capabilities

- `agent-programs`: the common profile's command runs its program
  through the launcher, and its started, began-otherwise and process
  readings are the launcher's word.
- `seam-contracts`: the agent seam says whether a profile starts its
  program through flume's launcher and the process its start's word
  names; an agent's command carries the launcher's words; the terminal
  seam reads the process an agent's program runs in; a start begun
  otherwise names another process.
- `seam-fakes`: the fake agent scripts a start through the launcher and
  its process, and the fake terminal the process it reads.
- `terminal-profiles`: herdr and the printed command run the launcher
  in the program's place; herdr reads the program's process.
- `command-line`: `flume ack launched`.
- `clause-coverage`: S74 cited by this change's scenarios and no longer
  named unbuilt.

## Impact

- boomdrivedev/blueprints: no amendment. The statement at `48a94da`,
  the revision the clause list holds, states 35, 161, 171, C.4, C.5 and
  S74 as built here.
- `docs/adr`: record 0026 amended with this change's choices, which are
  the construct's; rules core.10 and core.60 hold the outcome, and no
  rule changes.
- `crates/flume-seams`: `AgentCommand::launcher`, `Agent::launches`,
  `Agent::launched`, `Terminal::program_process` with `ProgramProcess`,
  and `Unsettled::process`, each with its default, only added.
- `crates/flume-core`: the reading of a start through the launcher,
  used wherever the core reads that an agent started or began
  otherwise.
- `crates/flume`: the launcher, `flume ack launched`; the common
  profile's command and readings; herdr's and the printed command's
  runs of the launcher, and herdr's process reading; the claude hook's
  comparison of a start's variables shared with the launcher.
- `crates/flume-fakes`, `crates/flume-conformance`: the fakes' scripts;
  the seams' scenarios; a new `engine/agent_launcher.rs`; the clause
  list.
- `site/`: the places-and-herdr guide, and the commands and state
  references.
- No new crate, plugin or site part, so no new moon project; no new
  dependency and no new pinned tool.
