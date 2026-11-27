# Each gate task that builds a binary builds it in its own folder

## Why

cargo writes `target/debug/<binary>` again at each build, also at a
build that has nothing to do. Each such build gives the file a new
inode. A `cargo test` of a package that has integration tests does the
same (cargo 1.98.1, the version that the tree pins). A gate runs its
tasks at the same time in one checkout. sluice found this race on its
own gate (sluice 7fc9ca6). There, a test started the binary while the
cargo of another task wrote it, and the test found no file.

flume's gate has the same race. Four tasks build `target/debug/flume`:

| Task | Build | Layer |
|---|---|---|
| `flume:test` | `cargo test -p flume` | check-fast |
| `flume:tools` | `cargo test -p flume --features tools` | check-tools |
| `flume:live` | `cargo test -p flume --features live` | check-live |
| `plugin-claude:test` | `cargo build -p flume`, in `tests/hooks.sh` | check-fast |

`flume:tools` and `flume:live` build the binary with other features.
Thus a test of `flume:test` can run a binary of other features, and the
test does not tell. Two tasks also build `target/debug/coverage`:
`conformance:test`, for the integration tests, and
`conformance:coverage`, which runs it.

## What Changes

- **Each task that builds a binary has its own target folder.**
  `flume:tools`, `flume:live`, `plugin-claude:test` and
  `conformance:coverage` each set `CARGO_TARGET_DIR` to a folder of
  their own under `target/`. `flume:test` and `conformance:test` keep
  the workspace's `target`.
- **A check holds the tasks to this.** `root:targets` runs
  `check-targets.sh`, which finds each task that builds a binary of the
  workspace and its folder. It fails and names the tasks where two tasks
  build one binary in one folder. `root:targets-cases` runs its cases
  on scratch workspaces.

No clause changes. The check is a gate check (core.21).
