## ADDED Requirements

### Requirement: Each outcome is confirmed once acted on

A settle SHALL confirm each outcome its inbox or a decided submit gives,
through the merge queue seam's confirm, once the place's state holds what
it decided, so a later settle is given only what is new; an outcome of a
submission other than the one flume last made SHALL decide nothing and
be confirmed with no act, and one whose act was not written SHALL be left
unconfirmed for the next settle (111, 112, 124, 225).

#### Scenario: A second settle is given no outcome already acted on

- **WHEN** a fix's submission fails, a settle sends code back, and the place is settled again
- **THEN** the first settle confirms the failure once code's start is held, and the second reads the inbox, is given nothing, confirms nothing and sends nothing back (111, 225)

#### Scenario: An earlier submission's outcome given again

- **WHEN** the fix is submitted anew and the inbox gives the first submission's failure again, as a flume that never confirmed it left it
- **THEN** the settle sends nothing back, holds no verdict for the open submission, and confirms the earlier failure (99, 225)

#### Scenario: An outcome not acted on, or not confirmed

- **WHEN** the second submission fails while every write of the place is refused, then a settle acts on it while the merge queue refuses confirms, then a settle runs with confirms taken
- **THEN** the first settle confirms nothing and leaves the failure pending, the second sends code back once and leaves it pending, and the third sends nothing back again and confirms it (74, 124, 225)
