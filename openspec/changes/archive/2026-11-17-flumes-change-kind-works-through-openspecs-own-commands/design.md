# Design

## Context

See `proposal.md` for why. flume's own kinds are data the kinds profile
carries: each version a directory under
`crates/flume/src/profiles/kinds/weir/own/`, its `kind.toml` and its
prompts, read through `weir-kinds` like any other package and listed in
`OWN` in `crates/flume/src/profiles/kinds/weir/mod.rs`, oldest first so
the last is the newest (22, 154, 157; records 0022, 0054). A version
keeps its content once released, and `built_in.rs` records each
version's hash (156).

This base, `bolt/flume-builds-changes-through-openspec` cut from main at
514f1030, holds change@1, change@2 and change@3. The statement's change@4,
record 0064's (each stage's agent in `full`, each prompt saying its one
way to ask), is built on `bolt/flumes-settings-read-as-its-records-say`
and not on main. Main already reads a kind's `mode` for its agents
(core.30; record 0044), and the stand-in for Claude Code takes
`--permission-mode`.

A prompt names the change by `{{change}}`, its path, which the change
profile gives as `openspec/changes/<name>`, `<name>` the place's name,
`{{name}}` (127, C.3). The pinned OpenSpec is 1.14.3. Its skills under
`.claude/skills/openspec-*/` run its command line: `openspec new
change`, `openspec status --change <name> --json`, `openspec
instructions <part> --change <name> --json`, `openspec instructions
apply --change <name> --json` and `openspec validate`. Its archive skill
alone does not run `openspec archive`: it syncs the specs by the agent's
own hand and moves the change's folder. `openspec validate <name>
--strict` fails an added requirement whose text passes 500 characters
and leaves a modified one's to the archive, which `openspec archive`
refuses where the specs it makes fail (core.59).

The rules this design relies on: core.1, core.2, core.4, core.19,
core.21, core.30, core.31, core.33 and core.59. No page of
`docs/architecture/` but `core.md` binds the paths it touches.

## Goals / Non-Goals

**Goals:**

- flume's own change@5, whose stages work the change through OpenSpec's
  own command line and whose construct commits the change only once
  `openspec validate --strict` passes.
- change@5 the newest, and change@1 to change@3 unchanged byte for byte.

**Non-Goals:**

- flume's reading of a change written stays C.3's: planning complete as
  `openspec status` gives it (record 0075), with no strict check of its
  own.
- No stage layer of standing instructions (148; record 0021), no
  version of fix, chore or plan, and nothing of change@4's own files.
- No new clause or scenario of the statement, so the conformance suite's
  clause list keeps its revision.

## Decisions

### 1. A new version, change@5, whole on its own

A version's content never changes once released (156), and change@4 is
record 0064's (22, C.2), so the next version is 5. A package refers to
nothing outside itself (157), so change@5 is written whole: change@4's
content, its stages' agents given `mode = "full"` in the kind's layer
and each prompt's sentence on its one way to ask, word for word as
record 0064 gives it, with OpenSpec's commands in each stage. On this
base, `OWN` lists change@5 after change@3. Where the bolts meet, change@4
goes between them, and neither version's content changes.

Not taken: change@4 here (record 0064's name, on another bolt), or
change@3 amended (156).

### 2. OpenSpec's command line, never an agent program's skills

Each prompt names the `openspec` commands its stage runs, run as the
repository's instructions run its tools. The commands give the agent
what the skills give it, the part's instruction and template, the
repository's `openspec/config.yaml` context and rules, and the apply
guidance, since the skills run the same commands.

Not taken: `/opsx:ff`, `/opsx:apply`, `/opsx:verify` and
`/opsx:archive`. Each skill tells its agent to ask the person in its
pane, which change@4's one way to ask rules out (28; record 0064). A
skill is installed for one program in one repository, and flume's own
kinds name no program (158). The archive skill never runs `openspec
archive`, whose application and check of the specs is what refuses an
overlong requirement.

### 3. The prompts name OpenSpec and the change by its name

A kind that reads a change runs only where the repository binds a change
profile (4), and OpenSpec is the one change profile (C.3), so change@5
names `openspec` and its commands with `{{name}}`, and names no other
tool, no runner (sluice, moon, cargo or devenv) and no `/opsx` command,
as change@3's verify names none. The prompts are the kinds profile's
data, and no code outside a profile names OpenSpec (core.1, core.2). The
conformance suite's fake harness, outside a profile, says "its change
tool's own commands" and names none.

### 4. The strict check is the construct's prompt's

The construct runs `openspec validate {{name}} --strict`, fixes the
change until it passes, and only then commits. Its deliverable says the
change passes it. The code stage holds any other write to the change, a
task it adds among them, to the same check before committing it.

Not taken: flume's reading of a change written requiring the strict
check. That would change C.3's read-change for every version's places in
flight (21), run another `openspec` process at every read of a standing
change (record 0075), and leave an agent quiet on an invalid change with
its stage undelivered and no words of why.

### 5. The archive is OpenSpec's, and undone whole when sent back

The verify reads every task done through `openspec instructions apply
--change {{name}} --json` and the change passing `openspec validate
{{name}} --strict`, and archives by `openspec archive {{name}} --yes`,
which applies the change's specs and checks the specs they make. It
commits the archive with the specs it wrote. Where the archive refuses,
it leaves the branch as its report's commit left it and stops short
having found work in the archive's words, which sends the change back to
code (131). A code stage started with words where the change has been
archived undoes the archive in one commit, the change moved back to its
directory and each spec the archive wrote returned to what it held
before, so a later archive applies the change's specs once.

### 6. change@5's declaration and prompts

`crates/flume/src/profiles/kinds/weir/own/change@5/kind.toml`:

```toml
# flume's own change@5, one version of one kind, built into the binary and read
# through weir-kinds like any other declaration (22, 157, C.2; records 0054,
# 0064, 0077). It is change@4's stages, each stage's agent given `full` in the
# kind's layer, the last of 32's, and each prompt saying that stopping short is
# its agent's one way to ask (28, 162; record 0064), each stage working the
# place's change through OpenSpec's own commands (C.3; record 0077). Each
# prompt is a path beside this file, in this version's own directory. A kind a
# repository or a launch declares by the same name stands over flume's own (19).

