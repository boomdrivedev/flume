---
number: 77
title: flume's own change works each change through OpenSpec's own commands, and its construct hands over only once OpenSpec's strict check passes
status: proposed
date: 2026-11-17
decision-makers:
- the brief of flumes-change-kind-works-through-openspecs-own-commands
- the construction agent of flumes-change-kind-works-through-openspecs-own-commands
---

# flume's own change works each change through OpenSpec's own commands, and its construct hands over only once OpenSpec's strict check passes

Status: proposed. The direction is the change's brief: "Each stage of flume's change kind uses OpenSpec's own commands: it writes the change through OpenSpec's instructions, builds it task by task, then verifies and archives it the same way. The first stage hands over only once OpenSpec's strict check passes on the change it wrote, so a broken spec is caught right there, not at archive." The brief was drawn from the user's question of 2026-11-16, whether specs break procedure because OpenSpec's skills are not used. The statement's 22 and C.2, amended on boomdrivedev/blueprints' branch `design/flume-change-kind-works-through-openspecs-own-commands` at 2e46f6e, state it. What this record decides beyond the brief is the construction agent's, made on the rules written and cheap to change, and stays proposed until the operator's yes.

Deciders: the change's brief, which decided the direction; the construction agent of `flumes-change-kind-works-through-openspecs-own-commands`, for the choices of its design's sections 1 to 5.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 2e46f6e, clauses 21, 22, 28, 154 to 158 and 162, and C.2 and C.3; records 0054, 0064 and 0075; rules core.1, core.4, core.19, core.30, core.33 and core.59; `crates/flume/src/profiles/kinds/weir/own/change@3/prompts/` on flume's main at 514f1030, none of whose prompts runs `openspec instructions`, its apply instructions, `openspec validate --strict` or `openspec archive`; change@4 of record 0064 on `bolt/flumes-settings-read-as-its-records-say`, not on main; OpenSpec 1.14.3 as the tree pins it, its skills under `.claude/skills/openspec-*/` and its validator (`dist/core/validation/validator.js`, an added requirement's text over 500 characters a warning that `--strict` fails, a modified one's left to the archive); the change's design, `openspec/changes/flumes-change-kind-works-through-openspecs-own-commands/design.md`.

## Context and problem statement

flume's own change@3 tells its agents to write a change, build its tasks, check the work and archive it, and names none of OpenSpec's commands for any of it. An agent writes the change's files by hand, ticks tasks by hand and archives by hand or by whichever command it reaches for. OpenSpec's instructions for each part, its project context and rules among them, its apply instructions and its strict check go unread. So a requirement OpenSpec's strict check refuses passes the construct and the code stage, and is first refused at the archive or at the gate's check of open changes (core.59). Three changes hit OpenSpec's 500-character limit on a requirement's text there the night the user asked.

## Considered options

1. **change@5: each stage works the change through OpenSpec's own command-line commands (chosen).** The construct makes the change with `openspec new change`, writes each part from `openspec instructions <part> --change <name> --json` until `openspec status` reads its planning complete, and commits it only once `openspec validate <name> --strict` passes. The code stage takes the tasks `openspec instructions apply --change <name> --json` gives and ticks each at the line it names. The verify stage reads the tasks done and the strict check passing, and archives by `openspec archive <name> --yes`.
2. The stages run OpenSpec's skills or slash commands, `/opsx:ff`, `/opsx:apply`, `/opsx:verify` and `/opsx:archive`. Each skill tells its agent to ask the person in its pane: to clarify a change, to pause on an unclear task, to confirm an archive with warnings. change@4's one way to ask, record 0064's, rules that out. A skill is installed for one agent program in one repository by `openspec init --tools`, and flume's own kinds name no program (158). The archive skill syncs the specs by the agent's own hand and moves the change's folder, never running `openspec archive`, whose application and check of the specs is what refuses an overlong requirement.
3. flume's reading of a change written requires `openspec validate --strict` passing. The stage could then not deliver an invalid change whatever its prompt said. It changes C.3's read-change for every version's places, those in flight among them (21), runs another `openspec` process at every read of a standing change (record 0075), and leaves an agent quiet on an invalid change with its stage undelivered and no words of why.
4. change@3 or change@4 amended in place. A version keeps its content once released (156), and change@4 is record 0064's, on another bolt.

