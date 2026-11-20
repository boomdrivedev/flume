# Tasks

## 1. The tools task

- [x] 1.1 In `crates/flume/moon.yml`, make `tools` a script that clears `RUST_TEST_THREADS` before `cargo test -p flume --features tools --test tools`, with a comment saying why (design 1). Check: `moon task flume:tools` shows the script, and `moon run root:layers` passes.
