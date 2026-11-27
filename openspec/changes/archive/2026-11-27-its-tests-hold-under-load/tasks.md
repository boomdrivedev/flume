# Tasks

Each task names the narrowest check that shows it done. Each check runs
as `sluice exec main -- <check>`.

## 1. The core

- [x] 1.1 In `crates/flume-core/src/agent_acts.rs`, a stop-short reads the prompt's receipt from the agent where no settle recorded it. It records the receipt with the end. A conformance scenario stops short before any settle reads the receipt. Check: `cargo test -p flume-conformance --test engine`.
- [x] 1.2 In `crates/flume-core/src/deliver.rs`, a settle records a receipt or an agent's word that it started only while its start runs. In a conformance scenario, a second settle ends the start lost before the first settle writes. Check: `cargo test -p flume-conformance --test engine`.

- [x] 1.3 In `crates/flume/src/profiles/terminal/herdr/mod.rs`, the profile reads an empty foreground again until the start's deadline. Check: `cargo test -p flume --features tools --test tools -- terminal_herdr:: open::`.

## 2. The tests

- [x] 2.1 In `crates/flume/tests/profiles/tools/terminal_rex.rs`, the stand-in sets its terminal before it reports idle. One `cat` drains the terminal into a pipe. One field split cuts a line at its Enters. The long-text test's stand-in reads three seconds after it reports idle. Check: `cargo test -p flume --features tools --test tools -- terminal_rex::`.
- [x] 2.2 In `crates/flume/tests/profiles/tools/open_sessions.rs`, s104's code stops short only after the test read its agent and asked for the second fix. Check: `cargo test -p flume --features tools --test tools -- open_sessions::s104`.
- [x] 2.3 In `crates/flume/tests/profiles/window_ghostty.rs` and `window_terminal.rs`, each harness waits for the profile's own bound. Only a program that never answers gets two seconds. Check: `cargo test -p flume --test profiles -- window_`.

## 3. The sweep

- [x] 3.1 Run `flume:tools` and `flume:test`, two copies of each at once, round after round on a busy host. Check: in the last rounds, no test fails but the pull request walkthrough that another change fixes.
