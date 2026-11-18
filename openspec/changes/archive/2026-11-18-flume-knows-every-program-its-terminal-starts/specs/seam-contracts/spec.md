## ADDED Requirements

### Requirement: The agent seam takes a model or refuses it, and names its program's kind

The agent seam SHALL say whether its program takes a name as its
model, refusing every name by default in words saying its profile names
no way to give a model; SHALL name its program's kind, by which a
terminal that starts a known agent program names it, none by default;
and its command SHALL pass the model the settings hand it as its
profile's way gives it, and none where they give none (88, 170, 172).

#### Scenario: A profile that keeps the seam's defaults

- **WHEN** an agent profile that keeps the seam's defaults is asked of the model `large` and its kind
- **THEN** it refuses `large` in words naming it, and names no kind (170, 172)

#### Scenario: The fake's models over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent, once taking every model and once scripted to take none
- **THEN** the first takes `large` and its command passes `--model large`, the second refuses `large` in the seam's default words, and each names the kind `agent` (88, 172)

### Requirement: The terminal seam says whether it starts a kind

The terminal seam SHALL say whether it can start a program of a kind
now, under the login environment with a given environment over it,
starting every kind by default, a refusal in its tool's words (4,
C.5).

#### Scenario: A terminal that keeps the seam's default

- **WHEN** a terminal profile that keeps the seam's default is asked whether it starts `agent` and `amp`
- **THEN** it starts each (C.5)

### Requirement: The settings carry a model and an effort per program

`Settings` SHALL carry its model and its effort each as one value or a
table of programs, each taken whole by `Settings::under`, the agent's
entry its program's, serialized to JSON as the string or the object it
was written as and read back unchanged; a held state written before
SHALL read its model and effort as it did, and a table that names no
program SHALL be refused where it is read (32, 172, 197).

#### Scenario: A held state with a model's table and an effort's

- **WHEN** a held state whose launch gives the model `{ agent = "large", other = "small" }` and the effort `{ agent = "high" }`, and whose kind gives the model `medium`, is written and read
- **THEN** it reads back unchanged, the entry for `agent` is `large` and for `nonesuch` none, and a state written with the model `"medium"` alone reads it as one value (32, 172, 197)

#### Scenario: A table that names no program

- **WHEN** a model is read from the JSON object `{}`
- **THEN** it is refused, saying the table names no program (172)
