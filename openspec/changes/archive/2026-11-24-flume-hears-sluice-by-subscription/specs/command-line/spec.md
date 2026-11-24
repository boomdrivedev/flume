## ADDED Requirements

### Requirement: host runs the host's resident

`flume host --every <bound>` SHALL run flume's resident until a SIGTERM
or a SIGINT, its bound a minute by default; it SHALL be refused, naming
why, with a bound that is not a positive duration, and while another
`flume host` runs on the host (208; record 0082).

#### Scenario: No bound

- **WHEN** `flume host --every 0` or `flume host --every soon` runs
- **THEN** it is refused, building no core

#### Scenario: A second resident

- **WHEN** a resident holds the host's lock in flume's state folder and a second takes it
- **THEN** the second is refused saying flume host already runs, and takes it once the first ended
