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

While a caller keeps watch over the host, its settles SHALL settle the
places where something is pending on its stated bound, and between
flume's commands nothing else SHALL settle a place but its agent's own
quiet. While no caller keeps watch, whatever terminals the host binds,
every command of flume's that acts on places SHALL settle the places
on the host where something is pending before it acts. Status and a
prompt's rendering SHALL settle nothing (119, 121, 122, 190, 208, 225).

#### Scenario: Killed with nobody watching

- **WHEN** no caller keeps watch, a code stage's agent stops with nothing delivered, and later a caller drops another place
- **THEN** nothing reads the stage until the drop, which settles the places where something is pending first, the killed stage's among them, and status then reports the stage lost (S27, 225)

#### Scenario: Crashed under a watch

- **WHEN** a caller keeps watch, and a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the caller's next settle reads the stage lost within its bound, with no other caller asking, and starts no agent; nothing settled the place before it (122, S28)

#### Scenario: A watched host with one stream unwatched

- **WHEN** a caller keeps watch, the host's terminal is `fake`, its `trial` names the terminal `other`, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (119, 190, 208, S91)

#### Scenario: A host with no runner

- **WHEN** no caller keeps watch, a chore's code stage delivers and the settle its agent's quiet runs submits the place, ten minutes of the fake clock pass with nothing asked, a caller then drops another place, and later settles with a bound of two minutes
- **THEN** the fake merge queue logs no step of `main` in the ten minutes, one in the drop, which settles the places where something is pending first, the submitted chore's among them, and one in the caller's settle, after which status gives the watch and its bound (46, 119, 121, 122, 124, 208, 225, S118)

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

A settle a caller asks with a bound SHALL record in the host's state, by
compare-and-set, when it began, the command running it and its bound,
and when it ended. A caller SHALL keep watch while that settle runs and
until twice its bound after it ended, or after it began where its
command died first, renewed by that settle alone, and SHALL decide
nothing of what it does; no terminal or plugin of one SHALL keep watch
or settle (46, 119, 120, 122, 208; core.13, core.16).

#### Scenario: A watch kept, then lapsed

- **WHEN** a caller settles with a bound of two minutes, and the fake clock then passes three minutes, then two more
- **THEN** the host's watch holds when that settle began, the command that ran it, when it ended and two minutes; flume runs unattended at three minutes, and attended at five, with no other settle between (119, 208, S118)

#### Scenario: No terminal keeps watch

- **WHEN** no caller has settled with a bound, and agents of the fake terminal's change state
- **THEN** flume runs attended, nothing settles at those changes, and a caller's drop settles the places where something is pending first (119, 122, 208, 225)

#### Scenario: A settle that runs past its bound

- **WHEN** a caller settles with a bound of one minute, and while that settle reads the host's places the fake clock passes five minutes
- **THEN** flume runs unattended throughout that settle and a minute after it ended, and attended once two minutes have passed since it ended (119, 208)

#### Scenario: A watch whose settle died

- **WHEN** the host's watch names a settle with a bound of one minute begun by another command, which has not ended it
- **THEN** flume runs unattended while that command lives, ten minutes on among it, and attended once the command has died, two minutes having passed since that settle began (76, 119, 208)

### Requirement: A watch's settle moves every place, an agent in it or none

A watch's settle SHALL settle each place where something is pending
whether or not an agent is in it, a submitted place and a base place
among them, and SHALL drive one round of each
open submission's base once per command where the merge queue moves
only while something drives it, a base place's onto its base and the
places on its branch onto it alike, so a submitted place moves on
within the watch's bound with no agent and no other caller (46, 120,
124, 125, 208, 225, I8, S109).

#### Scenario: A submitted chore with no agent

- **WHEN** a caller keeps watch with a bound of two minutes, a chore's code stage delivers, its agent ends, the place is submitted, and the fake merge queue lands it only after three rounds
- **THEN** each of the caller's settles logs one `step main` of the fake merge queue's, with no agent of the place live and no other caller's act; after the third the next settle reads the landing, delivers it and tidies the place (46, 124, 208, S109)

#### Scenario: A queue that moves on its own

- **WHEN** the same chore is submitted to a fake merge queue whose `main` moves on its own, as beside a live driver, and it lands there
- **THEN** each settle's step answers that the queue moves on its own, nothing else of the settle changes, and the next settle after the landing tidies the place (124, 125; sluice 135)

#### Scenario: Two depths in one settle

- **WHEN** a base place's submission to `main` and a change's submission to the base place's branch are open together, and a caller keeping watch settles once
- **THEN** the fake merge queue logs one `step main` and one `step <the base place's branch>`, each once in the command (124, 125, 202, 225)

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
it decided; an outcome of a submission other than the one flume last made
SHALL decide nothing and be confirmed with no act; one whose act was not
written, or whose confirm the merge queue could not take, SHALL be left
unconfirmed for the next inbox, acted on there no more than once (74,
111, 112, 124, 133).

#### Scenario: A second settle is given no outcome already acted on

