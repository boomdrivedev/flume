# Verify: root-changes-validates-each-open-change-as-archived

Verified on 2026-11-15 at `3fd671c0`, on
`change/root-changes-validates-each-open-change-as-archived`, cut from
`bolt/flumes-gate-checks-changes-as-archived`.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-gate-checks-changes-as-archived`
ran once and exited 0: 41 tasks completed, 40 passed, and sluice
recorded the 40 results under their keys for `3fd671c`, in 2 minutes 50
seconds. The base is a bolt, so its layer is `check-fast`;
`check-tools` is the bolt's merge onto main's to run.

The new and changed root tasks said:

- `root:changes`: "changes: 1 open, each valid under openspec validate
  --strict and archiving to specs that are", this change the tree's one
  open change.
- `root:changes-cases`: "changes-cases: every case holds".
- `root:specs`, `root:adrs`, `root:format`, `root:layers`,
  `root:hooks`, `root:toolchain`, `root:versions` and `site:build`
  passed.

moon warned, for `root:changes`, `root:changes-cases` and
`flume:test`, that it could not package the task's outputs into its
cache's archive. That is moon's cache, not a check, and failed nothing.

## The spec

the-gate's ADDED requirement, "Each open change is validated as it
would archive", is met by `check-changes.sh` as `root:changes` runs it:
it runs `openspec validate --changes --strict --no-interactive`, then
archives each open change alone, `openspec archive <change> --yes`, in
a scratch copy under `mktemp -d` holding the tree's `config.yaml`, its
specs and that change, and runs `openspec validate <id> --type spec
--strict` for each spec the archive wrote, one new or one whose bytes
differ from the tree's by `cmp`. It names each failing change with
OpenSpec's words indented under it, runs archive with `--yes` so an
unchecked task fails nothing, and writes only under its scratch folder.

Each scenario is proved by a case of `check-changes-cases.sh`, which
`root:changes-cases` ran and found holding:

| scenario | case | what the case reads |
|---|---|---|
| A change archive refuses | adds `One` to `alpha` again | non-zero exit, "adds-one-again would not archive", "already exists" |
| A new capability with no Purpose | adds `beta` with no `## Purpose` | non-zero exit, "adds-beta would not archive cleanly: the spec beta", "placeholder" |
| A modified requirement past OpenSpec's length | modifies `One` past 500 characters | non-zero exit, "makes-one-long would not archive cleanly: the spec alpha", "very long" |
| Each change alone, its tasks open | two changes each adding `Two` to `alpha`, one task unchecked each | exit 0, "changes: 2 open, each valid", every file under `openspec/` the same by `cksum`, no `archive` folder made |

A fifth case proves the existing scenario "An open change that does not
validate": non-zero exit, naming the change and `Two`.

The first scenario asks for OpenSpec's words naming the requirement.
A scratch tree holding the first case's change, run by hand through
`check-changes.sh`, gave them: `alpha ADDED failed for header
"### Requirement: One" - already exists`, under "changes: adds-one-again
would not archive: openspec archive refuses it:".

## The tasks

Every task is ticked, and each holds as the design says:

- 1.1: `a-hosts-full-status-read-takes-seconds`'s merge-queue-profile
  delta is cut as design section 5 gives it, word for word: the
  MODIFIED "Read, base and the place's data" without "Every branch
  asked at once", and the ADDED "One read for every branch asked at
  once" with that scenario. The change is archived as
  `openspec/changes/archive/2026-11-15-a-hosts-full-status-read-takes-seconds`,
  and `openspec/specs/merge-queue-profile`, `place-status` and
  `seam-contracts` carry its deltas. `root:specs` passed.
- 2.1: record 0072 is written, proposed, with the user's words of
  2026-11-15 quoted, Boomdrive's record 0050's choices cited, and the
  construction agent's choices each with the path not taken. core.58
  and its details follow core.51 and core.51's in
  `docs/architecture/core.md`, as design section 7 gives them, and the
  site's rules page mirrors core.58 after core.51. `site:build` passed.
- 3.1: `check-changes.sh` is Boomdrive's at `d9414da6` copied whole,
  read from boomdrivedev/boomdrive's repository by `git show
  d9414da6:check-changes.sh`: the two differ only in the header, which
  cites record 0072, core.58 and Boomdrive's record 0050 and core.61.
  `root:changes` runs `sh check-changes.sh`, its `env` dropped,
  `/check-changes.sh` among its inputs, its comment citing record 0072
  and core.58. `root:changes` and `root:format` passed.
- 4.1: `check-changes-cases.sh` holds the five cases of design section
  4, each over a scratch tree of its own under one `mktemp -d`;
  `root:changes-cases` is a check-fast task with the inputs design
  section 4 names and a comment citing record 0072 and core.58;
  `.moon/workspace.yml`'s comment and `CLAUDE.md`'s table name "open
  changes and their check's cases". `root:changes-cases`,
  `root:format` and `root:layers` passed.

## Outcome

The whole checks pass and the work holds: the change is archived.
