# Design

The rule is the user's: boomdrivedev/blueprints `design/principles.md`
16 as `3a5c19d` amends it, on `design/a-tools-own-validation-stays`
until the user pushes it, on their answer of 2026-11-15 on their card
`req_278df7ab2cc34988a3f5`: "OpenSpec's own validation stays; the rule
is about checks of what pages say". The principle as amended keeps
"OpenSpec's `openspec validate` of a kit's specs and its open changes,
each change validated as it would archive".

The design is Boomdrive's: boomdrivedev/boomdrive's record 0050 and
core.61, and its `check-changes.sh`, at `d9414da6` and `1fafff45` on
`bolt/boomdrives-gate-checks-changes-as-archived` (landed there as
#236). boom copies it, in its record 0004 and core.7, proposed, on
`chore/booms-changes-gate-validates-each-open-change-as-archived`.
flume takes the same design and cites Boomdrive's record as its owner
(CLAUDE.md, "The family").

OpenSpec is 1.14.3, the release `devenv.nix` pins. Its behaviour below
was read on 2026-11-15 in a scratch tree: `openspec validate --changes
--strict` passes an ADDED requirement its spec already holds and a
MODIFIED one its spec does not, saying only "Archive would refuse this
delta"; `openspec archive` refuses both; `openspec archive --yes`
archives a change whose tasks are unchecked; and `openspec validate
--strict` fails the placeholder Purpose archive writes for a new
capability and a requirement past 500 characters of text.

The rules relied on: core.19 (a spec changes only through an archived
change, so the leftover change is archived and never its specs edited);
core.21 (one task per check: `root:changes` checks the tree and
`root:changes-cases` checks the check); core.22 (the gate runs the
OpenSpec the tree pins, and a script the tree holds); core.51 (each
scratch tree is made under the temporary folder the merge queue gives
the place). kit:specs.3 and kit:testing.5: each scenario this change
adds is one a moon task proves. No area page exists; core.md binds
every path this change touches. core.1 does not reach a root gate
script, which is no module of flume's, as `root:specs` names
`openspec` today. The change bears on no clause of flume's statement
(core.19): it is the gate's.

## 1. What `root:changes` does

`root:changes` runs `sh check-changes.sh`, Boomdrive's script copied
whole, its header citing record 0072, core.58 and Boomdrive's record
0050. With `OPENSPEC_TELEMETRY=0` and `OPENSPEC_NO_UPDATE_CHECK=1`
exported, it:

1. Runs `openspec validate --changes --strict --no-interactive`, as the
   task ran before, and fails with OpenSpec's words where it fails.
2. For each open change, each folder under `openspec/changes` but
   `archive`, makes a scratch copy of `openspec/` holding the tree's
   `config.yaml`, its specs and that change alone, and runs `openspec
   archive <change> --yes` there. Where archive refuses, it fails:
   `changes: <change> would not archive: openspec archive refuses it:`,
   OpenSpec's words indented under it.
3. Runs `openspec validate <id> --type spec --strict` in the copy for
   each spec the archive wrote, one new or one whose bytes differ from
   the tree's, and fails where one fails: `changes: <change> would not
   archive cleanly: the spec <id> it writes fails openspec validate
   --strict:`, OpenSpec's words under it.
4. Says `changes: <n> open, each valid under openspec validate --strict
   and archiving to specs that are` where nothing failed.

It takes the folder holding `openspec/` as its one argument, the
current folder where none is given, so its cases run it over scratch
trees. It writes only under its scratch folder, `mktemp -d`, removed
at its exit.

The task's inputs gain `/check-changes.sh`; its `env` goes, since the
script exports both variables.

## 2. Which specs are held to `--strict`

Only the specs the archive writes. A spec the change leaves as it was
is `root:specs`' to check; holding every spec of the copy would fail
every open change for a spec on main that fails, each named for what
it did not write.

The script compares each spec the archive wrote with the tree's byte
for byte, by `cmp`, to know which it wrote. That reads no word and no
form of either page: it is part of OpenSpec's validation as archived,
which principle 16 as amended keeps. core.21 as
`bolt/flumes-checks-read-no-markdown` amends it says the same, "Not
ruled out: OpenSpec's validation of the specs and the open changes".

## 3. Each change alone, its tasks open

Each change archives at its own merge onto its base as it stands then,
so each is applied alone to the tree's specs. Two open changes that
conflict are found by the gate on the branch that lands second. A
change is open while its tasks are worked, so archive runs with
`--yes` and a task left unchecked fails nothing.

## 4. The check's cases

`check-changes-cases.sh`, run by the new check-fast task
`root:changes-cases`, proves the scenarios of `root:changes`: the four
this change adds and "An open change that does not validate", which no
check proved. It is laid out as `nix/tests/scripts.sh` is: each case a
scratch tree of its own under one `mktemp -d`, removed at its exit;
`sh check-changes.sh <tree>` run over it, its exit and its words read;
each failure printed as one line, `changes-cases: <case>: <what>`; a
non-zero exit where any case fails, and `changes-cases: every case
holds` otherwise.

Each tree holds `openspec/config.yaml` saying `schema: spec-driven`, a
spec `alpha` with a Purpose and one requirement, "One", with a
scenario, and the changes its case names, each with a proposal, a
`tasks.md` holding one task unchecked, and its delta. The cases write
these as data, which core.21 as the no-Markdown bolt amends it does
not rule out ("a change folder or another Markdown file into a scratch
repository as data").

| case | the change | what the check says |
|---|---|---|
| An open change that does not validate | adds to `alpha` a requirement "Two" with no scenario | fails, naming the change and "Two" |
| A change archive refuses | adds to `alpha` a requirement "One" | fails, naming the change, "would not archive" and "already exists" |
| A new capability with no Purpose | adds a capability `beta`, its delta with no `## Purpose` | fails, naming the change and `beta`, and "placeholder" |
| A modified requirement past OpenSpec's length | modifies "One" to more than 500 characters of text, with a scenario | fails, naming the change and `alpha`, and "very long" |
| Each change alone, its tasks open | two changes, each adding to `alpha` a requirement "Two" with a scenario | passes, every file under the tree's `openspec/` as it was, by `cksum` before and after, and no `archive` folder made |

Its inputs are `/check-changes.sh`, `/check-changes-cases.sh`,
`/devenv.nix` and `/devenv.lock`; it reads nothing else of the tree.
`.moon/workspace.yml`'s and `CLAUDE.md`'s lists of the root's checks
name it.

## 5. The tree's leftover change

`a-hosts-full-status-read-takes-seconds` landed on main as #117
(`24551f12`) with every task checked and was never archived. Its
merge-queue-profile delta's MODIFIED requirement, "Read, base and the
place's data", is 609 characters of text, so the spec its archive
writes fails `openspec validate --strict`, and `root:changes` would
fail the tree.

Its delta is cut in two, its meaning kept:

- A MODIFIED "Read, base and the place's data": "The profile SHALL read
  each branch's newest entry as "One read for every branch asked at
  once" says; answer the base at `origin/<base>` after one fetch where
  `.config/sluice.toml` lands the base on a remote and the local base
  otherwise; and give the place profile the readiness, `sluice equip
  <base> --at <revision> --json` with the reading of its steps, the
  teardown (`sluice release`) and the base's ref, reading no `[tests]`
  key for a place (9, 11, 20, 126, C.1, C.6, core.3).", with its
  scenarios but "Every branch asked at once".
- An ADDED "One read for every branch asked at once": "The profile
  SHALL read the newest entry of each branch asked from `sluice
  overview --json` and `sluice status <base> --all --json` for each
  base the overview names, run once for every branch of one repository
  asked at once (99, C.6).", with the scenario "Every branch asked at
  once".

Then it is archived, `openspec archive
a-hosts-full-status-read-takes-seconds --yes`, so the specs say what
landed at #117 (core.19). Boomdrive's script passed this cut, and this
change, in a scratch copy of the tree on 2026-11-15.

## 6. The spec

The-gate gains an ADDED requirement, "Each open change is validated as
it would archive", rather than a MODIFIED "The root project checks what
spans the tree". `bolt/flumes-checks-read-no-markdown` removes that
requirement and adds it again as "The root project checks what spans
the tree and reads no Markdown"; whichever bolt lands second, a
MODIFIED of the old name would be refused at its archive, by this very
check. The root requirement still holds as written: `root:changes`
still runs `openspec validate --strict` of the open changes.

## 7. The record and the rule

Record 0072, `docs/adr/0072-each-open-change-is-validated-as-it-would-archive-alone-against-the-trees-specs.md`,
proposed until the user's yes on it as a record: the user's words,
quoted; Boomdrive's record 0050's options and choices, cited as the
family's one design; this design's choices of section 9, each with the
path not taken. Its number is 0072 since 0071 is taken on
`bolt/flumes-checks-read-no-markdown` and on
`bolt/flumes-places-need-no-hand`.

core.58, under "The gate" after core.51, mirrored on the site's rules
page as written:

> **[core.58]** The gate holds the specs and every open change to
> OpenSpec's own `openspec validate --strict`, and each open change to
> archiving cleanly: `openspec archive` applies its deltas, alone, to a
> scratch copy of the tree's specs, and every spec it writes passes
> `openspec validate --strict`. (Record 0072, proposed; boomdrivedev/blueprints
> principle 16 as amended on 2026-11-15.)

Its details, after core.51's:

> **core.58** Rules out: an open change that `openspec archive`
> refuses, such as one adding a requirement its spec already holds or
> modifying one its spec does not hold; an open change that archives
> into a spec `root:specs` fails, such as a new capability with
> archive's placeholder Purpose or a MODIFIED requirement past
> OpenSpec's 500 characters, as `a-hosts-full-status-read-takes-seconds`
> would have written merge-queue-profile once it landed as #117; a
> change judged with the other open changes applied; the tree's specs
> or changes written by the check. Not ruled out: a change whose tasks
> are not all checked; a spec the change leaves as it was, which
> `root:specs` alone checks; two open changes that conflict, found by
> the gate on the branch that lands second; the check comparing a spec
> the archive wrote with the tree's byte for byte to know what it
> wrote, which reads no word or form of either (principle 16 as
> amended); the check's cases writing changes into scratch trees as
> data. Source: boomdrivedev/blueprints principle 16 as amended on
> 2026-11-15, on the user's answer on their card ("OpenSpec's own
> validation stays; the rule is about checks of what pages say");
> record 0072; boomdrivedev/boomdrive's record 0050 and core.61, the
> design this copies; core.21, core.22.

## 8. Cost

`root:changes` runs `openspec` once more for each open change and once
for each spec its archive writes, and `root:changes-cases` runs it for
five trees. Each run reads a few files.

## 9. The construction agent's choices

Each is cheap to change and recorded in record 0072.

1. **The script is flume's own, copied from Boomdrive's.** flume's gate
   runs files its tree holds at versions it pins (core.22). Not taken: a
   script one tool owns that flume's gate calls, which flume's tree
   neither holds nor pins. The cost: two copies can drift, so a change
   to the design is made in Boomdrive's record first and carried here.
2. **The cases are a task of their own.** `root:changes` checks the
   tree, and its cases check the script (core.21). Not taken: the cases
   inside `check-changes.sh`, which would run them over every tree it
   checks.
3. **The leftover change is cut and archived here.** The new check
   fails the tree without it, and its work landed at #117. Not taken:
   cutting its delta and leaving it open, which keeps the specs behind
   what landed and leaves its MODIFIED requirements to undo, at a later
   archive, what other changes wrote of them since. Not taken: editing
   `openspec/specs` to match, which core.19 forbids.
4. **The spec adds a requirement** (section 6). Not taken: a MODIFIED
   root requirement, which the no-Markdown bolt renames.
5. **The record is 0072** (section 7).
