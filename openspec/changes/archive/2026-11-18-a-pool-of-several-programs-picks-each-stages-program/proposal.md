# A pool of several programs picks each stage's program

## Why

flume runs every program its terminal starts, each bound by its own
profile or the common profile, started through flume's launcher and
read from its terminal where its profile builds no part (170, 171,
174). A stream's pool may hold accounts of several programs (144), but
a stage whose layers name no program still runs flume's default,
`claude`, whatever its pool holds: an account of any other program is
never used by such a stage, which is every stage of flume's own kinds,
and a pool holding no `claude` account refuses them all. Clause 32
item 8 says flume's default names a program only for an agent with no
pool, and 173 that the account chosen names it otherwise.

Record 0026, accepted by the user on 2026-10-30, decided it (program
option 2): where no layer names a stage's program and its agent has a
pool, the account chosen names it, among the accounts whose program
can run the stage; with no pool, flume's default runs it as today
(144, 145, 147, 173). The bolt `flume-runs-every-agent-its-terminal-can-start`
builds record 0026 change by change (signal
`2026-11-18-boomdrive-ops-cb94852a/01`); this is its last change,
clause 173 and S71.

## What Changes

- **The account chosen names the program of a stage whose layers name
  none.** flume's default names a program only for an agent with no
  pool (32 item 8). With a pool, the stage's usable accounts are those
  whose program can run it: a profile binds the program, its mode is
  mapped, its model given or none needed, its effort taken or none
  given, and the own settings given for it taken or none given (144,
  169, 172, 173, 196, 197); the strategy chooses among them as it does
  today (145). Status gives the program from `account` (56; core.30).
- **The intent keeps the program its last agent ran.** A stage whose
  layers name no program takes the account of the intent's affinity
  for that program while it can run the stage and is usable, and
  otherwise goes where the strategy chooses, the affinity for the
  chosen program moving or being written as 147 says (147, S72's
  affinity).
- **A wait and a refusal read the accounts that can run the stage.** A
  stage whose layers name no program waits for room while its pool
  holds an account that can run it, spent or full, and is refused at
  its launch or its start where the pool holds none, naming the stage,
  the pool and why each of its programs cannot run it (4, 32, 145;
  core.8, core.32).
- **A start's settings are resolved for the account's program.** Once
  the account is chosen, the stage's settings are resolved with its
  program, so the mode, the model, the effort and the own settings the
  start passes are that program's entries, and its terminal is asked
  whether it starts that program under the account's environment, as
  a program a layer names is (32, C.5).
- **A conversation stays with its program.** A lost or hibernated
  start's account is taken first while it can run the stage and is
  usable; a spent agent's stage may move to an account of another
  program, and then starts afresh, since no profile carries a
  conversation to another program (146, 179, 213).
- **The clause list builds 173 and S71.** S72 needs a plan stage and
  the plan part (163, 164), neither built, so it is named for record
  0024's plan stage's change, which also adds the plan to what makes
  an account able to run a stage that delivers a plan (173).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `account-pools`: a stage whose layers name no program runs on the
  program of the account chosen among those that can run it; the
  intent's affinity for the program its last agent ran; a wait and a
  refusal read the accounts that can run the stage.
- `launching`: flume's default names a program only for an agent with
  no pool.
- `place-status`: an agent whose program its account named gives the
  program from `account`.
- `seam-contracts`: an intent's affinity holds the program its last
  agent ran.
- `seam-fakes`: the fake agent scripts its way to stop short, so a
  prompt's preview shows which program's profile filled it.
- `clause-coverage`: 173 and S71 built, S72 named for the plan stage's
  change.

## Impact

- boomdrivedev/blueprints: no amendment. The statement at `48a94da`,
  the revision the clause list holds, states 32 item 8, 144, 145, 147
  and 173 as built here.
- `docs/adr`: record 0026 amended with this change's choices, which
  are the construct's; rules core.30 and core.32 hold the outcome, and
  no rule changes.
- `crates/flume-seams`: `Affinity::last`, only added.
- `crates/flume-core`: the settings resolved for a program an account
  names; the programs that can run a stage; the account chosen among
  their accounts; the queue's waiting stages, the launch's check and
  the prompt's preview read by them; the affinity's last program.
- `crates/flume-fakes`: the fake agent's scripted way to stop short.
- `crates/flume-conformance`: the engine's scenarios of a pool's
  program; the clause list.
- `site/`: the agent-settings guide, the configuration reference and
  the state reference.
- No new crate, plugin or site part, so no new moon project; no new
  dependency and no new pinned tool.
