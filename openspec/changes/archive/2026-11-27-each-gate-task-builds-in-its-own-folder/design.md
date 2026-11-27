# Design

## 1. Which tasks build a binary

A task builds the binaries of a package where its command or script
runs `cargo build`, `cargo run`, `cargo test` or `cargo nextest run`
with `-p <package>`, and the package has a binary target. The check
also reads each `.sh` file that the command or script names, because
`plugin-claude:test` builds the binary in `tests/hooks.sh`. `cargo
metadata --no-deps --offline` gives the packages that have a binary
target. A lint builds no binary: `cargo clippy` writes metadata only.

A `cargo test` of a library writes no binary at `target/debug/`, so the
tests of `seams`, `core` and `fakes` keep the workspace's `target`. A
folder of their own would only add builds.

## 2. The folder of a task

The folder is the task's `CARGO_TARGET_DIR`, which moon gives with
`$workspaceRoot` expanded. With no `CARGO_TARGET_DIR`, cargo builds in
the workspace's `target` from each folder of the workspace. cargo reads
a relative `CARGO_TARGET_DIR` from the folder that the task runs in. Each
new folder is under `target/`, which git ignores and `root:format`
does not read.

## 3. The cost

The gate runs rustc without sccache (`devenv.nix`). Thus each new
folder builds its crates again in the gate. On main, `flume:tools` does
this. On each branch, `plugin-claude:test` and `conformance:coverage`
do this. A
session's shell runs rustc under sccache, so there the cost is small.
sluice accepted the same cost for the same race.

## 4. How the check reads moon.yml

No pinned tool reads YAML, because `devenv.nix` does not pin `yq`. Thus
`check-targets.sh` reads each `moon.yml` with awk, as `check-layers.sh`
does. It reads `env` inline or on the lines below it.
