# Tasks

## 1. The version

- [x] 1.1 Pin git-cliff in `devenv.nix` to the version of the nixpkgs that `devenv.lock` pins, and assert it on the packages of the shell. Have `check-toolchain.sh` hold git-cliff to that pin. True when `sluice exec <base> -- moon run root:toolchain` passes and names git-cliff at its pin.
- [x] 1.2 Add `next-version.sh`, as sluice's. It prints the tag of the next release of the commit checked out, from the tags present, or nothing where that tag exists. Add its cases, `check-next-version.sh`, as the check-fast task `next-version` of the root project. True when `sluice exec <base> -- moon run root:next-version` passes, and fails where the script prints a tag that exists.

## 2. The workflow

- [x] 2.1 Put the runs of pushes to main of `.github/workflows/ci.yml` in the concurrency group `release`, with no run in progress cancelled. Remove the group of the job `release`. Have the job `version` run `sh next-version.sh` under the git-cliff of the nixpkgs that `devenv.lock` pins. Have `nix:test` hold the workflow to this by its text. True when `sluice exec <base> -- moon run nix:test` passes, and fails with `cancel-in-progress` true.
- [x] 2.2 Keep the workflow and the scripts to their formatters, actionlint, shellcheck and shfmt (core.21). True when `sluice exec <base> -- moon run root:format` passes.

## 3. The specs

- [x] 3.1 Archive this change so the specs reconcile (core.19). True when `sluice exec <base> -- moon run root:specs` passes.
