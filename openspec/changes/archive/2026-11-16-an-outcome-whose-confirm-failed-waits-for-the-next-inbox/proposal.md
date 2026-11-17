# An outcome whose confirm failed waits for the next inbox

## Why

Record 0072 confirms each outcome once flume has acted on it, and its
scenarios assumed every settle reads the inbox: the settle after a
confirmed failure read it and was given nothing, and a confirm the merge
queue could not take was retried at the next settle. Record 0012 as
amended 2026-11-16 has a settle ask the inbox only while the place's
submission is open, and once as flume submits the place anew, since a
place whose outcome flume holds has nothing in the merge queue to
collect (111, 133). The two met on merging main into this branch.
Clause 225 already leaves an outcome whose confirm failed "pending for
the next inbox"; the settling spec's scenarios said the next settle.

## What Changes

- **The settle after an outcome acted on asks no inbox (111, 133).**
- **A confirm the merge queue could not take is retried at the next
  inbox (225).** That is the one asked as flume submits the place anew,
  where flume holds the outcome; the outcome there sends nothing back
  again and is confirmed.
- The code is unchanged: this is the reading of the gate that merged
  with record 0072.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `settling`: an outcome whose confirm the merge queue could not take is
  confirmed at the next inbox, and the settle after an outcome acted on
  asks none.
