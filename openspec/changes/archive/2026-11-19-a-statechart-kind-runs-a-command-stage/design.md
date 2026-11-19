# Design

## Context

See proposal.md for why. The statement is read at boomdrivedev/blueprints
main: 4, 13, 14, 25, 27, 49, 51, 56, 223, 224, I1, I8, S130, C.2, C.5
and C.7 say what this change builds, C.5's paragraph on a command stage
(added 2026-11-18) among them. The rules that bind are
`docs/architecture/core.md`'s. This design relies on core.1, core.2,
core.4, core.5, core.10, core.13, core.14, core.16, core.17, core.29,
core.42 and core.60, and amends core.9 for a command stage's end.

## Goals / Non-Goals

**Goals:**

- A kind holding a command stage launches and runs as 223 says, the
  command run through the terminal by flume's launcher as its child.
- Every rule flume holds for a statechart kind's stage of agents holds
  as before.

**Non-Goals:**

- A command run past its stage's limit reported working past it (118):
  a command stage names no limit here.
- A link of a package's program outside the package checked at the
  launch: a path naming `..` or an absolute path is refused, and a link
  is left to the launcher, which runs the program the path names.

## Decisions

1. **Both templates in one text.** `atoms::templates()` is
   `stage@1.toml` then `command@1.toml`, each a statechart kind of its
   own; the kinds seam's `read_statechart` reads every statechart kind of
   the templates' text, keyed by its name. A place launched before this
   change holds the stage template alone, which reads as before.
2. **The command template reuses the stage template's atoms.**
   `start_sessions` records the stage's one session, proven by
   `stage.started`, and `stage.end` reads `passed` or `failed`, so the
   tick, its proofs and the snapshot are unchanged.
3. **A session carries its command.** `ChartSession.command` holds the
   command with `{{params}}` filled with the place's `params` path, and
   a program by a path found from the declaration's file: the
   repository's declaration's folder within the place, a launch's
   declaration's folder, or a host source's package folder.
4. **A run is a start with no account.** `start_command` writes a live
   start under the next identity with a fresh secret, no account, no
   settings and an empty prompt, its record marked `command`, and asks
   the stream's terminal to `run` it with `FLUME_AGENT`, `FLUME_SECRET`,
   `FLUME_PLACE` and `FLUME_STAGE` and the host's launcher words. It
   takes no room: no claim under the room lock is made.
5. **The launcher writes beside the start's record.** Its word is
   `launched/<identity>.json`, as for an agent; a command's exit is
   `ran/<identity>/exit.json`, `{"exit": <code>}` with no code where a
   signal ended it, and its output `ran/<identity>/output.log`. The core
   reads them through the store's `read_file`.
6. **`StageState::Failed`.** A command stage's end that is no delivery
   is `failed`, an end like any other, recorded once and delivered to
   the place's recipients. Its `StageEnd.ran` gives the exit and the
   output's path, and its words say how the command exited.
7. **A failure's words go to the next agent.** The tick takes a command
   stage's `failed` as it takes a stage sent back: its words are kept on
   the chart and given the next sessions that run an agent; a command
   stage's session takes no words.
8. **herdr's run.** The pane is made as an agent's, the link put first
   on its `PATH`, and once its shell is at its prompt `herdr pane run
   <pane> <line>` runs the quoted line, herdr 0.9.5's own command for a
   program in a pane. Its state is herdr's reading of the pane, stopped
   once the shell is its foreground again.