[kinds.change]
version = 5
branch = "change"
starts_unready = false
lands = true

[[kinds.change.stages]]
name = "construct"
prompt = "prompts/change-construct.md"
deliverables = ["change"]
agents = [{ mode = "full" }]

[[kinds.change.stages]]
name = "code"
prompt = "prompts/change-code.md"
deliverables = ["tasks"]
agents = [{ mode = "full" }]
# One agent for the whole change, each task checked by its own check
# alone, and no repeat (22, S119; record 0054).

[[kinds.change.stages]]
name = "verify"
prompt = "prompts/change-verify.md"
deliverables = ["archived"]
agents = [{ mode = "full" }]
on_fail = "code"
```

`prompts/change-construct.md`:

```markdown
You are the construct stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.

{{brief}}

Write the change at {{change}} from the brief through OpenSpec's own commands, run as the repository's instructions run its tools: its proposal, its specs, its design and its tasks, each task a checkbox line. Write no code of the change: a later stage builds it, one agent working through every task in order.

Where the change does not yet stand, make it with `openspec new change {{name}}`. Read `openspec status --change {{name}} --json` for the parts the change needs and the order they come in, and write each part from `openspec instructions <part> --change {{name}} --json`: its instruction and template give the part's shape and what it holds, and its context and rules bind what you write without being copied into it. Read the status again after each part, until it reads the change's planning complete.

Give each task the narrowest check that shows that task done, named so it runs as written: one test by name, one package's tests, or a lint of the files the task touches. No task runs the repository's whole checks, and no task checks a task group or every part the change touches: the task groups are headings for reading, and the verify stage runs the repository's whole checks once, after every task is done.

Then run OpenSpec's strict check, `openspec validate {{name}} --strict`, and fix the change until it passes. Commit the change on {{branch}} only once it passes. Your deliverable is the change written: the stage is done once a commit on {{branch}} holds the change with its planning complete and passing `openspec validate {{name}} --strict`, and you are quiet.

If you cannot go on without an answer, stop short: {{needs}}

