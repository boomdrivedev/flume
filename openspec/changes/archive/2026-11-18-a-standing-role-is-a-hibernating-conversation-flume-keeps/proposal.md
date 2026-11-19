# A standing role is a hibernating conversation flume keeps

## Why

The operator ruled that a role's session be one flume keeps rather than
one a person opens and leaves standing: "flume can launch these on
demand but can also be configured with predefined standing agents", and
"these are agents that we're defining". Of its repositories: "In an
ideal world we're not linking a repo to a standing agent. They should be
able to work against more than one", and "why not just have it work one
way for all of them with no special consideration?" Of its projects:
"launched as related flume kinds, similar to ops", so that "the user
would be able to see everything in the multiplexer the same way they
always would" (record 0069, accepted 2026-11-14).

The statement holds it at boomdrivedev/blueprints 786d040, merged to
main on the user's word of 2026-11-13: a role whose kind's stage is a
conversation that hibernates is a standing role, one place flume keeps
at the role's address, a folder of checkouts of the repositories its
entry names, detached at each base's tip and moved there at each resume,
committing nothing; a role's entry names the kinds its session launches
its projects as; a place it launches naming no recipient reports to its
address, and a delivery there resumes it (7, 56, 132, 193, 213, 214,
C.2, C.7, S128).

The base holds the rest of record 0069: hibernate and resume (212, 213),
a message, a role's deliveries, the deliver part and a launch from a
role's run reporting to its address (53 to 55, 214, 215). What no code
holds yet is the standing role's place, its entry's repositories and
kinds, a delivery resuming a role whose place hibernates, and flume's
MCP serving `hibernate` and `resume`, which the hibernation change left
to this one.

## What Changes

- **A standing role's entry (193, C.7).** A role's entry may give
  `repositories`, the locations of the repositories its standing role
  reads, in place of `repository`; and any run's entry may give `kinds`,
  each kind its session launches its projects as with the words of when
  it serves.
- **A standing role (193, 212).** A role whose kind has one stage, a
  conversation that hibernates, is a standing role: its entry gives
  `repositories`, its kind lands nothing, and its wake is refused naming
  why where either does not hold.
- **Its place (193, C.1, core.40).** One folder under flume's own data
  directory, at the location its first repository's place of its kind
  and name would have, holding a checkout of each repository detached at
  its base's tip on no branch. It has no branch, takes no readiness,
  commits nothing and lands nothing, and is made whole or not at all.
- **Moved at each start (193, 213).** Each start of its stage, a resume
  among them, moves each checkout to its base's tip read where the base
  lives; a move git would make over a change in the checkout is not made,
  the checkout left where it was and status saying why.
- **The place seam (B.1, C.1).** Three operations for a standing role's
  checkouts: make them, `git worktree add --detach`; move one, `git
  checkout --detach` with no force; remove them, `git worktree remove`
  with no teardown, then what stands at the folder.
- **A delivery resumes it (213, 214).** A batch pending at a role's
  address whose place's stage hibernates resumes that stage, the batch
  its prompt; a role's place with a batch pending is not hibernated on
  its idling.
- **Its prompt (28, 193).** Every prompt of a role's place gives, after
  the roles a stop-short may name, the kinds its entry names with their
  words and how to launch one so its ends come back.
- **Status (56).** Each standing role's address with its place and
  whether it hibernates; a standing role's place with each checkout, its
  base, its revision, when it was last moved or why not, and what it
  holds.
- **Tidying (62, 65, core.18).** A standing role's place, once its
  caller's finish makes it done, is removed checkout by checkout, kept
  whole while any checkout holds work, and no branch is removed for it.
- **MCP (C.2).** `flume mcp` serves `hibernate` and `resume`, each the
  command line's act.

## Capabilities

### New Capabilities

- `standing-roles`: a role whose kind's stage is a hibernating
  conversation, kept as one place of detached checkouts moved at each
  start, its prompt naming the kinds it launches its projects as, and
  its address's deliveries resuming it.

### Modified Capabilities

- `role-waking`: a run's entry gives `repositories` and `kinds`; a
  standing role's wake.
- `hibernation`: a delivery resumes a role whose place hibernates.
- `seam-contracts`: the place seam's operations on a standing role's
  checkouts and their types.
- `place-profile`: the git worktree profile's checkouts.
- `place-status`: a standing role's address and place.
- `tidying`: a standing role's place tidied checkout by checkout.
- `claude-code-plugin`: `hibernate` and `resume` over MCP.

## Impact

- `crates/flume-seams`: `RoleRun` gains `repositories` and `kinds`;
  `PlaceRecord` gains `checkouts`; the place seam gains three
  operations with `Checkout` and `Moved`; `ResumedBy::Delivery`.
- `crates/flume-core`: a new `standing_role.rs`; `wake.rs`,
  `launch.rs`, `start.rs`, `prompt.rs`, `batch.rs`, `hibernate.rs`,
  `settle.rs`, `tidy.rs`, `status.rs`, `answer.rs`.
- `crates/flume`: `host/roles.rs`, the git worktree place profile,
  `commands/mcp.rs`, `commands/print.rs`.
- `crates/flume-fakes`: the fake place's checkouts.
- `crates/flume-conformance`: the place seam's scenarios and the
  engine's `standing_roles.rs`; the clause list unchanged, its scenarios
  citing what it holds at 31f9f1c.
- `site/`: the roles file's keys, standing roles, the MCP tools.
- `docs/adr/0069`: this change's construction choices.
