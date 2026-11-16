---
number: 71
title: flume's tasks read no Markdown, and its writers keep what a page says
status: proposed
date: 2026-11-15
decision-makers:
- Chuck Swanberg
- the chore flumes-tasks-read-no-markdown's code stage
---

# flume's tasks read no Markdown, and its writers keep what a page says

Status: proposed. The decision is the user's, boomdrivedev/blueprints principle 16 at `b2e8c85`, written 2026-11-11 from their words that day; the chore's brief names what goes and what stays. The choices below that neither names are the code stage's own, made on the rules written and cheap to change.

Deciders: the user. Their words, on sluice's `root:installs`: "I don't want fucking checks on instructions. That has got to fucking stop. That is my biggest pet peeve. We do not fucking write code tests against Markdown files." The brief of the chore `flumes-tasks-read-no-markdown`: "flume's tasks read no Markdown file of its own: root:adrs and site:prose go. OpenSpec's validation, the site's build, and tests that write a README.md or a change folder into a scratch repository as data stay." The chore's code stage, on the rest.

Sources: boomdrivedev/blueprints `design/principles.md` 16 at `b2e8c85`; sluice's core.21 as its `4295ca8` amends it, and its record 0112, which took `root:installs` away; the OpenSpec change `flumes-tasks-read-no-markdown`; rule core.21.

## Context and problem statement

Principle 16: no check, test or gate task in any kit reads a Markdown file, a doc, a README, a skill, a site page, an instruction, a spec or a record; a rule about what a page must say is kept by its writers and enforced by no task. flume's gate read four:

1. `root:adrs`, `check-adrs.sh`, read every record's frontmatter and name for its number.
2. `site:prose` ran Vale over the site's pages.
3. The Pages workflow ran Vale over the pages and the README before it built the site.
4. The herdr plugin's `manifest`, `test` and `install` tasks read the plugin's README's TOML blocks: the manifest's check failed where no block was the popup's entry `lib/common.sh` gives, and the setup and install tests took their configuration from the README.

The brief names the first two. The last two read Markdown just the same, and a reading the rules decide.

## Considered options

For Vale:

1. Keep it in the shell, `prose`, for a writer to run on their own pages, and in no task or workflow.
2. Take Vale, `.vale.ini`, `styles/` and `prose` out of the tree.

For the Pages workflow's prose job:

1. It goes, and the site is built and published with nothing reading its words.
2. It stays, as a workflow and no moon task.

For the herdr plugin's tests:

1. The tests write the popup's entry as `lib/common.sh` gives it and the sidebar's rows into their scratch configuration, from a script of the tests', `tests/config.sh`; the README gives a person the same, kept by its writers.
2. The tests keep reading the README.

## Decision outcome

Vale is option 1. A writer running a tool on their own page keeps what it says, which principle 16 asks; a task that fails a candidate on it enforces, which it forbids. Option 2 takes a tool from the writers that cost the gate nothing, and is cheap to take later.

The prose job is option 1. A workflow that holds the site back for a page's words is a check reading Markdown, whatever runs it.

The herdr plugin's tests are option 1. The entry the tests prove is the code's, `lib/common.sh`'s, the one the startup's words give; the README is a page.

What stays, as the brief says and by the same reading: OpenSpec's validation of the specs and the open changes, the change tool's own reading of its format; the site's build; Claude Code's `plugin validate` of the plugin, its command among it, Claude Code's own reading of its format; the kinds' prompts and the plugin's files the binary carries and sends as its own; and tests that write a `README.md`, a change folder or another Markdown file into a scratch repository as data.

### Consequences

- `root:adrs`, `check-adrs.sh`, `site:prose` and the Pages workflow's prose job are gone. A record's number is kept by its writer, who takes the next free one; nothing fails two records sharing a number.
- The herdr plugin's `tests/config.sh` writes the configuration its setup and install tests read; its manifest's check reads `herdr-plugin.toml` alone.
- core.21 sets Markdown aside, citing principle 16 and this record.
- The specs `the-gate` and `herdr-plugin` change through the OpenSpec change `flumes-tasks-read-no-markdown`.
