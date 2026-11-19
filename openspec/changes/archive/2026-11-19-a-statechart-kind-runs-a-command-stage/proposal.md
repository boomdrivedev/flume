# A statechart kind runs a command stage

## Why

A release's move is a command, not an agent's work: update@1 runs it
first and reaches an agent only where the move or the gate fails (223,
224, S130). flume runs statechart kinds, but binds the stage template
alone, so a kind holding a command stage refuses its launch, and a
launch carries no parameters for a command to read. The statement at
boomdrivedev/blueprints main says how: a command stage is a state whose
`machine` is `command`, run through the terminal as a program of a pane
or block by flume's launcher, which runs the command as its child in the
place, writes its exit and its output beside the start's record and
settles the place (223, 224, C.2, C.5, C.7).

## What Changes

- **flume binds the command template (223, C.2).** `command`, a
  statechart of flume's own beside `stage`: it asks for its stage's one
  session, the command's run, then ends `passed` or `failed`. A new
  place holds both templates' text.
- **A launch carries parameters (224).** `flume launch --params-file`
  gives a file read whole; flume keeps it with the place and writes it to
  the place's own folder as `params`, and reads no word of it.
- **A launch of a kind with a command stage is taken or refused (4,
  223, 224).** Refused where the command is missing or names a program
  by an absolute path or `..`, where the stage names a prompt, agents or
  a deliverable other than its commits, where its `on_fail` names no
  stage-template state, where a stage sends a place back to a command
  stage, and where the command asks for `{{params}}` and the launch
  gives none.
- **A command stage runs as a program, never an agent (223, C.5).** Its
  one session takes no room, no account and no points and has no
  prompt; its start is recorded with a fresh identity and secret, and
  the terminal's new `run` opens a pane or block running a link named
  for the place's kind that leads to flume's launcher. herdr runs it by
  `pane run`, never `agent start`; Rex runs it as any program, waiting
  on no Program Status; the printed command prints it.
- **The launcher runs a command start's command as its child (223,
  C.2).** `flume ack launched` checks the start as for an agent, then,
  for a start its record marks a command's, runs the program as its
  child in the place, shows and keeps its output whole in
  `ran/<identity>/output.log`, writes `ran/<identity>/exit.json`, and
  starts `flume settle <place>`.
- **Its end is read from its recorded exit (25, 223; core.9).** A run
  is begun once its launcher's word stands, delivered once it exited 0
  with a commit made since it started, and failed where it exited
  otherwise, committed nothing, began otherwise than recorded, or its
  terminal reads it stopped with no exit recorded. A failure is an end
  delivered to the place's recipients like any stage's, and the end
  carries the command's exit and where its output lies, which status
  gives.
- **A failure is taken on as the kind says (112, 223).** `stage.end`
  reads `failed` while the stage's failures taken on are under its
  bound; the kind's transition takes it on, and the next stage that
  runs an agent starts with the failure as its words. Past the bound
  the stage keeps working and the place waits on its caller.
- **A retry runs a command again; words are refused (49, 223).** A
  retry naming the command stage by its path, or its session, runs its
  command again from the place's head under the next start; one with
  words is refused naming the stage. A caller's stop ends a run that
  works, with no force, since it is no agent (51).
- **core.9 is amended** for a command stage's end, which is read from
  its launcher's recorded exit and the place's branch.
- **The suite builds 223, 224 and S130.**

## Capabilities

### New Capabilities

- `command-stages`: a statechart kind's command stage: the command
  template, a launch's parameters, the run through the terminal and the
  launcher, its end, a failure taken on, a retry and status.

### Modified Capabilities

- `statechart-kinds`: the templates flume binds are `stage` and
  `command`.
- `command-line`: `flume launch --params-file`.
- `seam-contracts`: the store reads a file of a place's folder, the
  terminal seam runs a command, and the host names flume's launcher.
- `clause-coverage`: 223, 224 and S130 built.

## Impact

- `crates/flume-seams`: `ChartStage.command`, `ChartSession.command`,
  `Launch.params`, `PlaceRecord.params`, `StartRecord.command`,
  `StageEnd.ran` with `Ran` and `Exited`, `StageState::Failed`, the
  paths `params_path`, `launched_word`, `ran_exit` and `ran_output`,
  `Store::read_file`, `Terminal::run`, `Host::launcher`.
- `crates/flume-core`: `command@1.toml`, `atoms.rs`, `command.rs`;
  `launch.rs`, `chart.rs`, `start.rs`, `acts.rs` and `held.rs`.
- `crates/flume`: the weir kinds profile reads `command`; the herdr,
  Rex and printed terminals' `run`; the launcher's child run; the state
  directory's `read_file`; `--params-file`; the real host's launcher.
- `crates/flume-fakes`: the fake store's `read_file`, the fake
  terminal's `run`, the fake host's launcher.
- `crates/flume-conformance`: the command stages' scenarios; the clause
  list.
- `docs/architecture/`: core.9 amended.
- `site/`: the kinds guide's command stages.