When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.
{{#words}}
You are started again with these words:

{{words}}
{{/words}}
```

`prompts/change-code.md`:

```markdown
You are the code stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}. You build the whole change, every task of it.

Read the change at {{change}}, its proposal, design, specs and tasks. Work through its tasks as OpenSpec's own apply instructions give them, run as the repository's instructions run its tools: read `openspec instructions apply --change {{name}} --json` for the tasks still to do, each with the file and line of its checkbox, and for the context and guidance it gives, which bind how you work. Take the tasks in order, the task groups only headings: make each task true, run the check it names as it names it, and once its check passes tick the task at the line the instructions give; read the instructions again before the next task. Commit as you go on {{branch}}. Run no check but a task's own: not the repository's whole checks, not a task group's and not every part the change touches. The verify stage runs the repository's whole checks once, after you. Leave every heading of the tasks file as it is.

Where you write anything of the change but a task's tick, a task you add among it, run `openspec validate {{name}} --strict` and commit what you wrote only once it passes.

Your deliverable is the tasks ticked: the stage is done once a commit on {{branch}} holds every task of the change ticked and you are quiet. Leave the branch where your last commit puts it; what happens to it after this stage is not yours to do.

If you cannot go on without an answer, stop short: {{needs}}

When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.
{{#words}}
This stage was started again with these words:

{{words}}

Where the words name files contested and the base's revision the merge queue tested against, take that revision in before anything else: run `git merge <the revision the words name>` in this place, which merges it into {{branch}}, resolve each of the files contested, and commit the merge. Never rebase, and never rewrite a commit already on {{branch}}.

Where the change has been archived on {{branch}}, undo its archive first, in one commit: move the change back out of the archive to {{change}}, and return every spec the archive wrote to what it held before the archive. Then fix what the words name, and record the fix as a task of the change naming its own check, ticked once that check passes.
{{/words}}
```

`prompts/change-verify.md`:

```markdown
You are the verify stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.

First run the repository's whole checks once, on {{branch}} as it stands, as the repository's instructions name them. Fix nothing yourself and run them no second time: what fails is the code stage's to fix.

Then check the work on {{branch}} against the change at {{change}} through OpenSpec's own commands, run as the repository's instructions run its tools: every task done as `openspec instructions apply --change {{name}} --json` reads them, the change passing `openspec validate {{name}} --strict`, and every requirement and scenario of its specs met by the work. Write what you find as a report, `verify.md` in the change's directory, {{change}}/verify.md, the whole checks' outcome and OpenSpec's among it, and commit it on {{branch}}.

Where the whole checks pass and the work holds, archive the change with OpenSpec's own archive, `openspec archive {{name}} --yes`, which applies the change's specs and checks the specs they make, and never by moving the change or writing its specs yourself; commit the archive on {{branch}}, the report and every spec it wrote with it. Your deliverable is the change archived: the stage is done once a commit on {{branch}} holds the change archived and you are quiet.

Where the whole checks fail, the work falls short or the archive refuses, archive nothing, leaving {{branch}} as your report's commit left it: stop short having found work, naming what fails in its own words, so the change goes back to its code stage with your words. {{needs}}

When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.
{{#words}}
You are started again with these words:

{{words}}
{{/words}}
```

Each prompt keeps change@3's order: what differs between starts, the
words, comes last (core.31, core.33). Each prompt file ends with one
newline, as change@3's do.

### 7. The fakes

`flumes_own()` in `crates/flume-conformance/src/fake_harness.rs` gains
change@5 after change@3: change@3's stages at version 5, no mode given,
as the fake's other versions give none, with prompts of its own that name
no tool (core.1):

- construct: `Write the change {{change}} through its change tool's own commands, committing it once its strict check passes.\n\n{{brief}}\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}`
- code: `Build {{change}} as its change tool's apply instructions give it, every task of it, running each task's own check and ticking it once the check passes.\n\n{{needs}}{{#words}}\n\n{{words}}{{/words}}`
- verify: `Verify {{change}} against the work, commit the report, and archive it through its change tool's own archive.\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}`

The engine's scenarios read the version a launch resolves and the code
stage's starts, not a stage's mode, which the kinds profile's own test
reads of the built-in declaration.

### 8. The engine's and the walkthroughs' scenarios

`engine/launching.rs` gives the launching spec's scenarios: a bare
`change` runs change@5, `change@3`, `change@2` and `change@1` named run
their versions, and `change@6` is refused naming versions 1, 2, 3 and 5.
`engine/running_through.rs` keeps its change@3 scenario under a
repository lock recording `flume/change@3`, still citing S119, and adds
one for a bare `change` running change@5, citing 22 and 154.
`engine/list.rs` reads `change@5` for a bare change's place. The
walkthroughs that launch a bare `change` run change@5, its agents
started with `--permission-mode bypassPermissions` from `kind`, which
the stand-in takes.

### 9. The site

`site/src/content/docs/guides/kinds.mdx` lists `flume/change@5` among
flume's own kinds, says a bare `change` takes the newest, `change@5`,
unless a lock records another, and says in a sentence that change@5
codes as change@3 does, gives its agents the mode `full`, works the
change through OpenSpec's own commands, and hands its change to code
only once OpenSpec's strict check passes, to any host's operator
(core.35). The agent settings guide already says to accept Claude Code's
warning once for each account before its agents run `full`.

### 10. The statement and the record

The statement's 22 and C.2 carry change@5 at boomdrivedev/blueprints
2e46f6e, on the branch
`design/flume-change-kind-works-through-openspecs-own-commands`, change@5
the newest in place of change@4 (core.19). Record 0077, proposed, holds
the decisions above with the paths not taken. No rule of
`docs/architecture/core.md` changes: none binds what a kind's prompts
say, and the kinds profile's spec holds change@5's by their scenarios.

## Risks / Trade-offs

- [change@4 is not on this base] → change@5 carries change@4's content
  whole, so neither depends on the other's files. Where the bolts meet,
  `OWN`, `OWN_HASHES`, the fake harness, the scenarios naming flume's
  versions, the kinds guide and the specs merge to hold change@1 to
  change@5, and a version flume does not hold is refused naming all
  five. Neither version's content changes, so neither hash does.
- [A modified requirement past OpenSpec's limit passes the strict
  check] → `openspec archive` at verify refuses it and sends the change
  back to code, and in this repository `root:changes` fails it first
  (core.59).
- [OpenSpec's commands move with a release] → the prompts name commands
  and the facts the agent reads, not fields of their JSON; a release
  that renames a command is a new version of change (156).
- [An agent without `openspec` where the repository runs its tools] →
  the agent stops short naming it, as for any tool the repository's
  instructions name; flume's own reading of a change needs `openspec`
  already (record 0075).

## Migration Plan

None: a place keeps the version its launch read (21), and a repository
that locks change@3 keeps launching it (155).
