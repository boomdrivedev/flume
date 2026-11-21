# clause-coverage Specification

## Purpose
Keeps the conformance suite honest about the statement: every scenario
cites the clauses it exercises, and a check in the gate fails when a
clause, an invariant or a scenario of the statement is cited by nothing
(106, section 12).

## Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or
removes SHALL change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names boomdrivedev/blueprints at 3a3b287, the revision that states how flume runs statechart kinds; Parts A and B hold every clause that revision states, 175 to 177 in Part A and 178 in Part B; the scenarios run to S131; and it names as unbuilt what it named before these changes but 173, S71, 175 to 178, S76 and S77, which they build and their scenarios cite; S72 among it is named by record 0024's plan stage's change beside 163, 164 and S68, and nothing is named by record 0026's change or by records 0028 and 0029's

### Requirement: Every scenario cites its clauses

Every scenario of the suite, a seam's or the engine's, SHALL cite the
clauses it exercises, and the check SHALL fail naming a scenario that
cites nothing or cites a number that is no clause, invariant or
scenario of the statement.

#### Scenario: A citation of no clause

- **WHEN** a scenario cites 100, which the statement does not hold
- **THEN** the check fails naming the scenario and 100

### Requirement: A clause cited by nothing fails the suite

The coverage check SHALL fail naming every clause of Part A and Part B,
every invariant and every scenario of section 11 that no scenario of
the suite cites, but for those the list names as built by no change yet,
and SHALL fail naming any of those a scenario cites.

#### Scenario: A clause left out

- **WHEN** the check runs over scenarios that between them leave clause 70 out
- **THEN** it fails naming 70

#### Scenario: The suite as this change leaves it

- **WHEN** the coverage check runs over the suite as this change lands it
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario the list holds but those named as unbuilt is cited, 173 and S71 by the engine's scenarios of a pool's program

### Requirement: Seam scenarios run against any profile

Each seam's scenarios SHALL be generic over that seam's interface, so a
profile unit runs the same scenarios against its real tool as the fake
passes, and SHALL sit below one marker per seam where a profile unit
adds scenarios of its own (106).

#### Scenario: The place scenarios over another implementation

- **WHEN** the place scenarios are run against a second implementation of the place seam that admits a make over an existing branch
- **THEN** the scenario citing 83 fails

### Requirement: A profile's scenarios run side by side

The conformance suite SHALL offer a runner that takes a profile's
scenarios a bounded number at a time, each over a fresh harness made in
its own thread, and that names every scenario that failed in the
scenarios' order, as the one-at-a-time runner names them; a profile
whose harnesses share nothing, each on a server and in a scratch of its
own, SHALL run its scenarios on it (106).

#### Scenario: The herdr profile's terminal scenarios

- **WHEN** the herdr profile's terminal scenarios run in the tools layer
- **THEN** they run five at a time, each on a herdr server of its own, and a failure is named with its scenario's name in the scenarios' order (106)

### Requirement: A harness waits only for what a scenario awaits

The terminal harness SHALL give what a recipient's session took once it
took the number of inputs the scenario awaits, or by the harness's wait
where it takes fewer, and at once where none is awaited; and SHALL give
the seam a never-up program is opened on, its own by default or one
with the shortest bound on a start its terminal takes (43, 94, 95, 106).

#### Scenario: A session at work

- **WHEN** the scenario of a session at work reads what the session took before its send
- **THEN** it reads at once, and after the refused send it awaits one more input for the harness's wait and finds none (54, 95)

#### Scenario: A program that never comes up under herdr

- **WHEN** the herdr harness is asked for the seam a never-up program is opened on
- **THEN** it gives the profile with a start bound of 3,001 milliseconds, and the open answers not come up (35, 94)
