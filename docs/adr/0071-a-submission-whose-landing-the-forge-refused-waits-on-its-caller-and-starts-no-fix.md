---
number: 71
title: A submission whose landing the forge refused waits on its caller and starts no fix
status: accepted
date: 2026-11-16
decision-makers:
- boomdrive-design
---

# A submission whose landing the forge refused waits on its caller and starts no fix

Status: accepted, boomdrive-design's. The statement's 112, 130 and 131, rules core.24 and core.27, and sluice's record 0124 decide it, and it is cheap to change.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 99, 112, 124, 130, 131, 133 and C.6's inbox and read-submission rows, as `8b73d11` amends them on its branch `design/flume-reads-a-forge-refusal`; sluice's statement 152 and its record 0124, built by sluice's change `a-runs-life-is-written-first-and-actions-runs-its-checks`, landed on `bolt/sluice-is-refactored-and-runs-on-pools` at `44e33d4` (its specs `refused-landing` and `submission-wait`); `crates/flume/src/profiles/merge_queue/sluice/mod.rs` on flume's main at `ae7b42c7`, whose inbox runs `sluice inbox --confirm --json` and keeps only `landed`, `failed` and `conflict`, and whose read of a submission's state refuses any other; record 0012; rules core.6, core.24 and core.27.

## Context and problem statement

sluice now holds a landing the forge refuses, as when push protection finds what looks like a secret or a rule on the branch forbids the push. Each submission the landing carried is `refused`, with the forge's words. sluice blames none of them and decides no outcome for any. It tells each submitter, under `<id>:refused`, as it tells an outcome. The landing stays held until one of its submissions leaves or the operator pushes it again.

flume knows three decided outcomes: landed, failed and in conflict. Its inbox confirms everything sluice hands it and then keeps only those three, so a refusal would be confirmed and lost. Its read of a submission's state would read `refused` as not understood. The place would show as submitted and undecided, with nothing to say why, while its landing held the branch.

## Considered options

1. **Held, waiting on its caller.** The refusal is read as its own state, with the forge's words. It starts no stage and counts toward no bound. The place waits on its caller (130) while its submission stays open and is read at each settle (chosen).
2. **Read as a failed submission.** The refusal starts the fix round its kind names, with the forge's words as its brief (112, 131), as a failure does.
3. **Read only in status.** The place stays submitted and undecided, and status shows the forge's words. Nobody is told, and the place never waits on its caller.

## Decision outcome

Option 1.

- **Neither failed nor landed.** A held submission starts no stage and no base place's fix, and counts toward no bound. sluice chose not to blame a held landing's submissions, since the forge may refuse every landing for a reason of the forge's own, such as sluice's identity losing the right to push. Option 2 would blame each place in turn, start a fix agent with nothing in its work to change, and, where one landing carries several places' submissions, start a fix in each of them when at most one holds what the forge refused.
- **The caller decides.** The place waits on its caller with the forge's words, and stalls and is delivered as any such wait is (130). The caller has three moves. A retry naming a stage has its agent take out what the forge refused, and the submission that follows withdraws the held one (sluice 67). A drop ends the place. Or the operator, once the forge's rule allows the landing, pushes it again. Option 3 tells nobody, and the branch stays held until someone happens to read status.
- **Still open.** The submission stays open while held. A landing the operator pushed again is read as landed at the next settle and makes the place's work over (113). A held landing dropped because another of its submissions left puts this one back in the queue, and the place's wait on its caller ends.
- **Nothing owed is lost.** No outcome sluice hands over is confirmed until flume has read it as one of its states. One it cannot read stays owed to the place and is named in status, so a state sluice adds after this one is never confirmed and dropped.

### Consequences

- A caller sees a place held by the forge as a wait of its own, with the forge's words, and needs no new kind of notice.
- A refusal whose cause is the place's work takes one caller's retry to clear, where option 2 would clear it with none.
- flume's sluice profile reads `refused` from the sluice release that carries sluice's record 0124. A host installs that release only once flume reads it.
