## MODIFIED Requirements

### Requirement: A change read through OpenSpec's files

The change profile SHALL read a change at a revision from its files:
whether it exists, its tasks each ticked or not, and whether it is
archived; whether its planning is complete it SHALL read as the
requirement "Planning complete read through OpenSpec's status" says.
Reading SHALL write nothing to the place (86, 87, C.3).

#### Scenario: Seam scenarios over the profile

- **WHEN** the change seam's scenarios run against the profile over a scratch repository's changes
- **THEN** each passes, the same revision giving the same answer

#### Scenario: Agreement with OpenSpec's own reader

- **WHEN** a scratch change is read at each of five revisions, from created to archived
- **THEN** existence, planning complete and the tasks agree with what the pinned `openspec status` gives at the same revision (59)

## ADDED Requirements

### Requirement: Planning complete read through OpenSpec's status

The change profile SHALL read whether a standing change's planning is
complete through `openspec status --change <name> --json` in a checkout
of the revision, whatever the change's schema, and SHALL count none of
the change's files for it: a change `openspec status` reads no such
change of SHALL read not planned, and an archived change that no longer
stands SHALL read planned (59, C.3).

#### Scenario: A change that skips specs is planned

- **WHEN** a scratch spec-driven change whose `.openspec.yaml` declares `skip_specs: true` holds its proposal, design and tasks and no delta spec
- **THEN** read-change gives its planning complete, as the pinned `openspec status` gives it at the same revision (59)

#### Scenario: A change with no delta spec and no marker is not planned

- **WHEN** the same change's `.openspec.yaml` declares no `skip_specs`
- **THEN** read-change gives its planning not complete, as the pinned `openspec status` gives it at the same revision (59)

#### Scenario: A change OpenSpec cannot read is not planned

- **WHEN** the same change's `.openspec.yaml` gives `skip_specs` a value that is no boolean, which the pinned `openspec status` refuses as invalid metadata
- **THEN** read-change gives the change standing and its planning not complete
