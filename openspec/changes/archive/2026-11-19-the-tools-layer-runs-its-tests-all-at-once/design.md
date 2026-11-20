# Design

## 1. Where the bound is cleared

`devenv.nix` gives a sluice run `CARGO_BUILD_JOBS` and
`RUST_TEST_THREADS` from `SLUICE_JOBS`, so runs the host lets go at once
share its cores. That fits a build and the fast layer's tests, which
use a processor the whole time. The tools tests wait: a run with every
test at once used 1,447 seconds of test time in 111 seconds of wall
time, about thirteen tests at once, for a quarter more test time than the
1,159 seconds two at a time took.

The task clears the variable itself, as a moon `script`, rather than
`devenv.nix` dropping it, so the one task whose tests wait is the one
that runs them all at once. A fixed `--test-threads` would name a count
for one host; libtest's own count is the host's processors.

## 2. What was ruled out

The gate's sandbox refused nothing a test waits on. A `log stream` of
Sandbox reports through the run showed the run's mark on cargo's
writes to `~/.cargo` package locks, a test herdr server's update check
(`curl` to herdr.dev, refused its name lookup) and its writes under the
user's `~/.local/state/herdr`; each fails at once and no test's time
grew. sluice's executor bounds no processor: its `--concurrency` is
moon's, and `cargo test` is one moon task.
