## ADDED Requirements

### Requirement: CI tries a download GitHub turns away again

CI SHALL run each step that downloads from GitHub, the devenv shell's
inputs, the flake's, the nixpkgs convco comes from and the crates of
weir, through `try-again.sh`. `try-again.sh` SHALL run its command
again where it fails and the last lines of its output say HTTP 429 or a
5xx, or GitHub's rate limit, up to three runs in all, pausing a minute
before the second and two before the third, and SHALL exit with the
command's own status at once on any other failure and on the third
refusal.

#### Scenario: A download GitHub turned away once

- **WHEN** `root:try-again` runs `try-again.sh` over a stand-in whose first run fails saying nix's `HTTP error 429` and GitHub's "rate-limited", and whose second passes
- **THEN** the stand-in runs twice, `try-again.sh` pauses 60 seconds between, says it was turned away at run 1 of 3, and exits 0

#### Scenario: A download GitHub turned away at every run

- **WHEN** `root:try-again` runs `try-again.sh` over a stand-in that fails each run with status 7 saying GitHub's "API rate limit exceeded"
- **THEN** the stand-in runs three times, `try-again.sh` pauses 60 and then 120 seconds, says it was turned away at each of 3 runs, and exits 7

#### Scenario: A failure that is no refusal

- **WHEN** `root:try-again` runs `try-again.sh` over a stand-in that fails a check, one that fails with GitHub's `HTTP error 403` and no rate limit, and one that fails a check forty lines after a refusal nix tried again
- **THEN** each stand-in runs once, `try-again.sh` pauses not at all, and exits with the stand-in's status

#### Scenario: The workflow's downloads from GitHub

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** every step of the jobs `fast` and `flake` that enters the devenv shell, the job `conventional`'s `nix shell` of convco and the job `binaries`' `cargo build` each run through `sh try-again.sh`
