## MODIFIED Requirements

### Requirement: Outcomes collected from sluice's inbox

The profile SHALL collect a place's pending outcomes by `sluice inbox
--confirm --json` in the place, confirming each on receipt: landed with
its commit, failed with the merge queue's words, or in conflict with the
files contested, each carrying sluice's `notice` as its words, a
conflict's base revision read from the record's `tested.on`, each
delivered once; and SHALL read a withdrawal from read-submission, never
from the inbox (99, 101, 133, C.6; sluice 51).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry collected once

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice
- **THEN** the first collect gives the failure with sluice's notice, and the second gives nothing (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

## ADDED Requirements

### Requirement: A submit's decided answer is confirmed

Where sluice answers a submit with a decided entry's record, the profile
SHALL confirm it by `sluice confirm <key>` of the record's `delivery`
where it is owed, run where the submit ran, so no later inbox gives it
(101, 112, 133, C.6; sluice 51; sluice record 0102).

#### Scenario: A decided answer is confirmed

- **WHEN** sluice answers the place's submit with its entry's record in conflict with an entry queued ahead
- **THEN** submit answers the conflict, and the inbox collected after it gives nothing (112, 133)
