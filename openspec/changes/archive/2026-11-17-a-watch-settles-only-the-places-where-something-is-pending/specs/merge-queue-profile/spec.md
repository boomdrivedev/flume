## MODIFIED Requirements

### Requirement: A submit's decided answer is confirmed

Where sluice answers a submit with a decided entry's record, the profile
SHALL answer it with the key of the record's `delivery` where it is
owed, and SHALL confirm nothing itself: the core confirms it by that key
once it has acted on it, and until then an inbox gives it too (101, 112,
133, C.6; sluice 51; sluice record 0102).

#### Scenario: A decided answer is confirmed

- **WHEN** sluice answers the place's submit with its entry's record in conflict with an entry queued ahead
- **THEN** submit answers the conflict with the key `<id>:conflict`, the inbox collected after it gives that entry's conflict under that key, and once it is confirmed by that key the inbox gives nothing (112, 133)

### Requirement: Outcomes collected from sluice's inbox until confirmed

The profile SHALL collect a place's pending outcomes by `sluice inbox
--json` in the place, never with `--confirm`, each with its entry's `id`
and its `delivery`'s key where owed, given again until confirmed:
landed with its commit, or failed or in conflict with sluice's `notice`
as its words, a conflict's base revision its `tested.on`; and SHALL read
a withdrawal from read-submission (99, 101, 133, C.6; sluice 51).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry given until it is confirmed

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice, the outcome confirmed by its key, and the inbox collected again
- **THEN** the first two collects give the failure with sluice's notice, its entry's id and the key `<id>:failed`, the confirm answers confirmed, the third collect gives nothing, and a second confirm of the key answers confirmed (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

### Requirement: An outcome confirmed by sluice confirm

The profile SHALL confirm an outcome by `sluice confirm <key> --json` in
the place, reading exit 0, confirmed now or before, and exit 1, nothing
owed under the key, as confirmed, and any other exit as sluice unread in
its words, the outcome left owed for the next inbox (101, 133, C.6;
sluice 51).

#### Scenario: A failed submission's outcome confirmed once flume acted on it

- **WHEN** the gate fails a fix's first submission through the pinned sluice, flume sends the place back to code, and the place is settled again
- **THEN** sluice reads the failed entry's delivery `done`, and the second settle sends nothing back again (112, 133)
