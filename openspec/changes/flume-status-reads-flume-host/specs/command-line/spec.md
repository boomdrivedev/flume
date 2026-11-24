## MODIFIED Requirements

### Requirement: host runs the host's resident

`flume host --every <bound>` SHALL run flume's resident until a SIGTERM
or a SIGINT, its bound a minute by default, serving status on its
socket meanwhile and removing the socket when it stops; it SHALL be
refused, naming why, with a bound that is not a positive duration, and
while another `flume host` runs on the host (208; record 0082;
flume-host-serves-status-and-its-subscription-on-its-socket).

#### Scenario: No bound

- **WHEN** `flume host --every 0` or `flume host --every soon` runs
- **THEN** it is refused, building no core

#### Scenario: A second resident

- **WHEN** a resident holds the host's lock in flume's state folder and a second takes it
- **THEN** the second is refused saying flume host already runs, and takes it once the first ended

## ADDED Requirements

### Requirement: status asks flume host

`flume status`, with or without `--json` and `--line`, and the MCP tool
`status`, SHALL ask the host's `flume host` on its socket and print its
document as `flume --json status` prints an answer, building no core and
starting no process. Where no `flume host` answers, they SHALL be
refused, naming the socket and the host's service manager
(flume-host-serves-status-and-its-subscription-on-its-socket;
the-service-manager-starts-each-resident).

#### Scenario: Asked through the resident

- **WHEN** a place is launched on a scratch host whose `flume host` runs, and `flume status <place> --json` is asked
- **THEN** the answer equals the core's own status of that place over the same state

#### Scenario: No flume host

- **WHEN** `flume status` is asked where nothing answers on flume host's socket
- **THEN** it is refused, its words naming the socket and the service manager
