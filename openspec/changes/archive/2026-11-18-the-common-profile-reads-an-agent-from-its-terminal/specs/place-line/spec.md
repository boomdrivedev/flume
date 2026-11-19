## MODIFIED Requirements

### Requirement: A line asked alone reads only what it holds

A line asked alone SHALL read the place's held state, the change at the place's head where its kind has a change, and the live agent's asking mark from its profile where its profile builds asking, and nothing else: no terminal operation, no merge-queue operation and no host reading. It SHALL write nothing and start nothing, and a change it cannot read SHALL leave the tasks and the group out (57, 58, 171).

#### Scenario: Nothing called and nothing written

- **WHEN** a place's line is asked alone over the fakes while its code agent works
- **THEN** the fake terminal and the fake merge queue record no call, and the fake store's version of the place is unchanged (57)

#### Scenario: A change that cannot be read

- **WHEN** a change@2 place's line is asked while the fake change cannot read its change
- **THEN** the line is given with no tasks part and the stage's name alone (58)

## ADDED Requirements

### Requirement: The line reads asking from the held state where the profile reads none

Where the live agent's profile does not build asking, the line's state
SHALL be `asking` where it would be for a profile's asking mark, while
the last settle of its stage's end read the agent asking, from the
place's held state and from no terminal (90, 171; core.52).

#### Scenario: An agent whose terminal reads it asking

- **WHEN** a place's begun code agent reads the five from its terminal, and a settle reads it asking, then a later settle reads it working
- **THEN** its line's state is `asking` after the first settle and `working` after the second, the line asked alone calling no terminal (90, 171; core.52)
