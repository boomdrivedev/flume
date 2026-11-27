## ADDED Requirements

### Requirement: Logic is tested as a unit

A test of flume's logic SHALL be a unit test in the crate that holds the
logic. It SHALL drive the logic through its seams and the fakes of
`flume-fakes`, or through the conformance world. A unit test SHALL start
no process, no git and no network. A unit test of a wait SHALL use the
fake host's clock or tokio's paused clock.

#### Scenario: A command's refusal of its words

- **WHEN** a test proves that `flume settle --every 0` is refused with words that name the bound
- **THEN** the test runs `commands::run` over a core of the conformance world in its own process, and starts no process

#### Scenario: A profile's call to its tool

- **WHEN** a test proves how the herdr profile reads a folder-trust question on an agent's screen
- **THEN** the test gives the profile a recording of herdr's answers, and starts no herdr

### Requirement: A small end-to-end set runs the real binary

Exactly one end-to-end test SHALL prove the main path of each command,
each seam with a real tool, and each walkthrough. The gate SHALL run at
most 45 end-to-end tests. Each end-to-end test SHALL end each process
that it starts.

#### Scenario: A command's main path

- **WHEN** the gate proves `flume list`
- **THEN** one test runs the built binary's `list` on a scratch repository, and no other end-to-end test has `list` as its main path

#### Scenario: A seam with a pinned tool

- **WHEN** the herdr profile reads a recording of herdr's answers in a unit test
- **THEN** the end-to-end test of the terminal seam runs the profile against the pinned herdr, and proves that recording

### Requirement: Each behaviour has exactly one proof

A behaviour SHALL be proven by exactly one test. A test that proves
again what another test proves SHALL go.

#### Scenario: A binary test that a conformance scenario already proves

- **WHEN** a test through the binary proves that `launch` cuts a place at origin's base after a fetch, and a conformance scenario also proves it
- **THEN** the test through the binary goes, and the conformance scenario stays its proof

### Requirement: The tools layer runs a bounded count of tests at one time

The `flume:tools` task SHALL run at most four tests at one time, so
that the processes of its pinned tools are bounded. Every other task
SHALL keep the share of the host's cores that a sluice run is handed
for its builds and tests (core.21).

#### Scenario: A gate run on a host that hands each run two cores

- **WHEN** sluice runs `flume:tools` with `SLUICE_JOBS` set to 2
- **THEN** cargo builds with two jobs, and libtest runs at most four tests at one time

## REMOVED Requirements

### Requirement: The tools layer runs its tests all at once

**Reason**: The tools layer holds only the end-to-end tests of the pinned tools, and their processes are bounded.

**Migration**: The requirement "The tools layer runs a bounded count of tests at one time" holds the task's count.
