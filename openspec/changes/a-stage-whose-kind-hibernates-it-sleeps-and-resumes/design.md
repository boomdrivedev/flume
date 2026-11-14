# Design

## Context

### What is decided

Record 0069, accepted by the user on 2026-11-14, is the decision:
hibernate and resume are acts of their own beside the stage's state
machine, a kind declares the stages they apply to, and the idle time is
the life of the prompt cache. Its statement is clauses 212 and 213 at
boomdrivedev/blueprints 30dce35, on `design/a-stage-declares-it-hibernates`,
with 42, 49, 53 to 56, 109, 179 and 198 amended to meet them, C.2 and
C.7 naming the hibernate time, and scenarios S124, S125 and S127; and
weir's 53, 58 and C.4 at the same revision giving a simple kind's stage
`hibernate`. Rules
core.10 and core.32 are amended with it: a resume of a hibernating
stage starts exactly one fresh agent under an identity of its own,
carrying the stage's own session, on the account the hibernated agent
ran under while that account is usable and otherwise on the one 145
chooses where it reaches the conversation, and is no retry.

boomdrive-design ruled on the three points the construct put to it, at
30dce35: weir names the field, `hibernate`, a bool weir returns as
written and leaves out when omitted, its meaning the reader's, so flume
reads it absent as never (weir 53, 58, C.4), weir's unit
`a-stage-may-declare-that-it-hibernates` adding it to weir-kinds; the
hibernate time is `hibernate_after`, the host's or the launch's alone as
`keep_open` is (212, C.2, C.7); and a resume's agent carries its
start's identity with the resume's number after it,
`<stage>:<start>.<resume>` (213). The change goes on to code before
weir's revision lands, its weir pin the one task that waits on it.

This change builds 212 and 213 and their meeting with 42, 49, 51 to
53, 109 and 198. It builds nothing of 214 and 215, a delivery to a
role's address resuming the role and the deliver part, nor `message`
replacing `tell` (53), nor flume's MCP serving `hibernate` and
`resume` (C.2): the changes that build the role's deliveries and the
standing role carry those, and each calls this change's resume.

### What stands in the code

On `bolt/flumes-roles-sleep-and-wake` at 2d180f0b:

- **An agent kept open.** `open_sessions.rs` keeps a stage ended short
  needing an answer live until its end's `open_until`; a settle's
  `close_session` reads it let go (`ClosedBy::Time`, `Stopped`,
  `AgentEnded`, `AccountSpent`), ends it once quiet, and the stage waits
  on its answer, which is then a retry with a fresh agent.
- **A carried start.** A spent agent's stage (`spent.rs`) and a lost
  conversation's retry (`acts.rs`) put the stage waiting for room with
  `RoomWait.carry`, a `CarryFrom` naming the start whose conversation
  the next start carries and its `CarryCause`. `Engine::claim`
  (`start.rs`) takes a lost start's account first, asks the agent
  profile's `carry`, waits while `live` reads a process running the
  conversation, and starts the stage afresh with `StartReason::Lost`'s
  words where nothing carries it. `uncarried.rs` reads a carried start
  never begun and starts the stage afresh after it.
- **A start's number.** `Identity` is `flume:<repository>:<place>:<stage>:<start>`;
  `claim` takes `next_identity`, the stage's `starts` plus one, and
  writes `stage.starts` from it. The Claude Code profile keeps a start's
  record under its identity, the herdr and Rex profiles name its agent
  from it, and the status line reads it back from `FLUME_AGENT`.
- **A conversation's quiet.** `end.rs` reads a running conversation's
  agent for loss alone: it tracks no quiet, since a conversation has no
  quiet limit (44, 109).
- **The kind's stages.** The kinds profile maps weir's `Stage` into the
  seams' `Stage` (`conversation`, `waits`, `repeats`, …). weir's
  `Stage`, at the revision the tree pins (52b9552), gives no
  `hibernate`: there it falls into its unread `sections`, which core.4
  forbids flume to read a field from.

## Goals / Non-Goals

**Goals:** 212 and 213 as the statement holds them, with S124, S125
and S127 as conformance scenarios; the acts 212 lists taken on a
hibernating stage; status, the line and the list saying it.

**Non-Goals:** 214, 215, `message`, the MCP tools, a role's standing
place; section 10's open question on Claude Code's resume dialog, which
this change needs no answer to on the pinned release (decision 9).

## Decisions

### 1. The kind's word is weir's; flume's seam carries a bool

