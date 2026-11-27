# flume reports a repository's binding

## Why

On 2026-11-26, a caller offered a launch of flume/fix@1 in a repository
that bound no merge queue. flume refused the launch two times. The first
refusal said that `.config/sluice.toml` was missing. The second refusal
said that `.config/flume.toml` named no `merge_queue` in `[profiles]`. A
caller must know this before it offers a launch. Only flume knows it:
flume reads `[profiles]` only in a launch, and only flume knows which
kinds land. A caller must not read `.config/flume.toml` itself, because
each tool owns its facts.

## What Changes

- **Status reads a repository's binding.** A status ask can name
  repositories by their folders. For each one, the answer gives the
  binding at the default branch. It gives the merge queue that the
  binding names, and whether that queue lands the default branch. It
  gives each kind that flume can run there, with whether it lands. Where something is
  missing, the answer says which file or which table.
- **The read fetches nothing and changes nothing.** It reads the default
  branch as `origin` was last fetched, else the repository's own branch,
  and says which (core.15, and Boom invariant
  `status-reads-each-fact-where-it-lives-and-changes-nothing`).
- **`flume host` serves it in its status and its subscription.** The
  ask's `repositories` names the folders. The runner holds one
  subscription, and each document of it holds the report (Boom invariant
  `one-subscription-per-plugin`).
- **`flume status --binding`** asks for the repository that the command
  runs in, or the one that `--repository` names. The MCP tool `status`
  takes `binding`.
- **The kinds seam gains read-binding-held**, and the merge-queue seam
  gains lands-on. The sluice profile answers lands-on from
  `.config/sluice.toml` with no fetch and no sluice command.
- **The Claude Code plugin gains the skill `setting-up-flume`.** It
  writes the `[profiles]` table of `.config/flume.toml`, reads back the
  report, and hands the merge queue's own setup to sluice's skill
  `setting-up-sluice`.

## Capabilities

### Modified Capabilities

- `place-status`: status gives each repository that its ask names.
- `command-line`: `flume status --binding`.
- `kinds-profile`: the binding as the repository holds it, with no
  fetch.
- `merge-queue-profile`: whether sluice lands a base, with no fetch.
- `claude-code-plugin`: the skill `setting-up-flume`, and `binding` on
  the MCP tool `status`.

## Impact

- `crates/flume-seams`: `BindingHeld`, `HeldFrom` and
  `Kinds::read_binding_held`, and `LandsOn` and `MergeQueue::lands_on`.
- `crates/flume-fakes`: the fake kinds and the fake merge queue answer
  the new operations.
- `crates/flume-core`: `src/repository.rs`, and `repositories` in
  `Answer::Status`.
- `crates/flume`: the weir and sluice profiles, `src/served.rs`, the
  `status` command, the MCP tool `status` and the printed answer.
- `crates/flume-conformance`: the full status answer lists the new
  fields.
- `plugins/claude`: `skills/setting-up-flume/SKILL.md`, which the binary
  also writes into the plugin that it gives its agents.
- boomdrive's runner names its repositories in its subscription, and its
  protocol version 7 carries the report, in a change of its own.
