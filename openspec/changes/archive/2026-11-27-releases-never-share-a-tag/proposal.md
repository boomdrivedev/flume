# Releases never share a tag

## Why

Two pushes to main that land close together can compute one tag. The
job `version` of `ci.yml` reads the tags when its run starts. The job
`release` of the other run can push its tag after that. The concurrency
group of the job `release` makes the two pushes of tags take turns. But
each run computed its version before it waited. flume's release of one
landing failed so, and a person ran it again. On 2026-11-27, sluice's
release of its push #200 failed with "fatal: tag 'v0.110.20' already
exists". The release of push #199 pushed v0.110.20 after the run of #200
computed the same version.

## What Changes

- **The runs of pushes to main take turns.** The whole of `ci.yml` is in
  the concurrency group `release` for a push, and no run cancels a run
  in progress. Each run waits for the run before it to end. A run of a
  pull request is in a group of its own and waits for no other run.
- **The job `release` has no group of its own.** In the group of its
  run, it would wait on its own run.
- **Each release reads its version from the tags as its run starts.** The
  job `version` runs `next-version.sh`, which runs git-cliff on the tags
  that the run fetched. A run after another release so gives the next
  version. Where a release of a later push already holds each commit of
  the run, it gives nothing, and the run releases nothing.
- **A newer push can replace a waiting run.** GitHub keeps one run
  waiting in a group, and a newer run replaces a waiting one. Main only
  moves forward, so the commit of the newer run holds each commit of the
  waiting one. The newer run releases those commits too.
- **`devenv.nix` pins git-cliff.** The devenv shell and the workflow run
  the git-cliff of the nixpkgs that `devenv.lock` pins. `devenv.nix`
  asserts its version (core.22), and `root:toolchain` holds it to that
  pin.
- **`root:next-version`** holds `next-version.sh` to its cases on scratch
  repositories: one push, two pushes in a row, a late run after a later
  release, and chores only.
- **`nix:test`** holds the workflow to its group and its version step, by
  the text of the workflow.

sluice made the same change in its commit "fix(release): two pushes to
main never reckon one tag". flume takes the same group, the same script
and the same cases, so the two tools release in one way.

The run of the second push releases the next version or finds nothing
to release. It never computes a tag that another run pushed. Each run
starts after the run before it ends, so it reads the tag that the
earlier release pushed. Where the earlier run released, the second run
releases the next version. Where the earlier run found nothing to
release, the second run releases the commits of the two runs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: the runs of pushes to main take turns, and each release
  reads its version from the tags as its run starts.

## Impact

- `.github/workflows/ci.yml`: the group of the workflow, the job
  `version` and the job `release`.
- `next-version.sh`, new, and its cases, `check-next-version.sh`, in the
  new task `next-version` of the root project. shellcheck and shfmt own
  both, as every `*.sh` of the tree (core.21).
- `devenv.nix`: git-cliff, pinned, on the packages of the shell.
  `check-toolchain.sh` holds it to its pin.
- `nix/tests/workflow.nix`: the test of the group and the version step.
- No new crate, plugin or site part, so no new moon project.
