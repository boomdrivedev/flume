# An outcome handed again holds no stage

## Why

Eight places on deneb stayed `code · started` for 9 to 33 hours on
2026-11-14 and 2026-11-15, every task ticked, the deliverable committed
and herdr reading the agent waiting; `flume settle --on-quiet` ended
none. Each pane showed `/reload-plugins plannotator`, which Plannotator's
auto-reload typed into the idle session after the agent's last turn.

The reload held none of them. Read on deneb on 2026-11-15, the stuck
start of `change--the-runners-stop-ends-its-calls-in-flight` (code, run
4) has its started ack, and its received ack holds the digest of the
prompt flume sent; asking, own work and the agent's own record read as a
quiet agent's. Its held state shows the start never read begun: no
record, no `begun_at`, its head never read moved. Every stuck start was a
code start after a send-back, its place's earlier submission failed or
in conflict: `fix--main--a-refused-request-leaves-the-runners-reports-going`,
`change--a-change-that-skips-specs-is-planned`,
`change--a-stage-may-declare-that-it-hibernates` and
`change--the-hosts-collector-reaches-sluice-and-flume` each held a code
start with no record and no `begun_at`.

The hosts run flume 0.21.2 and sluice 0.93.2. That sluice keeps an
outcome owed until its reader confirms it, and 0.21.2's profile never
confirmed one: `sluice inbox --json` in the stuck place still hands
`230:failed`, owed and never confirmed. The settle's submission step
collects first, and counted every outcome handed as a change, though
flume had acted on it already and wrote nothing; so the step ended
there, and each of the settle's 64 steps collected the same failure
again. No step read the stage's prompt received or its end.

main's profile confirms on receipt since 0.22.1. Sluice still hands an
outcome again wherever a confirm did not reach it (sluice 51), and while
a merge queue hands one again the settle must still read the place's
stage (40, 46).

The statement holds as it stands at boomdrivedev/blueprints bf9e16f, as
the design ruled: a stage whose deliverable is in place ends at the
first settle that reads its agent quiet, whatever ran in its session
after its last turn (40, the glossary's quiet, 46, C.4's started row).

## What Changes

- **Collecting changes something only where the place's state moved
  (46, 74, 133).** The submission's step reads the place's held version
  before and after it collects; an outcome flume acted on before, handed
  again, writes nothing, so the step goes on to the submission's record
  and the stage's step. The held version is the measure because
  `update` writes only a state that differs, so it moves exactly where
  an outcome was acted on; a settle beside it that writes costs one more
  step and nothing else. Two other ways were not taken: collect's own
  answer, which the change confirming each outcome once acted on rewrites
  on its own branch; and the stage's step taken after every submission
  step, which would read the stage but still leave a due submission
  unmade while an outcome is handed again, since collecting comes first
  in the submission's step.
- **A conformance scenario** binds a merge queue that hands each
  outcome again at every inbox, and shows a change sent back by a failed
  submission has its code ended in the settle that reads it delivered
  and quiet, the failure acted on once.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `settling`: an outcome handed again, one already acted on, counts as
  no step taken, with a scenario of a failure handed again.
