# standing-roles Specification

## Purpose
A standing role is a role whose kind's stage is a conversation that
hibernates: flume keeps one place for it at its address, a folder of
checkouts of the repositories its entry names, detached at each base's
tip and moved there at each start, committing nothing; it takes its
address's deliveries in batches, hibernates between them, launches its
projects as places of the kinds its entry names, which report back to
it, and ends only on its caller's finish (7, 132, 193, 212 to 214).

## Requirements

### Requirement: A standing role is a role whose kind's stage is a hibernating conversation

A role whose kind has one stage, a conversation that hibernates, SHALL
be a standing role, its entry giving `repositories`. Its wake SHALL be
refused, naming the address and why, where its kind lands, where its
stage's prompt asks for `{{branch}}`, `{{base}}`, `{{change}}` or
`{{report}}`, where its entry gives `repositories` and its kind is no
standing role's, or where its entry gives `repository` and its kind is
one (4, 27, 193, 212, C.7).

#### Scenario: A standing role woken

- **WHEN** acme's roles file declares `east/ops` with the kind `desk`, whose one stage `talk` is a conversation that hibernates and lands nothing, and `repositories` naming the world's repository and `/blueprints`, and an end comes for `east/ops`
- **THEN** the settle launches `desk--east-ops`, its role `east/ops`, and its talk starts under the session name `east/ops` (132, 193)

#### Scenario: A landing kind

- **WHEN** the entry names a kind whose one stage is a conversation that hibernates and that lands
- **THEN** `east/ops` is not woken, and status names the address and that a standing role's kind lands nothing, since its place commits nothing (193)

#### Scenario: A branch asked for

- **WHEN** the entry's kind's talk asks for `{{branch}}`
- **THEN** `east/ops` is not woken, and status names the part, which nothing fills for a place with no branch (27)

#### Scenario: Repositories for a run's kind

- **WHEN** the entry gives `repositories` and the kind `turn`, whose stage is no conversation
- **THEN** `east/ops` is not woken, and status names that only a standing role's entry gives `repositories` (193)

#### Scenario: One repository for a standing role's kind

- **WHEN** the entry gives `repository` and the kind `desk`
- **THEN** `east/ops` is not woken, and status names `repositories`, which a standing role's entry gives in its place (C.7)

### Requirement: A standing role's place is a folder of detached checkouts

A standing role's place SHALL be the location the place seam gives a
place of its first repository named `<kind>--<name>`, holding a
checkout of each repository its entry names at `<location>/<folder>`,
its folder its location's last part less a trailing `.git`, detached at
its default branch's tip read where its merge queue says the base lives.
It SHALL have no branch and take no readiness (9, 126, 193, C.1;
core.40, core.47).

#### Scenario: Two checkouts

- **WHEN** `east/ops` is woken with `repositories` naming the world's repository and `/blueprints`, each on `main`
- **THEN** the fake place holds `repository` and `blueprints` under the place's location, each detached at its `main`'s head, the place's branch is empty, no readiness step runs, and the held state names both checkouts with their revisions

#### Scenario: The base read where it lives

- **WHEN** the world's repository binds the fake merge queue landing `main` on its remote, whose `main` is ahead of the host's
- **THEN** that checkout is detached at the remote's `main`, and status gives where its base was read (126)

### Requirement: A standing role's place is made whole or not at all

The wake SHALL write the held state naming every checkout before any is
made, and SHALL be refused, leaving nothing it made, where a
repository's default branch, binding or base cannot be read, or where
the place seam does not make every checkout (2, 3, 76, 83; core.8,
core.17).

#### Scenario: A checkout refused

- **WHEN** the fake place refuses the checkout of `/blueprints`
- **THEN** the wake is refused naming it and why, no checkout stands, no held state of the place stands, and the end that woke `east/ops` stays pending

#### Scenario: A repository with no default branch

- **WHEN** git names no default branch of `/blueprints`
- **THEN** the wake is refused naming `/blueprints`, and nothing is made

### Requirement: Each start of a standing role moves its checkouts

Each start of a standing role's stage that opens an agent, a resume and
a retry among them, SHALL, once it has claimed its room and before its
agent is opened, move each checkout to its base's tip, read as at the
wake, and record each move's revision, where it read the base and when,
in one write (9, 126, 193, 213; core.5, core.13).

#### Scenario: Resumed onto a newer tip

- **WHEN** `east/ops`'s talk hibernated, `/blueprints`'s `main` gains a commit, and a caller's `flume resume` of its place resumes it
- **THEN** before the resume's agent opens, the `blueprints` checkout is detached at the new head, the other where it stood, and status gives each checkout's revision and when it was moved (193, 213)

#### Scenario: A retry

- **WHEN** `east/ops`'s talk was stopped, `main` moves, and a retry starts it afresh
- **THEN** the fresh start's agent opens with each checkout at its base's tip

### Requirement: A move over a change is not made

A move git would make over a change in the checkout, or whose base
cannot be read, SHALL NOT be made: the checkout SHALL stay where it
stood, the start SHALL go on, and status SHALL give the move not made
with when, the tip and the words, until a later move is made. No change
in a checkout SHALL be discarded (193, 213; core.18).

#### Scenario: A file changed in a checkout

- **WHEN** the fake place is told `blueprints` holds a change the move to `main`'s new head would overwrite, and a resume starts
- **THEN** the resume's agent opens with `blueprints` where it stood, the change kept, and status gives the move not made with the fake's words and the tip

#### Scenario: Moved at the next start

- **WHEN** the change is gone and the talk is resumed again
- **THEN** the checkout is moved, and status gives no move not made

### Requirement: A role's prompt gives the kinds its entry names

Every prompt of a role's place that gives the roles a stop-short may
name SHALL give after them the kinds its entry names, each with the
words of when it serves, and how to launch one so its ends come back,
read from the sources' roles at the start; and none where the entry
names none. flume SHALL refuse no launch for its kind not being named
(28, 193; core.31).

#### Scenario: The kinds in the first prompt

- **WHEN** `east/ops`'s entry gives `kinds` `fix` with "a failing landing or check" and `change` with "a change a design asks for", and its talk starts
- **THEN** its prompt gives, after the roles a stop-short may name, flume's words on launching by `flume launch` in the work's repository naming no recipient, then `change` and `fix` each with its words

#### Scenario: A kind the entry does not name

- **WHEN** the talk's agent launches a chore from its place, naming no recipient
- **THEN** the launch is taken, its recipient `east/ops` (193)

### Requirement: A standing role's launches report to it and resume it

A place a standing role's session launches naming no recipient SHALL
report to the role's address, and its ends SHALL wait there with the
address's other deliveries and resume the role where its stage
hibernates (7, 132, 213, 214).

#### Scenario: Two places launched and the role asleep

- **WHEN** `east/ops`'s talk launches a fix and a chore naming no recipient and hibernates, the chore stops short with a question, and later both deliver
- **THEN** the chore's question resumes the talk with it as its prompt; once the talk hibernates again, the two ends resume it as one prompt; and while they ran, status gave both under `east/ops` (56, 214, S128)

### Requirement: A standing role ends at its caller's finish

A standing role's place SHALL end only on its caller's finish, its stop,
drop or done; its finish SHALL declare it done with no other word and
tidy it, and the next end for its address SHALL wake a new place with
new checkouts (109, 114, 132, 193).

#### Scenario: Finished, then woken again

- **WHEN** a caller finishes `east/ops`'s talk while it hibernates, and an end later comes for `east/ops`
- **THEN** the place is done and tidied, its checkouts removed, and the end wakes `desk--east-ops` again, its checkouts at each base's tip then
