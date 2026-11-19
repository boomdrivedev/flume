# statechart-kinds Specification

## Purpose
flume launching and running a statechart kind on weir's engine beside
simple kinds: what a launch of one is refused for, the stage template
flume binds and its sessions, one tick a settle over evidence read at
one head, a stage's end, the run's end and its submission, and the
decisions a caller answers (4, 175 to 178, S76, S77).

## Requirements

### Requirement: A launch of a statechart kind is taken

A launch of a statechart kind from a repository's declaration or a
launch's SHALL make its place, keep the kind and the stage template as
the launch read them, record its statechart's first state as the engine
starts it, and settle the place once, so a first stage's sessions start
at the launch (2, 21, 175, 176).

#### Scenario: A statechart kind launched

- **WHEN** a repository declares a statechart kind whose one region starts at a state running the stage template with one agent, and it is launched
- **THEN** the launch makes the place, its held state holds the kind's statechart and the stage template's text, its region holds its initial state, and the stage's one session has started (2, 21, 175, 176)

### Requirement: A launch naming what flume does not bind is refused

A launch of a statechart kind SHALL be refused, naming it and making
nothing, where its statechart names a template other than `stage` and
`command`, a rule other than `every-file`, an evidence other than
`region.entered`, `place.submission` and `place.submissions`, or any
effect (4, 175, 178, 223).

#### Scenario: A template flume does not bind

- **WHEN** a statechart kind's state runs `session`
- **THEN** its launch is refused naming the template, and the fakes record no make and no held state (4, 175)

#### Scenario: An atom flume does not bind

- **WHEN** a statechart kind's transition reads `ev = "lamp.switch"`, and another's performs `do = "start_sessions"`
- **THEN** each launch is refused naming the atom, and nothing is made (4, 178)

#### Scenario: A rule flume does not bind

- **WHEN** a stage-template state's agents give `rule = "every-persona-definition"`
- **THEN** its launch is refused naming the rule and the state (4, 175)

### Requirement: A stage flume cannot run is refused

A launch of a statechart kind SHALL be refused, naming the state and
making nothing, where a state that runs the stage template gives no
`prompt`, no deliverable or one flume cannot read, names `{{file}}` in a
deliverable while its agents are a list, or joins by a count above the
agents it lists (4, 175, C.2).

#### Scenario: A stage with no prompt

- **WHEN** a stage-template state's `params` give agents and deliverables and no `prompt`
- **THEN** its launch is refused naming the state and its prompt (4, C.2)

#### Scenario: A file no rule gives

- **WHEN** a stage-template state lists two agents and delivers `committed:out/{{file}}`
- **THEN** its launch is refused naming the state and the part (4, C.2)

#### Scenario: A count above the agents listed

- **WHEN** a stage-template state lists two agents and joins by `count:3`
- **THEN** its launch is refused naming the state and its join (4, 175)

### Requirement: A statechart kind's landing fix is refused

A launch of a statechart kind that names its landing fix SHALL be
refused, naming the kind and its `landing_fix`, until how that fix lands
beside the statechart's own place is designed (4, 175).

#### Scenario: A landing fix named

- **WHEN** a statechart kind that lands names `landing_fix = "flume/fix@1"`
- **THEN** its launch is refused naming the field, and nothing is made (4, 175)

### Requirement: The stage template is flume's own

flume SHALL bind one template, `stage`, a statechart of its own in
weir's format, whose region `run` starts at `working`, plans
`start_sessions` there until `stage.started` proves it, and moves to the
final state `passed`, `sent-back` or `stopped` that `stage.end` names
(175, 178, C.2).

#### Scenario: The template's ends

- **WHEN** a stage's evidence reads `stage.end` as `passed`, then in another place as `sent-back`, then in a third as `stopped`
- **THEN** each stage's region `run` moves to the final state of that name at that settle, and the kind's `{ final = "<end>" }` transitions read it at the next (175, 176)

### Requirement: Each settle ticks a statechart place once

A settle of a place of a statechart kind SHALL step its sessions, read
the place's branch's head once, serve every evidence at it, tick its
statechart once, write the next state by compare-and-set with each
effect the tick planned performed, and step the sessions those effects
started; flume SHALL move no region itself (176, core.42).

#### Scenario: One tick over one head

- **WHEN** a statechart place is settled twice with nothing changed between
- **THEN** the first settle's tick fires what it fires, and the second fires nothing and performs no effect, since every effect's proof is present (74, 176)

#### Scenario: A sibling sees a move at the next settle

- **WHEN** two regions run side by side and one reads the other by `{ region = { name, in } }` as the other moves
- **THEN** the reader moves at the settle after the one the other moved in (176)

### Requirement: A stage's sessions

`start_sessions` SHALL record one session for each agent a stage lists,
with its settings, or, for `every-file`, one for each file of the
branch at the settle's head matching its glob, numbered from 1 in that
order, then start each; a session SHALL be named `<path>#<entry>/<n>`,
its entry counting the stage's entries from 1 (30, 33, 175, C.2).

#### Scenario: One session per persona

- **WHEN** a stage's agents are `{ rule = "every-file", pattern = "personas/*.md" }` and the branch holds `personas/a.md`, `personas/b.md` and `notes/c.md`
- **THEN** the stage records two sessions, `stages.test#1/1` for `personas/a.md` and `stages.test#1/2` for `personas/b.md`, each starting one fresh agent whose identity names its session (30, 33, 175)

#### Scenario: A stage entered again

- **WHEN** a stage sent back is entered again
- **THEN** its sessions are recorded afresh under entry 2, and those of entry 1 stay in status as they ended (175, 176)

### Requirement: A session runs by flume's own rules

