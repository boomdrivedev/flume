# Tasks

The work is on `fix/flumes-gate-checks-changes-as-archived-1`, cut from
`bolt/flumes-gate-checks-changes-as-archived`, with main at `ba66899`
merged in. Record 0073 and rule core.59 are written.

## 1. The spec

- [ ] 1.1 Validate with `openspec validate the-gates-archive-check-cites-core-59 --strict`, then archive with `openspec archive the-gates-archive-check-cites-core-59 --yes`. True when `sluice exec bolt/flumes-gate-checks-changes-as-archived -- moon run root:specs root:changes` passes and `openspec/specs/the-gate/spec.md` cites core.59 and not core.58.
