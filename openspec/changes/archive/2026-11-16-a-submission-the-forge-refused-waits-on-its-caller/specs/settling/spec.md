## ADDED Requirements

### Requirement: A submission whose landing the forge refused leaves the place waiting on its caller

A settle that reads the place's submission held, from its inbox or the
merge queue's record, SHALL have the place wait on its caller to retry
the stage its kind names for a failed submission, with the forge's
words, while it waits on that submission alone, the wait's words and
since standing as first written. It SHALL start no stage, launch no
fix, count no going-back, take no verdict and drive no round for it
(112, 130, 131, 133, 203; core.58).

#### Scenario: Held, nothing started

- **WHEN** the fake merge queue holds a fix's submitted landing with the forge's words, and three settles run
- **THEN** the first after the round has the place wait on its caller to retry code, its words the forge's, since that settle; the place opens no agent, takes no verdict, counts no send-back and its base is stepped no round after that settle; its submission reads held and open; and the wait's since is unchanged at the third (112, 130, 133; core.58)

#### Scenario: A base place's held submission

- **WHEN** a base place whose kind names a fix is submitted at its caller's retry, and the fake merge queue holds its landing with a record that carries no words
- **THEN** the base place waits on its caller with words saying the merge queue's record gives none, and launches no fix (203; core.53, core.58)

#### Scenario: A held landing stalls

- **WHEN** a fix's landing is held and five minutes pass under no hold's words
- **THEN** the place is stalled, and its recipient is delivered once that it stalls, the move a retry of code (130)

### Requirement: A held submission stays open until it lands, returns, leaves or is replaced

A held submission SHALL stay the place's open one, read again at each
settle. Its landing SHALL make the work over; its return to the queue
SHALL end the wait; its withdrawal SHALL leave the place waiting as any
does; and a caller's retry naming a stage SHALL run it and the stages
after it, the submission that follows withdrawing the held one (112,
113, 124; sluice 67, 152).

#### Scenario: Pushed again

- **WHEN** a fix's held landing is pushed again and lands
- **THEN** the next settle reads it landed, the place's work is over, and the place is tidied (113, 124)

#### Scenario: Back in the queue

- **WHEN** a fix's held landing is dropped, its submission back in the queue, and the queue then lands it
- **THEN** the settle that reads it waiting ends the wait and steps the base, and a later settle reads it landed (112, 124)

#### Scenario: A retry past a held landing

- **WHEN** a change whose kind allows one send-back has its landing held, its caller retries code with words, code and verify deliver, and the fake merge queue fails the submission that follows
- **THEN** code starts with a fresh agent and the caller's words, the submission that follows withdraws the held one, and its failure sends the place back to code with the merge queue's words, the one send-back the kind allows (112, 131; sluice 67)

#### Scenario: A held landing withdrawn

- **WHEN** a hand withdraws a fix's held submission
- **THEN** the next settle leaves the place waiting for its caller, status naming the withdrawal, and starts nothing (112)
