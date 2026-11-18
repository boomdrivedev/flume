# Design

## Context

### What is decided

Record 0069, accepted by the user on 2026-11-14, is the decision: a
standing role is a role whose kind's stage is a hibernating
conversation; a role's entry names the kinds its session launches its
projects as, which its prompt gives; a place a role launches naming no
recipient reports to the role's address, so its ends join the role's
deliveries and resume it; a standing role's place holds a detached
checkout of each repository its entry names, moved to its base's tip at
each resume, and commits nothing; flume refuses no kind a role's entry
does not name. Its statement is boomdrivedev/blueprints 786d040, merged
to main on the user's word of 2026-11-13: clauses 7, 56, 132, 193, 213
and 214, C.2's MCP tools, C.7's roles file, and S128. Rules core.10 and
core.32 are amended with the record.

The record left open, among others, what a resume does with a file
changed in a checkout (its Consequences); decision 6 answers it for this
change as the construction's own choice.

### What stands in the code

On `bolt/flumes-roles-sleep-and-wake` at 3ecc4f88:

- **A role's run.** `host/roles.rs` reads a source's roles file into
  `RoleEntry::Run(RoleRun)`, `RoleEntry::Standing(StandingAgent)` (an
  agent flume did not open, 132) or `RoleEntry::Carried`. A run's entry
  gives one `repository`. `Engine::wake_address` (`wake.rs`) finds the
  run that stands by `held.role`, else builds a `Launch` from the entry
  and calls `make_place(launch, Some(address), None)`. `settle.rs`'s
  `run_over` declares a role's run done once its last stage delivered or
  finished, so a conversation's finish ends it.
- **A place.** `make_place` (`launch.rs`) binds the repository, reads
  the base where its merge queue says it lives (`base_now`), resolves the
  kind, writes the held state `Making`, asks the place seam's `make`
  (`git worktree add -b`), readies it, and starts its first stage. A
  base place takes another path after the held state is written
  (`open_base_place`). The place seam (`flume-seams/src/place.rs`) has
  `location`, `make`, `ready`, `read`, `remove` and `remove_branch`, and
  one profile, `profiles/place/git_worktree`.
- **Hibernation.** `hibernate.rs` hibernates and resumes a stage;
  `resume_stage` puts it waiting for room with a `CarryFrom` whose
  `resumed_by` is `Caller`, `Message` or `Answer`, and `start.rs`'s
  claim starts the resume, its prompt the wait's words or
  `StartReason::Woken`'s, then the way to stop short and the roles.
- **A role's deliveries.** `batch.rs` hands a recipient's batch only
  to a session the terminal reads waiting; a role's run's start claims
  its address's batch for its prompt (`claim_for_start`) where it is no
  resume. A hibernating role's place has no session, so a batch pending
  at its address waits there for good, and the wake step passes over its
  address since its run stands.
- **A launch from a role's run.** `recipients_of` takes the address of
  the role's run `$FLUME_PLACE` names, `recipients_from` `role-run`, and
  status gives the places reporting to an address (`reporting`).
- **MCP.** `commands/mcp.rs` serves `status`, `message`, `launch` and
  `wake`.

## Goals / Non-Goals

**Goals:** 193's standing role whole: its entry, its place of detached
checkouts, their moves, its prompt's kinds, its deliveries resuming it,
its status and its tidy; 214's batch resuming any role whose place
hibernates; C.2's `hibernate` and `resume` over MCP; S128 on a standing
role, as conformance scenarios.

**Non-Goals:** section 10's question on Claude Code's resume dialog;
credentials for an agent on a machine the operator does not set up;
whether roles move from a source's roles file to the host's
configuration; the word for a standing role, which the statement keeps
as "standing role" and this change keeps too; crew moving its roles,
which is crew's.

## Decisions

### 1. A standing role's entry gives `repositories`, and any run's `kinds`

A run's entry may give `repositories`, an array of locations, each as
`repository` is read (absolute or under `~`), in place of `repository`
(193, C.7). It is refused, the entry declaring nothing for its address
and status naming why (191), where it is empty, names one location
twice, gives two repositories whose folders (decision 3) are one, or
stands beside `repository` or `base`. Each checkout is at its
repository's default branch, read at the wake as a run's base is where
its entry names none.

