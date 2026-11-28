# The inbox asks sluice host

## Why

`flume host` settles each place where something is pending, and each
place with an open submission collects its inbox at each settle. The
sluice profile collects it by `sluice inbox --json` in the place, so
each such place starts one sluice process at each settle. On deneb on
2026-11-27, with 11 places' submissions open, those processes were 1360
of the 3135 samples of flume host's profile. Every other read of the
profile already asks sluice host's socket and starts no sluice process
(flume-reads-sluices-records-on-sluice-hosts-socket;
residents-ask-each-other-by-socket). sluice host serves the `inbox` read
on its socket from sluice 0.104.0, and the tree pins 0.110.0.

The inbox is also how a settle starts sluice host again where its
process ended with work in play (S109; sluice 135). A sluice command
starts it on a bare install, as sluice's own commands do, and a read on
the socket starts nothing (the-service-manager-starts-each-resident).

## What Changes

- **The inbox asks sluice host's socket.** The profile asks sluice host
  the `inbox` read that `sluice inbox --json` in the place asks: the
  place's folder as its folder and its worktree, the JSON answer,
  confirming nothing, and the `SLUICE_AGENT` the command would carry as
  its caller where one is set. It reads the answer's `output` as it
  read the command's. While sluice host answers, no inbox starts a
  sluice process.
- **Where no sluice host answers, the inbox runs `sluice inbox --json`.**
  A socket with no host, an ask with no answer or a refusal of the ask
  falls back to the command in the place, which starts sluice host on a
  bare install and refuses where a service manager owns it. So S109
  holds as it did: a settle after sluice host's process ended starts
  it again.
- **The stand-ins answer the inbox on the socket.** The stand-in sluice
  host of the documents' tests answers `inbox` from its fixture, and
  the stand-in host of a base that takes pull requests answers it with
  no outcomes, as its stand-in `sluice` did, sluice delivering them to
  the request.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: a new requirement, "The inbox asked of sluice
  host": the inbox is asked of sluice host's socket, and is `sluice
  inbox --json` only where no sluice host answers. The requirements
  that named the inbox a sluice command now name it as that one
  reads it.

## Impact

- `crates/flume/src/profiles/merge_queue/sluice/ask.rs`: the `inbox` ask.
- `crates/flume/src/profiles/merge_queue/sluice/mod.rs`: the inbox asks
  the socket and falls back to the command; two unit tests.
- `crates/flume/tests/profiles/merge_queue_sluice_documents.rs` and
  `crates/flume/tests/profiles/tools/walk.rs`: the stand-in hosts answer
  the inbox.
- No new crate, plugin or site part, so no new moon project.
