# A stage whose kind hibernates it sleeps and resumes

## Why

A conversation stays in its pane until its caller finishes it (109),
holding its room and its account all day, and an agent kept open for
an answer is let go after an hour and its stage then restarts afresh on
its answer's retry (198, 49). The operator ruled that a session be let
sleep once its prompt cache no longer pays, and woken in its own session
when someone speaks to it: "Call it hibernate and resume, and keep it
separate from the standard state machine concept." "You're keeping it
open as long as you're benefiting from the cache and once you can't
benefit from the cache anymore, you might as well hibernate it." "if
it's stuck waiting for an answer, it should be allowed to hibernate and
then be woken up when the answer comes" (record 0069, accepted
2026-11-14).

The statement holds it at boomdrivedev/blueprints 81288ba, clauses 212
and 213, with 42, 49, 53 to 56, 109, 179 and 198 amended to meet them,
and scenarios S124, S125 and S127. Rules core.10 and core.32 are amended
with the record: a resume is one fresh agent under an identity of its
own carrying its stage's own session, on the account 213 names, and is
no retry. No code holds any of it yet.

## What Changes

- **A stage may hibernate (212).** A kind declares, by weir's field on
  a stage, that the stage hibernates. flume's own kinds declare none.
- **It is hibernated three ways (212).** A conversation's, once its
  agent has been quiet for its hibernate time with no question open, no
  own work running and nothing being handed to it; any such stage's
  with a live agent, at its caller's `flume hibernate`, which is refused
  for a stage whose kind does not hibernate it and for a place with no
  live agent; and any such stage's, once its agent kept open for an
  answer reaches the end of its time, in place of being let go. Hibernating ends the agent once it is quiet, keeps its
  session, frees its room and its account, counts no start and no retry,
  and leaves the stage neither ended nor moved on.
- **The hibernate time (212).** The agent's settings give it, in the
  forms `keep_open` takes, from the launch's settings and the host's
  agents tables alone, as `keep_open` is given; flume's own one hour
  where none does; `0` never hibernates a stage on its idling.
- **A resume (213).** A caller's `flume resume`, a tell to the place,
  and the answer to the stop-short a hibernated agent waits on, by a
  tell or a retry with words, each resume it: one fresh agent under an
  identity of its own, the stage's start number kept, carrying the
  hibernated agent's conversation where its profile can, on the account
  that agent ran under while it is usable and otherwise on the one 145
  chooses, waiting for room and for no process running its conversation
  as a carried start waits. Its prompt is the tell's words, or the
  question with its answer, or flume's words at a bare resume, then the
  way to stop short. Where nothing reaches its conversation, the stage
  starts afresh with that prompt behind flume's words.
- **The acts on a hibernating stage (212).** A stop, a finish, a drop
  and a done are taken as of any stage, with no agent to end; a retry
  starts the stage afresh as any retry does, but one with words naming
  a stage that waits on an answer, which is that answer.
- **Status, the line and the list** give whether the current stage
  hibernates, since when and why it hibernates, its hibernate time with
  its layer, and each resume with what started it and whether it carried
  its conversation (56). The line's state is `hibernating`.
- **The command line** gains `flume hibernate <place> [--force]` and
  `flume resume <place>`, and `flume launch --agent
  [<stage>.]hibernate_after=<time>`.

## Capabilities

### New Capabilities

- `hibernation`: a stage whose kind hibernates it is hibernated once
  idle past its hibernate time, at its caller's word or at the end of
  its keep-open wait, and a caller's resume, a tell or its answer
  resumes its own session.

### Modified Capabilities

- `caller-acts`: a stop, a finish and a tell of a hibernating stage; a
  retry of one.
- `carried-conversations`: a resume carries its stage's own
  conversation, beside a start after a spent agent and a lost
  conversation's retry.
- `stage-runs`: an agent kept open of a hibernating stage is hibernated
  at its time in place of being let go; a hibernating conversation is
  never stuck, lost or working past its limit.
- `launching`: the hibernate time resolved at each start, as
  `keep_open` is.
- `kinds-profile`: a stage's `hibernate` read through weir's crate; a
  declaration gives no hibernate time.
- `command-line`: `hibernate`, `resume`, and `--agent hibernate_after`.
- `place-status`: a stage's hibernation and its resumes.
- `place-line`: the state `hibernating`.
- `seam-contracts`: an agent's identity names its resume; a kind's
  stage says whether it hibernates.

## Impact

- `crates/flume-seams`: `Identity` gains a resume; `Stage` gains
  `hibernates`; `Settings` gains `hibernate_after`; the held state gains
  `StageState::Hibernating`, a start's hibernation, its resumes and its
  handing mark, `CarryCause::Hibernated`, `ClosedBy::Hibernated`, and
  `Act::Hibernate` and `Act::Resume`.
- `crates/flume-core`: a new `hibernate.rs` (the settle's step and the
  caller's act), `start.rs` (a resume's claim), `acts.rs` (stop, finish,
  tell, retry), `open_sessions.rs` (the kept-open agent's end of time),
  `end.rs` (a conversation's quiet), `settings.rs`, `status.rs`,
  `line.rs`, `answer.rs`.
- `crates/flume`: `commands/mod.rs` and `commands/launch.rs`,
  `host/config.rs`, `profiles/kinds/weir/mod.rs`, the Claude Code
  status line's reading of an identity, the herdr and Rex agent names.
- `crates/flume-conformance`: the clause list, and `engine/hibernation.rs`.
- `site/`: the commands, the kinds' fields, the settings and the
  stage states.
- The kinds profile reads `hibernate` from weir's `Stage`, which the
  revision of weir the tree pins does not give: weir's format names the
  field first, at a revision the tree's pin moves to (core.4, core.22).
- Out of this change: a delivery to a role whose place hibernates
  resuming it (214), the deliver part (215), `message` replacing
  `tell` (53) and flume's MCP serving `hibernate` and `resume` (C.2),
  which the changes that build 214 and 215 and the standing role carry.
