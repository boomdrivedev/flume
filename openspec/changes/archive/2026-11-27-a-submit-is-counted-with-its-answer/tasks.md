# Tasks

## 1. The held state and status

- [x] 1.1 In `crates/flume-seams/src/held.rs`, add `submitting` to `SubmissionHeld`. Add `SubmissionHeld::made`. Add `submitting` to `SubmissionStatus` in `crates/flume-core/src/answer.rs` and give it in `status.rs`. Check: `sluice exec main -- moon run seams:test core:test`.

## 2. The submit

- [x] 2.1 In `crates/flume-core/src/submit.rs`, make the claim set `submitting` with no count (design 1). Make `submitted` count the submit and record the answer, a refusal among them, in one write. Read `made` in `line.rs`, `telemetry.rs` and `waits_on_its_submission` (design 2). Check: `sluice exec main -- moon run core:test core:lint`.

## 3. The scenario

- [x] 3.1 In `crates/flume-conformance/src/engine/running_through.rs`, add the scenario that reads the place before each write (design 3). It reads a request, an entry and a refusal. Add `submitting` to the answers' form. Check: `sluice exec main -- moon run conformance:test`. Against the `submit.rs` of main, the scenario fails.
