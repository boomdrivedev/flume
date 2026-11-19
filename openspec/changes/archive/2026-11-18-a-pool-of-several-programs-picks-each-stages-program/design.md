# Design

## Context

See proposal.md for why. The statement is read at boomdrivedev/blueprints
48a94da, the revision the clause list holds: 32 item 8, 144 to 147,
153, 169, 172, 173, 179, 196, 197, 213, S71 and S72. Record 0026
decided that a pool's account names the program of a stage whose
layers name none (program option 2); this design decides how it is
built. No page of `docs/architecture/` but `core.md` exists, and its
rules core.2, core.8, core.14, core.16, core.26, core.30, core.32 and
core.39 bind the areas touched.

### What the tree holds now

- **flume's default program** (`crates/flume/src/profiles.rs`,
  `DEFAULT_PROGRAM` and `defaults`) is `claude`, given in the default
  layer to every stage, pool or none. The conformance world's default
  is `agent` (`crates/flume-conformance/src/engine/world.rs`).
- **Settings** (`crates/flume-core/src/settings.rs`). `layers` and
  `resolve` fold the eight layers into `Resolved`, keeping each
  setting's layer in `from`; `resolution` binds the program's profile
  by `agent_for` and checks the stage's mode, model, effort and own
  settings on it by `mode_for`, `model_taken`, `effort_taken` and
  `own_refused`; `kind_started` asks the stream's terminal whether it
  starts the program's kind under an environment. `Layer::Account`
  (`crates/flume-seams/src/kinds.rs`) names a variable an account
  gives.
- **Accounts** (`crates/flume-core/src/accounts.rs`). `AccountRoom::usable`,
  `PoolRoom::usable`, `runs` and `strategy` each take one program;
  `choose_account` takes the conversation's account, then the
  affinity's for the program, then the strategy's, and returns
  `Choice::Wait` where no program is resolved; `affinity_after` writes
  the affinity for the program started; `pool_runs` refuses a launch
  whose pool holds no account of a stage's program.
- **The start** (`crates/flume-core/src/start.rs`, `claim`) resolves
  the stage, chooses the account for its program under the room lock,
  asks the terminal under the account's environment, asks the stage's
  profile whether a stopped or lost conversation carries, and records
  the start with `from` and the account's variables at
  `Layer::Account`.
- **The queue** (`crates/flume-core/src/room.rs`). Each `Waiting`
  stage holds the one program its settings resolve, by which
  `Queue::runs` and `Queue::wants` read its pool.
- **The launch** (`crates/flume-core/src/launch.rs`) resolves each
  stage, asks the terminal under no account's environment, and calls
  `pool_runs`.
- **The prompt's preview** (`crates/flume-core/src/acts.rs`, `prompt`)
  fills with the profile its settings resolve.
- **The affinity** (`crates/flume-seams/src/store.rs`, `Affinity`)
  keeps an account per program and its moves.
- **The clause list** (`crates/flume-conformance/src/clauses.rs`) names
  173, S71 and S72 unbuilt under `PROFILES`, and 163, 164 and S68 under
  `PLAN`.

## Goals / Non-Goals

**Goals:**

- A stage whose layers name no program runs on the program of the
  account its pool's strategy or its intent's affinity chooses, among
  the accounts that can run it, its start passing that program's
  entries (32 item 8, 144, 145, 147, 173, S71).
- With no pool, flume's default runs it as today (32 item 8, 173).
- The core decides the program; each profile answers only for its own
  program, as it does today (core.2, core.39).

**Non-Goals:**

- A stage that delivers a plan, and the plan among what makes an
  account able to run one (163, 164, 173): record 0024's plan stage's
  change adds the plan stage and the plan part, and with them the
  plan's check here. S72 tells of a plan stage, so it is named for that
  change.
- The terminal's start among what makes an account usable (see
  Decisions).
- A change to how a stage whose layers name its program chooses its
  account.

## Decisions

### 1. The account names the program at `Layer::Account`, flume's default only with no pool

`resolve` takes the program an account names, none where it names
none. Where a layer above the default names the stage's program, that
program stands and the account's is ignored; where only the default
names one and an account's program is given, that program stands in
its place with its layer `account`, and the stage's own settings are
read for it. `Resolved::names_program` says whether a layer above the
default names one. Status already gives a setting's layer by name, so
the program reads `account` with no new field (core.14, core.30).

Not taken: dropping the program from `Defaults` and filling it at each
start with no pool, which would change the engine's defaults for every
caller and leave status no layer to give for it; and a new layer
`pool`, when the program is the account's as its variables are.

### 2. The programs that can run a stage are read by the start's own checks

`Engine::runs_on(place, stage, programs)` resolves the stage once for
each distinct program of the pool's accounts, in the pool's order, and
runs on each `resolution`'s own checks: a profile binds it (105), and
its mode, model, effort and own settings are taken (169, 172, 196,
197). It answers each program with its resolution or with the words
that refuse it, so one reading serves the choice, the wait, the
refusal and the start, and a program that can run the stage at its
choice is one whose start passes the same checks (173).

Not taken: a check of its own beside `resolution`, which could drift
from what a start refuses; and a reading per account, when every
account of a program reads the same.

### 3. The terminal's start is asked after the choice

