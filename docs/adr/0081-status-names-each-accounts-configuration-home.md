---
number: 81
title: Status names each account's configuration home
status: accepted
date: 2026-11-18
decision-makers:
- agentplot-design
- the construct stage of change/flumes-status-names-each-accounts-claude-code-folder
---

# Status names each account's configuration home

Status: accepted. The outcome, that status gives each account's configuration home, is agentplot-design's ruling of 2026-11-11 on the user's answer to their card, accepted in boomdrivedev/boomdrive's record 0044, point 3, and written into the statement at boomdrivedev/blueprints 0fd6a43 (clause 56 and C.4's `home` row). Points 2 to 6 below are the construct stage's own choices, each cheap to change.

Deciders: the user, answering their card `req_020ff29dc00c49ec98e8` on 2026-11-11 ("Every session on those accounts, mine included"); agentplot-design, ruling that flume names each account's configuration home in its own status, a field of its own, while the environment stays masked; the construct stage of `change/flumes-status-names-each-accounts-claude-code-folder`, for how flume reads and gives it.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clause 56 and C.4's `home` row, as amended at 0fd6a43; boomdrivedev/boomdrive's record 0044, point 3, and its amendment of 2026-11-18, under which the set installs every tool's Claude Code plugins, flume's among them, into each home; record 0017, as amended 2026-10-29, which keeps an account's variables out of status; rules core.14, core.30 and core.32; flume as built on `bolt/flume-names-each-accounts-claude-code-folder` at e9b23ef9 (`crates/flume-core/src/answer.rs`'s `AccountStatus`, `crates/flume-core/src/room.rs`, `crates/flume-core/src/settings.rs`'s `expand` and `account_env`, `crates/flume/src/profiles/agent/claude/carry.rs`'s `home`); the OpenSpec change `flumes-status-names-each-accounts-claude-code-folder`, whose design gives each point.

## Context and problem statement

The Boomdrive set installs its Claude Code plugins into every Claude Code home on a host, with Claude Code's own commands. On a host whose pools run Claude Code under several accounts, each account is a home of its own, named by the `CLAUDE_CONFIG_DIR` its `env` gives. Only flume's host configuration says which homes those are, and the set reads no other tool's configuration.

flume's status gives each account of each pool, with its agents running, its bound, its spent time and its reading. It never gives a value of an account's variables: the host's configuration values give each as `<set>`, and an agent's settings name `CLAUDE_CONFIG_DIR` from its account as `<set>` (record 0017; core.30). A configuration home is a path and no secret, so the ruling gives it as a field of its own beside the account, and every variable stays masked where status gives it.

## Considered options

For where the set reads each home:

1. **flume's status, a `home` on each account (the ruling).** One field, read where flume reads the account.
2. **flume's host configuration, read by the set.** Each reader would learn flume's layers, its `~` and `$VAR` reading and its account format.
3. **The account's `CLAUDE_CONFIG_DIR` unmasked in status.** It unmasks a variable, and a reader would still need to know which variable names a home for each program.

For which part of flume names the home:

1. **The agent profile names the variable, and the core reads it (chosen).** The seam gains one operation, naming the variable of an account's environment that gives its program's configuration home, none by default, as it names the variable read over an effort. The core reads that variable's value with the account's other variables, against the login environment.
2. **The profile reads the home from the account's environment.** A profile would read `~` and `$VAR`, which core.30 keeps to the core's reading of an account's `env`.

## Decision outcome

1. **Status gives each account of a pool its configuration home**, as its program's profile names it, the one value of an account's variables it gives. Claude Code's profile names `CLAUDE_CONFIG_DIR`. A program whose profile names none, and a program no profile runs, has none. Every variable stays `<set>` wherever status gives an account's variables: in an agent's settings and in the host's configuration values.
2. **The home is read as a start reads it.** Its value has its `~` and each `$VAR` read against the host user's login environment, as the account's `env` is read at a start (29, 144), so the folder status gives is the one the account's agents run under. Not taken: the value as written, which leaves each reader to read `~` and `$VAR`, perhaps otherwise than flume does.
3. **An empty value is none**, as Claude Code 2.1.296 reads an empty `CLAUDE_CONFIG_DIR` and as the profile's carry already does.
4. **A value the login environment cannot read gives no home, with why.** Where the value names a variable the login environment lacks, the account gives no home and `home_unread` names the account's variable and the one it lacks, as a stage's refusal does, never the value. Not taken: none with no words, which a reader would take for an account that sets no home.
5. **The JSON form gives `home` on every account**, a path or `null`, so a reader tells an account with no home from a flume that gives none, and `home_unread` only while it holds words. The form only gains fields (core.14).
6. **The person's form names the home under its account's line**, and says nothing where it has none.

### Consequences

- core.30's and core.32's details name the home in status as allowed, in this record's commit.
- A host with no pool runs its agents under the login environment, whose home is Claude Code's own default; status names no home for it.
- A tool that installs into each account's home reads the homes from `flume status --json`, `host.pools[].accounts[].home`, and never from flume's configuration.
