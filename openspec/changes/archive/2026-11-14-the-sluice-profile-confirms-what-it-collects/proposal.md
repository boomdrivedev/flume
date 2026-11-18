# The sluice profile confirms what it collects

## Why

Every flume place's checks failed on deneb before any check ran. The
tree pinned sluice 0.71.0, and that sluice refuses the host's
`~/.config/sluice/config.toml`, which swancloud writes for the sluice
the hosts run, 0.93.2: "unknown field `credential`, expected one of
`app_id`, `key`, `discovery`" at `[forge]`, so `devenv shell -- sluice
exec <base>` exits 3.

Moving the pin to 0.93.2 broke the merge-queue profile. Since 0.79.0
sluice keeps an outcome owed until its reader confirms it, and hands it
over again at every `sluice inbox` until then (sluice 51; sluice record
0102). That record names flume's profile as a caller that keeps what it
is given and passes `--confirm`, which the profile never did. Under
0.93.2 each settle collected the same landing, failure or conflict
again, nine of the tools layer's tests failing, the walkthroughs that
send a place back among them.

The statement is amended to match at boomdrivedev/blueprints
`design/flume-confirms-sluices-outcomes` (C.6's two inbox rows); 101 and
133 hold as they stand once the profile confirms on receipt.

## What Changes

- **The pin.** `devenv.nix` pins sluice 0.93.2 with its three release
  checksums, the release the hosts run, so the tree's sluice reads the
  host's configuration (core.22).
- **The inbox confirms on receipt (101, 133, C.6).** The profile
  collects by `sluice inbox --confirm --json`.
- **A submit's decided answer is confirmed (112, 133, C.6).** Where
  sluice answers a submit with a decided entry's record, the profile
  runs `sluice confirm <key>` of the record's `delivery` where it is
  owed, where the submit ran. Until then an inbox beside it may hand the
  outcome over too, and the core acts on it once, as it does any outcome
  two settles read.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: outcomes are collected by `sluice inbox
  --confirm --json`, and a submit's decided answer is confirmed by its
  delivery's key.
