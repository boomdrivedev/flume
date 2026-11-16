# Tasks

The work is on `fix/main--flume-confirms-a-sluice-outcome-it-acted-on`,
cut from `main`. The statement is read at boomdrivedev/blueprints
`aa710c6`, on `design/flume-confirms-an-outcome-it-acted-on` (101, 111,
124, 133, 225, C.6); sluice's at v0.93.2 (sluice 51; sluice record
0102). Record 0072 holds the choices.

## 1. The seam and the fakes

- [x] 1.1 Give the merge-queue seam `confirm(place, key) -> Confirmed`, the inbox's outcomes as `Owed { submission, outcome, key }` and a decided submit's `key`, with their round trips (133, 225; core.2). Verify with `sluice exec main -- moon run seams:test`.
- [x] 1.2 Keep the fake merge queue's outcomes under `<id>:<state>` until confirmed, a conflict at submit owed to the place, `leave_pending` naming its submission, and `refuse_confirms`; give the fake store `refuse_writes_of` (225). Verify with `sluice exec main -- moon run fakes:test`.

## 2. The core

- [x] 2.1 Have a settle confirm each outcome, and a decided submit's, once the place's state holds what it decided; confirm with no act an outcome of a submission other than the one flume last made; leave unconfirmed one whose act was not written (111, 112, 124, 225; core.6). Verify with `sluice exec main -- moon run conformance:test`, the scenario "a failed submission's outcome is confirmed once flume acted on it".

## 3. The sluice profile

- [x] 3.1 Collect by `sluice inbox --json`, each outcome with its record's `id` and its `delivery`'s key where owed; confirm by `sluice confirm <key> --json` in the place, exits 0 and 1 confirmed; answer a decided submit with its key and confirm nothing in the profile (101, 133, 225, C.6). Verify with `sluice exec main -- cargo test -p flume --features tools --test tools merge_queue_sluice`.
- [x] 3.2 Have the walkthrough of a failed submission read the failed entry's delivery `done` once the place went back to code, and a second settle send nothing back. Verify with `sluice exec main -- cargo test -p flume --features tools --test tools walkthroughs::a_failed_submission_goes_back_to_code_and_lands_on_the_second`.

## 4. The clause list, the rule, the site

- [x] 4.1 Read the statement at `aa710c6` in `crates/flume-conformance/src/clauses.rs`, 225 in Part B (core.19). Verify with `sluice exec main -- moon run conformance:coverage`.
- [x] 4.2 Amend core.6's details with record 0072, and the site's merge-queue guide's reading of the outcome (core.35). Verify with `sluice exec main -- moon run site:prose`.

## 5. The spec

- [x] 5.1 Validate with `openspec validate flume-confirms-an-outcome-once-it-acted-on --strict`, then archive with `openspec archive flume-confirms-an-outcome-once-it-acted-on --yes`.
