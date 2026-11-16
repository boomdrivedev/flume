# settling Specification

## Purpose
What moves a place on with no process of flume's own: a settle run when
its agent goes quiet, when the merge queue decides and when the
terminal watches, or before every acting command where nothing watches;
and a landing kind's submission, waited on and sent back when it fails
(I7, I8; 40, 46, 111 to 113, 119 to 122, 124).

## Requirements

### Requirement: A settle takes every step the place allows

A settle of a place SHALL read every fact of it again and take each
step Part A then allows: collect the outcomes of its submission, read
its current stage's end, end a delivered stage's quiet agent and start
its next stage unless that stage waits on its caller, submit a landing
kind's place whose last stage delivered, and tidy a place whose work is
over; and then start the stages waiting for room on the host, as many
as there is room for. A settle SHALL be safe to repeat (40, 46, 70,
74).

#### Scenario: A commit and a quiet agent

- **WHEN** a code stage's fake agent commits, goes quiet, and the agent seam's on-quiet settles the place
- **THEN** the agent is ended and the next stage started in that settle, before any watch runs (S29, 121)

#### Scenario: A held next stage

- **WHEN** a construct delivers and the launch holds every later stage and the submission
- **THEN** the settle ends the agent and starts nothing more (S7, 47)

#### Scenario: Room freed by a settle

- **WHEN** a settle ends one place's agent on a full host while another place waits for room
- **THEN** the same settle starts the waiting place's stage (46, 70)

### Requirement: Attended and watched

While a caller keeps watch over the host, its settles SHALL settle each
place on its stated bound, and between flume's commands nothing else
SHALL settle a place but its agent's own quiet. While no caller keeps
watch, whatever terminals the host binds, every command of flume's
that acts on places SHALL settle every place on the host before it
acts. Status and a prompt's rendering SHALL settle nothing (119, 121,
122, 190, 208).

#### Scenario: Killed with nobody watching

- **WHEN** no caller keeps watch, a code stage's agent stops with nothing delivered, and later a caller drops another place
- **THEN** nothing reads the stage until the drop, which settles every place first, and status then reports the stage lost (S27)

#### Scenario: Crashed under a watch

- **WHEN** a caller keeps watch, and a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the caller's next settle reads the stage lost within its bound, with no other caller asking, and starts no agent; nothing settled the place before it (122, S28)

#### Scenario: A watched host with one stream unwatched

- **WHEN** a caller keeps watch, the host's terminal is `fake`, its `trial` names the terminal `other`, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (119, 190, 208, S91)

#### Scenario: A host with no runner

- **WHEN** no caller keeps watch, a chore's code stage delivers and the settle its agent's quiet runs submits the place, ten minutes of the fake clock pass with nothing asked, a caller then drops another place, and later settles every place with a bound of two minutes
- **THEN** the fake merge queue logs no step of `main` in the ten minutes, one in the drop, which settles every place first, and one in the caller's settle, after which status gives the watch and its bound (46, 119, 121, 122, 124, 208, S118)

### Requirement: A landing kind's place is submitted at its head

Once a landing kind's last stage has delivered and its agent has ended,
a settle SHALL submit the place's own branch at its head to its base
through the merge queue seam, holding no wait on the verdict, unless the
launch holds the submission or the place's work is over. A landed place
SHALL be tidied by flume, the merge queue removing nothing of it. A kind
that does not land SHALL never reach the merge queue (66, 111, 113).

#### Scenario: A fix lands and the merge queue removes it

- **WHEN** a fix's code stage delivers, its agent goes quiet, the place is submitted, and the fake merge queue lands it
- **THEN** the merge queue removes nothing; the next settle collects the landing, and flume removes the place through the place seam and its branch when it holds nothing its base lacks, and keeps no held state of it (S1, 66, 67, 113)

#### Scenario: A landing before flume submits

- **WHEN** a held submission's place is landed by a hand's submission of its own branch
- **THEN** the core reads the work over, never makes the held submission, and tidies the place (S30, 63)

#### Scenario: Another place's landing

- **WHEN** a place's work reaches its base inside another place's landing
- **THEN** the place stays and its next stage waits on its caller (S8, 63)

### Requirement: A failed submission sends the place back

