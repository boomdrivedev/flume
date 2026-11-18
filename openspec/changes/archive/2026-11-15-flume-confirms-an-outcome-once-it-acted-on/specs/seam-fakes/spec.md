## ADDED Requirements

### Requirement: The fake merge queue keeps an outcome until it is confirmed

The fake merge queue SHALL give each outcome it holds for a place with
the submission it decides under the key `<id>:<state>` at every inbox
until a confirm of that key, logged as `confirm <key>`; SHALL keep a
conflict it answers at submit owed to the place under its key; SHALL
leave an outcome of a named submission pending when told; and SHALL
answer every confirm unread while told to refuse confirms (133, 225).

#### Scenario: A landing given until it is confirmed

- **WHEN** a submission lands and the inbox is read twice, the landing confirmed by its key, and the inbox read again
- **THEN** both reads give the landing under `<id>:landed`, the confirm answers confirmed, and the last read gives nothing

### Requirement: The fake store refuses every write of a place while told

The fake store SHALL refuse every write of a place it is told to, as
moved, writing nothing, until it is told to take them again (75, 102).

#### Scenario: A place's writes refused

- **WHEN** every write of a place is refused and a settle acts on a failed submission
- **THEN** nothing of the place is written, and once its writes are taken again the next settle acts on it
