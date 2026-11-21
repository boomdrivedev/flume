# flume runs statechart kinds

## Why

Records 0028 and 0029 bring statechart kinds beside simple kinds and
run them on the family's engine, and weir now reads a statechart kind
in its kind format and carries the engine (weir 94 to 99, at cc9d41f).
flume still pins a weir that reads neither, and a statechart kind in a
repository's declaration refuses every launch there. This change runs a
statechart kind on weir's engine, each settle ticking it once over
evidence read at one head, beside simple kinds run as today (4, 175 to
178, S76, S77; record 0079, proposed).

## What Changes

- **weir at one revision (core.22, core.42, C.2).** weir-kinds,
  weir-delivery and weir-pack move to weir's main at cc9d41f, and
  weir-engine is taken from that same revision.
- **A statechart kind read and held (19, 21, 84, 85, 175).** The kinds
  profile reads a repository's and a launch's statechart kinds through
  weir's crate into the seam's `Kind`, which gains its statechart: its
  title, its declaration's text and each stage-template state's
  parameters. A place keeps it, with the stage template's text, as its
  launch read them.
- **flume binds the stage template (175, 178, C.2).** `stage`, a
  statechart of flume's own in weir's format: it asks for its stage's
  sessions, then ends `passed`, `sent-back` or `stopped`. Its sessions
  are listed agents, or one per file the rule `every-file` matches at
  the settle's head; they share its prompt, and its deliverables may
  name `{{session}}` or `{{file}}`.
- **A launch of a statechart kind is taken (4).** It is refused only
  for a template, rule or atom flume does not bind, a stage with no
  prompt or no deliverable, a `{{file}}` where no rule gives one, a
  count above the agents listed, or a landing fix, which waits on a
  change of its own (175).
- **Each settle ticks once (176).** A settle steps the place's
  sessions, reads the branch's head once, serves the evidence the
  statechart and the template read, ticks once and performs what the
  tick plans: `start_sessions` records the stage's sessions and starts
  each.
- **A session is one agent's life by flume's own rules (30, 33, 175).**
  Each session is held as a stage of the place, named
  `<path>#<entry>/<n>`, so starts, prompts, ends, stop-shorts, answers,
  holds, retries and status serve it as they serve a simple kind's
  stage. A statechart place may run several agents, one per session.
- **The run's end is the submission (111, 114, 130, 176).** Once every
  top region holds a final state, a kind that lands is submitted and one
  that does not waits on its caller. The outcome is evidence: a
  transition out of a final state may take a failure back to a stage;
  where none does, the place waits and a retry submits again.
- **Decisions are answered by `flume answer` (130, 176).** A place
  waits on its caller while a decision stands; `flume answer <place>
  <region> <answer>` gives it once, and the engine applies it once.
- **Status and the list give a statechart place (56).** Its regions,
  counters, decisions, answers and stages with their sessions; the list
  names its live stages. Status's JSON only gains fields (core.14).
- **The suite reads the statement at 3a3b287 (core.19).** 175 to 178,
  S76 and S77 are built and cited.
- **The site** documents statechart kinds, the stage template and
  `flume answer` (core.35).

## Capabilities

### New Capabilities

- `statechart-kinds`: flume launching and running a statechart kind on
  weir's engine: what a launch of one is refused for, the stage
  template and its sessions, one tick a settle over evidence at one
  head, a stage's end, the run's end and its submission, decisions and
  their answers, and a simple kind's place run as before.

### Modified Capabilities

- `kinds-profile`: a statechart kind read through weir's crate at the
  revision the workspace pins, its prompt beside its declaration, and
  its held text read back into the engine's types.
- `seam-contracts`: the kinds seam reads a held statechart kind, git's
  history gives a revision's files, and the types of a statechart kind,
  a session and a statechart place's held state.
- `seam-fakes`: the fake kinds reads a statechart kind's held text
  through weir's crate, and the fake git gives a revision's files.
- `held-state`: a statechart place's engine state, its stages' entries
  and its answers are held in its one document.
- `place-status`: status gives a statechart place's statechart, its
  decisions and answers, its stages and each live session's agent.
- `the-list`: a statechart place's line names its live stages.
- `command-line`: `flume answer` answers a decision.
- `clause-coverage`: the suite reads the statement at 3a3b287, 175 to
  178, S76 and S77 built.

## Impact

- `Cargo.toml`: weir's four crates at cc9d41f; `time` 0.3.
- `crates/flume-seams`: `kinds.rs` (`KindStatechart`, `ChartStage`,
  `Kind.statechart`, the kinds seam's `read_statechart`), `held.rs`
  (`SessionHeld`, `ChartHeld`, `HeldState::declared`), `history.rs`
  (`files`), `round_trip.rs`.
- `crates/flume-core`: `stage@1.toml` and `atoms.rs`; `launch.rs`'s
  `runs` and the place's first state; `chart.rs`, the tick and its
  effects; the per-stage code reading a stage's declaration through
  `declared` and taking a stage's index; `submit.rs`, `status.rs`,
  `line.rs`, `list.rs` and `acts.rs` (`answer`).
- `crates/flume`: the weir kinds profile's statechart mapping and
  `read_statechart`, git's `files`, `flume answer`, the person's lines.
- `crates/flume-fakes`: the fake kinds' `read_statechart` through
  weir-kinds, the fake git's `files`.
- `crates/flume-conformance`: statechart scenarios, S76 and S77; the
  clause list at 3a3b287.
- `site/`: the kinds guide and the configuration, state and commands
  references.
- No change to weir, sluice or herdr. boomdrivedev/blueprints' branch
  `design/flume-runs-statechart-kinds` at 3a3b287 holds the statement.
