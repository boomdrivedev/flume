# Design

## Context

See proposal.md, Why. The statement is boomdrivedev/blueprints
`design/flume/requirements.md` at 48a94da, which holds 0fd6a43: clause
56 gives each account of a pool "its configuration home as its profile
names it, the one value of the account's variables status shows", and
C.4's `home` row names Claude Code's, the `CLAUDE_CONFIG_DIR` the
account's variables give, and none where they give none. Record 0081
(accepted) holds how flume reads it, and core.30's and core.32's
details allow it.

As built on this base:

- `Engine::host_status` in `crates/flume-core/src/room.rs` builds each
  `PoolStatus` from the queue's pools, each `AccountStatus` from the
  host's `Account` (`crates/flume-seams/src/host.rs`: `name`,
  `program`, `env`, `agents`) and its room. `AccountStatus`
  (`crates/flume-core/src/answer.rs`) says it gives "never a value of
  its environment or a path".
- `Engine::account_env` in `crates/flume-core/src/settings.rs` reads
  each value of an account's `env` with `expand`, its `~` and each
  `$VAR` against the host user's login environment (`Host::login`),
  or refuses naming `env.<name>` and the `$VAR` the login environment
  lacks. A start and `live_envs` read the account this way.
- The agent seam (`crates/flume-seams/src/agent.rs`) names the variable
  its program reads over an effort by `effort_variable`, none by
  default. Claude Code's profile (`crates/flume/src/profiles/agent/
  claude/`) reads its configuration home as `CLAUDE_CONFIG_DIR`, an
  empty value none, in `carry.rs`'s `home` and in `live.rs`.
- `pool_lines` in `crates/flume/src/commands/print.rs` prints each
  account's line and, under it, its spent words, its reading and what
  holds it past its threshold.
- The answers' suite (`crates/flume-conformance/src/engine/
  answers.rs`) holds every field of its sample answers in `FIELDS`.

No area page of `docs/architecture/` exists; `core.md` binds the
change. Rules it relies on: core.1 and core.2 (the core names no tool,
each seam through one interface: the core reads no `CLAUDE_CONFIG_DIR`
of its own), core.14 (one form that only gains fields), core.15 (status
reads the host's configuration where it lives, when asked), core.30
(an account's `env` read against the login environment, and no value
of it in status but the home record 0081 allows), core.32 (flume
writes nothing in a configuration home), core.35 (the site speaks to
any host's operator) and core.45 (the host's configuration values give
every `env` value as `<set>`).

## Goals / Non-Goals

**Goals:**

- Each account of a pool gives its configuration home in status, read
  as its agents' starts read it, so a tool installing into each account's
  home reads it from `flume status --json` (56, C.4).
- The core names no program's variable: the profile names it (core.1).

**Non-Goals:**

- No home for the login environment where no pool runs a stage: the
  statement names a home per account, and a host with no pool runs
  Claude Code's own default home.
- No home on an agent's account in a place's status: the home is given
  once, with its account among the host's pools, and the agent's
  settings keep naming the variable as `<set>`.
- Nothing is installed, written or checked in a home: whether the
  folder exists is the installing tool's to find (core.32).
- No new pinned fragment of Claude Code: `CLAUDE_CONFIG_DIR` as its
  configuration home is what the carry and the live rows already rest
  on, read from the pinned release by their check-tools tests.

## Decisions

### 1. The profile names the variable; the core reads it

The agent seam gains `home_variable(&self) -> Option<&'static str>`,
none by default, beside `effort_variable`. Claude Code's profile gives
`Some(carry::CONFIG_DIR)`, the constant its carry already names. The
fake agent scripts it.

Alternative: `home(&self, env) -> Option<String>`, the profile reading
the home from the account's environment. It would put a second reader
of an account's `~` and `$VAR` in a profile, where core.30 keeps that
reading to the core's `account_env`, and a profile would see the raw
`env` it never sees today.

### 2. The home is read as a start reads it