The seams' `Stage` gains `hibernates: bool`, false where the kind
declares nothing and in a held state written before, as `repeats` is
(140; core.4). The weir kinds profile fills it from weir's `Stage`'s
`hibernate` alone, read through weir-kinds at the revision the tree
pins, true where the declaration writes `true` and false where it
writes `false` or leaves it out, and never from `sections` (weir 53,
58; core.4, core.22). Every other part of this change is built and
proved on the fake kinds, which give the seam's `Stage` directly; the
pin moves to the revision of weir that lands
`a-stage-may-declare-that-it-hibernates` in this change's last kinds
task, which waits on that revision alone.

*Path not taken:* reading `hibernate` from `sections` until weir names
it, which core.4's details rule out for `repeat` and `standing` alike.

### 2. The hibernate time is a setting the host or the launch gives

`Settings` gains `hibernate_after`, one time in the forms `keep_open`
takes (`<n>`, `<n>s`, `<n>m`, `<n>h`), resolved at each start from the
layers `keep_open` is resolved from: the launch's settings and the
host's agents tables, a stage's own first in each; flume's own one
hour where none gives it (`HIBERNATE_AFTER`, beside `KEEP_OPEN`); `0`
hibernates no stage on its idling. A repository's settings and a kind's
declaration that give it fail to read, as a `keep_open` does, since how
long an agent holds the host's room and an account is the host's or the
launch's (198, 212, C.2, C.7). Status gives it with its layer (56).

`hibernate_after` is kept apart from the kind's `hibernate`, a stage's
bool, so one key is never both (212, C.2; weir 53).

*Paths not taken:* reusing `keep_open`, which 212 keeps apart (an
agent kept open for an answer and a conversation's idling are different
waits); a kind's declaration giving the time, which 198's reason for
`keep_open` rules out the same way.

### 3. A resume's identity: the start's number and its resume

`Identity` gains `resume: u32`, zero for a start, serialized only where
it is not, so a held state and an identity written before read the
same. It is written `flume:<repository>:<place>:<stage>:<start>` for a
start and `flume:<repository>:<place>:<stage>:<start>.<resume>` for the
`<resume>`th resume of that start, as 213 spells it. A resume counts no
start (48, 212, 213), so
the stage's `starts` and the start's `n` stay; a stop-short's key, an
answer's count and a deliverable's "since it started" stay the start's.
Every agent still carries an identity no other live agent carries
(core.10): the start record's folder, the herdr and Rex names and the
started ack are keyed by the whole identity. The herdr and Rex names
spell a resume's start `<start>r<resume>` within their legal
characters, before the digest of the whole identity. The status line
reads both spellings back.

*Paths not taken:* a resume as the stage's next start number, which
would count a start 212 says it does not; reusing the hibernated
agent's identity, which core.10 forbids.

### 4. Where a hibernation is held

On the hibernated start, a `HibernationHeld`: since when, why
(`idle`, `caller`, `kept_open`), and the account it ran under. And the
start's `resumes`, each a `ResumeHeld`: when, what started it (`caller`,
`tell`, `answer`), the resumed agent's identity, the words it was given,
and whether it carried the conversation, as `Conversation` says it. A
bare resume's words are flume's own and none are kept of it, so no
fresh start after it repeats them. A resume's start takes them over
from the start it resumes, so status gives each resume of the stage's
current start (56). A retry's fresh start holds none, and so does the
fresh start a resume makes where it could not carry its conversation,
whose own `conversation` says why. A stage waiting to resume keeps in
its `RoomWait.carry` what resumed it, `CarryFrom.resumed_by`, so the
claim that admits it under the room lock records it.

The stage's state:

- A running stage hibernated, a conversation's on its idling or at its
  caller's word, or any stage at its caller's word, reads
  `StageState::Hibernating` (word `hibernating`). Its end stays none:
  it neither ended nor moved on (212).
- A stage ended short whose agent was kept open stays
  `StageState::EndedShort`, waiting on its answer as before (42, 130);
  its end's close records `ClosedBy::Hibernated`. `waits_on_answer`,
  the stall and the key read it as today.

A hibernating stage waits on no move of its caller's: it is never read
stalled (130). One ended short waits on its answer as any does.

*Path not taken:* `Hibernating` for the ended-short case as well, which
would make every reading of a stage waiting on its answer learn a
second state.

### 5. When a conversation is idle