Each session SHALL start, wait for room, take an account, be prompted,
end, stop short, be answered, be held, be retried and be reported as a
simple kind's stage is, under the layers that give every stage's
settings with its entry's settings as its own; a place of a statechart
kind SHALL run at most one agent per session (30, 32, 42, 45, 49, core.10).

#### Scenario: Several agents in one place

- **WHEN** a stage of three sessions starts on a host with room for two
- **THEN** two agents run in the place and the third waits for room, starting once one of the two ends (30, 70)

#### Scenario: A session asks and is answered

- **WHEN** a session stops short needing an answer and its caller tells it the answer naming the stop-short's key
- **THEN** the place waits on that answer, the answer reaches the session kept open, and the other sessions work on (42, 198)

#### Scenario: A lost session retried

- **WHEN** a session's agent goes with nothing delivered and nothing said, and its caller retries naming the session
- **THEN** the place waits on its caller until the retry, which starts the session again with one fresh agent (45, 49)

### Requirement: A session's deliverables are its stage's

A session SHALL deliver its stage's deliverables, each read as 24 and
25 read a stage's, made since that session started, with `{{session}}`
and `{{file}}` in a path filled with its number and its rule's file; a
deliverable naming neither SHALL be read the same for every session
(24, 25, 175, C.2).

#### Scenario: Each persona's own report

- **WHEN** a stage of two sessions delivers `committed:results/{{file}}` and only `results/personas/a.md` is committed
- **THEN** the session for `personas/a.md` reads delivered and the other does not (24, 175)

### Requirement: A stage's end

A stage SHALL end passed once its join is met by sessions that
delivered; sent back once a session found work while the stage was sent
back fewer times than its bound; stopped once a session found work past
it, or its sessions are too few for its join. A session live once its
stage ended SHALL be ended once quiet (31, 112, 131, 175).

#### Scenario: A join of all, of any and of a count

- **WHEN** three sessions run, two deliver and the third works on, once under `all`, once under `any` and once under `count:2`
- **THEN** under `all` the stage has not ended; under `any` it passed at the first delivery; under `count:2` it passed at the second; and each passed stage's working session is ended once quiet (31, 175)

#### Scenario: Found work sends the stage back with its words

- **WHEN** a session stops short having found work and its stage has been sent back fewer times than its `max_send_backs`
- **THEN** the stage ends sent back, and the sessions of the stage the kind enters next are prompted with the found words (112, 131, 175)

#### Scenario: Past its bound

- **WHEN** a session finds work after its stage was sent back as often as its bound
- **THEN** the stage ends stopped (112, 175)

#### Scenario: A rule that matches no file

- **WHEN** a stage's rule matches no file at the settle's head
- **THEN** the stage ends stopped, and status names the rule and its pattern (175)

### Requirement: The run's end submits or waits

Once every top region of a statechart holds a final state and no
session of its place is live, a place whose kind lands SHALL be
submitted as after a last stage's delivery, and one whose kind does not
land SHALL wait on its caller, or end done for a role's run (111, 114,
130, 176).

#### Scenario: Persona passes and is submitted

- **WHEN** a kind's build stage delivers and its test stage passes, its top region reaching `passed`, and the kind lands
- **THEN** flume submits the place's branch through the merge queue once, as after a last stage's delivery (111, 176, S76)

#### Scenario: A run that does not land

- **WHEN** a statechart kind that does not land reaches a final state in each top region
- **THEN** the place waits on its caller, whose done makes its work over (114, 130)

### Requirement: The submission's outcome is evidence

`place.submission` SHALL read the outcome of the place's newest
submission as flume confirmed it, and `place.submissions` how many times
flume submitted it; a failure no transition takes SHALL leave the place
waiting on its caller, whose retry submits again (112, 130, 133, 176).

#### Scenario: A failure taken back to a stage

- **WHEN** a kind's final state `passed` has `{ when = { ev = "place.submission", is = "failed" }, to = "build" }` and the merge queue fails the submission
- **THEN** the next settle moves the region to `build`, and the stage's sessions are prompted with the merge queue's words (112, 176)

#### Scenario: A failure nothing takes

- **WHEN** a kind's final states read no submission and the merge queue fails it
- **THEN** the place waits on its caller with the merge queue's words, and the caller's retry submits it again (112, 130)

### Requirement: A decision waits on its caller and is answered once

While a decision stands the place SHALL wait on its caller. `flume
answer` SHALL take an answer naming the region whose state carries a
standing decision, and refuse one naming no such region or given while
an earlier answer to it is unapplied; each tick SHALL be given the
oldest answer unapplied, applied once (74, 130, 176, core.54).

#### Scenario: A stopped stage's decision answered

- **WHEN** a kind's region reaches `stopped`, whose decision answers `retry` or `drop`, and the caller answers `retry` naming the region
- **THEN** the place waited on its caller from the decision's standing, the next settle applies the answer and moves the region to the stage `retry` names, and status gives the answer applied (130, 176)

#### Scenario: An answer refused

- **WHEN** a caller answers a region whose state carries no decision, then answers a standing decision twice before a settle
- **THEN** the first is refused naming the region, the second taken, and the third refused naming the answer taken, who gave it and when (176, core.54)

### Requirement: A simple kind's place runs as before

A place of a simple kind SHALL run by flume's own rules whatever
statechart kinds do, a held state written before this change reading
with no statechart, and nothing of it ticked on the engine (21, 177,
core.41).

#### Scenario: A change@2 place across the release

- **WHEN** a place of flume's own change@2, its held state written before this change, is settled after it while its code stage repeats
- **THEN** its next repetition, its verify stage and its submission run as before, and no tick is made for it (21, 140, 177, S77)
