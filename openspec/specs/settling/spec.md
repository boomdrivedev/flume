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

Every settle of a place whose submission is open SHALL collect the
outcomes pending for it through the merge queue seam's inbox and act on
each once, and SHALL drive its base one round through step, once per
repository and base in one command; a settle SHALL never submit the
place anew for an open submission. An outcome handed again, one already
acted on, SHALL count as no step taken, so it never keeps a settle from
reading the place's stage (40, 46, 111, 124, 125, 133).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test, and the fake merge queue lands it before the next settle
- **THEN** that settle collects the landing, never submits anew, and tidies the place (S34)

#### Scenario: Two places on one base

- **WHEN** one settle of every place finds two places' submissions open on one base
- **THEN** the fake merge queue's log holds one step for that base

#### Scenario: An outcome handed again

- **WHEN** a change's submission fails, the merge queue hands that failure again at every inbox after, and code, sent back, ticks every task and goes quiet
- **THEN** the settle that sent code back reads its prompt received, the next settle ends code and starts verify, and the failure is acted on once: one submission and one going-back (40, 46, 74, 133)

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

### Requirement: No inbox for a place with no submission open

A settle of a place flume has not submitted, or whose submission's
outcome it holds, SHALL ask the merge queue seam's inbox nothing, but
once as it submits the place anew after an outcome, taking nothing
pending then as the new submission's outcome (99, 101, 111, 133).

#### Scenario: The inbox is asked only while the submission is open

- **WHEN** a fix is settled before its submission, while its submission is open, while its fix round runs after the fake merge queue failed it with an outcome left pending for the place, and as its next delivery submits it anew
- **THEN** no inbox is asked before the submission or while the fix round runs, one is asked at each settle while it is open, the inbox is collected as it is submitted anew, the outcome left pending sends the place back no more, and the new submission lands (111, 133, 99, 101)

#### Scenario: A decided submission asks no inbox

- **WHEN** the fake merge queue withdraws a place's submission, or refuses it, and the place is settled again
- **THEN** the place waits for its caller and that settle asks the inbox nothing (112, 133)

### Requirement: Each outcome is confirmed once acted on

A settle SHALL confirm each outcome its inbox or a decided submit gives,
through the merge queue seam's confirm, once the place's state holds what
it decided, so a later settle is given only what is new; an outcome of a
submission other than the one flume last made SHALL decide nothing and
be confirmed with no act, and one whose act was not written SHALL be left
unconfirmed for the next settle (111, 112, 124, 225).

#### Scenario: A second settle is given no outcome already acted on

- **WHEN** a fix's submission fails, a settle sends code back, and the place is settled again
- **THEN** the first settle confirms the failure once code's start is held, and the second reads the inbox, is given nothing, confirms nothing and sends nothing back (111, 225)

#### Scenario: An earlier submission's outcome given again

- **WHEN** the fix is submitted anew and the inbox gives the first submission's failure again, as a flume that never confirmed it left it
- **THEN** the settle sends nothing back, holds no verdict for the open submission, and confirms the earlier failure (99, 225)

#### Scenario: An outcome not acted on, or not confirmed

- **WHEN** the second submission fails while every write of the place is refused, then a settle acts on it while the merge queue refuses confirms, then a settle runs with confirms taken
- **THEN** the first settle confirms nothing and leaves the failure pending, the second sends code back once and leaves it pending, and the third sends nothing back again and confirms it (74, 124, 225)
