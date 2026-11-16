---
number: 73
title: Each open change is validated as it would archive, alone against the tree's specs
status: proposed
date: 2026-11-15
decision-makers:
- Chuck Swanberg, by boomdrivedev/blueprints principle 16 as amended on 2026-11-15
- the construction agent of root-changes-validates-each-open-change-as-archived
---

# Each open change is validated as it would archive, alone against the tree's specs

Status: proposed, until the user's yes on it as a record. Point 1 is the user's rule, boomdrivedev/blueprints principle 16 as amended on 2026-11-15. Points 2 to 5 are boomdrivedev/boomdrive's record 0050, the family's one design for this check, which flume takes as Boomdrive wrote it. Points 6 to 10 are the construction agent's own choices in the change `root-changes-validates-each-open-change-as-archived`, made on the rules written and each cheap to change.

Deciders: the user, whose answer of 2026-11-15 on their card `req_278df7ab2cc34988a3f5` amended principle 16: "OpenSpec's own validation stays; the rule is about checks of what pages say". The principle as amended keeps "OpenSpec's `openspec validate` of a kit's specs and its open changes, each change validated as it would archive". The construction agent of the change `root-changes-validates-each-open-change-as-archived`, 2026-11-15, for points 6 to 10.

Sources: boomdrivedev/blueprints `design/principles.md` 16, at `3a5c19d` on `design/a-tools-own-validation-stays`, until the user pushes it; boomdrivedev/boomdrive's record 0050 and core.61, and its `check-changes.sh` at `d9414da6` on `bolt/boomdrives-gate-checks-changes-as-archived`; OpenSpec 1.14.3, the release `devenv.nix` pins, its `openspec archive` and `openspec validate`, read on 2026-11-15 in a scratch tree; the change `root-changes-validates-each-open-change-as-archived`, its design sections 1 to 9; rules core.19, core.21, core.22 and core.51.

## Context and problem statement

`root:specs` runs `openspec validate --specs --strict`, and `root:changes` ran `openspec validate --changes --strict`. A change can pass that and still fail at its archive, or archive into specs that `root:specs` then fails, found only after the gate has passed its branch:

- `openspec archive` refuses a delta that does not apply, an ADDED requirement its spec already holds or a MODIFIED one its spec does not hold. `openspec validate --changes --strict` reports it only as information, "Archive would refuse this delta", and passes.
- `openspec archive` checks the specs it rebuilds without `--strict`. A new capability whose delta gives no `## Purpose` gets archive's placeholder, and a MODIFIED requirement past OpenSpec's 500 characters is let through; `--strict` fails both once the change is archived.

The tree held such a change. `a-hosts-full-status-read-takes-seconds` landed on main as #117 with every task checked and was never archived: its merge-queue-profile delta's MODIFIED requirement was 609 characters of text, so the spec its archive writes fails `openspec validate --strict`.

## Considered options

Boomdrive's record 0050 weighed these, and flume takes its choices.

For what a change is applied to:

1. **Each change alone, applied to a scratch copy of the tree's specs.**
2. Every open change applied in turn to one copy.
3. Each change applied to the tree's own specs, which the check then puts back.

For which specs are held to `--strict` after the archive:

1. **Each spec the archive writes, a new one or one whose bytes differ from the tree's.**
2. Every spec in the copy, as `openspec validate --specs --strict` reads them.

For where the script lives, how its cases run and what becomes of the leftover change, the construction agent weighed the options in points 6 to 8.

## Decision outcome

1. **`root:changes` holds each open change to OpenSpec's validation as it would archive (the user's).** It still runs `openspec validate --changes --strict`. Then, for each open change, it runs `openspec archive` in a scratch copy and holds the specs that result to `openspec validate --strict`. A change that fails is named with OpenSpec's words. `check-changes.sh` does it, and core.59 says so.
2. **Each change is applied alone to a scratch copy of the tree's specs (Boomdrive's record 0050, its first option).** The copy holds the tree's specs, `openspec/config.yaml` and that change, and is made under `mktemp -d`, in the temporary folder the merge queue gives the place (core.51). Each change archives at its own merge onto its base as it stands then.
   - Not taken: every open change applied in turn. It judges a change by the other open changes and the order they are applied in, and neither is the change's own. Two open changes that conflict are found by the gate on the branch that lands second.
   - Not taken: the tree's own specs, put back after. A check writes nothing into the tree.