Any run's entry may give `kinds`, a table whose each key is a kind as a
launch names it and whose value is the words of when it serves, a string
that is not empty (193). flume checks the key against no source: a
role launches any kind, and flume refuses none for its entry not naming
it. A standing agent's entry (`agent`) and a carried entry give none, as
neither has a session flume prompts.

`RoleRun` gains `repositories: Vec<String>`, empty where the entry gives
`repository`, and `kinds: BTreeMap<String, String>`. Where the entry
gives `repositories`, `RoleRun::repository` holds its first, so every
path that finds a run by its repository, kind and name finds it.

*Paths not taken:* a `base` for each repository, as a table per
repository, which no role needs yet and an entry can gain later without
breaking one; `kinds` only on a standing role's entry, where 193 gives
every role its kinds' words; checking each kind against the sources,
which 193 rules out.

### 2. A standing role is known by its kind and its entry, at its wake

A kind is a standing role's kind where it has one stage, a conversation
that hibernates (193, 212): `standing_role(kind)` in a new
`flume-core/src/standing_role.rs`, beside `base_place(kind)`. The wake
of a role whose entry gives `repositories` resolves its kind in its
first repository at its default branch's revision, as a run's kind is
resolved, and is refused, naming the address and why, where the kind is
no standing role's kind, where it lands (the place commits nothing,
193), or where its stage's prompt asks for `{{branch}}`, `{{base}}`,
`{{change}}` or `{{report}}`, which nothing fills for a place with no
branch (4, 27). A wake whose entry gives `repository` and whose kind is
a standing role's kind is refused naming `repositories`, which a
standing role's entry gives in its place (C.7). A refused wake is kept
on the place whose end woke it and given by status, as any wake's
refusal is (132).

`standing.rs` keeps its meaning, a standing agent flume did not open
(132); the new module is named for the statement's "standing role".

### 3. Its place is one folder of checkouts, at its first repository's place location

A standing role's place is the folder the place seam gives as the
location of a place of its first repository named `<kind>--<name>`,
under flume's own data directory and inside no repository (C.1,
core.40); its id is that location, as every place's is. Inside it, each
repository's checkout is at `<location>/<folder>`, its folder the last
part of the repository's location as the entry gives it, a trailing
`.git` dropped, so a bare repository's folder holding `flume.git` gives
`flume`.

`PlaceRecord` gains `checkouts: Vec<CheckoutHeld>`, empty for every
other place, so a state written before reads none. Each holds the
repository's location, its address where its clone has one (core.55),
its folder, its base, its binding as the wake read it (21), the revision
it stands at, where that base was read (126), when it was last moved,
and the last move not made with when, the tip it was not moved to, and
why. The record's own `repository`, `address`, `binding`, `base`,
`base_revision` and `base_from` are its first checkout's, so every path
keyed by a place's repository (its identity, its stream's choice, its
status's grouping) reads one; its `branch` is empty, as it has none.
`PlaceRecord::standing_role()` is `!checkouts.is_empty()`.

*Paths not taken:* a place of no repository, in a folder of the host's
own, which every path keyed by a place's repository would have to learn
apart; a full clone per repository, which copies every object where a
worktree shares the repository's.

### 4. The place seam makes, moves and removes a standing role's checkouts

The statement's B.1 and C.1 give the place seam's operations and their
commands for a worktree on a branch; 193 gives a standing role's place
as checkouts detached on no branch, and neither names an operation for
them. The place seam gains three, the construction's choice until the
statement carries them (core.2, core.19), each all or nothing (83):

| operation | git worktree profile |
|---|---|
| `make_checkouts(location, checkouts)`: given the place's location and each `Checkout { repository, folder, revision }`, each checkout at `<location>/<folder>`, detached at its revision; refused where anything stands at the location | `git -c core.hooksPath=/dev/null worktree add --quiet --detach <location>/<folder> <revision>` in each repository, with no hook run; a refused add undoing every checkout made, what stands at the location and each repository's entry (`git worktree prune`) |
| `move_checkout(checkout, revision)`: the checkout moved to the revision, or kept with git's words, or gone | `git -c core.hooksPath=/dev/null checkout --quiet --detach <revision>` in the checkout, with no `--force`: git carries a change it can and refuses where it would overwrite one |
| `remove_checkouts(location, checkouts)`: each checkout removed with no teardown, then what stands at the location, then `places/<id>` once empty; or the checkout refused with git's words | `git worktree remove` with no `--force` for each, then the location and its group folder as `remove` clears them (record 0057) |

