## ADDED Requirements

### Requirement: The hibernate time is the launch's or the host's

An agent's `hibernate_after` SHALL be resolved at its start from the
layers `keep_open` is resolved from, taken whole from the first that
gives it of the launch's settings, the host's for the repository and
kind, then for the repository's every kind, the host's for the kind,
the host's for the place's stream, and the host's for every agent, a
stage's own first in each, else flume's own one hour; a repository's
settings and a kind's declaration SHALL give none (32, 158, 198, 212).

#### Scenario: From the host's kind

- **WHEN** the host gives `[kinds.research.agents] hibernate_after = "20m"`
- **THEN** a research place's talk agent starts with `hibernate_after` `20m` from `host-kind` (32, 212)

#### Scenario: No layer gives one

- **WHEN** no layer gives `hibernate_after`
- **THEN** the agent starts with `1h` from `default` (32, 212)

#### Scenario: Read at each start

- **WHEN** the host's `[agents] hibernate_after` changes from `1h` to `15m` while talk hibernates, and the place is resumed
- **THEN** the resumed agent starts with `15m` (32, 213)
