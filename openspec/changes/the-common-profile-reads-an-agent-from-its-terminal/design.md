# Design

## Context

See proposal.md for why. The statement is read at boomdrivedev/blueprints
48a94da, the revision the clause list holds: 31, 38, 89 to 93, 96, 134,
136, 171 items 2 to 11, 174, C.4's common profile, C.5's herdr send and
state rows, and S73. Record 0026 decided what the common profile reads
from its terminal; this design decides how it is built.

### What the tree holds now

- **The agent seam's parts** (`crates/flume-seams/src/agent.rs`).
  `Part` holds the eight parts flume reads a lesser way today, spent to
  own settings, `Part::ALL` in 171's order, each with its `name` and its
  lesser reading, `read`. `Parts` names those a profile builds; status
  gives the rest as the agent's `degraded` (`status.rs`). Claude Code's
  profile builds the eight; the fake agent's parts follow its scripts;
  the common profile builds none.
- **The common profile** (`crates/flume/src/profiles/agent/common.rs`)
  answers received, asking, own work and ended unknown and last words
  none, so a stage of its program starts through the launcher and goes
  no further: its prompt is never read received, and it is never quiet.
- **The core's readings** (`crates/flume-core`). `read_agent` and
  `state_of` in `reading.rs` read asking and own work from the agent
  seam and the state from the terminal, taking the terminal's asking
  only with the profile's mark; `read_out` takes last words from the
  agent seam; `deliver.rs` reads a prompt received from
  `Agent::received`, within 60 seconds of each sending, and sends once
  more; `acts.rs`'s `hand` reads a message's words received the same way;
  `agent_acts.rs`'s stop check, `line.rs`'s asking and `status.rs`'s
  last words each ask the agent seam.
- **The terminal seam's send** (`terminal.rs`) delivers text, its held
  input cleared first, and answers sent or refused. herdr's is `agent
  send-keys <pane> ctrl+c` then `agent prompt <pane> <text>`.
- **A start's held record** (`held.rs`, `StartHeld`) keeps
  `first_quiet`, the first reading of its agent quiet with nothing
  delivered, and `asking_since`, written by the settle that reads a
  begun stage's end.
- **The clause list** (`crates/flume-conformance/src/clauses.rs`) names
  171, 173 and S71 to S73 unbuilt under `PROFILES`.

## Goals / Non-Goals

**Goals:**

- A stage of a program the common profile binds runs through: its
  prompt received on the terminal's word, its agent asking, quiet and
  ended as its terminal reads it, and its stage delivered (171 items 2
  to 5, S73).
- No part is read from the terminal where its profile builds it, and
  status names each part read so (93, 170, 174; core.39).
- No profile reads a terminal, and the core names no tool (core.1,
  core.2).

**Non-Goals:**

- The plan part (164) and the usage part (152), which join the parts
  with the changes that add them to the seam, as record 0026's
  amendment of 2026-11-18 has it.
- The prompt's argument under the printed command (C.4): the printed
  command reads no turn and sends nothing, so a stage of such a program
  stays as the launcher's change left it there.
- The program a pool's account chooses (173, S71, S72).
- The refusal of a program herdr reads only through its integration
  (S75), already built.

## Decisions

### 1. Five more parts, read by the core

`Part` gains `Received`, `Asking`, `OwnWork`, `Ended` and `LastWords`,
before `Spent`, so `Part::ALL` holds thirteen in 171's order: received,
asking, own work, ended, last words, spent, windows, send-back, guard,
on-quiet, mode, carry, own settings. Their words are `received`,
`asking`, `own_work`, `ended` and `last_words`; their names `received`,
`asking`, `own work`, `ended` and `last words`; and each lesser reading
says it is read from the terminal and how, naming no terminal (core.1):

- received: "read from the terminal, received once it sees the turn the
  sending began, the agent working or asking, within 60 seconds";
- asking: "read from the terminal, asking while it reads the agent waiting
  on a question";
- own work: "read from the terminal, none once two settles a minute or more
  apart read the agent waiting, and running until then";
- ended: "read from the terminal, ended once it reads the agent stopped";
- last words: "read from the terminal, what it shows of the agent".

A profile that builds one of the five answers it from its program's
own record, as every profile does now. For a part it does not build,
the core reads the terminal and asks the profile nothing; the profile
answers the seam's lesser way, unknown or none, where it is asked, as
the agent seam's scenarios hold every profile to (93). Claude Code's
profile names all thirteen. The common profile builds none, its answers
unchanged, so status names every part degraded with its agent.

Not taken: one part, "read from the terminal", standing for the five,
which would leave a profile that reads its program's asking but not
its own work no way to say so; and the profile reading the terminal
itself, which would put a terminal's reading inside an agent profile
(core.2), as the launcher's change found for started.

### 2. The core's readings, each in one place

`reading.rs` gains one reading for each part the core reads, used
wherever the core reads it now, so a settle, status, a message and the
stop check read alike:

- `Engine::read_asking(agent, folder, start)`: the profile's answer
  where it builds asking; otherwise the terminal's state, asking as yes,
  working, waiting and stopped as no, unknown as unknown. It takes the
  place of every `agent.asking` the core reads: `read_agent`, the
  prompt's sending in `deliver.rs`, the message's wait in `acts.rs` and the
  stop check in `agent_acts.rs`.
- `state_of`: the terminal's asking is asking where the profile's mark
  says so, as now, or where the profile does not build asking (90, 171
  item 3), and unknown otherwise; the terminal's unknown is stopped
  only where the profile builds ended and its record says the session
  ended, so a profile that does not build ended is read stopped by the
  terminal alone (134, 171 item 5).
- `Engine::read_own_work(agent, folder, start, state)`: the profile's
  answer where it builds own work; otherwise, from the state as the core
  reads it and the start's held `waiting` (decision 4): none where the
  agent is read stopped, or waiting with `waiting.at` a minute or more
  after `waiting.since`; running where it is read working, asking, or
  waiting short of that; unknown where its state is unknown (31, 91,
  171 item 4).
- `Engine::read_last_words(agent, folder, start)`: the profile's answer
  where it builds last words; otherwise what the terminal shows of the
  agent, as a dialog's words are trimmed, none where it shows nothing
  (43, 136, 171 item 5). It takes the place of `agent.last_words` in
  `read_out` and in status.

`read_agent` reads `started` as now, `asking` by `read_asking` and
`own_work` by `read_own_work`. The ended part needs no reading of its
own: `state_of` is the one place the core reads it.

### 3. Received: a send that watches the turn

The terminal seam gains:

```rust
/// send, watching the turn: `text` delivered as `send` delivers it, its
/// held input cleared first, with the terminal's word whether it saw the
/// turn the sending began, the agent working or asking, within `within`.
fn send_watching(&self, agent: &Identity, text: &str, within: Duration) -> SentWatching {
    SentWatching::Refused { reason: NO_TURN.into() }
}

