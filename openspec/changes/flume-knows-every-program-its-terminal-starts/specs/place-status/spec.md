## ADDED Requirements

### Requirement: Status gives an agent's model

Status SHALL give the current agent's model as its layer wrote it, a
table of programs as written, with the layer it came from, the program
whose entry of a table the agent took, none for a name given whole, and
the name its command passed; an agent whose settings give no model
SHALL be given none, in the JSON form and the person's (56, 172;
core.14).

#### Scenario: A model given whole

- **WHEN** status is asked of a place whose agent took `large` from `--agent model=large`
- **THEN** the agent gives the model `large` from `launch`, no entry and `large` passed, in the JSON and the person's form (56, 172)

#### Scenario: A table's entry

- **WHEN** status is asked of a place whose agent took the `agent` entry of `{ agent = "large", other = "small" }` from `host-kind`
- **THEN** the agent's model reads the table as written from `host-kind`, the entry `agent` and `large` passed, and the person's form names the entry and the name passed (56, 172)

#### Scenario: No model

- **WHEN** status is asked of a place whose agent's settings give no model
- **THEN** the JSON form holds no `model` for the agent beyond its settings', and the person's form names none (172)

## MODIFIED Requirements

### Requirement: Status gives an agent's effort

Status SHALL give the current agent's effort as its layer wrote it,
with its layer, the program whose table entry it took, the value
passed, and that it was not read back; none where its settings give
none. Where the agent's account's environment names the variable its
program reads over the effort, status SHALL name that variable as
outranking it, never its value, in the JSON form and the person's (56,
197; core.30).

#### Scenario: An effort from the launch

- **WHEN** status is asked of a place whose agent took `xhigh` from `--agent effort=xhigh`
- **THEN** the agent's settings give the effort `xhigh` from `launch`, passed and not read back, in the JSON and the person's form (S103, 56)

#### Scenario: An effort's table entry

- **WHEN** status is asked of a place whose agent took the `agent` entry of `{ agent = "high", other = "max" }` from `kind`
- **THEN** the agent's effort reads the table as written from `kind`, the entry `agent` and `high` passed, and the person's form names the entry (56, 197)

#### Scenario: An account's variable that outranks it

- **WHEN** status is asked of a place whose agent took `high` from `host-kind` and whose account's `env` names the variable the fake names as read over its effort
- **THEN** the agent's effort names that variable as outranking it, and no value of it (56, 197)

#### Scenario: No effort

- **WHEN** status is asked of a place whose agent's settings give no effort
- **THEN** the agent's settings give no effort, and the JSON form holds no `effort` for it (197)
