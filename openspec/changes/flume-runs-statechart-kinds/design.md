# Design

## Context

See proposal.md for why. The statement is read at
boomdrivedev/blueprints 3a3b287, on its branch
`design/flume-runs-statechart-kinds`: section 3, section 9, 4, 56, 130,
175, 176, 178 and C.2 there say what this change builds, and record
0079 (proposed) holds the choices below. The rules that bind are
`docs/architecture/core.md`'s; no area page exists. This design relies
on core.1, core.2, core.3, core.4, core.9, core.10, core.13, core.14,
core.16, core.17, core.22, core.41, core.42, core.52 and core.54, and
departs from none.

Today:

- The workspace pins weir at 52b9552 (`Cargo.toml`), whose
  `weir-kinds` reads no statechart kind and which has no engine. weir's
  main at cc9d41f carries both: `weir-kinds` returns a declaration's
  statechart kinds in `Declaration.statecharts` (`StatechartKind`: the
  engine's `Statechart`, its title, label, branch, lands, landing fix
  and each state's five stage parameters by the state's path), refuses
  a package of one, and adds `hibernate` to a simple stage; `weir-engine`
  gives `start`, `tick`, `decisions` and `names` over `Statechart`,
  `ObjectState` and `Snapshot`, with no store, clock or evidence of its
  own, its times `time::OffsetDateTime` and its values `toml`'s.
- A stage is an index: `HeldState.stages: Vec<StageHeld>` runs
  position for position with `place.kind.stages`, `held.current` names
  the stage most recently started, and 36 reads of
  `held.place.kind.stages[index]` across `flume-core` give a stage's
  declaration. `start.rs`'s `claim` refuses a start while any start of
  the place is live, and `settle.rs`'s `step` dispatches on the current
  stage's state alone.
- `launch.rs`'s `runs` refuses a kind that asks for more than flume
  runs; `base.rs`'s `base_place` is a kind that lands with no stages.

## Goals / Non-Goals

**Goals:**

- A statechart kind runs on weir's engine with flume deciding no
  transition, and every rule flume holds for an agent holds for each
  session, by the code that holds it today.
- A simple kind, and a held state written before, run exactly as
  before (177, S77).

**Non-Goals:**

- A statechart kind's landing fix (175): refused until a change of its
  own designs how it lands beside the place's worktree.
- The command template and a launch's parameters (223, 224): the
  update ladder's change, after this one.
- Templates other than `stage`, rules other than `every-file`, effects
  a kind names, and another statechart run inside a state: each is
  refused as unbound (4).
- A package of a statechart kind: weir's read-package refuses it.
- A Claude Code or herdr plugin command for `flume answer`.

## Decisions

### 1. weir's four crates at one revision

`weir-kinds`, `weir-delivery` and `weir-pack` move from 52b9552 to
cc9d41f, and `weir-engine` joins them at that revision, in the
workspace's `[workspace.dependencies]` (core.22, core.42, C.2). `time`
0.3, the version `Cargo.lock` already holds through weir, is added for
the engine's times. `flume-seams` and `flume-core` take `weir-engine`,
as they take `weir-delivery` (core.1 names no family crate); `flume`
and `flume-fakes` take `weir-kinds`. Not taken: `weir_kinds::weir_engine`
re-exported in place of a dependency, which hides the pin `Cargo.toml`
states.

### 2. A statechart kind is the seam's `Kind` with its statechart

