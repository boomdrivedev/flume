## MODIFIED Requirements

### Requirement: The fake agent's parts follow its scripts

The fake agent SHALL build each part its scripts give it and degrade
each other: each of received, asking, own work, ended and last words
unless scripted to read it from its terminal, then answering the lesser
way; send-back, guard and on-quiet unless scripted off; windows unless
scripted to report none; spent always; the mode, carry and own settings
once scripted. It SHALL pass the agent seam's parts scenarios under each
of its harnesses (106, 171, 174).

#### Scenario: The fake scripted as each harness scripts it

- **WHEN** the fake agent is scripted as its harness does by default, then to carry none, take no own settings, report no windows, offer no send-back, guard or on-quiet, read asking alone from its terminal, and read the five from its terminal
- **THEN** its parts build every part by default, and each script degrades its own parts alone: carry, own settings, windows, send-back, guard and on-quiet together, asking alone, and received, asking, own work, ended and last words together, each of those it degrades then answering unknown, or none for last words (171, 174)

#### Scenario: The fake as a new fake makes it

- **WHEN** a fake agent no test has scripted is asked its parts
- **THEN** received, asking, own work, ended, last words, send-back, guard, on-quiet, windows and spent are built, and the mode, carry and own settings degraded (174)

## ADDED Requirements

### Requirement: The fake terminal scripts a watched send's turn

The fake terminal SHALL deliver a watched send as its send delivers
text, refuse it where its send refuses, and answer that it saw the turn
begin unless a test scripted another answer for that agent, a turn not
seen with words or a refusal, each asking logged `send-watching
<identity>` (106, 171, C.5).

#### Scenario: A turn seen, then not

- **WHEN** a test sends an agent `go` by the fake terminal's watched send, scripts that agent's turn not seen with words and sends `again`, and sends to a stopped agent
- **THEN** the first answers the turn seen, the second the turn not seen in those words, both texts are delivered and logged, and the third is refused with nothing delivered (171, C.5)