- **WHEN** a fix's submission fails, a settle sends code back, and the place is settled again
- **THEN** the first settle confirms the failure once code's start is held, and the second asks the inbox nothing, confirms nothing and sends nothing back (111, 133)

#### Scenario: An earlier submission's outcome given again

- **WHEN** the fix is submitted anew and the inbox gives the first submission's failure again, as a flume that never confirmed it left it
- **THEN** the settle sends nothing back, holds no verdict for the open submission, and confirms the earlier failure (99, 133)

#### Scenario: An outcome not acted on, or not confirmed

- **WHEN** the second submission fails while every write of the place is refused, then a settle acts on it while the merge queue refuses confirms, then a settle runs with confirms taken, and then the fix round delivers and flume submits the place anew
- **THEN** the first settle confirms nothing and leaves the failure pending, the second sends code back once and leaves it pending, the third asks no inbox and leaves it pending, and the fourth's inbox gives it again, which sends nothing back and is confirmed (74, 111, 124, 133)

### Requirement: A watch reads only the places where something is pending

A watch's settle, and the settle an acting command or a wait runs first
where none keeps watch, SHALL read only the places where something is
pending, read from each place's held state, the host's deliveries and
the clock alone, and SHALL read nothing of a place that waits only on
an answer, its caller's word or a drop: neither its merge queue, its
terminal nor its agent (119, 208, 225; core.62).

#### Scenario: Places with nothing pending

- **WHEN** a host holds a fix waiting on an answer whose agent has ended, a chore whose launch held its submission, waiting on its caller, a fix whose code stage was read lost, and a fix whose code agent runs, and a caller settles with a bound
- **THEN** the fake merge queue, terminal and agent program are asked nothing of the first three, the running fix is read, and status gives each of the first three as it gave it before that settle (42, 45, 47, 225)

#### Scenario: An unwatched drop

- **WHEN** no caller keeps watch, a fix waits on an answer whose agent has ended, and a caller drops another place
- **THEN** the drop's settle asks the fake merge queue, terminal and agent program nothing of the waiting fix (119, 225)

### Requirement: A settle of the pending places moves each place in time

A settle of the pending places SHALL move every place no later than a
settle of every place would: a place SHALL be pending once a time it
waits on has come, a stall's bound or a hold's among them, while an end
of it or a wake for it waits to be handed on, and while its document
kept once tidied may go; and the settle SHALL start the stages waiting
for room the host now has, whether or not a place is pending (43, 62,
70, 130, 132, 146, 225).

#### Scenario: A stall's time comes

- **WHEN** a fix waits on an answer whose agent has ended, under no hold's words, and a caller settles with a bound of one minute four minutes after the fix began to wait and again six minutes after
- **THEN** the first settle reads nothing of the fix, and the second records its stall and delivers it to the fix's recipients (130, 225)

#### Scenario: A hold's bound passes

- **WHEN** a fix's waiting move is held for ten minutes, and a caller settles with a bound of one minute nine minutes after the hold and again eleven minutes after
- **THEN** the first settle reads nothing of the fix, and the second lapses the hold and records the fix stalled (130, 225)

#### Scenario: An end handed on once its recipient waits

- **WHEN** a fix's code stage delivers while its recipient's pane asks a question, so no notice is sent, the fix's next stage held, and the recipient's pane then waits
- **THEN** the watch's next settle hands the end to the recipient by notice, and the settle after it reads nothing of the fix (43, 225)

#### Scenario: Room the host now has

- **WHEN** a fix's code stage waits for room since its pool's one account is spent until a time, no place on the host being pending, and a caller settles with a bound once that time has passed
- **THEN** that settle starts the code stage on that account (70, 146, 225)

#### Scenario: A kept document let go

- **WHEN** a fix landed and was tidied, its document kept for its recipient's pending end, and the recipient then confirms that end
- **THEN** the watch's next settle removes the kept document (62, 225)

#### Scenario: A base place at each watch

- **WHEN** a base place whose kind names a fix waits on its caller with no place on its branch, and a caller keeping watch settles twice
- **THEN** each settle forecasts the base place's branch against its base through the fake merge queue (203, 225)

### Requirement: A full settle reads every place

`flume settle` naming no place and stating no bound SHALL settle every
place on the host, a place with nothing pending among them, at a
caller's or a person's asking; a move a hand makes outside flume of a
place that waits only on its caller SHALL be read at such a settle or
at its caller's next act on the place, and never by a watch (63, 225;
S30).

#### Scenario: A held submission landed by a hand

- **WHEN** a chore's launch held its submission, a hand lands the chore's branch through the fake merge queue, and a caller keeps watch
- **THEN** the watch's settles read nothing of the chore, and a settle naming no place and stating no bound reads its work over and tidies it, never making the held submission (S30, 63, 225)

### Requirement: flume host keeps watch over the host

`flume host`, flume's resident, SHALL settle the places where something
is pending as a watch's settle does, at once, again at each change its
merge queue's subscription hears, and again a minute after its last
settle where it heard none; changes heard together, and while a settle
runs, SHALL make one settle after it (208, 225; record 0082; core.13).

