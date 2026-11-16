## MODIFIED Requirements

### Requirement: A wait names whom it waits on

A wait's span SHALL name whom the place waits on and for which move:
its caller, for a hold, an answer, a lost stage, a prompt never begun,
a readiness that failed, a last stage that does not land, a landing the
forge refused, or any other move status gives; the host, for room; the
merge queue, from the submission to its landing, its refusal or its
landing held by the forge; or a person, while the agent asks or its
program shows a dialog (217; 70, 111, 112, 130).

#### Scenario: A wait on the caller for an answer

- **WHEN** a start stops short needing an answer, the place waits forty minutes, and a retry is taken
- **THEN** the trace holds a span on the caller for an answer of forty minutes, from the settle that read the stop-short to the retry (217, S129)

#### Scenario: A wait on the merge queue

- **WHEN** a place's last stage delivers, is submitted, and lands fifty minutes later
- **THEN** the trace holds a span on the merge queue from the settle that submitted it to the one that read it landed, naming that it landed (217, S129)

#### Scenario: A wait for room

- **WHEN** a place's next stage waits for its stream's room and starts at a later settle
- **THEN** the trace holds a span on the host for room, from the settle that first left it waiting to the one that started it (217, 70)

#### Scenario: A person while the agent asks

- **WHEN** a stage's agent asks a question of its terminal's person and is answered at a later settle
- **THEN** the trace holds a span on a person from the settle that read it asking to the one that read it working (217, 90)

#### Scenario: A landing the forge refused

- **WHEN** a place's submission is read held at a settle and back in the queue at a later one
- **THEN** the merge queue's wait ends at the first, naming that it was held, a wait on the caller for `forge_refused` runs from the first to the second, and a wait on the merge queue begins again at the second, from the submission's return to the queue (217, 112, 130)