`Engine::host_status` reads, for each account, the value its program's
profile's variable has in the account's `env`, through the same
`expand` against `Host::login` that `account_env` uses. The program's
profile is `Engine::agent(&account.program)`; a program no profile
runs has none. The account's other variables are not read for it, so a
variable the login environment lacks elsewhere in the `env` leaves the
home readable. A small `Engine` function in `settings.rs`, beside
`account_env`, gives `(home, home_unread)` for an account, so the
refusal's words are written once.

Alternative: the value as written, `~/.claude-work-1`. Each reader
would read `~` and `$VAR` itself, perhaps otherwise than flume does,
and could name another folder than the one the agents run under.

### 3. Empty is none; unreadable is none with why

An empty value gives no home, as Claude Code 2.1.296 reads an empty
`CLAUDE_CONFIG_DIR` and as `carry::home` already does. A value naming
a `$VAR` the login environment lacks gives no home and `home_unread`,
`env.<variable> names $<lacking>, which the host user's login
environment lacks`, the words a stage's refusal gives, with no value.

Alternative: none with no words. A reader would take an account it
cannot install into for one that sets no home.

### 4. The form

`AccountStatus` gains `home: Option<String>`, `#[serde(default)]` and
always serialized, `null` where none, so a reader tells an account with
no home from a flume before this; and `home_unread: Option<String>`,
`#[serde(default, skip_serializing_if = "Option::is_none")]`, as
`spent_unread` is. Its doc comment says it gives the home and no other
value of its environment or path. `FIELDS` in the answers' suite lists
both, its sample answer giving one account a home and another
`home_unread` (core.14).

### 5. The person's form

`pool_lines` prints, under an account's line and before its spent
words, `      configuration home <path>` where it has one, `      no
configuration home: <words>` where it is unread, and nothing where it
has none. Its doc comment names the home as the one path it gives.

### 6. The proofs

- The seam's default: `a_profile_that_names_no_way_to_give_an_effort`'s
  neighbour in `crates/flume-seams/src/round_trip.rs`, a new
  `a_profile_that_names_no_home`, holds `Bare.home_variable()` none.
- The fake: `the_agents_home_is_scripted` in
  `crates/flume-fakes/tests/fakes.rs`.
- Claude Code's profile: `the_profile_names_claude_config_dir_as_its_home`
  in `crates/flume/tests/profiles/agent_claude.rs`.
- The engine: three scenarios in `crates/flume-conformance/src/engine/
  place_status.rs`, the place-status spec's "Each account's home", "A
  home the login environment cannot read" and "The home given and its
  variable masked", citing 56 and C.4.
- The form: `FIELDS` in `answers.rs` and the existing
  `every_field_listed` scenario.
- The person's form: `an_accounts_configuration_home` in
  `crates/flume/src/commands/print.rs`'s tests.
- The built binary: `each_claude_code_accounts_home_in_status` in
  `crates/flume/tests/commands.rs`, a host's pool of two Claude Code
  accounts under a scratch `HOME`.

### 7. The site

`site/src/content/docs/reference/state.md`'s `pools` row gains `home`
and `home_unread`; `guides/agent-settings.md`'s "A pool's accounts and
their readings" shows a home line and drops "Status gives no path" for
"Status gives no path but each account's configuration home"; and
`guides/claude-code.mdx` gains a short section saying where a tool that
installs into each account's Claude Code finds the homes (core.35).

## Risks / Trade-offs

- [A home's path in status is a value of an account's variables] →
  It is a folder and no secret (record 0081), the one value given, and
  the variable stays `<set>` everywhere else; core.30's details name it
  as allowed.
- [A reader takes `null` for the login environment's `~/.claude`] →
  The state reference says `null` is an account that gives none, whose
  agents run Claude Code's own default home.
- [The host's configuration changes between status and an install] →
  Status reads the configuration at each asking (core.15); a tool reads
  it again before each install.

## Migration Plan

None: the form only gains fields, and nothing is written.
