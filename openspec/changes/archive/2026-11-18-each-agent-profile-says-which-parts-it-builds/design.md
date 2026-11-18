# Design

## Context

The agent seam (B.4) is `flume_seams::Agent`, in
`crates/flume-seams/src/agent.rs`; Claude Code's profile, in
`crates/flume/src/profiles/agent/claude/`, is the one agent profile,
and `profiles.rs` binds no other program. A profile says of four parts
whether it builds them, each by a method of its own: `send_back` and
`on_quiet` (115, 121), `reports_windows` (211) and `mode_read_back`
(169). Of every other part it says nothing: `spent` answers unknown,
`carry` not offered and `own` refuses by the seam's defaults, and
`guard` answers not offered, without a word that the part is degraded.
The core reads `reports_windows` for an account with no reading
(`flume-core/src/accounts.rs`) and `mode_read_back` for status's mode
(`flume-core/src/status.rs`); no code of the core reads `send_back` or
`on_quiet`. Status names no degraded part. The suite's clause list
(`crates/flume-conformance/src/clauses.rs`) names 170 to 174 and S71 to
S75 unbuilt, under `PROFILES`.

The statement is read at boomdrivedev/blueprints main 48a94da, the
revision the clause list holds: 93, 170, 171, 174, C.4 and S73. The
decision is record 0026, accepted by the user on 2026-10-30. The rules
this change rests on are `docs/architecture/core.md`'s, the one page of
the area: core.1 and core.2 (the core names no program, and reaches the
agent seam through one interface), core.14 (status's form only gains
fields), core.19 (clauses cited by number) and core.39 (each part
built or degraded as its profile declares, status naming each degraded
part with the agent). It departs from none.

## Goals / Non-Goals

**Goals:**

- One declaration per profile of the parts it builds, from which every
  answer of whether a part is offered is read.
- Status naming each degraded part of the current agent with how flume
  reads it.
- Claude Code's profile building every part, the fake's parts following
  its scripts, and 174 built in the clause list.

**Non-Goals:**

- The common profile, its launcher and its entries, and flume reading
  received, asking, own work, ended and last words from the terminal
  (171.1 to 171.5): record 0026's change of the common profile, which
  builds 170 to 173 and S71 to S75. No program but Claude Code is
  accepted by this change.
- The model's table of programs (172), the program a pool's account
  names (173), herdr's send waiting on a turn and OMP's integration
  (C.5, S75).
- The parts no seam has yet, usage (152) and plan (164).

## Decisions

### 1. The parts are the eight flume reads a lesser way today

