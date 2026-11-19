## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or
removes SHALL change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names boomdrivedev/blueprints at 3a3b287; Parts A and B hold every clause that revision states; the scenarios run to S131; and it names as unbuilt what it named before this change but 223, 224 and S130, which this change builds and its scenarios cite
