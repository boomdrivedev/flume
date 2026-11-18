# Tasks

The work is on `fix/main--a-settle-reads-sluice-only-for-a-submission-in-flight`,
cut from `main`. The statement is read at boomdrivedev/blueprints
`design/flume-a-settle-reads-sluice-only-for-a-submission-in-flight` 8a6b9d5
(111) and `design/flume-confirms-an-outcome-it-acted-on` aa710c6 (225);
records 0012 as amended 2026-11-16 and 0072.

## 1. The settle

- [x] 1.1 The scenario "a failed submission's outcome is confirmed once flume acted on it" reads no inbox at the settle after the failure is confirmed, and confirms the failure whose confirm was refused as flume submits the place anew (111, 133, 225). Verify with `sluice exec main -- moon run conformance:test`.

## 2. The records

- [x] 2.1 Record 0072 carries an amendment line naming the next inbox, and record 0012's amendment and core.6's detail name the confirm made as flume submits the place anew (core.6; records 0012, 0072).

## 3. The spec

- [x] 3.1 Validate with `openspec validate an-outcome-whose-confirm-failed-waits-for-the-next-inbox --strict`, then archive with `openspec archive an-outcome-whose-confirm-failed-waits-for-the-next-inbox --yes`.
