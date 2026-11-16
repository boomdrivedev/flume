# flume confirms an outcome once it acted on it

## Why

sluice keeps each outcome owed to the worktree it decides until its
reader confirms it, and hands it over again at every `sluice inbox`
until then (sluice 51; sluice record 0102). On deneb, where flume 0.21.2
confirmed nothing, sluice handed eight places' held outcomes over 3,322
to 60,862 times each, every read holding the repository's fetch lock
and reaching GitHub, and every flume command queued 10 to 18 minutes
behind the others.

The profile now collects by `sluice inbox --confirm`, which confirms
each outcome as sluice hands it over, before flume has acted on it: a
settle whose verdict's write is refused loses a failure for good. And
an outcome carries no submission, so one handed again after the place
was submitted anew would be read as the new submission's.

The brief: "Confirm each outcome once Flume has acted on it (recorded
the submission's end and handed it to the place's recipients), so a
later settle is handed only what is new; a test shows a second settle
hands over no outcome already acted on."

The statement is amended at boomdrivedev/blueprints `aa710c6`, on
`design/flume-confirms-an-outcome-it-acted-on`: 101, 111, 124 and 133
amended, 225 the merge queue's confirm, and C.6's inbox and confirm
rows. Record 0072 holds the choices.

## What Changes

- **The seam (133, 225; core.2).** The merge-queue seam gains
  `confirm(place, key)`, answering confirmed or unread. The inbox gives
  each outcome as `Owed`, with the submission it decides and the key
  that confirms it, again until it is confirmed; a decided submit
  carries its key.
- **The core (111, 112, 124, 225; core.6).** A settle acts on each
  outcome its inbox or a decided submit gives, then confirms it once the
  place's state holds what it decided. An outcome of a submission other
  than the one flume last made decides nothing and is confirmed with no
  act. One whose act was not written is left unconfirmed, and one whose
  confirm the merge queue could not take is given again; each is acted
  on once.
- **The sluice profile (C.6).** It collects by `sluice inbox --json`,
  reading each record's `id` and its `delivery`'s key where it is owed,
  and confirms by `sluice confirm <key> --json` in the place, exits 0
  and 1 confirmed. A submit's decided answer carries its key, and the
  profile confirms nothing on its own.
- **The fakes.** The fake merge queue keeps each outcome under
  `<id>:<state>` until it is confirmed, logged as `confirm <key>`, and
  can refuse confirms; the fake store can refuse every write of a place.
- **The clause list** reads the statement at `aa710c6`, 225 in Part B.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: outcomes collected by `sluice inbox --json`
  and confirmed by `sluice confirm <key>` once flume acted on them.
- `seam-contracts`: the inbox gives each outcome again until it is
  confirmed, and the seam confirms one.
- `settling`: each outcome is confirmed once acted on, and a second
  settle is given none.
- `seam-fakes`: the fake merge queue keeps an outcome until it is
  confirmed, and the fake store refuses a place's every write.
