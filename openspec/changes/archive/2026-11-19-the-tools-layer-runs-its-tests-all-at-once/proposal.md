# The tools layer runs its tests all at once

## Why

Main's gate requires `flume:tools`, 183 tests, and the gate cut it off
at ten minutes where a run in a session's shell takes under two. Run
once each way on deneb at 05e0dd24 with libtest's per-test times:

| | Wall time | Test time, summed |
|---|---|---|
| `cargo test` in the devenv shell | 111 s | 1,447 s |
| the same under `sluice exec main`, the gate's sandbox | 581 s | 1,159 s |

No test is slower under the gate: the ten longest each took the same
or less (`open_sessions::s104…` 40.4 s against 44.4 s). The gate ran
them two at a time. A run sluice starts is handed `SLUICE_JOBS`, the
host's cores over its runs at once, 12 over 5 on deneb, and
`devenv.nix` sets `RUST_TEST_THREADS` to it, so libtest ran two tests
at a time: 1,159 seconds over two threads is the 581 seconds the run
took. The tools tests spend their time waiting on herdr's and Rex's
sessions, the stand-in's answers and the walkthroughs' settles, not on
a processor, so the share of the cores is the wrong bound for them.

## What Changes

- **`flume:tools` runs its tests all at once.** The task clears
  `RUST_TEST_THREADS` before `cargo test`, so libtest takes as many
  tests at once as the host has processors, as in a session's shell.
  Builds keep `CARGO_BUILD_JOBS` from `SLUICE_JOBS`, and every other
  task keeps `RUST_TEST_THREADS`.

No test, scenario or clause changes what it holds.