The terminal is asked whether it starts the chosen program's kind
under the chosen account's environment, as every start asks it today,
and refuses the stage in its words (32, C.5). 173 lists what makes an
account able to run a stage, and the terminal is not among it; herdr
answers only by a call under each account's environment, which the
room lock would hold for every account of the pool.

Not taken: the terminal's start among what makes an account usable.

### 4. The choice takes the programs that can run the stage

`AccountRoom::usable`, `PoolRoom::usable`, `runs` and `strategy` take
the programs that can run the stage, one where a layer names it, so
the strategy chooses over every usable account of any of them in the
pool's order (145). `choose_account` takes them in place of the one
program, and for a stage whose layers name none:

- the conversation's account first, while it is usable and its
  program can run the stage (145, 179, 213);
- then the affinity's account for the program its last agent ran,
  while that program can run the stage and the account is usable
  (147);
- then the strategy's choice; the intent moves for that last program
  only where the strategy chose an account of the same program because
  the affinity's was spent, past its threshold or full, as for a named
  program, and a start on another program moves nothing (147).

`affinity_after` writes the affinity for the chosen account's program
as it does today, and `last`, the program the start runs, at every
start with a pool, named or not. A stage whose layers name no program
and whose pool holds no account that can run it is `Choice::NoAccount`
in words naming the stage, the pool and each program with the words
that refuse it (4, 32, 145).

Not taken: trying the affinity's account for every program that can
run the stage, which 147 does not say; and the strategy preferring the
last program's accounts, when 147 sends the start where 145 chooses.

### 5. The affinity holds its last program

`Affinity::last`, an optional program, serialized only where it is
held, so a document written before reads with none and is written
again unchanged in form (153; core.16). With none, a stage whose
layers name no program goes to the strategy.

Not taken: reading the last program from the intent's places' starts,
which reads places the store's affinity document exists to spare.

### 6. The start resolves for the chosen program

In `claim`, a stage whose resolved settings name a program, or whose
stream has no pool, runs as today. Otherwise the start reads the
programs that can run it, chooses the account among their accounts,
and takes the chosen program's resolution; the session it opens in is
read from the settings before the program, which name no program
(199). The start's `from` holds the program at `Layer::Account`.

A start after a spent, lost or hibernated agent asks its profile
whether the conversation carries only where the chosen account runs
the earlier start's program; otherwise its conversation stays, with
the words that the account chosen runs another program, and no
profile is asked (146, 179, 213). No profile reads another program's
session as its own.

Not taken: asking the chosen program's profile of another program's
conversation.

### 7. The queue and the launch read the same programs

Each `Waiting` stage holds the programs that can run it: the named
one, those of its pool that can run it, or none where it has no pool,
read once per queue reading. `Queue::runs` passes over none of them:
a stage whose pool holds no account of any is claimed, which refuses
it; `Queue::wants` reads the pool's usable accounts of any of them,
so the soonest time an account is back counts only accounts that can
run it (145, 173; core.26).

The launch reads a stage whose layers name no program on a pool its
stream or the host declares by `runs_on` over the declared pool, and
is refused, with nothing made, where no program can run it, in the
same words a start gives (4, 145; core.8). Such a stage's terminal is
asked at its start, under its account's environment (decision 3). A
stage that names its program, or has no pool, is read as today.

Not taken: reading the programs at the claim alone, which would leave
the queue counting a stage's want from accounts that cannot run it.

### 8. The prompt's preview reads the choice without writing

`prompt` reads the queue once and asks `choose_account` what a start
would choose now, writing nothing; where no account is usable, it
takes the first account in the pool's order that can run the stage;
with no pool, flume's default. It fills with that program's profile
(29, 119). The fake agent gains a scripted way to stop short, so a
scenario sees which profile filled it.

Not taken: flume's default profile for every preview, which would
show a stage that runs on another program a way to stop short its
agent will not be given.

### 9. The clause list builds 173 and S71

`PROFILES` names nothing more; 173 and S71 are built, and cited by the
engine's scenarios of a pool's program in
`crates/flume-conformance/src/engine/account_pools.rs`. S72 moves to
`PLAN` beside 163, 164 and S68. The statement holds as written: 173's
plan condition holds for no stage until a stage delivers a plan.

Not taken: building the plan part here, which is record 0024's plan
stage's whole change.

### 10. Record 0026 is amended with these choices

These choices are the construct's, made on the rules written and
cheap to change, and are written as an amendment of record 0026 with
the path not taken, as its earlier amendments are. No rule changes:
core.30 and core.32 already state the outcome.

## Risks / Trade-offs

- [A scenario of the suite relied on flume's default while its pool
  holds accounts of several programs] → the account-pools,
  carried-conversations and streams scenarios run in their tasks, and
  a scenario whose subject is a named program names it in its kind.
- [A stream's terminal starts not every program of its pool, as Rex
  starts only `claude`] → the stage is refused at its start naming
  the program and the terminal's words, as for a program a layer
  names (32, C.5); the host keeps such accounts out of that stream's
  pool.
- [Each queue reading resolves a waiting stage once per program of its
  pool] → the checks are the profile's table lookups, and no tool is
  called.
- [An intent's affinity for its last program is written by every
  start with a pool] → by compare-and-set, as the affinity already is
  (153; core.16).

## Migration Plan

An affinity document written before reads with no last program. A
host with no pool, or whose pools hold accounts of one program, sees
no change but the program's layer, `account`, where its pool's account
names it and no layer does. Rolling back leaves `last` unread.
