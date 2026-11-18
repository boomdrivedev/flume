## MODIFIED Requirements

### Requirement: Status gives the host's watch

Status SHALL give, for the host, the watch a caller keeps: its bound in
the host's `watch`, a bound or none, when its newest settle began as
`watch_settled`, when that settle ended as `watch_ended`, none while it
runs, and, while flume runs attended after a watch lapsed, when it
lapsed as `watch_lapsed`. The person's form SHALL say whether a caller
keeps watch, with its bound and when it last settled, or since when
none has (56, 61, 119, 208, 225; core.14).

#### Scenario: A watched host and an unwatched one

- **WHEN** a caller settled with a bound of two minutes one minute ago, and status is asked; then five minutes pass with no such settle, and status is asked again
- **THEN** the first gives `watch` with 120 seconds, `watch_settled` and `watch_ended`, and the person's form prints `a caller keeps watch: the places where something is pending settle every 2m, last at <time>`; the second gives `watch_lapsed` four minutes after that settle ended, and prints `no caller keeps watch since <time>: the places where something is pending settle at each command of flume's that acts` (119, 208, 225, S118)
