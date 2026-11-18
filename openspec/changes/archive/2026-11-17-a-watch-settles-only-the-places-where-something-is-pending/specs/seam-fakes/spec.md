## MODIFIED Requirements

### Requirement: The fake merge queue keeps an outcome until it is confirmed

The fake merge queue SHALL give each outcome it holds for a place with
the submission it decides under the key `<id>:<state>` at every inbox
until a confirm of that key, logged as `confirm <key>`; SHALL keep a
conflict it answers at submit owed to the place under its key; SHALL
leave an outcome of a named submission pending when told; and SHALL
answer every confirm unread while told to refuse confirms (133).

#### Scenario: A landing given until it is confirmed

- **WHEN** a submission lands and the inbox is read twice, the landing confirmed by its key, and the inbox read again
- **THEN** both reads give the landing under `<id>:landed`, the confirm answers confirmed, and the last read gives nothing
