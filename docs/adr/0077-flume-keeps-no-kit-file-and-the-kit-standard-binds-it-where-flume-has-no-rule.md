---
number: 77
title: flume keeps no kit file, and the kit standard binds it where flume has no rule
status: proposed
date: 2026-11-17
decision-makers:
- Chuck Swanberg
- the chore a-leftover-kit-file-is-removed's code stage
---

# flume keeps no kit file, and the kit standard binds it where flume has no rule

Status: proposed. The removal is the brief's; the choices below that the brief does not name are the code stage's own, made on the rules written and cheap to change.

Deciders: the user, by the brief of the chore `a-leftover-kit-file-is-removed`, on the bolt `flume-builds-changes-through-openspec`: "The leftover .config/kit.toml file is gone, along with the lines in CLAUDE.md and the README that mention it. No tool reads it, and it still names the old agentplot organization." The chore's code stage, on the rest.

Sources: the brief; afterthought/kit-standard at main, read on 2026-11-17, its rules kit.1 and kit.4 and its `docs/orgs/`, which holds `afterthought.md`, `agentplot.md` and `willdan.md`; record 0066, which kept the file's org as a name; boomdrivedev's sluice, crew, boom and boomdrive, which keep no `.config/kit.toml`, and timber and weir, which keep one naming agentplot.

## Context and problem statement

`.config/kit.toml` named the kit standard's profile for flume, `code`, its org, `agentplot`, and the version of the standard it was last reconciled to, `unreleased`. kit:kit.1 asks a repository to keep that file. No tool reads it: the kit standard carries no reconcile tool, and nothing in this tree or the family's reads it. flume moved to boomdrivedev, and the kit standard has no page for that org; record 0066 kept `agentplot` in the file as a name and no address.

`CLAUDE.md`, the README and `openspec/config.yaml` each pointed at the file for the profile that binds.

## Considered options

1. Remove the file and its mentions, the kit standard still binding where flume has no rule, as `openspec/config.yaml` says, and this record its departure from kit:kit.1 (kit:kit.4).
2. Keep the file and name another org in it.
3. Remove the file and the kit standard's binding with it.

## Decision outcome

Option 1, as the brief asks. Option 2 names an org the kit standard has no page for, or one flume no longer belongs to, in a file nothing reads. Option 3 changes which rules bind flume where it has none of its own, which the brief does not ask and the designs and records written under the kit standard cite.

### Consequences

- `.config/kit.toml` is gone, with the README's row for it and the sentence of `CLAUDE.md` that named it.
- `openspec/config.yaml` keeps the kit standard's code profile binding where flume has no rule, cited as `kit:<id>`, and names this record where it named the file.
- flume departs from kit:kit.1: the profile is `code`, named in `openspec/config.yaml`; the org is boomdrivedev's, which the kit standard has no page for; and flume names no version of the standard it was reconciled to.
