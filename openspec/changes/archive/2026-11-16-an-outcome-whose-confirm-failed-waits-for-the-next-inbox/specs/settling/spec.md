## MODIFIED Requirements

### Requirement: Each outcome is confirmed once acted on

A settle SHALL confirm each outcome its inbox or a decided submit gives,
through the merge queue seam's confirm, once the place's state holds what
it decided; an outcome of a submission other than the one flume last made
SHALL decide nothing and be confirmed with no act; one whose act was not
written, or whose confirm the merge queue could not take, SHALL be left
unconfirmed for the next inbox, acted on there no more than once (74,
111, 112, 124, 225).

#### Scenario: A second settle is given no outcome already acted on

- **WHEN** a fix's submission fails, a settle sends code back, and the place is settled again
- **THEN** the first settle confirms the failure once code's start is held, and the second asks the inbox nothing, confirms nothing and sends nothing back (111, 133, 225)

#### Scenario: An earlier submission's outcome given again

- **WHEN** the fix is submitted anew and the inbox gives the first submission's failure again, as a flume that never confirmed it left it
- **THEN** the settle sends nothing back, holds no verdict for the open submission, and confirms the earlier failure (99, 225)

#### Scenario: An outcome not acted on, or not confirmed

- **WHEN** the second submission fails while every write of the place is refused, then a settle acts on it while the merge queue refuses confirms, then a settle runs with confirms taken, and then the fix round delivers and flume submits the place anew
- **THEN** the first settle confirms nothing and leaves the failure pending, the second sends code back once and leaves it pending, the third asks no inbox and leaves it pending, and the fourth's inbox gives it again, which sends nothing back and is confirmed (74, 111, 124, 225)
