## Purpose

What flume reads from an agent's terminal where its profile builds no
part of its program's own for it: whether it received a prompt, whether
it asks, whether its own work runs, whether it ended and its last words,
each read by the core from the terminal and never asked of the profile,
so a stage of a program with no profile of its own runs through (31,
38, 89 to 93, 134, 136, 171).

## ADDED Requirements

### Requirement: A part its profile does not build is read from the terminal

Where an agent's profile does not build received, asking, own work,
ended or last words, the core SHALL read that part from the terminal
its start recorded, as 171 says, and SHALL ask the profile nothing of
it; where the profile builds it, the core SHALL read it from the
profile alone, and never from the terminal (93, 171; core.2, core.39).

#### Scenario: Nothing of the five asked of a profile that does not build them

- **WHEN** a fix runs over a fake agent scripted to read the five from its terminal, its prompt's turn seen by the fake terminal, then read asking, then waiting at two settles a minute apart with its deliverable committed, status asked at each settle
- **THEN** the stage delivers, and the fake agent's log holds no `received`, `asking`, `own_work`, `ended` or `last_words` read of its start (93, 171)

#### Scenario: A profile that builds them reads no question from the terminal

- **WHEN** a begun fix's fake agent, as a new fake makes it, holds no asking mark while the fake terminal reads it asking
- **THEN** its state reads unknown and its asking no, and a message to it is refused with nothing typed (90; core.39)

### Requirement: A prompt is sent by the send that watches its turn

Where the agent's profile does not build received, the core SHALL send
the prompt, and a message's words, by the terminal's send that watches the
turn, within 60 seconds, and such a start SHALL name no record of its
agent's (38, 77, 95, 171).

#### Scenario: A terminal that reads no turn

- **WHEN** the fake terminal refuses the watched send, saying it reads no turn a sending began
- **THEN** code is reported never begun, its failure naming the terminal's words, and nothing is typed (38, 95)

#### Scenario: A message handed on the turn

- **WHEN** a caller messages a begun fix's agent, waiting and reading received and asking from its terminal, `look again`, and the fake terminal answers that it saw the turn begin
- **THEN** the message is handed, sent once by the watched send (53, 54, 171)

### Requirement: A prompt is received on the terminal's word that its turn began

Where the agent's profile does not build received, the core SHALL read
its prompt received where the terminal answers that it saw its turn
begin, or where a reading within 60 seconds of the sending reads the
agent working or asking; otherwise not received, sent once more after
the 60 seconds, and a second sending not received reported never begun
with what the terminal shows (38, 89, 171).

#### Scenario: The turn seen

- **WHEN** a fix's fake agent reads received and asking from its terminal and the fake terminal answers that it saw the turn its prompt began
- **THEN** code is begun after one sending, sent by the watched send and by no plain send, its prompt received at the time the terminal answered and its start naming no record (38, 89, 171)

#### Scenario: The turn not seen, then read working

- **WHEN** the fake terminal answers that it saw no turn, and reads the agent working ten seconds after the sending
- **THEN** code is begun at that reading with one sending made (38, 171)

#### Scenario: The turn never seen

- **WHEN** the fake terminal answers each sending that it saw no turn and reads the agent waiting throughout
- **THEN** the prompt is sent once more 60 seconds after the first, its held input cleared first, and once 60 seconds pass after the second, code is reported never begun with what the terminal shows (38, 95, 171)

### Requirement: An agent asks where its terminal reads it waiting on a question

Where the agent's profile does not build asking, the core SHALL read
the agent asking, and its state asking, where its terminal reads it
waiting on a question, and not asking where the terminal reads it
working, waiting or stopped; nothing SHALL be typed into it while it
asks (37, 90, 171).

#### Scenario: Asking as the terminal shows it

- **WHEN** a begun fix's agent reads asking from its terminal, the fake terminal reads it asking, a settle runs and a caller messages it `go on`; then the terminal reads it waiting and a settle runs
- **THEN** status reads it asking and its state asking, and the message is refused with nothing typed; after the second settle status reads it neither (37, 90, 171)

