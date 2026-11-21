# Tasks

Each task names the narrowest check that shows it done. Design sections
are this change's `design.md`.

## 1. Scenarios side by side

- [x] 1.1 In `crates/flume-conformance/src/scenario.rs`, add `pass_all_side_by_side`, sharing the failure report with `pass_all` (design 1). Check: `cargo test -p flume-conformance`.
- [x] 1.2 Run the herdr and Rex profiles' terminal scenarios on it, five at a time. Check: `cargo test -p flume --features tools --test tools -- --exact terminal_herdr::the_herdr_profile_passes_the_terminal_scenarios terminal_rex::the_rex_profile_passes_the_terminal_scenarios`.

## 2. A harness waits only for what is awaited

- [x] 2.1 Give `TerminalHarness::received` the inputs awaited, and `never_up_seam` a default of the harness's seam; the fake, printed, herdr and Rex harnesses follow, herdr's never-up seam at 3,001 milliseconds and Rex's at four seconds (design 2). Check: `cargo test -p flume --test profiles`.

## 3. The herdr trust reading

- [x] 3.1 In `crates/flume/src/profiles/terminal/herdr/mod.rs`, add `with_trust_reading(draw, settle)` in place of `TRUST_DRAW` and `TRUST_SETTLE` in `settled`, and give the herdr tests' profiles no draw and a settle of four seconds (design 3). Check: `cargo test -p flume --features tools --test tools -- terminal_herdr::`.

## 4. The pinned program read once

- [x] 4.1 In `crates/flume/tests/profiles/tools/claude_words.rs`, read the program once and find each fragment by `memchr::memmem` (design 4). Check: `cargo test -p flume --features tools --test tools -- claude_words::`.

## 5. Rex's keys over four blocks

- [x] 5.1 In `crates/flume/tests/profiles/tools/terminal_rex.rs`, type the every-key test's shares into four blocks side by side (design 5). Check: `cargo test -p flume --features tools --test tools -- --exact terminal_rex::relay_types_each_of_flumes_key_names_into_a_block`.
