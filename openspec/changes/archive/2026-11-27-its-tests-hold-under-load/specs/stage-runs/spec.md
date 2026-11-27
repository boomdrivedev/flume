## ADDED Requirements

### Requirement: A receipt is recorded only while its start runs

A settle SHALL record a prompt's receipt, or the agent's word that it
started, only while the start is live and its stage still starts. Thus
a settle never writes the stage begun over an end that another settle
recorded after the first settle read the stage (35, 38, 75).

#### Scenario: A receipt read before another settle ended the start

- **WHEN** a settle reads code's prompt received, and another settle ends code lost before the first settle writes
- **THEN** the first settle writes no receipt, and code stays lost with its start not live (35, 38, 75)

### Requirement: A stop-short records its prompt's receipt

Where no settle recorded a start's prompt received, the core SHALL read
the receipt from the agent when it takes the agent's stop-short. It
SHALL record the receipt with the end. The agent's deliver part then
takes the answer that the agent waits for (38, 42, 215).

#### Scenario: A stop-short before any settle read the prompt received

- **WHEN** a code agent with deliver takes its prompt and stops short before any settle reads the receipt, and a retry answers `notes.md`
- **THEN** the stop-short records the receipt, the deliver part takes the answer, and code reads begun under its first start (S4, 38, 42, 215)
