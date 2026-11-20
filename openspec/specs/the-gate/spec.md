# the-gate Specification

## Purpose
The checks a change to flume is held to and how they run: the commit
hooks, treefmt, the root project and each crate's and part's moon
project, the tools the tree pins, the layers `check-fast`, `check-tools`
and `check-live` and the branches that require each, and CI.

## Requirements

### Requirement: The commit hooks run in every worktree

The commit hooks SHALL read the hook config of the worktree the commit
is in, or the main checkout's where the worktree's shell has not written
one, and SHALL skip only where neither exists, so no commit needs
`PREK_ALLOW_NO_CONFIG`. They SHALL run convco on the message, treefmt on
the staged files, and `cargo clippy --workspace --all-targets -- -D
warnings` when a Rust file or the cargo manifests change (core.21).

#### Scenario: A commit in a fresh worktree

- **WHEN** a worktree whose devenv shell never ran commits a Rust file, and `root:hooks` drives that commit in a scratch clone
- **THEN** prek runs with the main checkout's config, convco, treefmt and clippy each run, and no `PREK_ALLOW_NO_CONFIG` is set

### Requirement: Every tracked file has a formatter or a named reason

treefmt SHALL fail on a tracked file no formatter claims and no exclude
names, every exclude SHALL carry a comment saying why, and the root
project's `format` task SHALL run `treefmt --ci` over the tree in the
check-fast layer (core.21).

#### Scenario: A file of a new type

- **WHEN** `root:format` runs over a tree holding a tracked file of a type no formatter claims
- **THEN** it fails naming the file

### Requirement: The tree's tools are the versions it pins

Every tool a gate task runs SHALL be the version `devenv.nix` or
`.prototools` pins, Claude Code, OpenSpec, herdr and sluice among them,
and `root:toolchain` SHALL fail when rustc, cargo, clippy or rustfmt is
not the build `.prototools` pins (core.22).

#### Scenario: A toolchain from outside the pin

- **WHEN** `root:toolchain` runs with a rustc on PATH other than the pinned one first
- **THEN** it fails naming the tool and both versions

### Requirement: Three layers, the gate and CI

The checks SHALL be tagged `check-fast`, `check-tools` or `check-live`;
`.config/sluice.toml` SHALL require `#check-fast` of a bolt and
`#check-fast` and `#check-tools` of main, under the sandboxed executor;
and CI SHALL run `:#check-fast` on Linux and macOS in the devenv shell
(core.21, core.22).

#### Scenario: The layers each task is in

- **WHEN** `root:layers` reads every project's tasks
- **THEN** every task carries exactly one of the three layer tags, or none with a comment saying why the gate cannot run it

### Requirement: The live walkthroughs pass under the pinned Claude Code

The `check-live` layer's walkthroughs SHALL carry a fix and a change
from their launch to their landing with the pinned Claude Code under
herdr, signed in by a test account's token, SHALL leave a stage whose
pane goes quiet with no deliverable not ended (39, S3), and SHALL have
a role's session take a batch through flume's plugin's deliver part,
its input left as typed (215); the operator runs them by hand, outside
`sluice exec`.

#### Scenario: The operator's run

- **WHEN** the operator runs `moon run flume:live` from the devenv shell with a test account's token
- **THEN** both walkthroughs land, and the quiet stage with no deliverable is not ended

#### Scenario: A role's session takes a batch with words half typed

- **WHEN** the live walkthrough wakes the standing role `w/ops`, whose talk reads its inbox and waits, types words into the talk's pane without submitting them, and gives `w/ops` two messages each asking for one word
- **THEN** the talk's transcript holds each message in a user turn and the word each asks for in its answers, holds the words typed in no user turn, the pane's input holds them once the talk waits again, and status gives each message handed to `w/ops` by `message` alone (54, 214, 215, C.4)

### Requirement: The commit hooks take a commit git revert wrote as CI does

The commit-msg hook SHALL take a message `git revert` wrote, its first
line starting `Revert "`, as CI's conventional check skips the commit by
`convco check --ignore-reverts`, and SHALL hand every other message to
convco, so a session and CI hold a commit to one rule (core.22).

#### Scenario: A revert committed after `--no-commit`

- **WHEN** `root:hooks` commits in its fresh worktree a revert `git revert --no-commit` prepared, with the message git wrote, and then a message with no type
- **THEN** convco passes the revert, which is committed, and refuses the message with no type

### Requirement: The flake is a project of its own

The flake SHALL be the moon project `nix`, at `nix/`, listed in
`.moon/workspace.yml` with `language: 'nix'`. Its `format`, `lint`,
`typecheck` and `test` tasks SHALL be in the check-fast layer. Its
`build` task SHALL be in no layer, with a comment above it saying the
gate's sandbox denies the daemon and the network it needs (core.21).

#### Scenario: The build task's layer

- **WHEN** `root:layers` reads the `nix` project's tasks
- **THEN** `format`, `lint`, `typecheck` and `test` each carry `check-fast`, and `build` carries none with a comment above it saying why the gate cannot run it

### Requirement: The flake's checks need neither the network nor the daemon

`nix:typecheck` and `nix:test` SHALL evaluate `nix/tests` offline,
read-only and against a dummy store, from the inputs `devenv.nix`
fetches outside the sandbox and gives as `FLUME_FLAKE_INPUTS`, and
SHALL refuse to run where those are not given.

