# A hook asks flume host to settle

## Why

When the Stop hook of Claude Code lets an agent stop, it starts
`flume settle <place> --on-quiet` with `setsid`. That process runs in a
session of its own, and it continues to run after the hook ends. The
hook does the same when an agent starts in a mode that its start did not
ask for. Thus flume runs a process detached, and a client does the work
of the resident.

The research of 2026-11-27, "The residents, reconciled with the code",
found that this breaks three claims for flume:

- `one-resident-per-tool`: a client starts a process that does the
  resident's work.
- `nothing-detached`: the settle continues to run after the hook ends.
- `a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place`:
  a settle that a hook asks for is a request to `flume host`, so a pane
  that closes ends only the client.

## What Changes

- **flume host settles a place on its socket.** After the hello, a
  client can ask `{"ask":"settle","place":"<location>"}`, with
  `"on_quiet":true` where the agent's own quiet asks for the settle.
  flume host answers one `taken` line at once and closes the
  connection. Then it settles the place itself. With `on_quiet`, it
  first reads the agent until the agent is quiet, for up to 60 seconds,
  on its own clock. A stop of flume host that comes during that read
  ends the read, and that place is not settled. A settle that
  started runs to its end before flume host ends
  (`workers-end-with-their-owner`).
- **The Stop hook asks flume host.** When the hook lets an agent stop,
  it sends the settle with `on_quiet` to flume host. When an agent
  starts in another mode than its start asked for, the hook sends the
  settle with no `on_quiet`. The hook starts no process. Where no flume
  host answers, the hook settles nothing: the next settle of the
  resident or of a caller moves the place (core.13).
- **The detached start goes.** flume has no code that starts a process
  in a session of its own.

This change builds these claims:
`a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place`,
`nothing-detached`, `one-resident-per-tool` and `flume-has-a-resident`.
The clause that it serves is 121: flume is told at once each time the
agent goes quiet, so that flume settles its place.

It closes these faults that the research found for flume:

- Row 1, `one-resident-per-tool`: the hook's settle is a client's work.
- Row 16, `nothing-detached`: the settle that the hook starts with
  `setsid` outlives the hook
  (`crates/flume/src/profiles/agent/claude/hook.rs:600`).
- Row 22, `a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place`:
  the settle that a hook asks for is not a request to `flume host`.

## Capabilities

### Modified Capabilities

- `agent-profile`: the Stop hook sends the settle to flume host and
  starts no process.
- `settling`: flume host takes a `settle` ask on its socket.

## Impact

- `crates/flume/src/served.rs`: the `settle` ask, the client that sends
  it, and the settle of the host's core.
- `crates/flume/src/profiles/agent/claude/hook.rs`: the Stop hook and
  the start in another mode send the ask. The detached start goes.
- `crates/flume/src/profiles.rs`: the hook's entry points take no
  binary.
- `crates/flume/src/commands/mod.rs`: flume host and `flume settle
  --on-quiet` use one read of an agent's quiet.
- `site/`: the Claude Code guide and the commands page say that the hook
  asks flume host.
