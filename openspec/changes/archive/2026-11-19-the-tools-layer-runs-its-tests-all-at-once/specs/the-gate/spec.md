## ADDED Requirements

### Requirement: The tools layer runs its tests all at once

The `flume:tools` task SHALL run its tests as many at once as libtest
takes on the host, never the share of the host's cores a sluice run is
handed, since its tests wait on the pinned tools' sessions and the
stand-in rather than on a processor; every other task SHALL keep that
share for its builds and tests (core.21).

#### Scenario: A gate run on a host that hands each run two cores

- **WHEN** sluice runs `flume:tools` with `SLUICE_JOBS` set to 2
- **THEN** cargo builds with two jobs and libtest runs the tests with no `RUST_TEST_THREADS`, as many at once as the host has processors
