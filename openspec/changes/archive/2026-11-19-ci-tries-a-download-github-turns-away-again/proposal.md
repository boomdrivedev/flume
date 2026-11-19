# CI tries a download GitHub turns away again

## Why

On 2026-11-18 the first attempt of CI's run on main for 6ca6783's
landing failed its `fast (macos-latest)` job before a check ran: the
devenv shell fetched nixpkgs f9a75f4 from
`api.github.com/repos/NixOS/nixpkgs/tarball/...`, and GitHub answered
"HTTP error 429 ... This request was rate-limited due to too many
requests from your network. Reduce the frequency of your requests or try
again later." nix's own five attempts spanned 50 seconds and each was
turned away, so the job failed, and the run passed only once it was
re-run by hand. Every job that downloads from GitHub can be turned away
so: `fast` and `flake` fetch the devenv shell's inputs, `flake` the
flake's, `conventional` the nixpkgs convco comes from, and `binaries`
the crates of weir, a git dependency on github.com; a release waits on
`fast`, `conventional` and `binaries`.

## What Changes

- **`try-again.sh`** runs its command and, where it fails and the last
  lines of its output say HTTP 429 or a 5xx, or GitHub's rate limit,
  runs it again: up to three runs in all, a minute's pause before the
  second and two before the third, as GitHub asks a client it turned
  away to wait at least a minute before it tries again. Any other
  failure, and the third refusal, exits with the command's own status at
  once. Only the end of the output is read: the tools try a refusal
  again themselves first, so a refusal they got past, earlier in the
  output, is not why the command failed. Each run again is a warning on
  the run's summary, saying what GitHub said.
- **Each step that downloads from GitHub runs through it:** `fast`'s
  `devenv shell -- moon run ':#check-fast'`, `flake`'s `devenv shell --
  moon run nix:build`, `conventional`'s `nix shell` of convco and
  `binaries`' `cargo build`. A run again of `fast` re-runs only what moon
  has not already passed in that job.
- **`root:try-again`** holds the script to its cases, each over a
  stand-in command and a stand-in `sleep`, and **`nix:test`** holds the
  workflow to running each such step through it, by the workflow's text
  as it holds the jobs `flake` and `conventional`.

Not taken: running every failed step again, which runs a failing check
three times; a retrying action from the marketplace, which runs again
on any failure and is one more action fetched from GitHub; more of
nix's own attempts, whose pauses are seconds where GitHub asks for a
minute, and which reach neither cargo nor git; and the other steps that
download, from channels.nixos.org, cache.nixos.org, the Rust project
and Ubuntu, and the actions, which GitHub's runner and each action
fetch and retry themselves.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: CI runs each step that downloads from GitHub through
  `try-again.sh`, which runs it again where GitHub turned a download
  away for a moment, held by `root:try-again` and `nix:test`.

## Impact

- `try-again.sh`, new, and its cases, `check-try-again.sh`, in the root
  project's new task `try-again`; shellcheck and shfmt own both, as every
  `*.sh` of the tree (core.21).
- `.github/workflows/ci.yml`: the four steps, and its header.
- `nix/tests/workflow.nix`: the steps' test, in `nix:test`.
- No new crate, plugin or site part, so no new moon project, and no new
  tool: the script runs `sh`, `tee`, `tail`, `sed`, `grep` and `sleep`.
