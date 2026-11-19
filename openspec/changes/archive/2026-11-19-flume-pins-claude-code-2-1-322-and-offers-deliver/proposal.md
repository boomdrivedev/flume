# flume pins Claude Code 2.1.322 and offers deliver

## Why

The hosts that run flume's agents run Claude Code 2.1.322, the release
C.4's deliver row was read on, while `devenv.nix` pinned 2.1.296. Every
entry of the Claude Code profile is flume's own code checked against the
release flume pins (core.39), and record 0069 built the plugin's deliver
part and left it unoffered until a chore moved the pin to a release that
runs a hooks module in an interactive session and read a batch taken
through it. This is that chore.

The entries were read again against 2.1.322's own program. Each holds as
2.1.296's did, the minifier's names aside: the command's options, each
hook event's input and how the release reads the hooks' answers, the
modes, the efforts, the plan's tool, the usage-limit words, the carry,
the record of a running session, its own settings and the keys the
profile reserves, and the folder-trust dialog. The release keeps one
credential more from the processes it starts,
`CLAUDE_CODE_SESSIONS_OAUTH_TOKEN`, and the engine types it writes for
a plugin's module gain what the module does not use. Nothing differs
from what flume's statement describes.

## What Changes

- **The pin.** `devenv.nix` pins Claude Code 2.1.322 by its four
  checksums from the release's manifest (core.22).
- **The fragments read again (core.39).** Each source fragment the
  check-tools tests find in the pinned program is 2.1.322's: the
  usage-limit words, the credentials kept from hooks, the modes, the
  carry, the running session's record, own settings and the reserved
  keys, and the efforts. The credentials gain
  `CLAUDE_CODE_SESSIONS_OAUTH_TOKEN` and the environment an
  organization's hook inherits, which the same builder gives.
- **Fragments for the entries no test read.** Check-tools tests find in
  the pinned program the input each hook event hands the plugin's hooks
  and how the release reads a SessionStart's context and a PreToolUse's
  refusal, `ExitPlanMode`'s input, the command's `--plugin-dir`,
  `--model` and the standing prompt's file, and the folder-trust
  dialog's choices and its mark of focus. The trust screens the tests
  hold are 2.1.322's.
- **The engine types** are the ones 2.1.322 writes; the module's
  `$.prompt.submit` with `asUser` is among what they type.
- **Claude Code's profile offers deliver (215, C.4; record 0069).** Its
  command sets `FLUME_DELIVER=1`, so the plugin's module takes every
  batch and message while its session waits and the terminal types none
  into a Claude Code agent's pane. A profile built without it, for a
  release that runs no hooks module, still sets nothing.
- **The stand-in takes its deliveries as the module does.** Where its
  session names `FLUME_PLACE`, `FLUME_AGENT` and `FLUME_DELIVER`, the
  check-tools stand-in for Claude Code runs `flume hook deliver` at its
  session's start, at each turn's end and every ten seconds while it
  waits, and takes what it prints as its next prompt, what was typed
  into its terminal kept there.
- **Two walkthroughs.** A check-tools walkthrough under herdr and the
  stand-in, and a live one under herdr and the pinned Claude Code: a
  standing role's session, words half typed in its input, takes messages
  for its address through the deliver part, nothing typed by flume, each
  recorded handed by `message`, and its input holds the words as typed.
- **The site** names 2.1.322 and says Claude Code's profile offers the
  deliver part (core.35).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `agent-profile`: Claude Code's modes and efforts are 2.1.322's; its
  credentials' fragments add the organization's hook's environment; it
  offers deliver at the pinned release; check-tools tests read the
  hooks' inputs, the command's options and the folder-trust dialog in
  the pinned release.
- `claude-code-plugin`: the mode compared at the first prompt cites
  2.1.322; the engine the module rests on names `$.prompt.submit` with
  `asUser`; a standing role's session takes its deliveries through the
  deliver part under herdr, its draft kept.
- `the-gate`: the live walkthroughs add a standing role's session taking
  a batch through the deliver part, its input left as typed.

## Impact

- `devenv.nix`: the pin and its comment.
- `crates/flume/src/profiles/agent/claude/`: every fragment read again,
  `hook::fragments`, `command_fragments`, `trust_fragments`, the
  credentials' list, `DELIVER` offered and `Claude::without_deliver`.
- `crates/flume/tests/profiles/`: the check-tools tests of the new
  fragments (`tools/claude_words.rs`), the stand-in's deliver part
  (`stand-in/claude.rs`), the standing role's source shared by the
  walkthroughs (`tools/walk.rs`), the two walkthroughs, the trust
  screens and the fixtures' release.
- `plugins/claude/engine-types/`: 2.1.322's engine types.
- `site/`: the Claude Code guide and the agent settings guide.
- No clause, record or rule changes: record 0069 decided the deliver
  part is offered once a pinned release is read running it, and
  core.39 asks every entry checked against the pin.
