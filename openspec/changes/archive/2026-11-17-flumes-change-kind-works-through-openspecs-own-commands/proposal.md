# flume's change kind works through OpenSpec's own commands

## Why

flume's own change@3 tells its agents to write, build, check and
archive a change, and names none of OpenSpec's commands for any of it:
no prompt runs OpenSpec's instructions, its apply, its strict check or
its archive. So a requirement OpenSpec refuses passes the construct and
the code stage and is first refused at the archive, as three changes
were on OpenSpec's 500-character limit the night the user asked whether
specs break procedure because OpenSpec's skills are not used
(2026-11-16). The brief answers it: each stage works the change through
OpenSpec's own commands, and the construct hands over only once
OpenSpec's strict check passes (record 0077, proposed).

## What Changes

- **flume's own change@5 (22, 156, 157, C.2).** A new version of
  flume's own `change`, built in beside change@1, change@2 and change@3,
  a package of its own whose content never changes. It carries change@4's
  content, record 0064's, which this base does not yet hold: each stage's
  agent given `mode = "full"`, and each prompt saying that stopping short
  is its one way to ask (28, 162). Its stages work the change through
  OpenSpec's own commands (C.3):
  - its **construct** makes the change with `openspec new change`,
    writes each part from `openspec instructions <part> --change <name>
    --json` until `openspec status` reads its planning complete, gives
    each task its narrowest check as change@3's does, and commits the
    change only once `openspec validate <name> --strict` passes;
  - its **code** stage, one agent, takes the tasks `openspec
    instructions apply --change <name> --json` gives, runs each task's
    own check and ticks it at the line those instructions name; any
    other write to the change is held to the strict check; sent back
    after an archive, it undoes the archive whole, the specs it wrote
    among it;
  - its **verify** runs the repository's whole checks once, reads every
    task done and the strict check passing, checks the work against the
    specs, commits `verify.md`, and archives by `openspec archive <name>
    --yes`, which applies the change's specs and checks the specs they
    make; where the archive refuses, it archives nothing and stops short
    having found work, in the archive's words (131).
- **OpenSpec's command line, never an agent program's skills.** Each
  prompt names `openspec` run as the repository's instructions run its
  tools, and no other tool and no skill or slash command for OpenSpec,
  whose skills ask the person in the pane and whose archive skill never
  runs `openspec archive` (design 2).
- **The strict check is the construct's own.** flume's reading of a
  change written is unchanged (C.3; record 0075).
- **The newest is the default (154, 155).** A launch naming no version,
  in a repository whose lock records none, runs change@5. change@1 to
  change@3 stay side by side, and a place launched at one keeps it (21).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `kinds-profile`: flume's own kinds gain change@5, its stages giving
  their agents `full`, and its three prompts working through OpenSpec's
  own commands and saying their one way to ask; `flume/change@5` among
  the packages; its code prompt among those that merge a conflict's
  base; a place launched at change@3 keeps it.
- `launching`: a launch naming no version runs change@5, one naming
  change@3 runs it, and a version flume does not hold is refused naming
  versions 1, 2, 3 and 5.
- `kind-sources`: the hashes recorded gain change@5's, the others
  unchanged.
- `running-through`: a bare change runs change@5 through with one agent
  coding its change; change@3's scenario launches it by name.

## Impact

- `crates/flume/src/profiles/kinds/weir/own/change@5/`: a `kind.toml`
  and its three prompts; `crates/flume/src/profiles/kinds/weir/mod.rs`
  lists it in `OWN`.
- `crates/flume/tests/profiles/kinds_weir.rs`, `built_in.rs` and
  `tools/walkthroughs.rs`: change@5 among flume's own, its hash, its
  prompts, and the walkthroughs of a bare `change`.
- `crates/flume-conformance/`: the fake harness's own kinds gain
  change@5; the kinds seam's scenario and the engine's launching, list
  and running-through scenarios name it.
- `site/src/content/docs/guides/kinds.mdx`: flume's own kinds and how
  change@5 works.
- `docs/adr/0077-…`, written with this change's plan; the statement's
  22 and C.2 at boomdrivedev/blueprints 2e46f6e, on the branch
  `design/flume-change-kind-works-through-openspecs-own-commands`.
- No new crate, plugin or site part, so no new moon project; no seam,
  core, weir or change-profile change; no rule of
  `docs/architecture/core.md` changes. The conformance suite's clause
  list keeps its revision, since the statement adds no clause or
  scenario number.