#### Scenario: The checks in the gate's sandbox

- **WHEN** `nix:typecheck` and `nix:test` run under `sluice exec`, in the gate's sandbox
- **THEN** each evaluates and reports `nix: <check>: every case holds`, having written nothing to any store

#### Scenario: No inputs given

- **WHEN** `nix:test`'s script cases run `nix/check.sh test` with `FLUME_FLAKE_INPUTS` unset
- **THEN** it exits non-zero saying the devenv shell sets it

### Requirement: CI builds the flake's package

CI SHALL run `nix:build` in the devenv shell on Linux and macOS for each
pull request and each push to main, so a package that no longer builds
fails before a release carries it.

#### Scenario: The workflow's build job

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** a job `flake` runs on `ubuntu-latest` and `macos-latest` on each pull request and each push to main, and its step runs `devenv shell -- moon run nix:build`

### Requirement: The flake's tools are the versions the tree pins

nix, nixfmt, statix and deadnix SHALL be the versions `devenv.nix`
pins, and `root:toolchain` SHALL hold each on `PATH` to its pin
(core.22).

#### Scenario: The nix tools at their pins

- **WHEN** `root:toolchain` runs in the devenv shell
- **THEN** it names nix, nixfmt, statix and deadnix each at the version `devenv.nix` pins, and fails naming the tool and both versions where one is not

### Requirement: The site installs its packages with no network

`site:install` SHALL install `site/bun.lock`'s packages offline, from
bun's cache as `site/packages.nix` builds it from the integrity the lock
gives each package, which `devenv.nix` builds outside the sandbox and
gives as `FLUME_SITE_PACKAGES`, and SHALL refuse to run where that is
not given (record 0059).

#### Scenario: The install in the gate's sandbox

- **WHEN** `site:install` runs under `sluice exec`, in the gate's sandbox, whose network reaches localhost alone
- **THEN** bun installs every package the lock names for the host from the cache, asking no registry

#### Scenario: No packages given

- **WHEN** `sh scripts/install.sh` runs in `site/` with `FLUME_SITE_PACKAGES` unset
- **THEN** it exits non-zero saying the devenv shell sets it

### Requirement: CI's conventional check runs the hook's convco

CI's conventional check SHALL run `convco check --ignore-reverts` with
the convco of the nixpkgs `devenv.lock` pins, the one the commit-msg
hook runs, and never the flake registry's, so a session and CI hold a
commit to one convco (core.22).

#### Scenario: The workflow's conventional job

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** the job `conventional` reads the nixpkgs `devenv.lock` pins and runs `convco check --ignore-reverts` from it, and names no `nixpkgs#convco` of the registry

### Requirement: The root project checks what spans the tree and reads no Markdown

The root project SHALL hold one check-fast task per check that spans
the tree: `format`, `toolchain`, `specs` and `changes` (`openspec
validate --strict` of the specs and the open changes) and `marketplace`
(the marketplace manifest and each plugin it lists); no task SHALL read
a Markdown file for its words or its form, a record's number among them
(core.21; record 0074).

#### Scenario: An open change that does not validate

- **WHEN** `root:changes` runs over an open change whose requirement has no scenario
- **THEN** it fails naming the change and the requirement

### Requirement: Each open change is validated as it would archive

`root:changes` SHALL run `openspec validate --changes --strict`, then
archive each open change alone, by `openspec archive`, into a scratch
copy of the tree's specs and configuration, every spec that archive
writes passing `openspec validate --strict`. It SHALL fail naming each
change that fails, with OpenSpec's words, fail none for a task left
unchecked, and write nothing in the tree (core.59).

#### Scenario: A change archive refuses

- **WHEN** `root:changes` runs over an open change that adds a requirement its spec already holds
- **THEN** it fails naming the change and saying `openspec archive` refuses it, with OpenSpec's words naming the requirement

#### Scenario: A new capability with no Purpose

- **WHEN** `root:changes` runs over an open change that adds a capability whose delta gives no `## Purpose`
- **THEN** it fails naming the change and the spec its archive writes, with OpenSpec's warning that the Purpose is a placeholder

#### Scenario: A modified requirement past OpenSpec's length

- **WHEN** `root:changes` runs over an open change that modifies a requirement to more than 500 characters of text
- **THEN** it fails naming the change and the spec its archive writes, with OpenSpec's warning that the requirement's text is very long

#### Scenario: Each change alone, its tasks open

- **WHEN** `root:changes` runs over two open changes that each add a requirement of one name to one spec, each with a task left unchecked
- **THEN** it passes, and the tree's specs and changes are as they were before it ran

### Requirement: The tools layer runs its tests all at once

The `flume:tools` task SHALL run its tests as many at once as libtest
takes on the host, never the share of the host's cores a sluice run is
handed, since its tests wait on the pinned tools' sessions and the
stand-in rather than on a processor; every other task SHALL keep that
share for its builds and tests (core.21).

#### Scenario: A gate run on a host that hands each run two cores

- **WHEN** sluice runs `flume:tools` with `SLUICE_JOBS` set to 2
- **THEN** cargo builds with two jobs and libtest runs the tests with no `RUST_TEST_THREADS`, as many at once as the host has processors
