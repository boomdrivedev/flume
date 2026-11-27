## MODIFIED Requirements

### Requirement: flume host serves status and a subscription to it

After the hello, `flume host` SHALL answer `status` with one `document`
line and close the connection. The line holds the answer that `flume
--json status` prints. `flume host` SHALL answer `subscribe` with a
`document` at once. It SHALL send another each time the answer differs
from the last one sent, until the client closes the connection. Each
ask MAY name a place and ask each place's line. A line that flume host
cannot read SHALL get one `error` line, and the connection SHALL close
(flume-host-serves-status-and-its-subscription-on-its-socket).

#### Scenario: A status

- **WHEN** a client says its hello and asks `{"ask":"status","line":true}`
- **THEN** flume host answers its version, protocol and pid, then one `document` holding the answer, and closes the connection

#### Scenario: A subscription

- **WHEN** a client subscribes, the answer stays the same at one change, and then differs at five changes made together
- **THEN** the client gets the first document, nothing at the change with the same answer, and one document for the five changes

#### Scenario: An ask it does not know

- **WHEN** a client asks `{"ask":"tidy"}`
- **THEN** flume host answers one `error` line naming the line, and closes the connection

## ADDED Requirements

### Requirement: flume host settles the place a hook asks for

After the hello, `flume host` SHALL answer `settle` with one `taken`
line and close the connection. The ask names a place by its location.
Then flume host SHALL settle that place itself. A settle that started
SHALL run to its end before flume host ends (121, one-resident-per-tool,
nothing-detached, workers-end-with-their-owner,
a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place).

#### Scenario: A settle with no wait

- **WHEN** a client asks `{"ask":"settle","place":"/p"}`
- **THEN** flume host answers `taken`, closes the connection, and settles `/p` one time without a read of its agent

### Requirement: flume host reads the agent's quiet on its own clock

Where a `settle` ask says `on_quiet`, `flume host` SHALL first read the
place's agent until it is quiet, for up to 60 seconds by its own clock.
Then it SHALL settle the place as the quiet of its agent starts a
settle. A stop of flume host during that read SHALL end the read, and
flume host SHALL not settle the place (121, workers-end-with-their-owner).

#### Scenario: A settle asked once the agent is quiet

- **WHEN** a client asks `{"ask":"settle","place":"/p","on_quiet":true}`, and the agent is quiet at the third read
- **THEN** flume host answers `taken` and closes the connection, reads the agent three times one second apart, and then settles `/p` one time

#### Scenario: An agent that never goes quiet

- **WHEN** a client asks to settle `/p` on its quiet, and the agent is never quiet
- **THEN** flume host settles `/p` one time, 60 seconds after the ask by its own clock

#### Scenario: A stop during the read of the quiet

- **WHEN** a stop comes to flume host while it reads the agent of a place that a client asked to settle on its quiet
- **THEN** flume host ends with no settle of that place
