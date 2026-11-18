# A settle reads sluice only for a submission in flight

## Why

Every acting flume command on deneb settles every place, and each
settle ran `sluice inbox` in each of deneb's 39 places one after
another. Each call takes the repository's `sluice-fetch.lock` and
fetches from GitHub, so `flume inbox --recipient boomdrive/ops` took
269 s where `flume list` took 1 s, and deneb's runner ran every pass
past due, deferring its acts, the user's lands among them, for hours.

The settle collected a place's inbox once flume had ever submitted it,
whatever came after: through its fix round, while it waited on its
caller after a withdrawal, a refusal or a failure past its kind's bound.
Record 0012 asks it "at every settle for each place with an open
submission". A place whose submission flume has not made, or whose
outcome it holds, has nothing in sluice to collect, since flume makes
every submission of a place (core.6). This is the design's ruling for
the fix.

## What Changes

- **The inbox is collected while the submission is open (111, 133).**
  A settle asks the merge queue's inbox for a place only while flume
  has submitted it and holds no outcome of it, at every settle that
  follows the submission until its outcome is collected, so no outcome
  is missed and an inbox that cannot be read is read again at the next
  settle.
- **Once more as flume submits the place anew (99, 101, 133).** Before
  a submission made after an outcome, by the place's next delivery or
  its caller's retry, the settle collects the inbox once: anything
  pending for the place then is of a submission whose verdict flume
  holds, or of none of the place's, and decides nothing of the new one,
  which the settles that read the inbox at each settle after an outcome
  drained before.
- **A withdrawal and a landing by hand are read from read-submission,
  as before (63, 99).**
- **Status names the inbox's unread words and passed-over branches only
  while it is read (133, 68).**
- sluice's inbox, its fetch and the merge-queue profile are unchanged.

The statement's clause 111 is amended to match at boomdrivedev/blueprints
`design/flume-a-settle-reads-sluice-only-for-a-submission-in-flight`;
record 0012 carries an amendment line and rule core.6 its detail.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `settling`: a submission's outcome is collected at the settles that
  follow it while it is open, and once as flume submits the place anew,
  and at no other settle.
