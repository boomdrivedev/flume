## MODIFIED Requirements

### Requirement: The stop check, the guard and on-quiet through hooks

The profile SHALL offer send-back, the guard and on-quiet through hooks
(108, 110, 112, 115, 116, 121, C.4). The Stop hook SHALL block a stop
with the core's stop-check words. When it lets the agent stop, it SHALL
ask `flume host` on its socket to settle the place once its agent is
quiet. It SHALL start no process (nothing-detached,
a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place).
The PreToolUse hook SHALL refuse a shell command that runs `sluice
submit`, `wt submit`, `wt merge` or `git push`. Its words SHALL say that
flume submits the place, and it SHALL record the refusal, which status
reads. It SHALL never refuse `git merge` into the place's branch.

#### Scenario: An agent that would stop with nothing made

- **WHEN** the Stop hook runs for a code stage's agent with no commit since its start
- **THEN** it blocks the stop with words naming commits and `flume needs` (S24)

#### Scenario: An agent tries to push

- **WHEN** the PreToolUse hook gets `git add -A && git push origin HEAD`
- **THEN** it refuses the command, and status lists the refused act (S25)

#### Scenario: An agent merges the base for a conflict

- **WHEN** the PreToolUse hook gets `git fetch origin && git merge 4f2a9c1` in the place
- **THEN** it allows the command and records no refusal (112)

#### Scenario: A conversation's agent stops

- **WHEN** the Stop hook runs for a conversation's agent
- **THEN** it lets it stop and blocks nothing (109)

#### Scenario: The stop's settle is flume host's

- **WHEN** the Stop hook lets a code stage's agent stop with its deliverable made, and `flume host` runs
- **THEN** no process that the hook started runs when the hook ends, and flume host settles the place: the next stage starts

#### Scenario: No flume host answers

- **WHEN** the Stop hook lets an agent stop, and nothing answers on flume host's socket
- **THEN** the hook lets the stop through, starts no process and settles nothing