pub enum SentWatching { Began, NotSeen { why: String }, Refused { reason: String } }
pub const NO_TURN: &str = "the terminal reads no turn a sending began";
```

serialized in snake case, and named as `Sent` answers `send`, since
the store's `Watched` is the host's watch. The default refuses, so a terminal that reads
no turn types nothing; under it a stage of such a program is reported
never begun, its failure naming why. Rex opens no program but `claude`,
and the printed command sends nothing, so neither needs it.

Where the agent's profile does not build received, the core sends the
prompt, and a message's words in `hand`, by `send_watching` with
`RECEIVED_WITHIN`, and reads them received:

- on `Began`, at once: the prompt's `received_at` and the start's
  `begun_at` are the time it answered, and the stage is begun;
- else, where a reading within 60 seconds of the last sending reads the
  agent working or asking, since the agent was read waiting and not
  asking when it was sent: the terminal saw its turn begin, as herdr's
  wait may not within its own five seconds;
- else not received, and after the 60 seconds the prompt is sent once
  more, and a second sending not received is reported never begun, as
  38 has it. `NotSeen` is a sending not received; `Refused` is a send
  refused, as `Sent::Refused` is now.

Such a start names no record, so `own_record` reads it its start's, as
for a start not yet received (77): a program started otherwise in its
pane is found by its process (S74). Every sending to the agent, its
prompt or a message, clears its `waiting`.

Not taken: a flag on `send`, which would change every terminal profile
and fake for one profile's need; and the reading of the agent working
alone, at the settles that follow a plain send, which misses a turn
that ends between two readings, where herdr's wait watches the turn as
it begins.

### 4. Own work: two settles a minute apart

`StartHeld` gains `waiting: Option<WaitingRead>`, `WaitingRead { since,
at }`: when a settle first read the agent waiting, and when one last
did; none while a settle reads it otherwise, left out of the JSON while
none, and none in a start written before. `OWN_WORK_AFTER`, 60 seconds,
is the span (171 item 4).

`Engine::read_waiting(id)`, run by the settle's `step` for a live
place before its stage's step, reads the current live start whose
profile does not build own work: where `state_of` reads its agent
waiting, it writes `waiting` with `since` kept, or now where none, and
`at` now; any other reading clears it; a profile that builds own work is
not read. Every sending clears it (decision 3). Status, the list and a
wait read it and write nothing (core.52), so own work is none only on
two settles' readings, as 171 says.

A stopped agent's own work reads none: its program no longer runs in
its part, and a stopped agent whose start began otherwise is ended and
its stage started afresh as any other (161).

Not taken: a time kept beside the held state, which core.16 rules out;
`first_quiet`, which counts from a reading already quiet; and own work
running for a stopped agent, which would keep such a start from ever
being ended (31).

### 5. Asking in the place's line

The line reads no terminal (57, 58; core.52). Where the live agent's
profile builds asking it reads the profile's mark, as now; where it
does not, it reads the start's `asking_since`, which the settle that
reads a begun stage's end writes from `read_asking` (90, 217).

Not taken: the line reading the terminal, which the list and a line
asked alone may not.

### 6. herdr watches the turn

herdr's `send_watching` finds the pane as `send` does, sends `agent
send-keys <pane> ctrl+c`, then `agent prompt <pane> <text> --wait
--until working --until blocked --timeout <ms>`, `within` in
milliseconds, 60000 from the core, under a bound of `within` and the
profile's `PAST_TIMEOUT`, so herdr's own timeout answers first
(core.29). An answer is `Began`; `agent_prompt_stalled` and `timeout`
are `NotSeen` in herdr's words; any other refusal, or no answer within
the bound, is `Refused` in herdr's words (C.5). herdr 0.9.5's `--wait`
needs a working or blocked state within 5000 ms of a submission from a
state not working, else answers `agent_prompt_stalled`.

### 7. The fakes

The fake agent's `reads_from_its_terminal(parts)` degrades each of the
five it is given, its parts then naming none of them, and each then
answers the lesser way where asked, so a scenario reads one part from
the terminal while the fake still builds the others; the conformance
suite's harness `reading_from_its_terminal` degrades all five. The fake terminal's `send_watching` delivers as its `send`
does, refuses as it does, and answers `Began` unless a test scripted
`watch_as(agent, SentWatching)`, each asking logged `send-watching
<identity>`.

### 8. The clause list

`REVISION` stays `48a94da`. 171 and S73 leave `unbuilt`, cited by the
engine's scenarios of an agent read from its terminal; 173, S71 and S72
stay under `PROFILES`, whose words name a pool's program alone. Each
scenario cites Parts A and B and section 11 alone, C.4 and C.5 named in
its words.

## Rules and clauses

- core.1, core.2: the core reads the terminal and the agent each
  through its seam; no profile reads a terminal; the parts' readings
  name no terminal.
- core.9: a stage's end is still read from its deliverable; the
  terminal's readings decide only whether its agent is quiet, asking or
  stopped.
- core.11: a prompt or a message is sent only to a waiting agent that
  started, its input cleared first, read received within 60 seconds or
  sent once more.
- core.14: status's form gains five words of `part`, and nothing else.
- core.16, core.17: `waiting` lives in the held document, and its
  writing is safe to repeat.
- core.29: herdr's watched prompt runs within its bound.
- core.39: a part is read from the terminal only where its profile does
  not build it, as 171 says.
- core.52, core.62: the line reads held state alone; a place with a
  live start is pending, so each watch's settle reads it.
- Clauses 31, 38, 43, 45, 89 to 93, 96, 119, 134, 136, 161, 170, 171,
  174, C.4, C.5, S73.

## Risks / Trade-offs

- [Own work the terminal cannot see] Background work a program runs
  while it waits ends when flume ends the agent → 171 accepts it; status
  names own work read from the terminal, so the kind's author and the
  host know.
- [A minute of waiting after every turn] A stage of such a program
  delivers no sooner than a minute after its agent goes quiet → as 171
  has it; Claude Code's profile, which builds own work, is not slowed.
- [Settles further apart than a minute] Under a watch whose bound
  passes a minute, two settles read the agent waiting once each, and a
  turn between them goes unseen → 171 reads two settles, whatever lies
  between; the deliverable is still read at the head the settle reads,
  so a stage delivers only what was committed.
- [A watched send waits] A prompt's sending waits up to herdr's 5000
  ms, and at most the 60 seconds, inside the settle or the start that
  sends it → no longer than the 60 seconds 38 reads a sending within,
  which a settle already waits.
