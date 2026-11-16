# Tasks

The work is on `change/root-changes-validates-each-open-change-as-archived`,
cut from `bolt/flumes-gate-checks-changes-as-archived`. The rule is
boomdrivedev/blueprints principle 16 as `3a5c19d` amends it; the design
is boomdrivedev/boomdrive's record 0050 and its `check-changes.sh` at
`d9414da6` on `bolt/boomdrives-gate-checks-changes-as-archived`, read
from boomdrivedev/boomdrive's repository by `git show
d9414da6:check-changes.sh`. Every
scratch tree is made under `mktemp -d`, in the temporary folder the
merge queue gives the place (core.51).

## 1. The tree's leftover change

- [x] 1.1 Cut `openspec/changes/a-hosts-full-status-read-takes-seconds/specs/merge-queue-profile/spec.md` as design section 5 says: the MODIFIED "Read, base and the place's data" in the words given there, with its scenarios but "Every branch asked at once", and an ADDED "One read for every branch asked at once" in the words given there, with that scenario; then archive the change, `devenv shell -- openspec archive a-hosts-full-status-read-takes-seconds --yes`, so the specs say what landed at #117 (core.19). True when `sluice exec <base> -- moon run root:specs` passes.

## 2. The record and the rule

- [x] 2.1 Write `docs/adr/0072-each-open-change-is-validated-as-it-would-archive-alone-against-the-trees-specs.md`, number 72, proposed, as design section 7 says: the user's answer of 2026-11-15 on their card `req_278df7ab2cc34988a3f5`, quoted, and principle 16 at `3a5c19d`; Boomdrive's record 0050's options and choices, cited as the family's one design; and design section 9's choices as the construction agent's, each with the path not taken. In the same commit, add core.58 under "The gate" in `docs/architecture/core.md`, after core.51, and its details after core.51's, each as design section 7 gives it, and mirror core.58 as written on `site/src/content/docs/architecture/rules.md` after core.51. True when `sluice exec <base> -- moon run site:build` passes.

## 3. The check

- [ ] 3.1 Add `check-changes.sh` at the root, Boomdrive's at `d9414da6` copied whole, its header citing record 0072, core.58 and boomdrivedev/boomdrive's record 0050 as design section 1 says; have `root:changes` in `moon.yml` run `sh check-changes.sh`, its `env` dropped and `/check-changes.sh` among its inputs, its comment saying it validates each open change as it would archive (record 0072, core.58). True when `sluice exec <base> -- moon run root:changes` passes, this change the tree's one open change, and `sluice exec <base> -- moon run root:format` passes over the new script.

## 4. The check's cases

- [ ] 4.1 Add `check-changes-cases.sh` at the root with the five cases of design section 4, each over a scratch tree of its own, and the check-fast task `root:changes-cases` in `moon.yml` that runs `sh check-changes-cases.sh`, with the inputs design section 4 names and a comment citing record 0072 and core.58; name the cases among the root's checks in `.moon/workspace.yml`'s comment and in `CLAUDE.md`'s table of layers (core.21). True when `sluice exec <base> -- moon run root:changes-cases`, `sluice exec <base> -- moon run root:format` and `sluice exec <base> -- moon run root:layers` pass.
