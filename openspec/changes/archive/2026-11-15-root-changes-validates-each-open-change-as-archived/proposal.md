# root:changes validates each open change as archived

## Why

`root:changes` runs `openspec validate --changes --strict`. A change can
pass it and still fail at its archive, or archive into specs that
`root:specs` then fails, found only after the gate has passed its
branch:

- `openspec archive` refuses a delta that does not apply, an ADDED
  requirement its spec already holds or a MODIFIED one its spec does not
  hold, which `openspec validate --changes --strict` reports only as
  information and passes.
- `openspec archive` checks the specs it rebuilds without `--strict`. A
  new capability whose delta gives no `## Purpose` gets archive's
  placeholder, and a MODIFIED requirement past OpenSpec's 500 characters
  is let through; `--strict` fails both once the change is archived.

The tree holds such a change now. `a-hosts-full-status-read-takes-seconds`
landed on main as #117 with every task checked and was never archived:
its merge-queue-profile delta's requirement is 609 characters, so the
spec its archive writes fails `openspec validate --strict`.

The user amended boomdrivedev/blueprints principle 16 on 2026-11-15 on
their card `req_278df7ab2cc34988a3f5`: "OpenSpec's own validation stays;
the rule is about checks of what pages say". The principle as amended
keeps "OpenSpec's `openspec validate` of a kit's specs and its open
changes, each change validated as it would archive" (at `3a5c19d` on
`design/a-tools-own-validation-stays`, until the user pushes it).
Boomdrive did this first, in its record 0050 and core.61, and boom
copies it; flume's gate takes the same design.

## What Changes

- **`root:changes` validates each open change as it would archive.** It
  still runs `openspec validate --changes --strict`, then archives each
  open change alone into a scratch copy of the tree's specs, and holds
  every spec that archive writes to `openspec validate --strict`. A change
  that fails is named with OpenSpec's words. A task left unchecked fails
  nothing, and nothing in the tree is written. `check-changes.sh` does
  it, copied from Boomdrive's.
- **The check's own cases.** A new root task, `changes-cases`, runs
  `check-changes.sh` over scratch trees holding the changes the-gate's
  scenarios name and checks what it says.
- **The tree's leftover change archives.**
  `a-hosts-full-status-read-takes-seconds` has its merge-queue-profile
  delta cut within OpenSpec's length, the read of several branches at
  once a requirement of its own, and is archived, so the specs say what
  landed at #117.
- **The record and the rule.** Record 0072, proposed, and rule core.58
  say so, citing Boomdrive's record 0050 as the design this copies; the
  site's rules page mirrors core.58.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: adds the requirement that `root:changes` validates each
  open change as it would archive, with its scenarios.

`merge-queue-profile`, `place-status` and `seam-contracts` change by the
archive of `a-hosts-full-status-read-takes-seconds`, through that
change's own deltas, which this change cuts within OpenSpec's length and
adds nothing to.

## Impact

- `moon.yml`: `root:changes` runs `sh check-changes.sh`; a new
  check-fast task, `root:changes-cases`.
- New files: `check-changes.sh`, `check-changes-cases.sh`,
  `docs/adr/0072-each-open-change-is-validated-as-it-would-archive-alone-against-the-trees-specs.md`.
- `docs/architecture/core.md` and
  `site/src/content/docs/architecture/rules.md`: core.58.
- `.moon/workspace.yml`'s and `CLAUDE.md`'s lists of the root's checks.
- `openspec/changes/a-hosts-full-status-read-takes-seconds`: its
  merge-queue-profile delta cut, then archived.
- No crate, plugin, seam or profile changes, and no clause of flume's
  statement.