`read` is asked of each checkout as of any place, a `PlaceRef` whose
location is the checkout's and whose branch is empty: it gives git's
`(detached)` as its branch, its head and what it holds (80). The
seam's types gain `Checkout` and `Moved { Moved, Kept { reason }, Gone
}`. The place seam bound for the place's first repository is asked for
every checkout, since C.1 gives one place profile for every repository
(core.3). No operation moves or removes a branch (core.5).

The fake place keeps each checkout as it keeps a place, refuses a move
it is told holds a change in the way, and passes the place seam's new
scenarios with the git worktree profile.

### 5. It is made at its wake, whole or not at all

The wake of a standing role reads, for each repository its entry names,
its default branch, its binding, and its base's revision where its merge
queue's profile says the base lives (9, 126, C.1); a repository whose
default branch, binding or base cannot be read refuses the wake naming
it. `make_place` takes the checkouts beside the launch, writes the held
state with every checkout named before anything is made (core.17), and,
after the held state's create, takes the standing role's path as a
base place takes its own: `make_checkouts`, and on a refusal the held
state removed and the wake refused naming why (2, core.8). Nothing is
readied: no step of any repository's merge queue runs in a standing
role's checkout, its readiness empty (core.47). The first stage then
starts as any place's.

*Path not taken:* equipping each checkout at each move, which would run
each repository's toolchain setup at every resume for a place that runs
no gate and commits nothing.

### 6. Each start moves its checkouts; a move over a change is not made

Each start of a standing role's stage that opens an agent, a resume, a
retry and its first among them, moves each checkout to its base's tip
once the start has claimed its room and before its agent is opened: the
base read through the checkout's binding as the wake read it, where its
merge queue's profile says it lives, after flume's own fetch (9, 126,
C.1), then `move_checkout` (193, 213). Each outcome is recorded on its
checkout in one write: moved, its revision, where the base was read and
when; or not moved, with when, the tip, and the fetch's or git's words.
A checkout not moved stays where it stood, the start goes on, and status
names it until a later move is made. 193 says "at each resume"; every
start moves them, since a retry's fresh agent reads its repositories as
a resume's does.

This answers record 0069's open "what a resume does with a file changed
in a checkout": git's own checkout, unforced, carries a change git can
carry and refuses where the move would overwrite one, and flume takes
that refusal as the move not made.

*Paths not taken:* discarding the change (`--force`, `reset --hard`),
which removes work no commit holds, as core.18 forbids a tidy to; and
refusing the start, which would leave the role's batch unhandled for one
file a person can see named in status.

### 7. A delivery resumes a role whose place hibernates

`ResumedBy` gains `Delivery`, written `delivery`. Where a batch is
pending at a role's address (`batch.rs`'s `batch`, as `hand_batch`
reads it) and the address's run stands with its current stage reading
`Hibernating`, flume resumes that stage by `delivery` with no words of
its own (213, 214). The resume's start claims the address's batch under
the resume's identity, as a run's first start claims it, and its prompt
is the batch's words then the way to stop short and the roles; where
nothing is pending when its prompt is filled, flume's words for a bare
resume. Its claims are recorded handed once its prompt is read
received, and let go where the resume is read never begun (146, 214).

Both ways a batch is reached call it: `wake_address`, where the run of
the address stands, so the settle's wake step, `flume wake`, a message's
own wake and the host's settle's wake for messages each resume it; and
`hand_batch`, for a recipient that is a role's address whose run
hibernates, in place of a session it cannot reach. A stage ended short
whose agent was hibernated while kept open waits on its answer and takes
no delivery, as a role's run waiting on its answer takes none (42, 214).
It applies to every role's run whose stage hibernates, a standing
role's among them, since 214 says so of any role.

A role's place whose address holds a batch handed to none is not
hibernated on its idling: that batch is waiting to be handed to it
(212). A settle hands it before it reads the idle time, so this holds
only for a batch the terminal could not hand.

### 8. Its prompt gives the kinds its entry names

After the roles a stop-short may name, in every prompt that gives them,
a role's place's prompt gives the kinds its entry names, read from the
sources' roles as they stand at the start, as the roles are (28, 193;
core.31):

> Work that must be committed, or that runs past one turn, launch as a
> place of one of these kinds, by `flume launch` in the repository the
> work is in, naming no recipient, so its ends come back to you here:
> - `<kind>`: <words>

each kind in its key's order, none where the entry names none.
`{{place}}` gives a standing role's folder. The words name `flume`
alone, no program (core.1).

### 9. A place it launches reports to its address

The base already gives a launch naming no recipient, run where
`$FLUME_PLACE` names a role's place, that place's address as its
recipient, `role-run` (7, 132, 214). A standing role's place is a role's
place, `held.role` its address, so its launches report there unchanged;
this change proves it of a standing role, and that the ends of those
places resume it through decision 7 (S128).

### 10. Status

A standing role's place gives `checkouts`, each with its repository, its
address, its folder, its base, its revision, when it was last moved and
where its base was read, its last move not made with when and why, and
what it holds as the place seam reads it at the ask (56, 80; core.15);
its branch is given empty, and the person's form gives no branch line
for it. Each address whose standing role's place stands is given among
the host's addresses, with `standing_role`, its place and since when it
hibernates where it does (56); its deliveries pending and the places
reporting to it are given as for every address. Every field is added,
none changed (core.14).

### 11. Tidy and undo

A standing role's place, done at its caller's finish (109, 132), is
tidied as any place but by its checkouts: each read through the place
seam; one that cannot be read keeps the place, named; one holding a
change not committed, a file neither committed nor ignored, or a lock
keeps the place whole, its paths given under the checkout's folder (65,
core.18); otherwise `remove_checkouts`, a refusal keeping the place with
its words. No branch is removed or kept for it. A wake whose command
was gone mid-make has what it made undone by `remove_checkouts`, as a
place's worktree is (2, 76).

### 12. `hibernate` and `resume` over MCP

`flume mcp` serves `hibernate`, taking `place` and `force`, and
`resume`, taking `place`, each turned into the command line's words and
run by its parser and act, its result the act's JSON and a refusal
marked an error, neither read-only (60, C.2).

### 13. The suite's list stays at 31f9f1c

The engine's new scenarios cite clauses the list holds at 31f9f1c (7,
56, 132, 193, 212, 213) and give S128 and 214 in their names' words, as
the message change's do; the chore that moves the list past 210 finds
them to cite.

### 14. The site and the record

The site's roles guide, its configuration reference and its commands
reference say what a standing role is, its entry's `repositories` and
`kinds`, and the MCP tools (core.35). Record 0069 gains this change's
construction choices, decisions 1 to 7, each with the path not taken.

## Rules

- core.1, core.2, core.3: the checkouts reached through the place seam
  alone; git named only in its profile.
- core.5: a move checks out a revision detached and moves no branch.
- core.8, core.17: the place written whole before anything is made,
  and made whole or not at all.
- core.10, core.32: a delivery's resume is one fresh agent carrying its
  stage's own session, on the account 213 names.
- core.13: every move inside a start inside a command.
- core.14, core.15: status reads each checkout where it lives.
- core.18: a checkout holding work keeps its place.
- core.19: B.1 and C.1 should carry decision 4's operations.
- core.28, core.54: a role's address and its run as the base gives them.
- core.31: the kinds' words after every part a stage's starts share.
- core.35: the site speaks to any host's operator.
- core.40: the folder under flume's own data directory.
- core.47: no readiness runs in a checkout.

`docs/architecture/core.md` is the one page; no area page exists.

## Risks / Trade-offs

- **A fetch per repository at each start.** A standing role's start
  waits on one fetch for each repository it reads, inside the settle
  that starts it. A role reads few repositories, and a fetch that fails
  leaves its checkout where it stood.
- **An unmoved checkout is stale.** An agent may read a checkout git
  would not move; status names it, and the next start tries again.
- **The first repository is the place's.** A standing role's place is
  listed under its first repository. An entry that reorders its
  `repositories` while its place stands changes nothing of it, since a
  run is found by its role first; its next place, after a finish, is
  under the new first.
