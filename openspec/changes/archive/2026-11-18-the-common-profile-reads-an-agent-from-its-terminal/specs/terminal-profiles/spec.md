## ADDED Requirements

### Requirement: The core reads herdr's blocked as asking for a profile that reads asking from the terminal

Where the agent's profile does not build asking, the core SHALL read
herdr's blocked, which the herdr profile reads as asking, as the agent
asking, with no mark of its profile's, and type nothing into the pane
(90, 96, 171).

#### Scenario: Blocked for a profile that reads asking from its terminal

- **WHEN** herdr reads a pane blocked whose agent's profile does not build asking
- **THEN** the core reads the agent asking, and types nothing into the pane (90, 171)

### Requirement: herdr's watched send waits on the turn

herdr's watched send SHALL clear the pane's held input with `agent
send-keys <pane> ctrl+c`, then send `agent prompt <pane> <text> --wait
--until working --until blocked --timeout <ms>`, the bound it is given
in milliseconds, within that bound and the profile's own past it (89,
95, 171, C.5; core.29).

#### Scenario: A turn seen

- **WHEN** an agent opened through a scratch launcher under the pinned herdr, a stand-in reported to herdr as `amp` that tells herdr it works as soon as it reads a line, is sent `go` by the watched send
- **THEN** it answers the turn seen, and the stand-in read `go` (89, 171, C.5)

### Requirement: herdr reads the turn from its answer

herdr's watched send SHALL answer the turn seen on herdr's answer; the
turn not seen, in herdr's words, on `agent_prompt_stalled` or
`timeout`; and a refusal, in herdr's words, on any other refusal, a
pane gone, or no answer within its bound (38, 171, C.5; core.29).

#### Scenario: No turn seen

- **WHEN** an agent opened through a scratch launcher under the pinned herdr, a stand-in reported to herdr as `amp` that reads a line and tells herdr nothing, is sent `go` by the watched send, and then `again`
- **THEN** each answers the turn not seen, naming `agent_prompt_stalled`, the held input cleared before the second, and nothing is left typed (38, 95, 171, C.5)
