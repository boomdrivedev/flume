# flume hears sluice by subscription

## Why

A merge queue that decides a submission calls flume nothing, so a
caller keeping watch looks for it (120, 208). On deneb that caller is
the runner, which runs `flume --json settle --every 60s` each minute.
Each settle with a submission in flight runs `sluice overview --json`
and `sluice status <base> --all --json`, whether or not sluice moved,
and a landing waits up to a minute to be read. The accepted decision
`flume-has-a-resident` gives flume one process of its own, `flume
host`, and the accepted decision
`residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop`
says how a resident learns of a change: from events, with a poll each
minute only where no event came. sluice host serves one subscription
for each client on its socket.

## What Changes

- **`flume host`, flume's resident.** A command the host's service
  manager runs and keeps running. It keeps watch over the host: it
  settles the places where something is pending as a watch's settle
  does, at once, at each change it hears, and a minute after its last
  settle where it heard none, each settle writing the host's watch
  (208, 225; core.62). A second one on the host is refused.
- **One subscription to sluice host.** It follows the repository of
  each live place whose submission is in flight and of each base place,
  on sluice host's socket, and settles when a followed repository's
  records move. It starts no sluice command and no sluice host. Changes
  heard together make one settle.
- **The host module runs it.** nix-darwin's module declares the launchd
  agent `flume-host` for the primary user; NixOS's declares the user
  unit `flume-host` for `services.flume.user`. Each runs the module's
  package's `flume host`, started at load and again when it ends.
- **Rule core.13 amended** by record 0082, proposed: `flume host` is
  the one process of flume's between its commands, and the one command
  that waits on the merge queue, by its subscription.

The runner's `flume --json settle --every 60s` is the runner's own to
stop, in a change of its own; until then both keep watch, and each
settle is safe to repeat (74).

## Capabilities

### Modified Capabilities

- `settling`: the resident keeps watch and hears its merge queue by one
  subscription.
- `command-line`: `flume host`.
- `host-module`: each platform declares the resident's service.

## Impact

- `crates/flume-core`: `Engine::followed`, the repositories a change
  in their merge queue can move a place of.
- `crates/flume`: the resident's tokio edge (`src/resident.rs`), the
  sluice profile's subscription
  (`src/profiles/merge_queue/sluice/subscription.rs`) and the `host`
  command; tokio, tokio-util and sha1 added.
- `nix/modules/darwin.nix`, `nix/modules/nixos.nix` and their tests.
- `docs/adr/0082-…`, `docs/architecture/core.md` and
  `core-details.md` (core.13).
