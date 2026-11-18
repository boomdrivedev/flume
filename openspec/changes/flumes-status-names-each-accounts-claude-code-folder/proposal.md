# Status names each account's configuration home

## Why

The Boomdrive set installs its Claude Code plugins, flume's among them,
into every Claude Code home on a host, with Claude Code's own commands
(boomdrivedev/boomdrive's record 0044, point 3, and its amendment of
2026-11-18). On a host whose pools run Claude Code under several
accounts, each account is a home of its own, named by the
`CLAUDE_CONFIG_DIR` its `env` gives, and only flume's configuration says
which homes those are. Status gives each account of each pool, but
never a value of its variables (core.30). The user answered on their
card of 2026-11-11 that every session on those accounts gets the
companions, and agentplot-design ruled that flume names each account's
configuration home in its own status, a field of its own, while the
environment stays masked. The statement holds it at boomdrivedev/
blueprints 0fd6a43: clause 56 and C.4's `home` row. Record 0081 holds
how flume reads it.

## What Changes

- **Status gives each account of a pool its configuration home (56,
  C.4).** The home is the value of the variable the account's program's
  profile names, read against the host user's login environment as a
  start reads the account's `env`; none where the profile names none or
  the account's variables give none or an empty value; and none, with
  why naming the variables and no value, where the login environment
  lacks a variable the value names. The JSON form gives `home` on every
  account, a path or `null`, and `home_unread` while it holds words; the
  person's form names the home under its account's line.
- **The agent seam names the variable of an account's configuration
  home (C.4).** One operation, none by default, as the seam names the
  variable read over an effort. Claude Code's profile names
  `CLAUDE_CONFIG_DIR`. The fake agent scripts the variable it names.
- **Every variable stays masked.** An agent's settings name
  `CLAUDE_CONFIG_DIR` from its account as `<set>`, and the host's
  configuration values give every `env` value as `<set>`, as before
  (core.30; record 0017).
- **The site** says that status gives each account's home and that a
  tool installing into each account's Claude Code reads it there
  (core.35).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `place-status`: the host's pools give each account its configuration
  home, the one value of an account's variables status gives.
- `seam-contracts`: the agent seam names the variable of an account's
  environment that gives its program's configuration home, none by
  default.
- `seam-fakes`: the fake agent scripts the variable it names as its
  home.
- `agent-profile`: Claude Code's profile names `CLAUDE_CONFIG_DIR` as an
  account's configuration home.

## Impact

- `crates/flume-seams/src/agent.rs`: the agent seam's `home_variable`,
  none by default; `crates/flume-seams/src/round_trip.rs`'s bare
  profile keeps the default.
- `crates/flume-fakes/src/agent.rs`: the fake agent scripts its home's
  variable.
- `crates/flume/src/profiles/agent/claude/mod.rs`: Claude Code's
  profile names `CLAUDE_CONFIG_DIR`.
- `crates/flume-core`: `AccountStatus` gains `home` and `home_unread`
  (`answer.rs`), read where status builds the host's pools
  (`room.rs`), with the account's `env` read as a start reads it
  (`settings.rs`).
- `crates/flume`: the person's form prints the home
  (`commands/print.rs`); a test of the built binary reads Claude Code
  accounts' homes (`tests/profiles/hooks_claude.rs`).
- `crates/flume-conformance`: the engine's scenario for the homes, and
  every `AccountStatus` it builds.
- `site/`: the state reference's `pools` row, the agent settings guide's
  pools section and the Claude Code guide.
- Rules: record 0081, with core.30's and core.32's details, landed
  before this change on its branch. No clause is added, so the clause
  list keeps reading the statement at 48a94da, which holds 0fd6a43.
- No change to the host's configuration, the merge queue, the terminal
  or the runner.