A running conversation whose kind hibernates it is read at each settle:
quiet (31), no question open (90), no own work running (91), and no
tell being handed to it. Its quiet runs from flume's first reading of
it quiet, as 44's does, kept in the start's `first_quiet`, and starts
again whenever flume reads it not quiet. The settle the agent's own
quiet starts (`flume settle <place> --on-quiet`, the Claude Code
plugin's Stop hook) starts it again too, since a turn ended then: a
person typing into the pane between two settles runs a turn no reading
of the watch need see. Once quiet for its hibernate time, the settle
hibernates it.

A tell being handed marks the start, in one write before it reads the
agent, `handing` until its bound and its two readings of receipt pass,
and clears the mark when it ends; a settle reads a start so marked not
quiet, its quiet starting again, and a caller's hibernate is refused
unforced while it stands (54, 212).

*Path not taken:* the agent's transcript's last write, which no part of
the agent seam gives (B.4) and which a part of its own would need a
clause for.

### 6. Hibernating an agent

`hibernate.rs` holds one step, used by the settle and the caller's act:
the agent read quiet, or, at a caller's forced hibernate, working (31);
the hibernation claimed on the start, its `HibernationHeld` written
while it is still live, in one compare-and-set write taken only where
no other has claimed it; the agent closed through its start's terminal;
and then, in one more write, the start read not live, with the stage's
state as decision 4 says. Two settles at once so close the agent once
and record its hibernation once (74, 75). A claim a settle finds
standing on a live start, its close or its last write not made, is
carried on by that settle once the agent is quiet. A close the terminal
refuses is a failed end, as elsewhere, and lets the claim go. Nothing
counts a start, a retry or a going back (48), and the place's room and
the account are free once the start is not live (70, 145).

*Path not taken:* closing the agent before any write, then recording
the hibernation, which lets two settles at once both close it.

For an agent kept open, `let_go_by`'s `ClosedBy::Time` reads
`ClosedBy::Hibernated` where the stage hibernates, and `end_let_go`
records the hibernation in the write that records the agent ended. An
answer that comes after the time passed and before the agent is ended
is taken as today's: refused by a tell, naming the retry, and a retry
with words ends the agent and starts afresh.

A caller's `flume hibernate <place>` is refused, naming the place, for
a place whose current stage its kind does not hibernate and for a
place with no live agent (212); it is taken for any stage whose kind
hibernates it with a live agent, a conversation's, one running, or one
kept open for its answer and not let go, an agent not quiet only when
forced (31). 212 refuses those two alone, so a running stage that is
not a conversation is taken too. The stage's agent is one that runs or
is kept open: an agent the terminal brought back in a lost stage's
pane is not the stage's (77), and a hibernate of a stage in any other
state is refused naming its state.

### 7. A resume is a carried start of the same start

A resume puts the stage waiting for room, with `RoomWait.carry` naming
the hibernated agent's identity and `CarryCause::Hibernated`, and
`RoomWait.words` the resume's prompt words, then calls `start_stage`,
so it waits for room, for its account and for no process running the
conversation as a lost conversation's retry does (49, 70, 207;
core.10). `claim` reads `Hibernated` as it reads `Lost`:

- the account the hibernated agent ran under first, while it is
  usable, else the one 145 chooses (213; core.32);
- the profile's `carry` asked for the account chosen;
- where it carries, the start is the resume: the identity of decision
  3, `stage.starts` kept, and a `StartHeld` whose `n`, `start_head`,
  `made_from`, `words`, `answered`, `group` and `resumes` are the
  hibernated start's, with the resume appended, its fresh secret,
  settings resolved now (32), account, terminal and session, its
  `conversation` recording `Hibernated` and carried, its record naming
  the session it resumes (161), and its prompt the resume's words then
  the way to stop short (28);
- where it does not, the stage starts afresh as a fresh start of its
  own, the next start number, its prompt the stage's with
  `StartReason::Hibernated`'s words before the resume's words: its
  conversation could not be carried, and what it committed and left is
  in the place (213, S127). It counts toward no bound of the kind's and
  waits on no caller, as a spent agent's fresh start does (146).

A resume never begun, or whose agent is not the stage's, is read as
`uncarried.rs` reads a carried start (146, 213): its agent ended once
quiet, and the stage started afresh after it with
`StartReason::Hibernated`'s words and the resume's words.

The resume's prompt words are, by what resumed it (213):

- a caller's `flume resume`: `StartReason::Woken`'s words, that the
  session resumes after it hibernated and what ran in it then has
  stopped;
- a tell: the tell's words, taken as the resume's at once, the tell
  answered with the place's status as a retry is, not handed within
  the tell's bound, since a resume starts an agent and the words are
  its prompt, received or read never begun as any prompt is (38, 53,
  54);
- an answer, a tell or a retry with words to a stage waiting on its
  answer whose agent hibernated: the question, then the answer
  (`asked`), recorded on the stop-short it answers with who gave it,
  the key it named and when, in the write that puts the stage waiting
  (42); its start's `answered` raised and each hold on the stage ended,
  as an answer in session does. A key names the stop-short as 42 and 49
  say, refused otherwise.

`told` carries, after the start's words and the answers it took in its
session, each resume's words, so a fresh start flume makes on its own
after a resumed agent, spent or never begun, loses none (42, 146).