### Requirement: Each settle records when it read the agent waiting

Where the agent's profile does not build own work, each settle of a
place SHALL record, for its live start, when it first and last read the
agent waiting, and clear it at any other reading and at every sending
to the agent; status, the list and a wait SHALL write nothing of it
(31, 171; core.16, core.52).

#### Scenario: Status counts no settle

- **WHEN** a settle reads the agent waiting, and status is asked 90 seconds later with no settle between
- **THEN** status reads its own work running, and the place's held state is unchanged by the ask (31, 171; core.52)

### Requirement: Own work is none once two settles a minute apart read the agent waiting

Where the agent's profile does not build own work, the core SHALL read
its own work none where the terminal reads it waiting and the settles'
two readings lie a minute or more apart, or where the terminal reads it
stopped; running while it reads it working, asking or waiting short of
the minute; and unknown where it reads its state unknown (31, 91,
171).

#### Scenario: Quiet after a minute's waiting

- **WHEN** a begun fix's agent reads the five from its terminal, its deliverable committed and the terminal reading it waiting, and the place is settled now, 30 seconds later and 60 seconds later
- **THEN** after the first two settles code is begun and status reads its own work running; at the third it is quiet and code delivers (31, 91, 171)

#### Scenario: A reading between starts the minute again

- **WHEN** the terminal reads the agent waiting at a settle, working at a settle 30 seconds later, and waiting at settles 60, 90 and 120 seconds later
- **THEN** code delivers at the settle 120 seconds later and at none before it (31, 171)

#### Scenario: A stopped agent runs no own work

- **WHEN** a fix's agent, its start read begun in another process than its launcher became, reads the five from its terminal and the terminal reads it stopped
- **THEN** that settle reads it quiet and starts code afresh, with no minute's wait (31, 161, 171)

### Requirement: An agent's end and last words are read from its terminal

Where the agent's profile does not build ended, the core SHALL read the
agent stopped from its terminal alone; where it does not build last
words, an end that carries the agent's last words SHALL carry what the
terminal shows of the agent, none where it shows nothing (43, 45, 134,
136, 171).

#### Scenario: Lost with its screen

- **WHEN** a begun fix's agent reads ended and last words from its terminal, the fake terminal shows `amp: out of credits` and then reads it stopped, nothing delivered
- **THEN** code is reported lost, its end's last words `amp: out of credits` (43, 45, 171)

#### Scenario: Stuck with its screen

- **WHEN** such an agent is read waiting with nothing delivered past the stage's limit, the terminal showing `nothing to do`
- **THEN** code is reported stuck, its end's last words `nothing to do` (43, 44, 171)

### Requirement: A stage of a program read from its terminal runs through

A stage whose agent's profile starts its program through flume's
launcher and builds none of the agent seam's parts SHALL start once the
terminal reads the launcher's process, take its prompt on the turn the
terminal sees, deliver once its agent is quiet, and be submitted as
any stage is, status naming each part degraded with how it is read
(35, 38, 170, 171, S73).

#### Scenario: S73 over the fakes

- **WHEN** a fix runs over a fake agent scripted to start through the launcher, reading the five from its terminal and offering no hooks, the fake terminal reading its program in the launcher's process 41 and seeing the prompt's turn, and its agent commits and is read waiting at two settles a minute apart
- **THEN** code starts and begins, status names received, asking, own work, ended, last words, send-back, guard and on-quiet degraded with the agent, each with how it is read, code delivers and the place is submitted (35, 38, 171, S73)

#### Scenario: amp under the pinned herdr

- **WHEN** a walkthrough under the pinned herdr launches a fix with `--agent program=amp`, its `amp` a stand-in that tells herdr it works as soon as it reads a line, commits, and tells herdr it is idle, and the walkthrough settles as a caller would, two of its settles a minute or more apart once amp is idle; and then launches a fix with `--agent program=amp --agent model=opus`
- **THEN** amp runs in the launcher's process, its prompt is received on herdr's word, status names received, asking and own work degraded, the first fix lands, and the second launch is refused naming `amp`, making nothing (35, 38, 170, 171, 172, S73)
