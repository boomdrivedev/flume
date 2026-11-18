---
number: 79
title: flume runs a statechart kind on weir's engine, its stage template running each session by flume's own rules
status: proposed
date: 2026-11-18
decision-makers:
- the brief of flume-runs-statechart-kinds
- the construction agent of flume-runs-statechart-kinds
---

# flume runs a statechart kind on weir's engine, its stage template running each session by flume's own rules

Status: proposed. The direction is the change's brief: "flume runs a statechart kind on weir's engine, taken at the weir revision it pins weir-kinds at, each settle ticking it once over evidence read at one head, beside simple kinds run as today, and stops refusing a launch of a statechart kind", drawn from records 0028 and 0029, both the user's, and queued on afterthought/crew-state's plan. Record 0029 left how each part fits to the build (ledger item T1.c). The statement's section 3, section 9, 4, 56, 130, 175, 176, 178 and C.2, amended on boomdrivedev/blueprints' branch `design/flume-runs-statechart-kinds` at 3a3b287, state what this record decides. Every choice beyond the brief is the construction agent's, made on the rules written and cheap to change, and stays proposed until the operator's yes.

Deciders: the change's brief, which decided the direction; the construction agent of `flume-runs-statechart-kinds`, for the choices of its design's decisions 1 to 12.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da and at 3a3b287, clauses 4, 18 to 25, 30, 33, 42, 56, 74, 103, 111, 112, 114, 130, 131, 133, 175 to 178, 223, C.2, S76 and S77; `boomdrivedev/blueprints:design/weir/requirements.md` at 48a94da, clauses 94 to 99 and C.4; boomdrivedev/weir at cc9d41f, `crates/weir-engine` (`tick`, `start`, `decisions`, `names`, `ObjectState`, `Snapshot`; a template started afresh on each entry and set aside under its attempt; an effect planned only while its proof is absent, at the region where it is planned) and `crates/weir-kinds` (`StatechartKind`, `Params`, `ParamAgents`, `Join`; a package of a statechart kind refused, design 12 of weir's `a-statechart-kind-is-read-in-weirs-kind-format`); `agentplot/flywheel-next` `definitions/stage.yaml`, `session.yaml` and `place.yaml`, the original's three templates; records 0004, 0022, 0028, 0029 and 0066; rules core.1, core.2, core.4, core.9, core.10, core.16, core.17, core.22, core.41, core.42 and core.54; `crates/flume-core` (`settle.rs`'s `step`, `start.rs`'s `claim`, `end.rs`, `submit.rs`'s `fix_round`) and `crates/flume-seams/src/held.rs` (`HeldState.stages`, `StageHeld`, `StartHeld`), where a stage is an index into its kind's stages and a place has one live start.

## Context and problem statement

Records 0028 and 0029 bring statechart kinds beside simple kinds and run them on weir's engine, and weir now carries both: `weir-kinds` reads a `[kinds.<kind>]` table holding `regions` into the engine's `Statechart`, with each state's stage-template parameters, and `weir-engine` ticks one object's state over one snapshot, planning each effect while its proof is absent. Which templates and atoms there are, and what a template's parameters mean, are the reader's (weir 95, 98). flume's statement named three templates, stage, session and place, and left how a statechart kind's landing fix lands to the change that runs statechart kinds (175).

The original's stage template ran one session submachine per agent, and its session and place templates ran an agent's whole life and a worktree's: start, liveness, exit reports, rebases, merges and removal. flume already runs every one of those by its own rules for a simple kind's stage: an agent's start under room and accounts, its prompt read back, its quiet, its stop-short and its answer, its deliverable at one head, its kept-open and hibernating session, and a place's launch, readiness and tidy. The engine starts one statechart per state, so a stage's several sessions are no submachine of it.

## Considered options

1. **flume binds the stage template alone, each session run by flume's own rules for a stage's agent (chosen).** The stage template is a small statechart of flume's own: it asks for its stage's sessions until they are recorded and ends passed, sent back or stopped by its stage's end, which flume serves from its sessions' ends. Each session is held as a stage of the place, with the declaration its stage gives it, and flume's start, prompt, end, stop-short, answer and status code runs it as it runs a simple kind's stage.
2. The three templates, each session an object of its own ticked under a session template, its owner the stage reading it by `children`. flume's agent life would be written twice, once as flume's rules and once as atoms of a session template ticked beside them, the two kept in step at every settle; the engine's `children` guard cannot count sessions delivered, so a join by count still needs evidence of flume's; and a place template has nothing to run, since flume makes and tidies the place at its launch and its end, and rebases and merges nothing (core.5).
3. flume's own reading of a statechart kind, its stages walked by flume's code in place of the engine's. Rule core.42 and 176 rule it out.

## Decision outcome

Option 1.

1. **weir at one revision.** The workspace takes weir-kinds, weir-delivery, weir-pack and weir-engine from weir's main at cc9d41f, moved together (core.22, core.42, C.2).
2. **A statechart kind is a kind with its statechart.** The kinds seam's `Kind` gains its statechart: its title, the declaration that holds it as the kinds profile spelled it, and each state's stage parameters, mapped onto the seam's types. The place keeps it, and the stage template's text, as its launch read them (21), and the kinds seam reads both into the engine's types at each settle. A base place is a kind that lands with no stages and no statechart.
3. **The stage template, `stage`, is flume's own,** written in weir's format in `flume-core`, which holds its text and names its atoms; the kinds seam reads it. Its region `run` starts at `working`, plans `start_sessions` there until `stage.started` proves it, and ends `passed`, `sent-back` or `stopped` as `stage.end` says. The session and place templates are not bound: a session is one agent's life by flume's rules, and the place is the worktree flume makes and tidies. A kind naming another template is refused naming it (4).
4. **A session is held as a stage of the place.** `start_sessions` appends one stage to the place's held stages for each session its stage's agents give: its name `<path>#<entry>/<n>`, its declaration the stage's prompt and deliverables with its own parts filled and its entry's settings. flume's per-stage code reads a stage's declaration through the held state, which gives a session's or the kind's, so every start, prompt, end, stop-short, answer, hold, retry, telemetry and status of a stage serves a session unchanged. A place of a statechart kind may have several live agents, one to each session at most (core.10).
5. **The prompt and the parts.** A stage-template state gives `prompt`, the template's own parameter, read as a simple stage's prompt is. A session's prompt and its `committed:` and `report:` deliverables may name `{{session}}`, its number from 1, and `{{file}}`, the file its rule gave it, so each session's deliverable is its own; a deliverable naming no part is shared, made since each session started.
6. **The rule `every-file`.** One session for each file of the place's branch at the settle's head matching a glob, read from git's history beside the seams. flume binds no other rule.
7. **A stage's end.** Passed once its join is met by sessions that delivered; sent back once a session found work while the stage has been sent back fewer times than its `max_send_backs`, flume's bound where it gives none; stopped once a session found work past it, or once its sessions are too few for its join. The found words go to the sessions of the stage the kind enters next, as a fix round's words do (112, 131). A session still live once its stage ended is ended once quiet (31).
8. **The atoms.** The stage template's `start_sessions`, `stage.started` and `stage.end`; a kind's `region.entered`, `place.submission` and `place.submissions`. A kind names no effect. Each reads or writes the place's held state through the store, or reads the merge queue's record as flume confirmed it (133).
9. **One tick a settle.** A settle of a statechart place steps its sessions, reads the branch's head once, serves the evidence at it, ticks once, writes the next state by compare-and-set with the effects performed, and steps the sessions the effects started. A region's move is seen by its siblings at the next settle (weir 96).
10. **The run's end is the submission.** Once every top region holds a final state, a kind that lands is submitted as after a last stage's delivery, and one that does not land waits on its caller (111, 114). The submission's outcome is evidence: a final state's transition that reads a failed or conflicting outcome moves the run on, the merge queue's words given to the next stage's sessions; where none does, the place waits on its caller, whose retry submits again.
11. **A decision is answered by `flume answer`.** While a decision stands the place waits on its caller (130). `flume answer <place> <region> <answer>` records the answer while that decision stands and none given it is unapplied; each tick is given the oldest answer not yet applied, and the engine applies it once.
12. **Deferred.** A statechart kind that names a landing fix is refused naming it until a change of its own designs how the fix lands beside the place's worktree (175); a kind's own `agents` section and layers naming a stage do not reach a session, whose settings are its entry's under the layers that give every stage's.

### Consequences

- Good: every rule flume holds for an agent holds for a session, by the code that holds it today, and a simple kind runs as it did, its held state written before read unchanged (177, S77).
- Good: the kind's own transitions, final states, decisions and counters run on the engine alone; flume moves no region and decides no transition (core.42).
- Good: a kind routes its own failed submission back to a stage, bounded by `place.submissions`, as a simple kind's last stage names its fix round.
- Bad: a stage boundary takes three settles, one to enter the stage, one to ask for its sessions and one to read its end, each a quiet agent or the watch's bound apart.
- Bad: two decisions standing at once whose answers share words may each take an answer given the other, since the engine reads a response for the object whole; one answer is given a tick at a time.
- Bad: sessions share one worktree, so a deliverable that names no part reads delivered for every session started before it was made.
- Bad: a statechart kind that lands cannot name its landing fix yet.
- The statement's section 3, section 9, 4, 56, 130, 175, 176, 178 and C.2 say it at boomdrivedev/blueprints 3a3b287. Rules core.10, core.41 and core.42 hold as written; no rule changes.
