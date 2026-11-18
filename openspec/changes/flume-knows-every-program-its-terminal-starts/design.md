# Design

## Context

Record 0026, accepted by the user on 2026-10-30, decided that flume
runs every agent program its terminal can start. The statement holds
the outcome at boomdrivedev/blueprints `48a94da`, the revision
`crates/flume-conformance/src/clauses.rs` reads: clauses 170 to 174,
C.4's common profile, C.5's herdr row and S71 to S75. Rules core.30 and
core.39 bind it. This change builds clauses 170 and 172 and the part of
C.5 a launch of them needs (signal `2026-11-18-boomdrive-ops-cb94852a/01`).
The change `each-agent-profile-says-which-parts-it-builds`, cut from the
same bolt, builds 174.

### What the tree holds now

- **The resolver.** `BuiltIn::agent` in `crates/flume/src/profiles.rs`
  binds Claude Code's profile to `claude`, by name or by a path whose
  file name is `claude`, and resolves every other program to nothing.
  `Engine::agent_for` then refuses the launch: "no agent profile runs
  the program <program> the stage <stage> names, from the layer
  <layer>".
- **herdr's kinds.** `terminal::herdr::KINDS` holds herdr 0.9.5's 24,
  each its program's canonical name: pi, claude, codex, gemini, cursor,
  devin, agy, cline, omp, mastracode, opencode, copilot, kimi, kiro,
  droid, amp, grok, hermes, kilo, qodercli, qwen, letta, maki and muse.
  herdr's `open` refuses a command whose kind is none of them. Rex's
  `KINDS` holds `claude` alone.
- **The model.** `Settings::model` is one string, passed by Claude
  Code's command as `--model <model>`. No profile is asked of it, so a
  model given for any program is passed as written.
- **The effort.** `Settings::effort` is one string, asked of the
  agent's profile at the launch and each start (`effort_taken`). Every
  reader refuses a table of programs "until the model's is built with
  it": the kinds profile, the host's agents tables, and the effort
  spec's requirement. Record 0044's design 2 says the effort's table is
  built with the model's.
- **The mode.** `ModeSetting` is one of flume's four or a table of
  programs, the agent's entry its program's, and a table naming none
  refused (`mode_for`). The agent seam's `mode` refuses every entry by
  default.
- **Pinned programs.** `devenv.nix` pins Claude Code 2.1.296 and herdr
  0.9.5, and no other agent program.

### What herdr reports of an integration

`herdr integration status`, run in a scratch home under the pinned
0.9.5 on 2026-11-18, prints one line per integration,
`<name>: <state> (<path>)`, its state `not installed`, `current (v<n>)`
or `outdated (v<n> < v<m>)`, the path under the home the environment
gives (`$HOME/.omp/agent/extensions/herdr-omp-agent-state.ts` for omp).
`herdr integration install omp` there prints that it installed the
integration, `omp: current (v10)` follows, and with no
`$HOME/.omp/agent/extensions` it prints "install omp first" and exits
0, installing nothing.

## Goals / Non-Goals

**Goals:**

- Every program herdr starts has an entry, its name and its kind, and
  is bound by an agent profile: Claude Code's for `claude`, the common
  profile for each other (170).
- A launch naming any of them, giving it nothing it cannot take, is
  taken.
- A model, an effort or a mode an entry names no way to pass refuses
  the launch or the stage, naming the program (170, 172, 197; core.39).
- A model and an effort may each be a table of programs, taken whole,
  its entry the agent's program's (172, 197).
