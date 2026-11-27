## MODIFIED Requirements

### Requirement: A prompt is received or reported never begun

The core SHALL send a prompt only once its agent waits and does not
ask. Within 60 seconds of each sending, it SHALL read whether the agent
received the prompt. Where the terminal can send, it SHALL send once
more. Where the agent received no sending, it SHALL report the stage
never begun (37, 38). A settle SHALL record a receipt, or the agent's
word that it started, only while the start is live and its stage still
starts. Thus a settle never writes the stage begun over an end that
another settle recorded after the first settle read the stage (35, 38,
75).

#### Scenario: A slow start and a second sending

- **WHEN** the fake agent takes four minutes to come up, then misses the first sending
- **THEN** its start reads unknown until it waits, the prompt is sent once more, it is received, and the stage reads begun (S5, 35)

#### Scenario: A terminal that cannot send

- **WHEN** the fake terminal refuses every send and the agent never records the prompt
- **THEN** the start carries the one sending and the stage reads never begun (38)

#### Scenario: A receipt read before another settle ended the start

- **WHEN** a settle reads code's prompt received, and another settle ends code lost before the first settle writes
- **THEN** the first settle writes no receipt, and code stays lost with its start not live (35, 38, 75)

### Requirement: A stop-short ends the stage short

The core SHALL take a stop-short only under two conditions. The
identity is that of the live agent of the place's current stage. The
secret is that start's secret. The stop-short SHALL say whether it needs
an answer or found work. The core SHALL end the stage short with the
agent's words whole and its kind. It SHALL start no next stage, but for
one case. A found from the kind's last stage starts its fix round where
that round comes before it. A stop-short after an answer taken in its
session SHALL be an end of its own. Its key SHALL differ from the key of
the end that the answer answered (41, 42, 43, 131, 198). Where no
settle recorded the prompt received, the core SHALL read the receipt
from the agent when it takes the stop-short. It SHALL record the receipt with the end. The agent's
deliver part then takes the answer that the agent waits for (38, 42,
215).

#### Scenario: A construct stops short

- **WHEN** a construct's agent stops short needing an answer, with its words
- **THEN** the stage ends short, no code stage starts, and status gives the words whole and that it needs an answer (S4)

#### Scenario: A stop-short with another start's secret

- **WHEN** a stop-short carries the place's identity and the secret of an earlier start
- **THEN** it is refused and the stage goes on (41)

#### Scenario: A second stop-short in one session

- **WHEN** a code agent answered in its session stops short again with other words
- **THEN** a second end of code's first start is recorded under its own key and delivered once to each recipient. The first end stays delivered. The agent is kept open for `keep_open` from the second stop-short (43, 74, 198)

#### Scenario: A stop-short before any settle read the prompt received

- **WHEN** a code agent with deliver takes its prompt and stops short before any settle reads the receipt, and a retry answers `notes.md`
- **THEN** the stop-short records the receipt, the deliver part takes the answer, and code reads begun under its first start (S4, 38, 42, 215)
