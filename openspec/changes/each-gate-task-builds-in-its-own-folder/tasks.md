# Tasks

## 1. The folders

- [x] 1.1 In `crates/flume/moon.yml`, set `CARGO_TARGET_DIR` of `tools` to `$workspaceRoot/target/tools` and of `live` to `$workspaceRoot/target/live`, with a comment that says why (design 2). Check: `moon task flume:tools --json` gives the expanded folder.
- [x] 1.2 In `plugins/claude/moon.yml`, set `CARGO_TARGET_DIR` of `test` to `$workspaceRoot/target/plugin-claude`. `tests/hooks.sh` reads the folder from `cargo metadata`. Check: `sluice exec main -- moon run plugin-claude:test`.
- [x] 1.3 In `crates/flume-conformance/moon.yml`, set `CARGO_TARGET_DIR` of `coverage` to `$workspaceRoot/target/coverage`. Check: `sluice exec main -- moon run conformance:coverage`.

## 2. The check

- [x] 2.1 Write `check-targets.sh` (design 1, 2 and 4) and its cases, `check-targets-cases.sh`, and add the tasks `root:targets` and `root:targets-cases` in the check-fast layer. Check: `sluice exec main -- moon run root:targets root:targets-cases root:layers`. Over the tree of main before this change, `check-targets.sh` fails and names `flume:test`, `flume:tools`, `flume:live` and `plugin-claude:test`.