A failed or conflicting submission SHALL send the place back to the
stage its kind's last stage names, that last stage where it names none,
started with a fresh agent and the merge queue's words as its words, the
stages after it run again and the place submitted again, while the
place's goings-back, by submission and by finding together, are fewer
than the kind allows, after which the place SHALL wait for its caller
(112, 131).

#### Scenario: A failure, then a landing

- **WHEN** the fake merge queue fails a change's submission with its words, then lands the second
- **THEN** code starts with a fresh agent and those words, code and verify deliver again, the place is submitted again, it lands, and its work is over (S19, S35)

#### Scenario: The count reached

- **WHEN** a kind allows one send-back and the fake merge queue fails both submissions
- **THEN** after the second failure the place waits for its caller's retry or drop and starts nothing (112)

### Requirement: A withdrawn or refused submission leaves the place waiting

A withdrawn submission SHALL leave the place waiting for its caller, and
a submission refused at submit likewise, status naming why (112).

#### Scenario: A withdrawn submission

- **WHEN** the fake merge queue gives a place's submission withdrawn
- **THEN** the place waits for its caller, starts nothing, and status names the withdrawal (112)

### Requirement: A submission's outcome is collected at the settles that follow

Every settle of a place flume submitted SHALL collect the outcomes
pending for it through the merge queue seam's inbox and act on each
once, and where its submission is open SHALL drive its base one round
through step, once per repository and base in one command; a settle
SHALL never submit the place anew for an open submission (111, 124,
125, 133).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test, and the fake merge queue lands it before the next settle
- **THEN** that settle collects the landing, never submits anew, and tidies the place (S34)

#### Scenario: Two places on one base

- **WHEN** one settle of every place finds two places' submissions open on one base
- **THEN** the fake merge queue's log holds one step for that base

### Requirement: A caller keeps watch over the host

A settle of every place a caller asks with a bound SHALL record, in the
host's state and before it settles, when it began and the bound it
stated, written by compare-and-set as every host document is. A caller
SHALL keep watch while the newest such settle began less than twice its
bound ago, and SHALL decide nothing of what the settle does; no terminal
and no plugin of one SHALL keep watch or settle (46, 119, 120, 122,
208; core.13, core.16).

#### Scenario: A watch kept, then lapsed

- **WHEN** a caller settles every place with a bound of two minutes, and the fake clock then passes three minutes, then two more
- **THEN** the host's watch holds when that settle began and two minutes; flume runs unattended at three minutes, and attended at five, with no other settle between (119, 208, S118)

#### Scenario: No terminal keeps watch

- **WHEN** no caller has settled with a bound, and agents of the fake terminal's change state
- **THEN** flume runs attended, nothing settles at those changes, and a caller's drop settles every place first (119, 122, 208)

### Requirement: A watch's settle moves every place, an agent in it or none

A settle of every place, as a caller keeping watch runs it, SHALL
settle each place the store holds whether or not an agent is in it,
and SHALL drive one round of each open submission's base once per
command where the merge queue moves only while something drives it, a
base place's onto its base and the places on its branch onto it alike,
so a submitted place moves on within the watch's bound with no agent
and no other caller (46, 120, 124, 125, 208, I8, S109).

#### Scenario: A submitted chore with no agent

- **WHEN** a caller keeps watch with a bound of two minutes, a chore's code stage delivers, its agent ends, the place is submitted, and the fake merge queue lands it only after three rounds
- **THEN** each of the caller's settles logs one `step main` of the fake merge queue's, with no agent of the place live and no other caller's act; after the third the next settle reads the landing, delivers it and tidies the place (46, 124, 208, S109)

#### Scenario: A queue that moves on its own

- **WHEN** the same chore is submitted to a fake merge queue whose `main` moves on its own, as beside a live driver, and it lands there
- **THEN** each settle's step answers that the queue moves on its own, nothing else of the settle changes, and the next settle after the landing tidies the place (124, 125; sluice 135)

#### Scenario: Two depths in one settle

- **WHEN** a base place's submission to `main` and a change's submission to the base place's branch are open together, and every place is settled once
- **THEN** the fake merge queue logs one `step main` and one `step <the base place's branch>`, each once in the command (124, 125, 202)

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