`flume-seams/src/kinds.rs`'s `Kind` gains `statechart:
Option<KindStatechart>`, left out of JSON while none:

```rust
pub struct KindStatechart {
    pub title: String,
    /// The kind's table spelled as a declaration of it alone, as the
    /// kinds profile read it (21).
    pub text: String,
    /// Each state that gives stage parameters, by its path.
    pub stages: BTreeMap<String, ChartStage>,
}
pub struct ChartStage {
    pub agents: ChartAgents,           // Entries(Vec<Settings>) | Rule { rule, pattern }
    pub join: Join,                    // All | Any | Count(u32); All where left out
    pub deliverables: Vec<Deliverable>,
    pub prompt: Option<String>,        // the text, read beside the declaration
    pub on_fail: Option<String>,
    pub send_back_limit: Option<u32>,
}
```

Its `stages` is empty, `starts_unready` false and `agents` default; its
`label`, `branch_prefix`, `lands`, `fix` and `fix_limit` are the
statechart kind's own. `base_place` becomes a kind that lands with no
stages and no statechart. The kinds seam gains
`read_statechart(&self, kind: &KindStatechart, template: &str) ->
Result<Charts, String>`, `Charts { chart: Statechart, templates:
BTreeMap<String, Statechart> }`, defaulting to a refusal (core.2,
core.3). Not taken: `Kind` an enum of the two formats, as weir's
`a-statechart-kind-is-read-in-weirs-kind-format` did not take it: every
reader of simple kinds would change. Not taken: the engine's types
given serde in flume, which would define the format's fields in flume
(core.4); the text is read back instead, and weir's 81 makes the same
text give the same statechart.

### 3. The kinds profile maps a statechart kind

`profiles/kinds/weir/mod.rs`'s `read` maps each of
`declaration.statecharts` by a new `map_statechart`: the kind's table,
taken from the parsed document, re-spelled by `toml` as `[kinds.<name>]`
alone; its title, label, branch, lands; its landing fix spelled into
`Kind.fix`; and each `params` path into a `ChartStage`, its entries'
settings by `map_agent`, its deliverables by `map_deliverable`, and its
`prompt` read from the engine state's own `params` table at that path,
a string read by the source's `beside` as a simple stage's prompt is,
`<at>.params.prompt` named where it cannot be read. Every other key of
`params` is ignored (23). `read_statechart` reads `text` and the
template's text by `weir_kinds::read_kinds`, each holding exactly one
statechart kind, and refuses naming the reason otherwise. A package of
one is refused by weir's read-package, as weir's design 12 says.

### 4. The stage template is the core's, its text in weir's format

`crates/flume-core/src/stage@1.toml` holds `[kinds.stage]`, version 1,
title "A stage's sessions, joined":

```toml
[kinds.stage.regions.run]
initial = "working"

[kinds.stage.regions.run.states.working]
transitions = [
  { when = { ev = "stage.end", is = "passed" }, to = "passed" },
  { when = { ev = "stage.end", is = "sent-back" }, to = "sent-back" },
  { when = { ev = "stage.end", is = "stopped" }, to = "stopped" },
  { when = { always = true }, to = "working", effects = [{ do = "start_sessions" }] },
]
```

with `passed`, `sent-back` and `stopped` final. The core includes it as
`STAGE_TEMPLATE` and names its atoms in `atoms.rs`: the template's
`start_sessions`, proved by `stage.started`, and `stage.end`; a kind's
`region.entered`, `place.submission` and `place.submissions`; the rule
`every-file`; the template `stage`. The self-transition is the engine's
idiom for asking until proven: once `stage.started` holds it plans
nothing, and the engine records no firing (weir 96). The text is the
core's because its atoms are the core's bindings (178); it is read
only through the kinds seam (core.4). Not taken: the text in the kinds
profile's `own/`, where the conformance suite, which `flume` depends
on, cannot read it.

### 5. The launch: what is refused, what is made

`runs` refuses a statechart kind, before anything is made (4, 175,
178):

- one naming a landing fix, naming `landing_fix`;
- by `weir_engine::names` over the chart read back: a `machine` other
  than `stage`, an `ev` other than the kind's three, or any `do`;
- for each state whose `machine` is `stage`, by its `ChartStage`: no
  stage parameters or no `prompt`; no deliverable, or one
  `Unreadable`; a rule other than `every-file`, or one with no pattern;
  `{{file}}` in a deliverable while its agents are entries; a count
  above its entries.

`make_place` then makes the place as for a simple kind, holds the kind,
the template's text and `weir_engine::start`'s state at the host's
clock (`ChartHeld`, decision 6), and settles the place once in place of
`start_stage(0)`. A launch's holds name a stage by its state's path,
checked against the chart's stage paths.

### 6. The held state: sessions are stages of the place

`flume-seams/src/held.rs`:

```rust
pub struct StageHeld { /* as today */ pub session: Option<SessionHeld> }
pub struct SessionHeld {
    pub stage: String,        // the state's path, `stages.test`
    pub entry: u32,           // from 1
    pub n: u32,               // from 1
    pub file: Option<String>, // the file its rule gave it
    pub declared: Stage,      // its declaration: the stage's prompt and
                              // deliverables with its parts filled, its
                              // entry's settings as its one agent
}
pub struct ChartHeld {
    pub state: ObjectHeld,                      // the engine's ObjectState, times in Seconds
    pub answers: Vec<AnswerGiven>,              // id `<region>#<n>`, region, answer, by, at
    pub stages: BTreeMap<String, ChartStageHeld>, // by path: recorded entry, send_backs
    pub words: Option<String>,                  // for the next stage's sessions
    pub over: bool,                             // every top region final at the last tick
}
```

`HeldState.chart: Option<ChartHeld>`, left out while none, and
`HeldState::declared(index) -> &Stage` gives a session's `declared` or
`place.kind.stages[index]`. A session's `StageHeld.name` is
`<path>#<entry>/<n>`, so `Identity.stage` names it (33), `index_of`
finds it, and holds, retries and status name it as a stage. Not taken:
a separate list of sessions beside the stages, which every per-stage
function would have to learn to walk.

