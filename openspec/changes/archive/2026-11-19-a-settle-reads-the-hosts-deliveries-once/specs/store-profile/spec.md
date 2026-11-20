## ADDED Requirements

### Requirement: A host document's version read from its opening bytes

The store profile SHALL read a host document's version alone from the
bytes the document opens with, `{"version":` and a whole number, as it
writes every host document, and SHALL read the document whole for its
version where its bytes open otherwise (153, C.7).

#### Scenario: A document's opening bytes

- **WHEN** a host document's opening bytes are pretty or compact JSON beginning with its version, or begin with another field, or end within the number
- **THEN** the first two give the version, and the others give none from the bytes, so the document is read whole for it (153)
