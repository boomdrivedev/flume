## MODIFIED Requirements

### Requirement: A wait drives no settle while a caller keeps watch

While a caller keeps watch over the host, a wait SHALL settle nothing
and read only the host's state. While none does, it SHALL settle the
places where something is pending before each reading, as every acting
command does, whatever terminal the host binds (119, 122, 208, 225;
core.13).

#### Scenario: Under a watch

- **WHEN** a wait follows a place while a caller keeps watch
- **THEN** the wait makes no call to any seam but the store, and the place moves on only by the caller's settles

#### Scenario: Attended under any terminal

- **WHEN** a wait follows a place while no caller keeps watch
- **THEN** the wait settles the places where something is pending before each reading (119, 208, 225)