#### Scenario: A minute with nothing heard

- **WHEN** the resident runs over a paused clock and hears no change for four minutes
- **THEN** it settled at once and then once each minute

#### Scenario: A change heard

- **WHEN** the resident hears a change thirty seconds after its first settle
- **THEN** it settles two seconds later, settles nothing in the minute after that settle, and settles once that minute ends

#### Scenario: Many changes heard together

- **WHEN** twenty changes are heard at once and one more while the settle they began waits
- **THEN** they make two settles, and nothing more settles with nothing heard since

#### Scenario: A stop

- **WHEN** the resident is stopped
- **THEN** it ends within a second, its subscription with it

### Requirement: The resident follows the repositories its merge queue can move

The resident SHALL follow, on one subscription to the merge queue's
resident, the repository of each live place whose submission is in
flight and of each base place, read from held state alone, and no
other; it SHALL ask nothing while it follows none, and SHALL hold the
subscription again a minute after it ended, settling each minute
meanwhile (124, 203, 225; record 0082).

#### Scenario: A chore followed while in flight

- **WHEN** a chore works, is submitted and stays open, and then lands
- **THEN** no repository is followed while it works, its repository while its submission is open, and none once it landed and was tidied

#### Scenario: The newest set followed

- **WHEN** the resident's settles give one repository, then two, then none
- **THEN** its subscription follows each set in turn

#### Scenario: A subscription that ended

- **WHEN** the subscription ends a second after the resident starts
- **THEN** it is held again a minute later, and the minute's settle ran meanwhile

### Requirement: The resident hears sluice on sluice host's socket

The sluice profile's subscription SHALL connect to sluice host's socket,
say its hello, ask a subscription and add and remove each followed
repository's folder, starting no sluice command and no sluice host; a
document SHALL be a change only where a branch's records' version,
revision or gone moved since the folder's last, and an error line or no
socket SHALL end it with words (record 0082).

#### Scenario: Followed, heard and ended

- **WHEN** a stand-in sluice host answers on a socket, sends a folder's first document, the same reading again, a document of a folder not followed, then a document whose branch's version moved, then the followed set changes, then it sends an error line
- **THEN** flume said its hello, asked the subscription and added the folder, heard a change only at the moved version, removed the old folder and added the new, and the subscription ended naming the error's words

#### Scenario: Nothing to follow, then no sluice host

- **WHEN** nothing is followed, then one repository is, and nothing answers on the socket
- **THEN** nothing is asked while nothing is followed, and the subscription ends naming the socket

#### Scenario: The socket is sluice's

- **WHEN** sluice's state folder is under a given `XDG_STATE_HOME` and `SLUICE_USER_SCRATCH` names the user's directory
- **THEN** the socket is `host-<key>.sock` there, the key the first 10 hex digits of the SHA-1 of the resolved state folder

### Requirement: flume host listens on its socket

`flume host` SHALL listen on `host-<key>.sock` in the user's own
directory, `flume-<uid>` in `/tmp` resolved or `$FLUME_USER_SCRATCH`,
the key the first 10 hex digits of the SHA-1 of flume's resolved state
folder, and SHALL answer a client's hello with its version, its
protocol, its process and when it started
(flume-host-serves-status-and-its-subscription-on-its-socket;
residents-speak-json-lines-over-a-unix-socket).

#### Scenario: The socket's name

- **WHEN** flume's state folder is under a given `XDG_STATE_HOME` and `FLUME_USER_SCRATCH` names the user's directory
- **THEN** the socket is `host-<key>.sock` there, the key the first 10 hex digits of the SHA-1 of the resolved state folder

### Requirement: flume host serves status and a subscription to it

After the hello, `flume host` SHALL answer `status` with one `document`
line, the answer `flume --json status` prints, and close the
connection; and `subscribe` with a `document` at once and another each
time the answer differs from the last one sent, until the client closes
it. Either ask MAY name a place and ask each place's line. A line it
cannot read SHALL get one `error` line, and the connection SHALL close
(flume-host-serves-status-and-its-subscription-on-its-socket).

#### Scenario: A status

- **WHEN** a client says its hello and asks `{"ask":"status","line":true}`
- **THEN** flume host answers its version, protocol and pid, then one `document` holding the answer, and closes the connection

#### Scenario: A subscription

- **WHEN** a client subscribes, the answer stays the same at one change, and then differs at five changes made together
- **THEN** the client gets the first document, nothing at the change with the same answer, and one document for the five changes

#### Scenario: An ask it does not know

- **WHEN** a client asks `{"ask":"settle"}`
- **THEN** flume host answers one `error` line naming the line, and closes the connection

### Requirement: flume host reads status again at each change

`flume host` SHALL read the status of each subscription again after
each of its settles, after each change its merge queue's subscription
hears, and two seconds after a file event in flume's state folder that
is not its log or a lock, the events of those two seconds making one
read (residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop;
one-read-in-flight).

#### Scenario: Read again after a settle

- **WHEN** the resident settles while a client subscribes
- **THEN** the subscription's status is read again, and sent where it changed
