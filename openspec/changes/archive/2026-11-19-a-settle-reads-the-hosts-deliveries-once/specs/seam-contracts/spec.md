## ADDED Requirements

### Requirement: The store gives a host document's version alone

The store seam SHALL give the version a host document stands at, none
where there is none, as a read of the document would give it, so a
reader holding the document at that version need not read it again
(102, 153).

#### Scenario: A host document's version read alone

- **WHEN** a host document's version is read before it is written, after another process wrote it, and after it is removed
- **THEN** the first and the last give none, and the second gives the version the write gave (102, 103, 153)