### 7. The per-stage code reads through `declared` and takes an index

Every read of `held.place.kind.stages[index]` in `flume-core` reads
`held.declared(index)`. `deliver`, `begun_after`, `read_end`,
`found_work` and `close_session` gain index-taking forms the simple
step calls with `held.current`. `claim`'s refusal of a second live
start holds for a simple kind's place, and for a statechart place
refuses only a second live start of the same session (core.10).
`Starts` gains `live_starts`; `due` submits only once none is live
(111). Neither refactor changes what a simple kind's place does.

### 8. One tick a settle

`crates/flume-core/src/chart.rs` holds the statechart's step, which
`step` runs for a place whose kind has a statechart, after the
submission and in place of the current stage's dispatch, at most once a
settle (176):

1. Each session with a live or unread start is stepped by its index as
   the current stage is today.
2. The branch's head is read once (`History::head`). Evidence is served
   at it: at each live region path `region.entered`, its entered time;
   at each live stage's template region `<path>.run`, `stage.started`,
   whether the stage's current entry is recorded, and `stage.end`
   (decision 10); at the object, `place.submission` and
   `place.submissions` (decision 11). The oldest answer not applied is
   the snapshot's one response (decision 12).
3. The kinds seam reads the held chart and template back; `tick` runs
   with the proofs `{ start_sessions: stage.started }`.
4. One `update` writes the tick's next state and performs each planned
   effect in order: `start_sessions` at `<path>.run` records the entry
   and its sessions (decision 9); a fired move of `<path>.run` to
   `sent-back` adds one to that stage's `send_backs` and keeps the found
   words in `words`; `over` is set from the top regions.
5. Each session recorded is started by `start_stage(index, words)`, the
   held words given to the first entry recorded after them and then
   cleared, and each live session of a stage no longer at `working` is
   ended once quiet (31).

An entry is the engine's own count: one more than the instances set
aside under `<path>#`, so a settle repeated reads the same entry and
records nothing twice (74, core.17). Not taken: ticking until nothing
fires within a settle, which 176 rules out; a stage boundary takes
three settles.

### 9. A stage's sessions

