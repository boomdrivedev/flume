## ADDED Requirements

### Requirement: Releases take turns and read their version as they start

The runs of `ci.yml` for pushes to main SHALL take turns in the
concurrency group `release`, and none SHALL cancel a run in progress.
No job SHALL name a group of its own. The job `version` SHALL run
`next-version.sh` on the tags present as its run starts. It gives the
next version after each release, or nothing where a release already
holds each commit of the run.

#### Scenario: One push

- **WHEN** `root:next-version` runs `next-version.sh` at a fix since the release `v0.1.0`
- **THEN** it prints `v0.1.1`

#### Scenario: Two pushes

- **WHEN** `root:next-version` runs `next-version.sh` at a second fix, after the release of the first fix pushed `v0.1.1`
- **THEN** it prints `v0.1.2`

#### Scenario: A late run

- **WHEN** `root:next-version` runs `next-version.sh` at the first fix, after the release of the second fix pushed `v0.1.2`
- **THEN** it prints nothing

#### Scenario: No bump

- **WHEN** `root:next-version` runs `next-version.sh` at a chore since the newest release
- **THEN** it prints nothing

#### Scenario: The group of the workflow

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** the group of the workflow is `release` for a push and one for each run of a pull request, with `cancel-in-progress: false`
- **AND** no job names a group of its own
- **AND** the job `version` runs `sh next-version.sh` under the git-cliff of the nixpkgs that `devenv.lock` pins

#### Scenario: git-cliff at its pin

- **WHEN** `root:toolchain` runs in the devenv shell
- **THEN** it names git-cliff at the version that `devenv.nix` pins
- **AND** where git-cliff has another version, it fails and names git-cliff and the two versions