3. **The specs the archive writes pass `openspec validate --strict`, though `openspec archive` validates them without it (record 0050).** `--strict` is how `root:specs` reads them once the change is archived.
4. **Only the specs the archive writes are validated (record 0050, its first option).** The script knows which by comparing each with the tree's byte for byte, by `cmp`, which reads no word and no form of either page and is part of OpenSpec's validation as archived, which principle 16 as amended keeps. A spec the change leaves as it was is `root:specs`' to check.
   - Not taken: every spec in the copy. A spec on the base that fails would fail every open change, each named for what it did not write.
5. **A task left unchecked fails nothing (record 0050).** The archive runs with `--yes`, and a change is open while its tasks are worked.
6. **The script is flume's own, Boomdrive's `check-changes.sh` at `d9414da6` copied whole (the construction agent's).** flume's gate runs files its tree holds, at versions it pins (core.22).
   - Not taken: a script one tool owns that flume's gate calls, which flume's tree neither holds nor pins.
   - The cost: two copies can drift. A change to the design is made in Boomdrive's record 0050 first and carried here (CLAUDE.md, "The family").
7. **The cases are a task of their own, `root:changes-cases` (the construction agent's).** `root:changes` checks the tree, and `check-changes-cases.sh` checks the script over five scratch trees, one for each scenario of the-gate's requirement and one for "An open change that does not validate", which no check proved (core.21).
   - Not taken: the cases inside `check-changes.sh`, which would run them over every tree it checks.
8. **The leftover change is cut and archived in this change (the construction agent's).** Its merge-queue-profile delta's MODIFIED "Read, base and the place's data" names the read of every branch at once by an ADDED requirement of its own, "One read for every branch asked at once", so each is within OpenSpec's length and its meaning is kept. The change is then archived, so the specs say what landed at #117 (core.19). The new check fails the tree without it.
   - Not taken: cutting its delta and leaving it open, which keeps the specs behind what landed and leaves its MODIFIED requirements to undo, at a later archive, what other changes wrote of them since.
   - Not taken: editing `openspec/specs` to match, which core.19 forbids.
9. **The-gate's spec gains an ADDED requirement, "Each open change is validated as it would archive" (the construction agent's).** `bolt/flumes-checks-read-no-markdown` removes "The root project checks what spans the tree" and adds it again under a new name, and a MODIFIED of the old name would be refused at whichever archive lands second, by this very check.
   - Not taken: a MODIFIED root requirement.
10. **The record is 0073 and its rule core.59 (the construction agent's).** main holds records 0071, a submission whose landing the forge refused, and 0072, flume's confirming of a merge queue's outcome, and rule core.58, the first's; 0071 is taken on `bolt/flumes-checks-read-no-markdown` too, and 0072 on `bolt/flumes-places-need-no-hand`. The change `the-gates-archive-check-cites-core-59` gives the-gate's requirement its rule's number.
   - Not taken: the archived change `root-changes-validates-each-open-change-as-archived` rewritten to these numbers. It says what was archived, as its commits do, under the numbers it was written with.

### Consequences

- Good: a change that would not archive, or would archive into specs `root:specs` fails, fails its own branch's gate, named with OpenSpec's words, before it lands.
- Good: a delta that adds a capability has to carry its own `## Purpose`, since archive reads a delta's Purpose only when it creates the capability and the placeholder fails `--strict`.
- Bad: `root:changes` runs `openspec` once more for each open change and once for each spec its archive writes, and `root:changes-cases` runs it for five trees. Each run reads a few files.
- Bad: flume's `check-changes.sh` and Boomdrive's can drift; the design is Boomdrive's record 0050's to change first.
- Rule core.59 says so, in the commit that writes this record, and the site's rules page mirrors it.
