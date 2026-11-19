# The common profile reads an agent from its terminal

## Why

flume takes a launch of any program herdr starts, each bound by its
own profile or the common profile (170), and a stage of a program the
common profile binds now starts through flume's launcher (35, 171 item
1). Nothing after its start is read: the common profile answers
received, asking, own work and ended unknown and last words none, so
its prompt is never read received and is reported never begun, and an
agent whose own work flume cannot read is never quiet (31). A stage of
amp, Codex or any program but Claude Code cannot run through.

Record 0026, accepted by the user on 2026-10-30, decided how flume
reads such an agent (171 items 2 to 11, C.4, C.5): a prompt is
received once the terminal sees the turn the sending began; the agent
asks while the terminal shows it waiting on a question; its own work
is none once it is read waiting at two settles a minute or more apart;
it has ended once the terminal reads it stopped, and its last words
are what the terminal shows. Usage limits, plans, send-backs, guards
and carrying a session over are not offered. Status names each part so
read with the agent, so nobody reads a degraded part as a built one
(170, 174).

The bolt `flume-runs-every-agent-its-terminal-can-start` builds record
0026 change by change (signal
`2026-11-18-boomdrive-ops-cb94852a/01`). This change is the common
profile's readings from its terminal, 171 items 2 to 11 and S73. The
refusal of a program herdr reads only through an integration not yet
installed, naming `herdr integration install <name>` (C.5, S75), is
built by `flume-knows-every-program-its-terminal-starts`, and nothing
of it changes here.

## What Changes

- **Five more parts a profile may degrade.** Received, asking, own
  work, ended and last words join the agent seam's parts, before spent,
  in 171's order, each with how flume reads it from the terminal (171,
  174). Claude Code's profile builds all thirteen; the common profile
  builds none, so status names every one degraded with the agent.
- **The core reads a part a profile does not build from the agent's
  terminal**, and asks the profile nothing of it, so no part is read
  from the terminal where its profile builds it (93, 171; core.2,
  core.39):
  - **received**: the prompt, or a tell's words, sent by a send that
    watches for the turn it began; received on the terminal's word that
    it saw the agent working or asking, or on a reading of the agent so
    within 60 seconds of the sending, and sent once more otherwise (38,
    89, 171 item 2);
  - **asking**: the terminal reading the agent waiting on a question
    (90, 171 item 3); the place's line, which reads no terminal, takes
    it from when a settle last read the agent asking (core.52);
  - **own work**: none once two settles a minute or more apart read
    the agent waiting with no other reading between, running until
    then, and none for an agent the terminal reads stopped (31, 91, 171
    item 4);
  - **ended and last words**: the terminal's stopped, and what it shows
    of the agent (43, 134, 136, 171 item 5).
- **The terminal seam sends and watches the turn.** A new operation
  sends text as `send` does and answers whether the terminal saw the
  turn the sending began, the agent working or asking, within a bound;
  a terminal that reads no turn refuses it. herdr builds it by `agent
  prompt --wait --until working --until blocked --timeout 60000`,
  reading `agent_prompt_stalled` and `timeout` as not seen (C.5).
- **A start holds when a settle read its agent waiting.** Each settle
  of a live start whose profile reads no own work writes when the
  agent was first and last read waiting, and clears it at any other
  reading and at every sending (31, 171 item 4; core.16, core.17).
- **What stays as it is.** Spent, usage and windows unknown; send-back,
  guard, on-quiet and carry not offered, the host's watch or an acting
  command settling the place; the mode recorded and not compared; own
  settings neither reserved nor taken (171 items 6, 7, 9 to 11). Each
  already holds for the common profile.
- **The clause list builds 171 and S73.** 173, S71 and S72 stay named
  for record 0026's last change, the program a pool's account chooses.
- **Outside this change.** The plan part (164) and the usage part
  (152) join the parts with the changes that add them to the seam; the
  prompt's argument under the printed command (C.4); the program a
  pool's account chooses (173).

## Capabilities

### New Capabilities

- `terminal-readings`: what flume reads from an agent's terminal where
  its profile builds no part of its program's own for it: received,
  asking, own work, ended and last words, each read by the core and
  never asked of the profile.

### Modified Capabilities

- `seam-contracts`: thirteen parts a profile may degrade, each with
  its word; the terminal seam's watched send; a start's held reading of
  its agent waiting.
- `seam-fakes`: the fake agent scripts reading the five from its
  terminal; the fake terminal scripts a watched send's turn.
- `agent-profile`: Claude Code's profile builds all thirteen parts.
- `agent-programs`: the common profile builds no part, so the core
  reads its five from its terminal.
- `place-status`: a program read from its terminal names its five
  parts degraded.
- `place-line`: the line reads asking from the held state for an agent
  whose profile reads no asking.
- `terminal-profiles`: herdr's watched send; the core reads herdr's
  blocked as asking where the agent's profile reads asking from the
  terminal.
- `clause-coverage`: 171 and S73 built, cited by the engine's scenarios
  of an agent read from its terminal.

## Impact

- boomdrivedev/blueprints: no amendment. The statement at `48a94da`,
  the revision the clause list holds, states 31, 38, 89 to 93, 171,
  C.4's common profile, C.5's herdr send and state rows and S73 as
  built here.
- `docs/adr`: record 0026 amended with this change's choices, which are
  the construct's; rules core.2, core.39 and core.52 hold the outcome,
  and no rule changes.
- `crates/flume-seams`: `Part`'s five new parts; `SentWatching`, `NO_TURN`
  and `Terminal::send_watching` with its default; `WaitingRead` and
  `StartHeld::waiting`, only added.
- `crates/flume-core`: the readings of a degraded part from the
  terminal, read wherever the core reads asking, own work, ended, last
  words or received; the watched send of a prompt and a tell; the
  settle's writing of `waiting`; the line's asking.
- `crates/flume`: Claude Code's parts; the common profile's doc
  comment; herdr's `send_watching`.
- `crates/flume-fakes`, `crates/flume-conformance`: the fakes' scripts;
  the seams' scenarios; a new `engine/terminal_readings.rs`; the clause
  list.
- `crates/flume/tests/profiles/tools`: herdr's watched send under the
  pinned herdr, and S73's walkthrough under the pinned herdr.
- `site/`: the places-and-herdr guide and the state reference.
- No new crate, plugin or site part, so no new moon project; no new
  dependency and no new pinned tool.
