## MODIFIED Requirements

### Requirement: settle --every keeps the host's watch

`flume settle --every <bound>` SHALL settle the places on the host
where something is pending, recording first the host's watch with its
bound and the command running it, and when its settle ended once it
has, the bound a duration as `flume wait --timeout` takes it; it SHALL
be refused, naming why, beside a place's name or with a bound that is
not a positive duration (61, 119, 208, 225).

#### Scenario: A watch over a scratch repository

- **WHEN** `flume settle --every 2m` runs on a scratch host holding one submitted place, and `flume status --json` runs after it
- **THEN** the settle stepped the place's base once, and status gives the host's `watch` with a bound of 120 seconds, `watch_settled` when the settle began and `watch_ended` when it ended (56, 208, 225)

#### Scenario: A watch with a place named

- **WHEN** `flume settle fix--one --every 2m` runs
- **THEN** it is refused naming `--every` and the place, nothing is settled, and no watch is recorded (208)
