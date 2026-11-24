---
number: 82
title: flume host keeps watch over the host and hears the merge queue by one subscription
status: proposed
date: 2026-11-24
decision-makers:
- the orchestrator of boomdrivedev's residents
- the place change/flume-hears-sluice-by-subscription
---

# flume host keeps watch over the host and hears the merge queue by one subscription

Status: proposed. It binds once the user says yes. Before that, rule core.13 reads as amended here only for the change that builds it.

Flume now runs one process of its own on each host, `flume host`, which the service manager starts and keeps running. It keeps watch over the host, as the runner did when it ran `flume settle --every 60s` each minute. It hears sluice through one connection to sluice host's socket, held open, and settles each time sluice's records move. Where it hears nothing for a minute, it settles once. Say yes if Flume should run this resident.

Deciders: the orchestrator of boomdrivedev's residents, choosing on 2026-11-24 that the smallest `flume host` take over the watch with its subscription; the place `change/flume-hears-sluice-by-subscription`, for how it does so.

Sources: the accepted decisions `flume-has-a-resident`, `residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop`, `residents-ask-each-other-by-socket`, `residents-speak-json-lines-over-a-unix-socket`, `one-subscription-per-plugin`, `one-read-in-flight`, `a-residents-core-stays-synchronous-behind-a-tokio-edge`, `a-resident-tracks-its-tasks-with-tokio-util-and-uses-no-actor-crate`, `the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock` and `the-service-manager-starts-each-resident`, and the narrative `the-residents`, all in Boom; `boomdrivedev/blueprints:design/flume/requirements.md`, clauses 119, 120, 122, 208 and 225; records 0051 and 0078; rule core.13; sluice host's subscription as sluice 0.105.1 serves it (`crates/sluice/src/host_process/subscription.rs` and `protocol.rs`); deneb on 2026-11-24, where the runner ran `flume --json settle --every 60s` each minute and each settle with a submission in flight ran `sluice overview --json` and `sluice status <base> --all --json`.

## Context and problem statement

A place moves only when a settle reads it (120). Some changes call Flume when they happen: an agent going quiet, a command stage's command ending, a caller's command. A merge queue deciding a submission calls nothing. Records 0051 and 0078 gave the look for those changes to a caller keeping watch: the runner ran `flume settle --every 60s` once a minute, and each such settle read the merge queue for every submission in flight. A landing waited up to a minute to be read, and the merge queue was read each minute whether or not it had moved.

The accepted decision that flume has a resident makes `flume host` that caller. The accepted decision that residents learn of change from events says how it learns: from events, with a poll each minute only where no event came. sluice host serves one subscription for each client on its socket, and sends a folder's overview each time it changes.

## Considered options

For who keeps watch:

1. **`flume host`, flume's resident (chosen).** One process of flume's, which the service manager starts and restarts. It writes the host's watch at each of its settles, as `flume settle --every` does.
2. **The runner, as before.** It cannot hold a subscription: each `flume settle --every` ends once its settle does.

For how the resident hears sluice:

1. **One subscription on sluice host's socket (chosen).** The resident asks sluice host as a resident asks another, by its socket, and starts no sluice command for it.
2. **`sluice overview --json --watch --stdin` as a child.** It is the same subscription, but the resident would run a client of another tool, which the accepted decision that residents ask each other by socket rules out.

For what is a change:

1. **A move of a branch's records' `version`, its `revision` or its `gone` in a followed repository's overview (chosen).** sluice moves `version` on every write of a branch's records, a submission, a landing, a blame or a conflict among them, and `revision` where the base moved. Candidates under test, the room and the sessions move far more often and decide nothing for Flume.
2. **Every overview sent.** During a run, sluice sends an overview each second.

## Decision outcome

- **`flume host` keeps watch.** It settles the places where something is pending, as a watch's settle settles them, at once, at each change it hears, and a minute after its last settle where it heard none. Each settle writes the host's watch with a bound of a minute, so Flume runs attended while the resident runs (119, 208, 225; core.62).
- **It holds one subscription to sluice host.** It follows the repository of each live place whose submission is in flight and of each base place, read from held state alone, and no other. Where it follows none, it asks sluice host nothing. Where no sluice host answers, or the subscription ends, it holds the subscription again a minute later. The settles each minute go on meanwhile. It starts no sluice host: the service manager starts each resident.
- **Changes heard together make one settle.** A change heard is followed by a settle two seconds later, and changes heard while a settle runs make one settle after it (one-read-in-flight).
- **It runs once on a host.** It holds a lock in flume's state folder, and a second `flume host` is refused.
- **The host module declares it.** nix-darwin's module gives the primary user the launchd agent `flume-host`. NixOS's module gives `services.flume.user` the user unit `flume-host`. Each runs the module's package's `flume host`, starts it at load and again when it ends, and leaves a terminal's server that a start brought up standing.
- **The edge is tokio's, and the core stays synchronous.** Each settle builds the core afresh and runs on tokio's blocking pool. A runtime of one thread runs the timers and the subscription. The subscription is the sluice profile's code (core.1).

### Rule core.13

Before: Flume starts no process that outlives a command of its own. No command of Flume's waits on the merge queue. Every settle runs inside a command, started by an agent going quiet, by a caller keeping watch over the host or by any other caller. A terminal or a plugin of one never starts a settle. Whether Flume runs attended is read from the host's watch alone.

Now: Flume starts no process that outlives a command of its own, and `flume host`, its resident, is a command of its own that the host's service manager runs. The resident keeps watch over the host with one subscription to the merge queue's resident, settling at each change it hears and a minute after its last settle. No other command of Flume's waits on the merge queue. Every settle runs inside a command, started by an agent going quiet, by the resident or another caller keeping watch, or by any other caller. A terminal or a plugin of one never starts a settle. Whether Flume runs attended is read from the host's watch alone.

### Consequences

- Rule core.13 and its details hold the outcome.
- The runner's `flume --json settle --every 60s` each minute is no longer needed for a host that runs `flume host`. The runner keeps it until its own change stops it. While both run, a place can be settled by each, and each settle is safe to repeat (74).
- A landing is read seconds after sluice writes it, where it was read up to a minute later.
- A host whose service manager runs no `flume host` keeps the watch a caller gives, as before.
- Statement clause 120, which says Flume has no daemon, reads as the accepted decision that flume has a resident amends it. New design is a claim in Boom, so the statement is not changed here.