A resumed stage is a running stage: its stop check, its quiet limit and
working past it hold from the resume's prompt received, its
`begun_at` (44, 110, 118, 213).

### 8. The acts on a hibernating stage

212 says a hibernating stage's place takes a finish, a stop, a drop or
a done as any place does, with no agent to end, and that a retry means
what it means of any stage:

- **stop** records the hibernation over and the stage `Stopped`,
  waiting on a retry or a drop (51); one ended short stays ended short,
  its close `Stopped`, its answer then a retry (42), whether the agent
  its close read hibernated was ended yet or not.
- **finish** of a hibernating conversation counts it delivered (109,
  40); of any other stage it is refused as today.
- **drop** and **done** make the place's work over (52, 114).
- **retry** with no words, or of a stage that is not waiting on its
  answer, starts the stage afresh, carrying nothing (49; core.32); with
  words naming a stage waiting on its answer whose agent hibernated, it
  is that answer, and resumes (decision 7).
- **tell** to a hibernating stage resumes it (53, 55, 213).
- **resume** of a place with no hibernating stage is refused, naming
  the place; of one whose agent hibernated waiting on its answer, naming
  the tell or the retry with words that gives the answer and resumes it.

### 9. The dialog Claude Code opens on resume

Record 0069 read Claude Code 2.1.322 opening "Resume this
conversation?" for a session idle past its cache. The tree pins 2.1.296
(core.39), whose resume opens no such dialog. Where a later pin opens
one, the start is held at a question its program asks before it takes
its prompt, which flume does not answer (34, 35): the place waits on a
person as for any such dialog. Section 10's question stays open; this
change neither answers it nor needs to on the pin.

### 10. Status, the line and the list

Status gives, of the current stage, whether its kind hibernates it, its
hibernate time with its layer, of one hibernating since when and why,
and each resume with what started it, its identity and whether it
carried its conversation or why not (56, 212, 213). The form gains
fields only (61). The line's state is `hibernating` for a stage so
held, ahead of `in conversation` (57); one ended short reads `waiting
on an answer`, as it does. The list gives the line's word (200). The
terminal's clues show it as a settle shows any state (139).

## Rules this design relies on

- core.4, core.22: the stage's word read from weir's `Stage` at the
  pinned revision alone (decision 1).
- core.10, core.32 as amended by record 0069: the resume's agent, its
  identity and its account (decisions 3 and 7).
- core.11, core.12: the resume's prompt sent as any prompt, the dialog
  left to a person (decisions 7 and 9).
- core.13: no process of flume's waits for a hibernate time; a settle
  reads it (decision 5).
- core.16, core.17: the hibernation held in the place's document,
  written by compare-and-set and safe to repeat (decisions 4 and 6).
- core.18: a hibernating stage's place is tidied as any, the live
  check reading its last conversation (decision 8).
- core.26: a hibernating agent holds no room (decision 6).
- core.30: the hibernate time a setting with its layer (decision 2).
- core.1, core.2: the core names no tool; the profiles carry the
  identity's spelling into their names (decision 3).

`docs/architecture/` has no area page; `core.md` binds the whole
change. No rule is departed from.

## Seams

- The kinds seam's `Stage` (decision 1), filled by the weir kinds
  profile, `crates/flume/src/profiles/kinds/weir/`.
- The agent seam's `carry` and `live`, unchanged, asked for a resume
  as for a lost conversation (decision 7), the Claude Code profile
  answering.
- The terminal seam's `close` and `open`, unchanged (decision 6).
- `Identity`, passed across every seam, gains the resume (decision 3);
  the herdr, Rex and Claude Code profiles spell it.

## Risks / Trade-offs

- **weir's revision.** The change cannot land whole until weir's
  `a-stage-may-declare-that-it-hibernates` lands: its weir pin task
  waits on that revision. → Every other task is built and proved on the
  fakes first; where the revision has not landed when the code stage
  reaches that task, the stage stops short naming it, as boomdrive-design
  ruled.
- **The statement's branch.** 212, 213, C.2, C.7 and weir's 53 and C.4
  are cited at 30dce35, on a design branch waiting on the user's push.
  → A later wording there that changes what they say is carried here
  before the change lands.
- **A turn no settle sees.** Under a terminal or a program with no
  on-quiet hook, a person's turn between two readings leaves the quiet
  clock running from the earlier quiet. → The degraded on-quiet part is
  named in status already (171); the hibernate time is one hour, and a
  resume carries the session, so the cost is a cache miss.
- **A transcript cleaned.** A host whose homes set no
  `cleanupPeriodDays` loses a transcript at thirty days (C.4, S127). →
  The resume starts afresh, saying so; the host's setting is
  boomdrivedev/boomdrive's.
