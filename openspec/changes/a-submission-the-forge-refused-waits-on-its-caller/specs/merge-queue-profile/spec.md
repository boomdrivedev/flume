## MODIFIED Requirements

### Requirement: Outcomes collected from sluice's inbox

The profile SHALL collect a place's pending outcomes by `sluice inbox
--json` in the place, never with `--confirm`: landed with its commit,
failed with the merge queue's words, in conflict with the files
contested, or held, its landing refused by the forge, each carrying
sluice's `notice` whole as its words, a conflict's base revision read
from the record's `tested.on`; and SHALL then confirm every outcome it
read by one `sluice confirm` of each one's `delivery.key`, run in the
place. An outcome in a state the profile does not read, or whose
outcome it cannot read, SHALL be left owed, unconfirmed, and passed over
by its branch with words naming it. The profile SHALL read a withdrawal
from read-submission, never from the inbox (99, 101, 133, C.6 at
boomdrivedev/blueprints `8b73d11` and `fc4323b`; sluice 51; core.58).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry collected once

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice
- **THEN** the first collect gives the failure with sluice's notice and confirms it by its key, and the second gives nothing (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

#### Scenario: A refused outcome

- **WHEN** a stand-in `sluice` answers the inbox with sluice `44e33d4`'s documents, a `refused` outcome under `7:refused` beside a `landed` one under `6:landed`
- **THEN** the inbox gives the landing and the outcome held with its `notice` whole, runs `sluice inbox --json` with no `--confirm`, and then one `sluice confirm` naming `6:landed` and `7:refused` (112, 133, C.6)

#### Scenario: An outcome in a state flume does not read

- **WHEN** the stand-in's inbox holds a `landed` outcome under `6:landed` and one in the state `parked` under `8:parked` on `main`
- **THEN** the inbox gives the landing, passes `main` over with words naming #8 and `parked`, and confirms `6:landed` alone, leaving `8:parked` owed (133; core.58)

## ADDED Requirements

### Requirement: read-submission reads a refused record as held

read-submission SHALL read a `refused` record of `sluice status <base>
--all --json` as held, with its `refused.words`. One that carries no
`refused`, as sluice `44e33d4` prints it, SHALL still be held, its words
saying sluice's record gives none, and never unread. No words SHALL be
read through a wait (99, C.6 at `04e5a1a`; core.6, core.13, core.58).

#### Scenario: A refused record with its words

- **WHEN** a stand-in `sluice` answers `sluice status main --all --json` with a `refused` record of the place's branch carrying `refused {words, at}`, as sluice `44e33d4`'s `report` prints it
- **THEN** read-submission gives the submission held with those words, and the stand-in's log holds no `wait` and no `status` of a number (99, C.6)

#### Scenario: A refused record without its words

- **WHEN** the stand-in answers with a `refused` record of the place's branch and one of the base place's branch `train/x` by `from.head`, each carrying no `refused`, as sluice `44e33d4`'s `status --all` prints them
- **THEN** read-submission gives each held, with words saying sluice's record gives none of the forge's words, neither unread nor passed over, and the log holds no `wait` (99, 203, C.6; core.58)
