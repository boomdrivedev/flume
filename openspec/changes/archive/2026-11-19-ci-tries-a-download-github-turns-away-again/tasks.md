# Tasks

## 1. The script

- [x] 1.1 Add `try-again.sh`, which runs its command again where it fails and the last forty lines of its output say HTTP 429 or a 5xx, or GitHub's rate limit, up to three runs, pausing 60 and then 120 seconds, and exits with the command's status at once on any other failure and on the third refusal; and its cases, `check-try-again.sh`, as the root project's check-fast task `try-again`. True when `sluice exec <base> -- moon run root:try-again` passes, and fails with the script reading its whole output, or not reading "rate limit exceeded", or pausing a minute each time.

## 2. The workflow

- [x] 2.1 Run each step of `.github/workflows/ci.yml` that downloads from GitHub through `sh try-again.sh`: `fast`'s and `flake`'s devenv shell, `conventional`'s `nix shell` of convco and `binaries`' `cargo build`; and have `nix:test` hold the workflow to it by its text. True when `sluice exec <base> -- moon run nix:test` passes, and fails with `flake`'s and `binaries`' steps run without it.
- [x] 2.2 Keep the workflow and the script to their formatters, actionlint, shellcheck and shfmt (core.21). True when `sluice exec <base> -- moon run root:format` passes.

## 3. The specs

- [x] 3.1 Archive this change so the specs reconcile (core.19). True when `sluice exec <base> -- moon run root:specs` passes.
