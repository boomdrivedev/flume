# Tasks

The work is on `fix/flumes-checks-read-no-markdown-1`, cut from
`bolt/flumes-checks-read-no-markdown`, with main at `f5e6858` merged in.
Record 0074 is written, and core.21 cites it.

## 1. The specs

- [x] 1.1 Validate with `openspec validate the-no-markdown-requirements-cite-record-0074 --strict`, then archive with `openspec archive the-no-markdown-requirements-cite-record-0074 --yes`. True when `sluice exec bolt/flumes-checks-read-no-markdown -- moon run root:specs root:changes` passes and `openspec/specs/the-gate/spec.md` and `openspec/specs/herdr-plugin/spec.md` cite record 0074 and not record 0071.
