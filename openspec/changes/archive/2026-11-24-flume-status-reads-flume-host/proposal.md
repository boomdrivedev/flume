# flume status reads flume host

## Why

On deneb, on 2026-11-24, about a dozen sluice processes started and
ended all the time. `boomdrive run` runs `flume --json status` about
each 15 seconds. Each status reads the merge queue of each repository,
and each read starts `sluice overview`, and `sluice status` for each
base. The accepted invariant `one-resident-per-tool` names this as a
break, and `reads-go-through-the-resident` says that each read of the
state of a tool goes to the resident of that tool.

## What Changes

- **`flume host` serves status on its socket.** It answers `status`
  with one document, and `subscribe` with a document at once and one at
  each change. It reads status again after each settle, after each change
  that its subscription to sluice host hears, and after a file event in
  flume's state folder (Boom decision
  `flume-host-serves-status-and-its-subscription-on-its-socket`).
- **`flume status` asks `flume host`.** It builds no core and starts no
  process. Where no `flume host` answers, it is refused, its words naming
  the socket and the service manager. The MCP tool `status` asks the same
  way.
- **flume reads sluice's records on sluice host's socket.** The sluice
  profile asks sluice host for `overview` and `status <base> --all`, and
  keeps the records of each base until their `version` moves. It starts
  no sluice command for a read and no sluice host (Boom decision
  `flume-reads-sluices-records-on-sluice-hosts-socket`).
- **The tree pins sluice 0.110.0**, whose sluice host answers those asks.
- **The tests run the resident with its watch**, and `sluice daemon`
  where they read sluice, as a host's service manager runs them (Boom
  invariant `a-tool-has-no-mode-that-only-its-tests-use`).
- **Rule core.15 amended**: flume host keeps a base's records between
  asks, until their version moves.

## Capabilities

### Modified Capabilities

- `command-line`: `flume host` serves its socket; `flume status` asks it.
- `place-status`: status is read by the resident; a base's records kept
  until their version moves.
- `merge-queue-profile`: the reads ask sluice host's socket; sluice host
  runs under the host's service manager.
- `settling`: the resident reads status again for its subscriptions.

## Impact

- `crates/flume`: `src/served.rs` (the socket, its server and its
  client), `src/resident.rs`, the `status` and `host` commands, the MCP
  tool `status`, and the sluice profile's reads
  (`src/profiles/merge_queue/sluice/ask.rs`); `notify` added.
- The tests' harnesses: `tests/common/resident.rs`, and each harness
  that asks status or reads sluice.
- `devenv.nix`: sluice 0.110.0.
- `docs/architecture/core.md` and `core-details.md` (core.15).
- boomdrive's runner holds one subscription to `flume host` in place of
  running `flume --json status`, in a change of its own.
