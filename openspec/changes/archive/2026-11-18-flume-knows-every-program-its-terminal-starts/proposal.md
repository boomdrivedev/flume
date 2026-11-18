# flume knows every program its terminal starts

## Why

herdr, flume's terminal, starts 24 kinds of agent program, and flume
runs one of them. Its resolver binds Claude Code's profile and resolves
every other program to nothing, so a launch that names `codex` or `amp`
is refused before anything is made. The user asked on 2026-10-30 that
flume support every agent its terminal supports out of the box, and
record 0026, accepted that day, decided how: every program herdr has a
kind for is bound by an agent profile, its own where flume carries one
and otherwise the common profile, each by an entry of flume's own (170).

The model is the one setting a layer cannot yet give per program. A
mode may be a table of programs (169), so a host whose pool mixes
programs gives each its own value. A model may not, though clause 172
says it may, and the effort's table waits on the model's by the
effort's own design (record 0044). A model given to a program that has
no way to take one is passed as written today, where 172 refuses it.

The bolt `flume-runs-every-agent-its-terminal-can-start` builds record
0026 change by change. This change is its entries and its model,
clauses 170 and 172 (signal `2026-11-18-boomdrive-ops-cb94852a/01`).

## What Changes

- **An entry for every program herdr starts.** flume's agent profiles
  hold one table, an entry for each of herdr 0.9.5's 24 kinds, each
  giving its program's name and herdr's kind for it (170, C.4).
  `claude`'s entry is bound by Claude Code's own profile. Every other
  entry's program is bound by the common profile, by its name or by a
  path whose file name is its name. A program with no entry is still
  refused, naming it.
- **Each entry's ways as checked against the release flume pins.** An
  entry gives a way to pass a model, an effort and each mode only once
  that way is checked against the program's release the tree pins
  (170; core.22, core.39). The tree pins Claude Code alone, whose
  profile is its own, so the other 23 entries hold their name and kind
  and nothing more.
- **A launch naming any of them is taken.** A stage whose settings name
  any program with an entry, and give it no model, no effort, no mode
  and no settings of its own, is taken at the launch. The common
  profile's command runs the program in the place under the start's
  identity and secret, by herdr's kind for it.
- **What a program cannot take is refused, naming it.** A model, an
  effort or a mode given for a program whose entry names no way to pass
  it refuses the launch, or the stage at a later start, naming the
  program, the stage and the layer (4, 32, 170, 172, 197). The agent
  seam gains a model reading, which refuses every name by default.
  Claude Code's profile takes every name and passes it by `--model` as
  written.
- **A model per program, and an effort with it.** `model` and `effort`
  may each be a table of programs in every agents table and at the
  launch, `--agent [<stage>.]model.<program>=<name>` and
  `--agent [<stage>.]effort.<program>=<word>`, taken whole as a mode's
  table is. The agent's entry is its program's, and a table that names
  none refuses as a mode's does, never passing to a later layer (169,
  172, 197).
- **A program herdr reads only through its integration.** OMP and
  MastraCode refuse the launch, and the stage at a later start, while
  `herdr integration status` reports their integration not installed,
  naming `herdr integration install <name>` (C.5, S75). The terminal
  seam gains that reading, and every other terminal starts every kind
  it starts today.
- **Status.** Each agent's model with its layer, the entry of a table
  it took and the name passed; its effort's entry beside it (56).
- **Outside this change.** The common profile's readings of 171, among
  them the launcher, received and own work from the terminal, and the
  prompt's argument under the printed command, so a stage of a program
  the common profile binds is taken but not yet run through. Which
  parts each profile builds (174), the change
  `each-agent-profile-says-which-parts-it-builds` beside this one. The
  program an account chooses (173). Each entry's flags for a program
  whose release the tree does not pin.

## Capabilities

### New Capabilities

- `agent-programs`: an entry for each program the terminal starts, its
  name and kind, bound by its own profile or the common profile; the
  common profile's command; a launch naming any of them taken; what an
  entry cannot take refused naming the program.
- `agent-model`: an agent's model, its program's own name or a table of
  programs, the agent's entry its program's; taken or refused by its
  profile at the launch and each start; none where none is given.

### Modified Capabilities

- `agent-effort`: an effort may be a table of programs, its entry the
  agent's program's.
- `agent-profile`: Claude Code takes every model name and passes the
  agent's entry by `--model`.
- `place-status`: an agent's model, its layer, its entry and the name
  passed, and its effort's entry.
- `command-line`: `--agent [<stage>.]model.<program>=<name>` and
  `--agent [<stage>.]effort.<program>=<word>`.
- `kinds-profile`: a declaration's agents tables give a model and an
  effort as a word or a table of programs.
- `store-profile`: the host's agents tables give a model and an effort
  as a word or a table of programs.
- `terminal-profiles`: herdr refuses a program it reads only through an
  integration not installed.
- `seam-contracts`: the agent seam takes a model or refuses it and names
  its program's kind; the terminal seam says whether it starts a kind;
  the settings carry a model and an effort per program.
- `seam-fakes`: the fake agent scripts the models it takes, and the
  fake terminal a kind it cannot start.
- `clause-coverage`: 170, 172 and S75 cited by this change's scenarios
  and no longer named unbuilt.

## Impact

- boomdrivedev/blueprints: no amendment. The statement at `48a94da`,
  the revision the clause list holds, states 170, 172, 197, C.4, C.5
  and S75 as built here.
- `docs/adr`: record 0026, accepted; rules core.30 and core.39 hold the
  outcome, and no rule changes.
- `crates/flume-seams`: `PerProgram`, `Settings::model` and
  `Settings::effort` of it; `Agent::model`, `Agent::kind` and
  `Terminal::starts`, each with its default; status fields, only added.
- `crates/flume-core`: `settings.rs` takes the agent's entry of a
  model and an effort and asks its profile of each, at the launch and
  each start, and asks the stream's terminal whether it starts the
  agent's kind; `status.rs` gives the model.
- `crates/flume`: `profiles/agent/programs.rs`, the table, and
  `profiles/agent/common.rs`, the common profile; the resolver binding
  both; Claude Code's `model`; herdr's `starts`; one reader of a model
  and an effort for the kinds profile, the host's agents tables and a
  role's entry; `--agent`'s tables; the person's form of status.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake agent's
  models and the fake terminal's kinds; new `engine/agent_model.rs` and
  `engine/agent_programs.rs`; the clause list.
- `site/`: the agent-settings, kinds, places-and-herdr and Claude Code
  guides, and the configuration, commands and state references.
- No new crate, plugin or site part, so no new moon project; no new
  dependency and no new pinned tool.
