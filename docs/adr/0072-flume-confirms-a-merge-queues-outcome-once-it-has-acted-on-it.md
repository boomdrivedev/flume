---
number: 72
title: flume confirms a merge queue's outcome once it has acted on it
status: proposed
date: 2026-11-15
decision-makers:
- Chuck Swanberg
- the code stage of the fix main--flume-confirms-a-sluice-outcome-it-acted-on
---

# flume confirms a merge queue's outcome once it has acted on it

Status: proposed. The brief of the fix `main--flume-confirms-a-sluice-outcome-it-acted-on` asked for it; its statement is flume's 101, 111, 124, 133 and the new 225 with C.6's inbox and confirm rows, as boomdrivedev/blueprints `aa710c6` amends them on its branch `design/flume-confirms-an-outcome-it-acted-on`, which waits on the user's push. The choices below that the brief does not name are the code stage's own, made on the rules written and cheap to change.

Amended 2026-11-16, proposed, by the code stage of fix `main--a-settle-reads-sluice-only-for-a-submission-in-flight` as it took this record in: decision 4's outcome left owed is given again at the next inbox a settle asks, as 225 says, and not at every settle. Record 0012 as amended the same day asks the inbox only while the place's submission is open, and once as flume submits it anew (111, 133). So the settle after an outcome acted on asks no inbox. An outcome whose confirm the merge queue could not take, flume holding its verdict, is given again and confirmed as flume submits the place anew. A landing's is left owed once the place is tidied. The path not taken: a mark in the place's state keeping the inbox read while a confirm is owed. Clause 111 as amended asks no inbox while flume holds the outcome, and a confirm the merge queue cannot take changes nothing flume decides.

Amended 2026-11-18, proposed, by the code stage of fix `flume-leaves-the-merge-queue-to-sluice-1` as it took this record in beside record 0071's held landing: the profile reads sluice's `refused` as held (record 0071), and a held outcome, which decides nothing, is acted on by the place's wait on its caller and confirmed once the place's state holds that wait, or holds nothing for it to hold where the place no longer waits on that submission alone; an earlier submission's is confirmed with no act, as any outcome's is. The merge queue's record gives a held submission again at each settle that reads it, so the confirm loses nothing (112, 133; core.58). The path not taken: a held outcome left owed until its submission lands, returns to the queue or leaves, which hands it over again at every inbox, the repeats this record ends.

Deciders: the brief, in its words: "Confirm each outcome once Flume has acted on it (recorded the submission's end and handed it to the place's recipients), so a later settle is handed only what is new; a test shows a second settle hands over no outcome already acted on." The code stage of that fix, on the rest.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 74, 99, 101, 111, 112, 124, 133, 225 and C.6 at `aa710c6`; sluice 0.93.2's `sluice inbox` and `sluice confirm`, their help and exits read on deneb on 2026-11-15; sluice 51 and sluice record 0102; the change `the-sluice-profile-confirms-what-it-collects`; record 0071; rules core.2, core.6, core.17 and core.58.

## Context and problem statement

Since 0.79.0 sluice keeps each outcome owed to the worktree it decides until its reader confirms it, and hands it over again at every `sluice inbox` until then (sluice 51; sluice record 0102). flume 0.21.2, which deneb ran, collected by `sluice inbox --json` and confirmed nothing, so sluice handed eight places' held outcomes over 3,322 to 60,862 times each, every read holding the repository's fetch lock and reaching GitHub; every flume command on deneb queued 10 to 18 minutes behind the others. The change `the-sluice-profile-confirms-what-it-collects` collected by `sluice inbox --confirm`, which confirms each outcome as sluice hands it over. That stops the repeats, but confirms an outcome before flume has acted on it: a settle whose write of the verdict is refused, or that ends between the read and the write, loses a failure for good, since read-submission sends a place's own failed entry back nowhere. And the outcomes carried no submission, so an outcome handed again after the place was submitted anew would have been read as the new submission's.

## Considered options

1. **When flume confirms.** (a) On receipt, `sluice inbox --confirm`. (b) Once the place's state holds what the outcome decided, by a `confirm` of the merge-queue seam, so an outcome a settle could not act on comes to the next.
2. **Where the confirm lives.** (a) In the sluice profile, after the core answers. (b) A seam operation the core calls, each outcome carrying the key it is confirmed by.
3. **An outcome of an earlier submission.** (a) Acted on, as today, wherever the verdict is clear. (b) Read by the submission it decides: only the place's own submission's is acted on, and any other is confirmed with no act.
4. **A confirm the merge queue cannot take.** (a) Named in status. (b) Left owed, the next inbox giving it again and the core acting on it no more than once.
5. **sluice's exit 1, nothing owed under the key.** (a) A refusal. (b) Confirmed.

## Decision outcome

1. **(b), the brief's.** The verdict's write records the end it makes for the place's recipients in the same write (held state's `record_ends`), so a held verdict is the act done; the confirm follows it. An act is safe to repeat (74), so an outcome given twice is acted on once.
2. **(b).** The core alone knows when it has acted, and a profile implements the seam and nothing of the core (core.2). The seam gains `confirm(place, key)`, the inbox's outcomes become `Owed { submission, outcome, key }`, and a decided submit carries its `key`. A merge queue that keeps nothing to confirm gives no key.
3. **(b).** An outcome decides the place only where its submission is the one flume last made, or, while the place's request is not yet read taken in, any entry but the one before it (99, 124). Under (a), an outcome handed again after the place was sent back and submitted anew would send it back again.
4. **(b).** A confirm that fails is retried at the next settle by the same path; writing its words into the place's state each settle would make every settle a write.
5. **(b).** sluice hands nothing over under a key it holds no outcome for.

### Consequences

- Rule core.6's details rule out an outcome confirmed before the place's state holds what it decided, and an earlier submission's outcome acted on.
- Good: a later settle is given only what is new, and an outcome is lost to no refused write or ended settle.
- Good: the eight places' unconfirmed outcomes on deneb are confirmed at the first settle of a flume with this record, an earlier submission's sending nothing back and the current one's acted on at most once.
- Bad: one more sluice call for each outcome, once.
- Holds record 0071's last point and core.58's: the profile gives the core only the states it reads, landed, failed and in conflict, and nothing is confirmed but what the core acted on, so a state it does not read, sluice's `refused` among them, stays owed. Reading `refused` as held, and naming it in status, is record 0071's to build.
- Elsewhere: deneb runs flume 0.21.2, which confirms nothing; the repeats end only once the host runs a flume with this record.
