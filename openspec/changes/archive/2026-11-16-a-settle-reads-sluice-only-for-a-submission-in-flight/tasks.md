# Tasks

The work is on `fix/main--a-settle-reads-sluice-only-for-a-submission-in-flight`,
cut from `main`. The statement is read at boomdrivedev/blueprints
`design/flume-a-settle-reads-sluice-only-for-a-submission-in-flight` 8a6b9d5
(111, 133); record 0012 as amended 2026-11-16.

## 1. The settle

- [x] 1.1 Collect the inbox in the submission's step only while the place's submission is open, by `submission_open` (111, 133). Verify with `sluice exec main -- moon run conformance:test`, "a settle asks the merge queue's inbox only while the place's submission is open".
- [x] 1.2 Collect the inbox once before a submission made after an outcome, by the place's delivery or its caller's retry, so nothing pending from before decides the new one (99, 101, 133). Verify with the same suite, that scenario and "a request's entry that failed sends the place back once".
- [x] 1.3 Name the inbox's unread words and passed-over branches in status only while the submission is open (68, 133). Verify with the same suite, "an inbox that cannot be read" and "a branch the merge queue cannot read".

## 2. The records

- [x] 2.1 Record 0012 carries the amendment line, and core.6's detail rules out the inbox asked for a place with no submission open (core.6; record 0012).

## 3. The spec

- [x] 3.1 Validate with `openspec validate a-settle-reads-sluice-only-for-a-submission-in-flight --strict`, then archive with `openspec archive a-settle-reads-sluice-only-for-a-submission-in-flight --yes`.
