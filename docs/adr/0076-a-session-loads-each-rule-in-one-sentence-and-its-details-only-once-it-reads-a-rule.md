---
number: 76
title: A session loads each rule in one sentence, and its details only once it reads a rule
status: proposed
date: 2026-11-16
decision-makers:
- Chuck Swanberg
- the chore flumes-sessions-load-under-the-bound's code stage
---

# A session loads each rule in one sentence, and its details only once it reads a rule

Status: proposed. The bound and the split are the brief's; the choices below that the brief does not name are the code stage's own, made on the rules written and cheap to change.

Deciders: the user, by the brief of the chore `flumes-sessions-load-under-the-bound`, on the bolt `flumes-sessions-load-less`: "Each agent session in flume loads less than 100,000 characters of rules. The long explanation under each core rule moves to a separate page that loads only when rules are read or changed." The chore's code stage, on the rest.

Sources: the brief; crew's `.claude/rules/`, whose links load its area pages for the paths each names; boomdrive-cloud's bolt `boomdrive-clouds-sessions-load-less`, launched beside this one with the same bound; rules core.21, core.61.

## Context and problem statement

Every session in this repository loads `CLAUDE.md`, which imports `docs/architecture/core.md`. On 2026-11-16, at main `d104a2f8`, the two held 137,332 characters: `CLAUDE.md` 6,815 and `core.md` 130,517, of which 100,004 were the section `Details`, each rule's long account of what it rules out, what it leaves open and its sources. A session that never reads or changes a rule, a code stage on a profile or a verify stage running the gate, carries all of it.

## Considered options

Where the details go:

1. A page of their own beside `core.md`, `docs/architecture/core-details.md`, its frontmatter's `paths` naming `docs/architecture/**`, loaded by a link to it under `.claude/rules/`.
2. A page of their own that loads for no path, a session reading it by hand when it needs a rule's details.
3. Each rule's details beside the area that rule binds, on area pages loaded by the paths they bind.

What loads it:

1. A file under `docs/architecture/` read, `core.md` and the details page among them.
2. A file under `docs/architecture/` or `docs/adr/` read.

Whether a check holds the bound:

1. No check: the bound is kept by its writers.
2. A task that counts the characters of the pages a session loads.

## Decision outcome

The details go to option 1. Claude Code loads a page under `.claude/rules/` whose frontmatter names `paths` once a session reads a file those paths match, and crew loads its area pages so, by links into `docs/architecture/`; the page stays where the rules are, and the link is what loads it. Option 2 loads it for no one, so a session changing a rule could amend the sentence without its details. Option 3 is a regrouping of flume's rules by area, which the brief does not ask and most rules, which bind across the tree, would not fit.

It loads on option 1. The brief says when rules are read or changed, and a rule is read or changed in `docs/architecture/`; a change to a rule edits `core.md`, which a session reads before it edits, so the details load before the edit. A record names its rule by id and changes it on `core.md` in the same commit. Option 2 loads the details for every record a session reads, a record's own context and options needing none.

The bound holds by option 1. A task that counts a page's characters reads a Markdown file for its form, which core.21 and boomdrivedev/blueprints principle 16 forbid.

### Consequences

- `core.md` holds the preamble and each rule's sentence, 31,085 characters; `core-details.md` holds every rule's details; `.claude/rules/core-details.md` links to it. A session's rules at its start are `CLAUDE.md` and `core.md`, 38,217 characters, beside the user's own instructions, under 4,000 on this host.
- core.61 states the bound and the split; `CLAUDE.md` says where a rule's details go and that they are amended in the same commit as the rule.
- The site's rules page points at the details page for each rule's details.
- Nothing holds the bound but its writers: a rule's sentence grown long, or a page added to every session's start, is caught by a reviewer reading core.61.
