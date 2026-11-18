# Design

## Context

See proposal.md for why. The statement is read at boomdrivedev/blueprints
48a94da, the revision the clause list holds: 35, 161, 171 item 1, C.4's
common-profile command and started rows, C.5's herdr row and S74.
Record 0026 decided the launcher (its start option 2); this design
decides how it is built.

### What the tree holds now

- **The agent seam** (`crates/flume-seams/src/agent.rs`). `started`
  answers whether the agent said it started, `started_without` how it
  began otherwise, each read from the agent's own record in the place's
  own folder. `AgentCommand` names the program, its arguments, its
  environment, its kind and the files it is started with. `Unsettled`
  names each variable that is missing or differs, whether the program,
  the session and the mode did.
- **The terminal seam** (`terminal.rs`). No operation names the process
  an agent runs in; herdr's `open` reads the pane's foreground once, by
  `pane process-info`, to refuse a start whose pane runs another program
  than the settings name (`Herdr::runs`, `runs_program` in
  `profiles/terminal/common.rs`).
- **The common profile** (`crates/flume/src/profiles/agent/common.rs`).
  Its command runs the program by name or path with its entry's kind,
  and its `started` answers unknown, so its start stays starting until
  the stage's limit.
- **Claude Code's started word** (`claude/hook.rs`). `flume ack started`
  takes the start's record from the core by `Engine::start_record`,
  given the start's identity and secret; `compare` checks each
  variable's digest keyed by the secret and the program, the parent's
  executable against the record's program found on `PATH`; a difference
  is written as the agent's `unsettled.json` and a settle started
  detached (`began_otherwise`, `settle_detached`).
- **The core's readings** (`crates/flume-core`). `agent.started` and
  `agent.started_without` are read in `reading.rs` (`read_agent`),
  `acts.rs` (a tell's wait for a waiting agent), `agent_acts.rs` (the
  stop check) and `deliver.rs` (a starting stage taken started, a prompt
  never begun taken begun). A start read begun otherwise is kept in the
  held state's `unsettled` and ended once quiet, its stage started
  afresh (`unsettled.rs`).
- **herdr's start.** The pane's `PATH` is led by
  `<folder>/herdr/<digest>/bin`, holding a link named for the kind to
  the program's absolute path, resolved on the command's `PATH`; herdr's
  `agent start --kind <kind> -- <args>` types `<kind> <args>` into the
  pane's shell.
- **Rex** refuses an open of any kind but `claude` (terminal-profiles,
  "Rex runs only a program that reports its state to it"), so it opens
  no program of the common profile.

## Goals / Non-Goals

**Goals:**

- A start of a program the common profile binds counts only on the
  launcher's word with nothing differing and the terminal's reading of
  the program in the launcher's own process, waiting (35, 171 item 1).
- A program in the pane under any other process is not the start's
  agent, and is ended and its stage started afresh as 161 does (S74).
- No terminal profile names an agent program, and the core names no
  tool (core.1, core.2).

**Non-Goals:**

- The common profile's readings of 171 items 2 to 11. Until they land
  a stage of such a program starts, and what follows its start, its
  prompt's receipt and its quiet among it, reads unknown.
- The prompt's argument under the printed command (C.4). The printed
  command prints the launcher's line, and reads no process, so such a
  start reads unknown there.
- Rex: it opens no program but `claude`.

## Decisions

### 1. The launcher is `flume ack launched -- <program> [<argument>…]`

