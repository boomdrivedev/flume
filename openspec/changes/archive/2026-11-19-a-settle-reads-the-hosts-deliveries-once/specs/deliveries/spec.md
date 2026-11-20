## ADDED Requirements

### Requirement: An engine reads the host's deliveries once for each version

An engine SHALL read the host's deliveries document whole only where
its version moved since the engine last read or wrote it, reading the
version alone before each ask, and SHALL work out what the deliveries
owe once for each version and each second of the host's clock; a write
of the engine's SHALL leave it holding what it wrote at the version the
write gave, and a write refused as moved SHALL read the document again
before it asks again (43, 75, 225, C.7; core.15, core.17).

#### Scenario: A watch's settle over three pending places

- **WHEN** three fixes for `ops` are delivered while its pane asks, the pane then waits, and a watch's settle runs, then another with nothing written since, then another after a second command confirms one of the ends
- **THEN** the first settle hands every end to `ops` and reads the host's deliveries whole once, the second reads them whole none, and the third reads them whole once and sends no notice again (43, 75, 225)
