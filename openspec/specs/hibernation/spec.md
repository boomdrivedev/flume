# hibernation Specification

## Purpose
A stage whose kind hibernates it sleeps once its prompt cache no longer
pays: flume ends its agent and keeps its session, freeing the agent's
room and its account, once a conversation has idled past its hibernate
time, an agent kept open for an answer has outlived its time, or its
caller asks; and the stage resumes in its own session at its caller's
resume, a message to the place or the answer its agent waits on, one
fresh agent carrying the hibernated agent's conversation where its
profile can and starting afresh where it cannot (212, 213).

## Requirements

### Requirement: A conversation idle past its hibernate time is hibernated

A settle SHALL hibernate the running conversation of a stage whose kind
hibernates it once its agent has been quiet for its hibernate time with
no question open, no own work running and no message being handed to
it (90, 91, 120, 212).

#### Scenario: An hour after the last turn

- **WHEN** a place of a kind whose talk stage is a conversation that hibernates is launched with no layer giving `hibernate_after`, its agent's turn ends, and settles read it quiet over an hour on the fake clock
- **THEN** the settle at the hour ends the agent through the fake terminal, talk reads `hibernating` since then, why `idle`, its start not live, and the host's agents running drop by one (70, 212, S124)

#### Scenario: Asking, own work, a message

- **WHEN** a hibernating conversation's agent is past its hibernate time and is asking a person, or its own work runs, or a message is being handed to it
- **THEN** no settle hibernates it until none holds and it has been quiet its hibernate time again (90, 91, 212)

#### Scenario: Zero never hibernates on idling

- **WHEN** a launch gives `--agent talk.hibernate_after=0` and talk's agent is quiet for a day
- **THEN** talk is never hibernated on its idling (212)

#### Scenario: A stage whose kind does not hibernate it

- **WHEN** a conversation of a kind that does not declare it hibernates is quiet for a day
- **THEN** it is never hibernated, and runs as a conversation does (109, 212)

#### Scenario: Two settles at once

- **WHEN** two settles read a conversation due to hibernate at once
- **THEN** its agent is closed once and its hibernation recorded once (74, 75)

### Requirement: A conversation's quiet runs from flume's reading of it

A conversation's quiet SHALL run from flume's first reading of its
agent quiet, and SHALL start again whenever flume reads it not quiet,
and at the settle the agent's own quiet starts, since a turn ended then
(44, 121, 212).

#### Scenario: A turn between two settles

- **WHEN** a hibernating conversation's agent is read quiet at t, then works and goes quiet again at t plus 50 minutes, its on-quiet settle running then, and a settle runs at t plus 70 minutes
- **THEN** talk is not hibernated, and a settle at t plus 110 minutes hibernates it (212)

### Requirement: Hibernating ends the agent and keeps its session

Hibernating SHALL end the agent once it is quiet, keep its session,
read its start not live, freeing its room and its account, and record
on the start since when and why; the stage SHALL neither end nor move
on, and SHALL count no start and no retry (31, 48, 70, 145, 212).

#### Scenario: Room freed

- **WHEN** talk hibernates idle on a host at its bound, another place's stage waiting for room
- **THEN** talk's starts are unchanged, its end is none, and the other place's stage starts (48, 70, 212)

### Requirement: An agent kept open is hibernated at the end of its time

Where the stage hibernates, an agent kept open for an answer whose time
has passed SHALL be let go hibernated, its close reading `hibernated`
where it would read its time, and ended once quiet; the stage SHALL
stay ended short, waiting on its answer under the same move and since
(42, 130, 198, 212).

#### Scenario: A code stage kept open its hour

- **WHEN** a code stage of a kind that hibernates it stops short needing an answer, kept open its hour, quiet, and a settle runs after the hour
- **THEN** its end's close reads `hibernated`, the agent is ended through the fake terminal, its start records it hibernated since then, why `kept_open`, and code reads ended short waiting on its answer (42, 198, 212, S125)

#### Scenario: Let go otherwise first

- **WHEN** the same code agent's account is spent within its hour
- **THEN** it is let go by its account spent, as any agent kept open, and is not hibernated (146, 198)

### Requirement: A caller hibernates a stage

`flume hibernate <place>` SHALL hibernate the live agent of a place's
current stage whose kind hibernates it, as a settle does, ending an
agent that is not quiet only when forced; it SHALL be refused, naming
the place, for a stage whose kind does not hibernate it and for a place
with no live agent, and refused unforced while a message is being
handed to the agent (31, 54, 212).

#### Scenario: A conversation hibernated at its caller's word

- **WHEN** a caller hibernates a place whose talk conversation hibernates, its agent quiet
- **THEN** the agent is ended, talk reads `hibernating` since then, why `caller` (212)

#### Scenario: A working agent

- **WHEN** a caller hibernates a place whose hibernating talk agent works, unforced, then forced
- **THEN** the first is refused naming the agent not quiet, and the forced one ends it and talk reads `hibernating` (31, 212)

#### Scenario: Refused

- **WHEN** a caller hibernates a fix place, whose kind hibernates nothing, and then a place whose hibernating talk is already hibernating
- **THEN** each is refused naming the place, the first that its kind does not hibernate the stage and the second that it has no live agent, and nothing changes (212)