A profile may degrade spent, windows, send-back, guard, on-quiet, the
mode, carry and own settings, in 171's order. For each, flume already
answers a profile that does not build it as 171 says: an account never
recorded spent and with no reading, no hook keeping the agent at work,
refusing its acts or settling its place, the mode recorded and not
compared, a spent agent's stage started afresh, and own settings
refused. Every other part of the seam is answered from the program's
own record by every profile (93). Command, started and stop-short are
never degraded (C.4's parts row).

Not taken: every part 171 lists now, received, asking, own work, ended
and last words among them. Status would say flume reads them from the
terminal, which it does not until the common profile's change. Each of
those joins the list with the change that gives flume its lesser
reading, as usage and plan join with the changes that add them to the
seam.

### 2. A profile names the parts it builds; flume owns how a degraded one is read

`Agent::parts()` gives a `Parts`, the set of parts the profile builds;
every other part is degraded. How flume reads a degraded part is one
sentence per part, on the seam's `Part`, with its name as the statement
writes it:

| `Part` | name | read, where its profile does not build it |
|---|---|---|
| `spent` | spent | unknown, so the account it runs under is never recorded spent |
| `windows` | windows | unknown, so the account it runs under has no reading |
| `send_back` | send-back | not offered, so it may go quiet with its deliverable missing |
| `guard` | guard | not offered, so nothing refuses its submitting, merging or pushing its place's branch |
| `on_quiet` | on-quiet | not offered, so the host's watch, or a command where none keeps watch, settles its place |
| `mode` | mode | recorded and not compared with the mode it began in |
| `carry` | carry | not offered, so a stage whose agent stopped at its account's usage limit starts afresh |
| `own_settings` | own settings | none reserved and none taken, so own settings given for its program are refused |

`Part` serializes to its word and refuses any other. The words name no
program (core.1).

Not taken: each profile writing its own words for a degraded part. 93
and 171 fix the lesser reading for every profile, and two profiles
could describe one reading two ways.

### 3. By default a profile builds none

`parts()` defaults to none built, as the seam's other defaults offer
nothing. A profile that does not declare shows every part degraded, so
no degraded part is ever read as a built one (170). Claude Code's
profile names each of the eight in turn rather than taking every part,
so a part added to the list later is built for Claude Code only by the
change that builds it, and the test that Claude Code builds every part
fails until then.

Not taken: a required method, which every stand-in profile of the
tests would have to write; and every part built by default, which reads
an undeclared part as built.

### 4. Whether a part is offered is read from the parts

`send_back`, `on_quiet`, `reports_windows` and `mode_read_back` become
the seam's provided readings of `parts()`. Claude Code's profile, the
fake agent and every stand-in implementation of the seam in the tree
drop their own answers to them. The answers that read a record, `spent`,
`windows`, `guard`, `carry`, `own` and `reserved`, cannot be read from
the parts; for a part a profile does not build, each answers the lesser
way, and the agent seam's scenario "A profile's answers agree with its
parts" holds every profile the suite runs to it.

Not taken: each profile keeping its own four answers beside its parts,
checked by a scenario, which is two declarations of one fact; and
removing the four methods, every caller asking `parts()`, which changes
specs that name them for no change of behaviour.

### 5. The fake's parts follow its scripts

The fake agent's `parts()` reads its scripts: send-back, guard and
on-quiet built unless `no_send_back`, `no_guard` or `no_on_quiet`;
windows unless `reports_no_windows`; spent always; the mode once
`read_mode_back` or `read_mode_at_prompt`; carry once `carry_to`; and
own settings once `take_own`. Its harness gains
`FakeAgentHarness::offering_no_hooks`, the three hooks scripted off,
beside `carrying_none`, `taking_no_own` and `reporting_no_windows`.

Not taken: a script per part that degrades it apart from the fake's
answers, which would let the fake's parts disagree with what it does.

### 6. Status gives the degraded parts alone

`AgentStatus` gains `degraded`, each a `DegradedPart` with its `part`
and `read`, in 171's order, left out of the JSON where empty, so a
Claude Code agent's form is unchanged (core.14). The core reads it
through the seam from the profile that binds the start's program, and
gives none where no profile binds it. The person's form gives each
after the agent's notes, as `    <name> degraded: <read>`.

Not taken: `parts`, every part built or degraded. 170 asks the
degraded ones named, and the built ones are every other.

### 7. The clause list builds 174

`PROFILES` names 170 to 173 and S71 to S75, record 0026's common
profile's, and no longer 174, which the agent seam's parts scenarios
cite with 93. 170 stays unbuilt: its first sentence, every program the
terminal can start, is the common profile's. The windows scenario
"A profile that reports no windows" cites 174 beside 211.

### 8. The statement holds; record 0026 takes these choices

170, 171, 174 and C.4 say what this change builds at 48a94da. C.4's
Claude Code table answers each part from Claude Code's own record, so
it needs no parts row. Not taken: a parts row on Claude Code's table, on
a blueprints branch, which says again what its rows say. Rule core.39
already holds the declaration and status's naming. Record 0026 is
amended with these choices, still accepted.

## Risks / Trade-offs

- [A profile overrides one of the four readings against its parts] →
  the agent seam's scenario "A profile's answers agree with its parts"
  fails for every profile the suite runs.
- [The fake as a new fake makes it degrades the mode, carry and own
  settings, so status's scenarios over it gain `degraded`] → each
  status scenario reads the fields it names; one that builds an agent's
  status whole builds it with `degraded`.
- [A part flume comes to read a lesser way is missing from Claude Code's
  list] → the test that Claude Code builds every part fails, naming the
  part.

## Migration Plan

None. The parts are answered, never held, so no held state changes;
status's JSON gains a field left out where empty.
