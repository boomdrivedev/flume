# Each agent profile says which parts it builds

## Why

Record 0026, accepted by the user on 2026-10-30, has flume run every
agent program its terminal can start, each part of the agent seam built
for the program from its own record or degraded, and status naming
every degraded part with the agent so nobody reads it as a built one
(170, 174). The agent seam has no `parts` today: a profile says only of
its windows and its mode whether it builds them, and status names
nothing degraded. Before any profile can degrade a part, the seam must
ask each profile which parts it builds, and status must say how flume
reads each part it does not. The conformance suite's clause list names
174 unbuilt, left to record 0026's change.

## What Changes

- **The agent seam's parts (174).** Given a program, its profile names
  each part of the seam it builds from the program's own record; every
  other part is degraded, with how flume reads it as 171 says. The parts
  a profile may degrade are those flume reads a lesser way today: spent,
  windows, send-back, guard, on-quiet, the mode, carry and own
  settings. A profile that declares nothing builds none of them.
- **One declaration.** Whether a profile offers send-back and on-quiet,
  reports windows and reads its mode back is read from its parts, so no
  profile says one thing there and another in its parts. A part a
  profile does not build is answered the lesser way: spent and windows
  unknown, guard and carry not offered, no own settings taken and no key
  reserved.
- **Claude Code's profile builds every part (C.4).** Status names no
  part of a Claude Code agent degraded.
- **The fake agent's parts follow its scripts.** Send-back, guard and
  on-quiet scripted off, windows not reported, no mode read back, no
  carry and no own settings each read as that part degraded.
- **Status names each degraded part with the agent (56, 170).** The
  current agent's JSON gains `degraded`, each part with how it is read,
  left out where none is; the person's form gives a line for each.
  Status's form only gains fields (core.14).
- **The clause list builds 174.** The agent seam's parts scenarios cite
  it, and 170 to 173 and S71 to S75 stay named for record 0026's
  common profile.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `seam-contracts`: the agent seam's parts and the type of a part;
  send-back, on-quiet, windows and the mode's read-back read from them.
- `seam-fakes`: the fake agent's parts read from its scripts.
- `agent-profile`: Claude Code's profile builds every part.
- `place-status`: each degraded part of the current agent, with how it
  is read.
- `clause-coverage`: 174 built, cited by the agent seam's parts
  scenarios.

## Impact

- `crates/flume-seams`: `Part`, `Parts` and `Agent::parts`;
  `send_back`, `on_quiet`, `reports_windows` and `mode_read_back` read
  from them, no longer each profile's own.
- `crates/flume-core`: status's `AgentStatus` gains `degraded`.
- `crates/flume`: Claude Code's profile declares every part; status's
  person's form names each degraded part.
- `crates/flume-fakes` and `crates/flume-conformance`: the fake agent's
  parts, the agent seam's parts scenarios, a fake harness offering no
  hooks, status's scenario of a degraded part, and the clause list.
- `site/`: the state reference's `degraded`, and the Claude Code guide
  saying its profile builds every part.
- `docs/adr/0026`: amended with the construct's choices, still
  accepted. Rule core.39 already says this; the statement's 170, 171,
  174 and C.4 hold as written at boomdrivedev/blueprints 48a94da.
- No new crate, plugin or site part, so no new moon project; no new
  tool.