### Requirement: A resume carries the stage's own session

A caller's `flume resume`, a message to the place, and the answer to
the stop-short a hibernated agent waits on SHALL each resume a
hibernating stage: one fresh agent under an identity of its own, the
stage's start number kept and its starts not counted, carrying the
hibernated agent's conversation where its profile can. Its prompt SHALL
be the message's words, or the question with its answer, or flume's
words at a bare resume, then the way to stop short (28, 30, 42, 53,
179, 213; core.10).

#### Scenario: A message next morning

- **WHEN** talk hibernated idle, and next morning its caller messages the place "where were we?"
- **THEN** a fresh agent `flume:<repository>:<place>:talk:1.1` starts on the account the hibernated agent ran under, its record naming the hibernated agent's session, its prompt "where were we?" then the way to stop short, talk's starts still 1, and the message is answered with the place's status (53, 213, S124)

#### Scenario: The answer three hours later

- **WHEN** code hibernated at the end of its keep-open time, and three hours later a message naming its stop-short's key answers "use notes.md"
- **THEN** the answer is recorded on the stop-short with who gave it, its key and when, code resumes its session with the question then the answer as its prompt, and once its agent commits and goes quiet code reads delivered (42, 198, 213, S125)

#### Scenario: A bare resume

- **WHEN** a caller resumes a place whose talk hibernates
- **THEN** talk's resumed agent's prompt is flume's words that its session resumes and what ran in it then has stopped, then the way to stop short (213)

#### Scenario: A resume of nothing hibernating

- **WHEN** a caller resumes a place whose current stage is running
- **THEN** it is refused naming the place, and nothing changes (213)

### Requirement: A resume's account and its wait

A resume SHALL run on the account the hibernated agent ran under while
it is usable, and otherwise on the one 145 chooses, and SHALL wait for
room, and while a process runs the conversation, as any carried start
waits (49, 70, 145, 207, 213; core.10, core.32).

#### Scenario: The hibernated account spent

- **WHEN** talk hibernated under account `a`, `a` is spent, and the place is resumed in a pool whose account `b` reaches the conversation
- **THEN** the resume takes `b` and carries the conversation, and status says why it moved (145, 213)

#### Scenario: Waiting for room

- **WHEN** a place is resumed on a host at its bound
- **THEN** its stage waits for room with the resume's words, and starts as the first waiting does once room frees (70, 213)

#### Scenario: A process runs the conversation

- **WHEN** a person resumed the hibernated session by hand, and the place is then resumed
- **THEN** the stage waits while the fake agent's live reads that process, and status names it; once none runs, the resume starts (207, 213; core.10)

### Requirement: A resume whose conversation cannot be carried starts afresh

Where no account the resume takes reaches the hibernated conversation,
or its transcript is gone, the stage SHALL start afresh as a start of
its own, its prompt the stage's with flume's words, that its
conversation could not be carried and that what it committed and left
is in the place, before the resume's words; and a resume never begun,
or whose agent is not the stage's, SHALL be read as a carried start
that does not begin is read (49, 146, 161, 179, 213).

#### Scenario: Forty days hibernated

- **WHEN** talk hibernated forty days ago, the fake agent's carry answers that its transcript is gone, and the place is messaged "carry on"
- **THEN** talk's second start is a fresh agent whose prompt is talk's, with flume's words that its conversation could not be carried and what it committed is in the place, then "carry on", and status gives the resume not carried, naming why (179, 213, S127)

#### Scenario: A resume never begun

- **WHEN** a resumed agent stops before it says it started
- **THEN** it is recorded so, and the stage starts afresh with flume's words and the resume's words, carrying nothing (146, 213)

### Requirement: A resumed stage is a running stage

A resumed stage's stop check, its quiet limit and working past it SHALL
hold from the resume's prompt received, and it MAY hibernate again; a
fresh start flume makes on its own after a resumed agent SHALL carry
each resume's words after the start's (42, 44, 110, 118, 146, 213).

#### Scenario: Working past its limit from the resume

- **WHEN** a code stage that hibernated is resumed two hours after it began, and its agent works 20 minutes from the resume under a 30-minute limit
- **THEN** it is not reported working past its limit (118, 213)

#### Scenario: Hibernating again

- **WHEN** a resumed talk agent is quiet for its hibernate time
- **THEN** talk hibernates again, and its second resume's agent is `…:talk:1.2` (212, 213)

#### Scenario: Spent after a resume

- **WHEN** a resumed code agent, resumed by a message "use notes.md", stops at its account's usage limit, and no account of its pool reaches its conversation
- **THEN** code starts afresh, its words flume's, then the start's words, then "use notes.md" (42, 146, 213)

### Requirement: A hibernating place is dropped or done with no agent to end

A drop of a place whose stage hibernates, and a done of one whose kind
does not land, SHALL make its work over with no agent to end, and its
tidy SHALL keep the place while a process runs its last conversation's
session, as any tidy does (52, 64, 114, 207, 212; core.18).

#### Scenario: A hibernating research place done

- **WHEN** a caller declares done a place whose talk hibernates, its kind landing nothing
- **THEN** its work is over, no agent is closed through the fake terminal, and the next settle tidies it (62, 114, 212)