- OMP and MastraCode are refused while herdr reports their integration
  not installed (C.5, S75; core.39's details).

**Non-Goals:**

- The common profile's readings of 171: the launcher, `flume ack
  launched`, and received, asking, own work, ended and last words read
  from the terminal. Until that change, each of those answers as the
  seam's default (design 5).
- Which parts each profile builds, and status naming the degraded ones
  (174): the change `each-agent-profile-says-which-parts-it-builds`.
- The program an account chooses for a stage whose layers name none
  (173), and an account usable only where its program takes the stage's
  model, effort and mode.
- Each entry's flags for a program whose release the tree does not pin,
  Codex, Gemini CLI, Copilot CLI and OpenCode among them: each comes
  with the change that pins its release (design 2).
- Rex starting any kind but `claude`: its `open` keeps refusing one.

## Decisions

### 1. One table of entries, in the agent profiles

`crates/flume/src/profiles/agent/programs.rs` holds `PROGRAMS`, one
entry per herdr kind, in `KINDS`' order:

```rust
/// A program flume runs: its name, the terminal's kind for it, and
/// each way its release takes a setting, none where no way is checked
/// against the release the tree pins (170, C.4; core.39).
pub struct Program {
    pub name: &'static str,
    pub kind: &'static str,
    pub model: Option<&'static str>,
    pub effort: Option<Effort>,
    pub modes: Option<[(Mode, &'static str); 4]>,
}
```

`Effort` holds the flag and the words its release takes. `entry(name)`
finds the entry a program's name or path names, by the path's file
name, as `BuiltIn::agent` finds `claude` today. `BuiltIn::agent` binds
`claude` to Claude Code's profile, every other entry to
`agent::common::Common` over its entry, and any other name to nothing,
so a program with no entry is refused as it is now.

The table is code, so a test reads it beside herdr's `KINDS` and
fails on a kind with no entry, an entry for no kind, or an entry whose
name is not its kind (170; core.39). C.4 gives each entry the terminal's
kind "its name where herdr's kinds hold it", and herdr's 24 kinds are
each their program's canonical name, so every entry's name and kind are
one word.

Alternatives considered:

- **A file of entries flume reads at its start.** Record 0026 rejects
  it: an entry mapping `edits` to a program's full-access flag would run
  every agent of it wider than asked, and nothing would check it
  against a release (core.39's details).
- **Claude Code's entry left out.** Its profile binds it, and its ways
  live in `claude/effort.rs` and `claude/mode.rs`. Its entry stays in
  the table so the test of herdr's kinds holds all 24, its ways empty
  there since its own profile answers them.

### 2. An entry's ways only from a pinned release

The tree pins Claude Code alone among agent programs, so the 23 other
entries hold their name and kind, and none a model flag, an effort or a
map of modes (170, C.4). The common profile's `model`, `effort` and
`mode` refuse every value where the entry names no way, in words naming
the program:

```
the entry for `amp` names no way to give it a model
```

and the core's refusal names the stage, the value as written, its layer
and the program before them, as it names a mode's (design 4).

Alternatives considered:

- **Pin Codex 0.165.0, Gemini CLI 0.66.0, Copilot CLI 1.0.104 and
  OpenCode 1.19.4 now, the four whose help record 0026 read, and write
  their flags.** Each pin is a tool `devenv.nix` fetches for every
  host that runs the gate, and each flag a `check-tools` test of the
  pinned binary, as `claude_words.rs` reads Claude Code's (core.22).
  That is four changes' worth of tools, each its own release to read,
  and record 0026 plans Codex's profile after the common profile. Not
  taken here.

### 3. A model and an effort per program

`crates/flume-seams/src/kinds.rs` gains one type for a setting given
as one value or a table of programs:

```rust
/// A setting as a layer gives it: one value for every program, or a
/// table of programs, each entry as written, the agent's taken by its
/// program's name (172, 197).
pub enum PerProgram {
    One(String),
    Programs(BTreeMap<String, String>),
}
```

It serializes as the string or the object it was written as, so a held
state written before reads its model and effort unchanged, and a table
that names no program is refused where it is read, as a mode's is.
`entry(program)` gives the agent's value, none where a table names no
entry for it. `written()` gives the form status and refusals name,
`opus` or `{ claude = "opus", codex = "gpt-5.5" }`, as
`ModeSetting::written` does. `Settings::model` and `Settings::effort`
become `Option<PerProgram>`, each still taken whole by
`Settings::under`, so a table is never merged with another layer's
(172).

The mode keeps `ModeSetting`: its bare word is one of flume's four and
any other is refused bare, where a model's or an effort's bare word is
the program's own.

One reader, `crates/flume/src/per_program.rs` beside `mode.rs`, reads
`model` and `effort` from TOML for the kinds profile's `map_agent`, the
host's agents tables and a role's entry: a string, or a table whose
entries are strings, and any other value, an empty table among them,
refused naming the key, the caller putting the table before it.
`--agent [<stage>.]model.<program>=<name>` and
`effort.<program>=<word>` build one table per stage as `mode.<program>`
does, and a name and a table for one stage are refused naming both.

### 4. The core takes the agent's entry and asks its profile

`resolution` in `crates/flume-core/src/settings.rs` resolves the
agent's model and effort as it resolves its mode: the setting the
first layer gives, its entry for the agent's program by
`program_name`, then the profile asked. A table naming no entry for the
program refuses in the mode's words:

```
the stage `code`'s model `{ codex = "gpt-5.5" }`, from the layer
`host-kind`, names no entry for `claude`
```

The agent seam gains:

```rust
/// Whether the agent's program takes `name` as its model, passed as
/// written; words naming the program where its profile names no way to
/// give one, which every name is by default (170, 172).
fn model(&self, name: &str) -> Result<(), String>;
```

A refusal reads as an effort's: "the stage `code`'s model `opus`, from
the layer `launch`, cannot run on `amp`: <the profile's words>". The
launch asks at every stage before anything is made, and a later start
asks again, the host's layers as they stand, its refusal leaving the
stage refused until a retry (4, 8, 27, 32).

`Resolution` gains `model` and `effort`, the entries taken. The
settings a claim hands `Agent::command` hold those entries, each as
`PerProgram::One`, so a profile reads one name and one word and never a
table. The start's record keeps the settings as the layers wrote them,
a table among them, with `from` naming each layer, so status gives the
table as written beside the entry its start passed (56).

### 5. The common profile

`crates/flume/src/profiles/agent/common.rs` holds `Common`, built over
an entry and the absolute path of the running binary, as Claude Code's
profile is built with it (core.60). Its command runs the program the
settings name, its name or its path, in the place: its entry's model
flag, effort flag and mode setting where the settings give them, which
no entry of this change names; the account's environment with
`FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE` over it,
as Claude Code's command sets them (C.4); its entry's kind; and no
prompt argument, folder-trust question or file. Its stop-short names
`flume needs` by the binary's absolute path, as Claude Code's does
(92; core.60).

Every other part answers as the agent seam's default: started,
received, asking, own work and ended unknown, last words none, spent
and windows unknown, send-back, on-quiet and guard not offered, carry
not offered, live unknown, no mode read back, no reserved key, and own
settings refused where any are given (93, 171.11, 195). The seam's
guarantee holds: nothing is read from the terminal for a part the
profile has no reading of yet (93; core.39). 171's change builds the
launcher and the terminal's readings over this profile.

So a stage of a program the common profile binds is taken, its agent
opened by herdr's kind, and its start not read started until that
change, status saying what it could not read (31).

### 6. herdr's integration, asked at the launch and each start

The terminal seam gains:

```rust
/// Whether the terminal can start a program of `kind` now, under the
/// login environment with `env` over it; every kind by default, which
/// `open` refuses as it does now where it cannot (C.5).
fn starts(&self, kind: &str, env: &BTreeMap<String, String>) -> Result<(), String>;
```

and the agent seam `kind()`, its program's kind, none by default:
Claude Code's `claude`, the common profile's its entry's, and the
fake's `agent`.

herdr's profile holds `INTEGRATED`, `omp` and `mastracode`, the kinds
whose state herdr 0.9.5 reads only through its integration. For one of
them it runs `herdr integration status` within its bound, under the
login environment with `env` over it, and reads the kind's line: `not
installed` refuses, naming `herdr integration install <kind>`; a status
that does not answer within the bound, or holds no line for the kind,
refuses naming what herdr gave (core.29); any other state starts it.
Every other kind starts without a call.

The launch asks the stream's terminal of each stage's kind with no
account's environment, before anything is made (4). Each start asks
the terminal its start records with the chosen account's environment,
so herdr reads the home the pane will hold, and a refusal leaves the
stage refused in the terminal's words until a retry, as a mode refused
at a later start does (27, 32; core.39's details).

Alternatives considered:

- **Refuse in herdr's `open`.** C.5 refuses the launch (4), and core.8
  refuses a launch whole that asks for more than flume runs; an `open`
  refusal comes after the place is made.
- **Read the status under the login environment alone.** An account's
  environment may move a program's home, as `CLAUDE_CONFIG_DIR` moves
  Claude Code's, and herdr installs the integration under that home.

### 7. Status and the JSON form

`AgentStatus` gains `model`, an object only where a layer gave one:
`name` as written, `layer`, `entry`, the program whose table entry it
took, none for a name given whole, and `passed`. `EffortStatus` gains
`entry` the same way. The person's form gives the agent's line ", on
the model opus (from launch)", adding ", the entry claude, passed as
opus" for a table, and the effort's ", the entry claude" the same way.
The JSON form only gains fields; a model or an effort given as a table
holds the object it was written as in the agent's settings, as a mode's
table does (56; core.14).

### 8. The no-tool check keeps its words

core.1's check names Claude Code among agent programs. The 23 other
names include `cursor`, `pi` and `amp`, words Rust and prose use, so a
word check would refuse `std::io::Cursor`. The table is the one place
the names are listed, and the test of design 1 holds it to herdr's
kinds.

### 9. The clause list

`crates/flume-conformance/src/clauses.rs` keeps its revision,
`48a94da`. 170, 172 and S75 leave `unbuilt`, each cited by this
change's scenarios: 170 by `engine/agent_programs.rs`, 172 by
`engine/agent_model.rs`, S75 by the terminal's refusal in
`engine/agent_programs.rs`. 171, 173, 174 and S71 to S74 stay unbuilt
under `PROFILES`, its words naming the common profile's readings, a
pool's program and the parts. The change beside this one moves 174 on
the same lines, which the later of the two to land takes in its merge.

## Rules and clauses

- core.1, core.2: the table, the common profile and herdr's integration
  check are profiles' code; the core asks `Agent::model`, `Agent::kind`
  and `Terminal::starts` and names no program or terminal.
- core.3: the agent is bound by its settings' program, from the
  resolver's table and nothing else.
- core.8: a launch whose stage's model, effort, mode or terminal is
  refused leaves nothing made.
- core.14: status only gains fields.
- core.22, core.39: an entry's ways only from the pinned release, and a
  model, an effort or a mode an entry cannot honour refusing the launch
  or the stage, naming the program.
- core.29: herdr's integration status read within its bound, a status
  that does not answer read as a refusal in herdr's words.
- core.30: the model the program's own name, passed as written.
- core.60: the common profile's stop-short names the binary by its
  absolute path.
- Clauses 4, 8, 27, 32, 56, 88, 92, 93, 169, 170, 172, 197, C.4, C.5
  and S75. There are no area pages, so `core.md` alone binds.

## Risks / Trade-offs

- **A taken stage that never begins.** Until 171's change, a stage on
  a program the common profile binds opens its agent and is never read
  started. The bolt lands on main only with that change, so no host on
  main sees it.
- **`Settings::model`'s type.** Every reader of the model changes with
  it. `PerProgram` takes `From<&str>` and `From<String>`, so a setting
  built from one name reads as it does today.
- **One `herdr` call per start of OMP or MastraCode.** Bounded, and
  never made for any other kind.

## Open Questions

- C.4's common profile names no way to give a program its standing
  prompt, and `Common::command` passes none, so no program the common
  profile binds receives the goals, the intent's brief or the stage's
  standing instructions. 171's change starts the first turn of such a
  program and meets it first: the statement's C.4 row, for the design
  role.