The launcher is a subcommand of flume's one binary, beside `ack
started` and `ack received`: the agent's word about its own start,
given by flume's own code in the program's place. It is
`profiles/agent/launcher.rs`, flume's own and naming no program (record
0026's consequences; core.1). Its arguments after `--` are the program,
by its absolute path as a terminal resolved it or by its name, and the
program's own arguments, passed whole.

Not taken: a launcher binary of its own, which would be a second
artifact to build, pin and ship beside flume, with no part that flume's
own binary lacks; and a launcher picked by `argv[0]`, a link named for
the kind making flume act as the program, which hides the subcommand
from its help and from a reader of the pane's command line.

### 2. What the launcher compares, and the word it writes once

The launcher reads `FLUME_AGENT` and `FLUME_SECRET`, takes the place's
own folder and the start's record from the core by
`Engine::start_record`, as Claude Code's started word does, and
compares:

- each variable the record names, by its digest keyed by the secret,
  missing or differing by name alone; the loop is the one
  `claude/hook.rs`'s `compare` runs, moved to
  `profiles/agent/start.rs` and called by both (core.30);
- the program it is to run with the record's program, each found as the
  command finds it: a path as given, a name on the launcher's `PATH`
  past any file that is the launcher's own binary, both canonical. The
  start's folder leads the pane's `PATH` with the kind's name linked to
  the launcher, so the record's `amp` found first would be the launcher
  itself.

Its word is `<folder>/launched/<identity>.json`, the identity's `/`
written `%` as Claude Code's record writes it: its process id, the
program's absolute path, and how it began otherwise, none where nothing
differed. It is written by create-new, once per start (core.17). A
second launcher under the start, as a person typing the kind's name in
the pane runs, finds it, leaves it, runs nothing and exits non-zero
naming the process the first became.

Where nothing differs it execs the program in its own process, the
environment as it stands, so the program keeps `FLUME_AGENT` and
`FLUME_SECRET` for `flume needs` (92). Where anything differs it runs
nothing, starts `flume settle <place>` detached, as `began_otherwise`
does, prints why on standard error, which the pane's screen then shows,
and exits non-zero. With no start under its identity and secret it
writes nothing.

Not taken: running the program as the launcher's child, which would put
the program in another process than the one the word names and leave a
launcher between the terminal and the program; and a word written at
every run, which would let a second launcher name a new process as the
start's.

### 3. The agent seam: `launches`, `launched`, and the command's `launcher`

- `Agent::launches(&self) -> bool`, false by default: whether the
  profile starts its program through flume's launcher, asked by the
  core before it takes a start.
- `Agent::launched(&self, folder, agent) -> Option<u32>`, none by
  default: the process the start's word names.
- `AgentCommand::launcher: Vec<String>`, empty by default and left out
  of the JSON while empty: the launcher's program by its absolute path,
  then its arguments, after which a terminal puts the program and its
  arguments.
- `Unsettled::process: bool`, false by default, left out of the JSON
  while false: the program runs in another process than the start's
  launcher became; its words, "in another process than the one its
  start's launcher became", name no process's id.

The common profile builds them: `launches` true; its command's
`launcher` its binary's absolute path with `ack launched --`, the same
binary its stop-short names (core.60); `started` yes once a word names
nothing differing, no before one or where one names something, unknown
where the folder cannot be read; `started_without` the word's; and
`launched` the word's process.

Not taken: a richer `started` answer carrying the process, which would
change the one reading every profile and fake answers; and the launcher
a `Part`, which 174's parts would then let a profile degrade, where
started is never degraded (C.4).

### 4. The terminal seam: `program_process`

`Terminal::program_process(&self, agent) -> ProgramProcess`:
`Runs { pid }`, the process the terminal reads running the agent's
program; `Nothing`, where the agent's part holds none or is gone; or
`Unknown { why }` in the terminal's words. The default answers unknown,
`the terminal reads no process`, so the printed command needs nothing.

herdr reads it by `pane process-info --pane <id>` within its bound
(core.29), the pane found as every operation finds it: the foreground
process that `runs_program` reads running the program its open checked,
which open now keeps with the pane in its record by its absolute path.
A record written before names none, and reads unknown, saying so.

Not taken: a reading of the pane's foreground whole, every process with
its arguments, for the core to match: the program's path is the
terminal's, resolved on the command's `PATH`, and the matching is
herdr's profile's already.

### 5. herdr and the printed command run the launcher

Where the command's `launcher` is not empty, herdr links the kind's name
in the start's folder to the launcher's program, and its agent start
passes the launcher's arguments, the program's absolute path and the
command's arguments, so the shell runs `<kind> ack launched --
/abs/amp …` and the launcher becomes `/abs/amp` in that process. Its
check that the pane's foreground runs the program the settings name
reads the program after the exec, as for any program. The printed
command's line puts the launcher's words before the program, by its name
as the settings give it, so a person who runs it starts it as a terminal
would. No terminal names the launcher's subcommand: each takes the
words whole (core.1).

### 6. The core reads a launched start in one place

`Engine::started(agent, folder, start)` and
`Engine::began_otherwise(agent, folder, start)`, in `reading.rs`, take
the place of every `agent.started` and `agent.started_without` the core
reads, so no reading of a start differs between a settle, a tell and the
stop check:

- **started.** For a profile that does not launch, its own answer. For
  one that does: its answer where it is not yes; otherwise the
  terminal's `program_process` for the start's agent, yes where it runs
  in the process the word names and `state_of` reads the agent waiting,
  no where it runs in no process, in another, or not yet waiting, and
  unknown where the terminal cannot read it (35).
- **began otherwise.** The profile's own answer; else, for a profile
  that launches, with its word's process read, the terminal reading the
  program in another process: `Unsettled { process: true }` (161, S74).
  `read_unsettled` then keeps it in the held state and ends the agent
  once quiet, as for any start begun otherwise, so the reading need not
  hold at every settle once it was read.

Not taken: the agent profile asking the terminal itself, which would put
a terminal's reading inside an agent profile (core.2); and the check
made once inside the terminal's open, which would miss a program run by
hand after the open returned (S74).

### 7. The clause list

`REVISION` stays `48a94da`. S74 leaves `unbuilt`, cited by the engine's
scenarios of a start through the launcher; 171, 173 and S71 to S73 stay
under `PROFILES`, whose words name the common profile's readings from
its terminal and a pool's program. No scenario of the suite cites 171
or S73, which the coverage check refuses while they are unbuilt.

## Rules and clauses

- core.1, core.2: the launcher and the common profile are flume's own in
  `profiles/agent/`; terminals take the launcher's words whole; the
  core reads the terminal and the agent each through its seam.
- core.10: an agent is its stage's on its launcher's word where its
  profile reads no word of its program's.
- core.17: the word is written once, so a second launcher changes
  nothing.
- core.29: herdr's `pane process-info` runs within the profile's bound.
- core.30: the word names variables by name alone, never a value.
- core.60: the launcher's words name the binary that builds the command
  by its absolute path, never whichever `flume` the pane's shell finds.
- Clauses 35, 41, 88, 92, 161, 171, C.4, C.5 and S74.

## Risks / Trade-offs

- [A start the launcher refuses waits on herdr's bound] The launcher
  exits, the pane's program never comes up, and herdr's agent start
  answers only at its 120 seconds → the settle the launcher started
  reads the start begun otherwise at once, and the screen names why.
- [herdr reads a launcher before its exec] For the moment before the
  exec, the pane's foreground is flume under the kind's name, with the
  program's absolute path among its arguments → `runs_program` reads it
  running the program, in the same process the word names, so neither
  check is misled.
- [A stage of such a program has no received or quiet yet] What
  follows its start reads unknown until the common profile reads it
  from the terminal (171 items 2 to 11) → the engine's scenarios of S74
  run over the fake agent, which scripts its quiet, and status names
  each degraded part, as 174 has it.
