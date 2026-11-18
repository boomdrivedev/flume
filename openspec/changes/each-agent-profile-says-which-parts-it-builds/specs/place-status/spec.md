## ADDED Requirements

### Requirement: Status names each degraded part of an agent

Status SHALL give, with the current agent, each part of the agent seam
its program's profile does not build, in 171's order, with how flume
reads it: in the JSON form as `degraded`, each with its `part` and
`read`, left out where none is, and in the person's form as a line for
each. It SHALL name no part degraded that the profile builds (56, 170,
174; core.14).

#### Scenario: A profile that offers no hooks

- **WHEN** status is asked of a place whose fake agent's profile offers no send-back, guard or on-quiet and builds every other part
- **THEN** the agent's `degraded` gives `send_back`, `guard` and `on_quiet` in that order, each with how flume reads it, and the person's form a line for each, as `send-back degraded: not offered, so it may go quiet with its deliverable missing` (56, 174)

#### Scenario: A profile that builds every part

- **WHEN** status is asked of a place whose agent's profile builds every part
- **THEN** the agent's JSON holds no `degraded`, and the person's form names no part degraded (56, 174; core.14)
