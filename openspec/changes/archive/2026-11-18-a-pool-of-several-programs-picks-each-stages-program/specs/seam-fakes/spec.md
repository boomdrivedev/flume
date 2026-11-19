## ADDED Requirements

### Requirement: The fake agent scripts its way to stop short

The fake agent SHALL give the way to stop short scripted for it, and
its own fixed words where none is scripted, so two fake programs give
two ways (41, 92).

#### Scenario: Two fake programs' ways

- **WHEN** the fake resolver binds `agent` to a fake agent scripted with no way to stop short and `other` to one scripted with its own words
- **THEN** `agent`'s profile gives the fixed words and `other`'s the scripted words, each the same for every start (41, 92)