For entries, one session for each, its entry's settings its own. For
`every-file`, `History::files(repository, head)` gives the revision's
files, and a glob of `*` within a path's part, `**` across parts and
`?` one character, matched by `crates/flume-core/src/glob.rs`, picks
them in path order. Each session's `declared` holds the stage's prompt,
its deliverables with `{{session}}` and `{{file}}` filled, and one
agent; prompts gain the parts `Session` and `File`, refused as any part
a stage cannot fill (27). Layers naming a stage reach no session, its
entry's settings standing as a stage's own and the layers giving every
stage's over them (32). Not taken: a crate for globbing, a dependency
for twenty lines.

### 10. A stage's end

`stage.end` for an entry, from its sessions' held states: `sent-back`
where one ended short having found work and `send_backs` is under
`send_back_limit`, flume's `defaults.send_backs` where it gives none;
else `stopped` where one found work, or where its rule gave no session,
or where too few sessions are left that have not ended for its join;
else `passed` where its join is met by sessions delivered; else none.
A session's found words are the stage's `words`. A session that stops
short needing an answer, is lost, stuck or never begun keeps the stage
working while the place waits on its caller for it, as today (42 to
45).

### 11. The run's end and the submission

At each tick `over` reads every top region at a final state. Turned
true, a kind that lands has `submission.due` set, and the existing
submission submits once no session is live (111); one that does not
land waits on its caller, or ends done for a role's run (114, core.28).
`verdict_at` for a statechart place records the outcome and starts no
fix round. `place.submission` is `open` while a submission is in
flight, else its outcome's word (`landed`, `failed`, `conflict`,
`withdrawn`, `refused`), none before the first; `place.submissions` is
`submission.submits`. The merge queue's words go to `words`. A tick
that leaves the run over after a failure leaves the place waiting on
its caller with them, and a retry with no stage sets `due` again
(112, 130).

### 12. Decisions and `flume answer`

`weir_engine::decisions` gives the decisions standing; `waiting_on`
gains `Waits::Decision`, its stage the region and its words the
decision's kind and answers, so the stall and its deliveries follow as
for any wait (130). `Engine::answer_decision(place, region, answer,
by)` in `acts.rs` takes an answer by compare-and-set only while a decision
stands at that region and no answer given there is unapplied, as
`AnswerGiven { id: "<region>#<n>", .. }`, then settles the place
(core.54). One answer a tick, the oldest, so a tick never holds two.
`flume answer <place> <region> <answer>` calls it as its caller.

### 13. Status and the list

`PlaceStatus` gains `statechart: Option<ChartStatus>` (regions with
state, since and final; counters; over; decisions; answers with
applied; stages with entry, join, send-backs and session names) and
`agents: Vec<AgentStatus>`, each live session's agent, both left out
while empty (core.14). Sessions are `stages` entries as any stage. The
place's line and `Listed.stage` name the live stages' paths, joined
(core.52). The person's lines in `print.rs` give them.

### 14. The statement's revision

`clauses.rs` reads 3a3b287: `REVISION`, no clause added or removed, and
`STATECHART` gone from `unbuilt`, its scenarios citing 4, 56, 130 and
175 to 178, S76 and S77 (core.19).

## Risks / Trade-offs

- [A stage boundary takes three settles] → each is a quiet agent's
  settle or the watch's bound apart; the watch's bound sets the pace.
- [Two decisions standing at once may take each other's answer, the
  engine reading responses for the object whole] → one answer a tick,
  the oldest; record 0079 says so, and a kind's author keeps answers of
  decisions that stand together apart.
- [A deliverable naming no part reads delivered for every session
  started before it was made] → `{{session}}` and `{{file}}`, and the
  site says so.
- [weir's newer `weir-kinds` adds `hibernate` to a simple stage] →
  task 1.1 builds every use, and the profile maps it nowhere until
  record 0069's change.
- [A held statechart read back by a later weir whose format moved] →
  the same pin reads kinds and ticks them (core.42), and a pin's move
  is a chore whose tests read held texts back.

## Migration Plan

A held state written before reads with no statechart, no session and no
engine state, and runs by flume's own rules (S77). Nothing is migrated.
A repository whose declaration holds a statechart kind, refused today,
reads once this lands. Rollback is the commit's revert, once no place
of a statechart kind stands.