## Decision outcome

Option 1.

- **A new version, change@5.** flume's own `change` gains version 5 (156, 157). change@4 is record 0064's, held on `bolt/flumes-settings-read-as-its-records-say` and not on this base, so change@5 is a package whole on its own, carrying change@4's content with it: each stage's agent given `mode = "full"` in the kind's layer, and each prompt saying that stopping short is its agent's one way to ask (28, 162; record 0064). change@5 is the newest, so a launch naming no version in a repository whose lock records none runs it (154, 155). change@1 to change@3 stay side by side, and change@4 joins them where the bolts meet; a place launched at any keeps it (21).
- **OpenSpec's command line, run as the repository runs its tools.** Each prompt names `openspec` and its commands, run as the repository's instructions run its tools, and names no other tool, no runner of the repository's and no agent program's skill or command for OpenSpec. A kind that reads a change runs only in a repository that binds a change profile (4), and OpenSpec is the one change profile (C.3). The prompts are the kinds profile's data, and no code outside a profile names OpenSpec (core.1).
- **The construct's strict check is its own.** Its prompt has its agent run `openspec validate <name> --strict` and commit the change only once it passes, the stage's deliverable the change passing it. flume's reading of a change written stays C.3's (record 0075).
- **The code stage's other writes to the change are held to it too.** A task the code stage adds, and any other write to the change but a tick, is committed only once `openspec validate <name> --strict` passes.
- **The verify archives only through OpenSpec's archive.** `openspec archive <name> --yes` applies the change's specs and checks the specs they make. A refusal archives nothing and sends the change back to code with the archive's words, through the stop-short a finding already makes (131). Sent back after an archive, the code stage undoes it whole in one commit, the change moved back to its directory and every spec the archive wrote returned to what it held before.

The construction agent's choices, each with the path not taken:

- **The command line over the skills** (design section 2): option 2 not taken, for the reasons it gives.
- **The strict check in the prompt** (design section 4): option 3 not taken, left to a later change should places be seen delivering changes OpenSpec refuses.
- **No new scenario of the statement**: change@5's scenarios cite 22 and 154, and the conformance suite's clause list keeps its revision, since the amendment adds no clause or scenario number. Not taken: a scenario of Part B for change@5, which would move the suite's list across every clause added since its revision.
- **No rule**: no rule of `docs/architecture/core.md` binds what a kind's prompts say, and the kinds profile's spec holds change@5's prompts by their scenarios, as it holds change@3's and change@4's (records 0054, 0064).

### Consequences

- Good: a change's parts follow OpenSpec's own instructions, the repository's `openspec/config.yaml` context, rules and apply guidance among them, and an added requirement OpenSpec's strict check refuses is caught by the construct that wrote it.
- Good: the archive is OpenSpec's own, so the specs it writes are the ones `openspec archive` makes and checks, and the gate's check of open changes (core.59) and the verify agree.
- Bad: a modified requirement grown past OpenSpec's limit still passes `openspec validate --strict`, which leaves a modified requirement's text to the archive. It is caught at verify's `openspec archive`, and in this repository by `root:changes` (core.59), not at the construct.
- Bad: every agent of change@5 needs `openspec` as the repository's instructions run it, as flume's own reading of a change already does (record 0075).
- Bad: change@5 sits beside change@3 with no change@4 on this base. Where the bolts meet, flume's own list, the recorded hashes, the kinds guide and the specs naming flume's versions merge to hold all five, change@4 between change@3 and change@5, and neither version's content changes.
- The statement's 22 and C.2 say it, at boomdrivedev/blueprints 2e46f6e.
