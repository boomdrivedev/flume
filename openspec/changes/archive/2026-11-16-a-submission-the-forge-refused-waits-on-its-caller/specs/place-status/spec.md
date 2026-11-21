## MODIFIED Requirements

### Requirement: Status says what a waiting place waits on

Status SHALL give a stop-short's words and whether it needs an answer or
found work, or a failed submission's words, or a held submission's with
the forge's words as the merge queue's record gives them, and, for a
place waiting on its caller, the move it waits on and since when, a
hold's words among it, and, for an answer, whether its agent is kept
open for it and until when, or when and why it was let go and whether it
is ended (56, 112, 130, 198).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer at a time t, its agent kept open
- **THEN** it gives the move waited on as a retry of construct, since t, the question whole, and its agent kept open until t plus its `keep_open`, in the JSON form and the person's (56, 130)

#### Scenario: Its agent let go

- **WHEN** status is asked of that place once its agent was let go by its time and ended
- **THEN** it gives the same move since t, and its agent let go with when, `time`, and when it was ended (130, 198, S104)

#### Scenario: A submission whose landing the forge refused

- **WHEN** status is asked of a fix whose submission a settle at a time t read held, the fake merge queue's record holding the forge's words
- **THEN** it gives the submission held with the record's words, and the move waited on as a retry of code for `forge_refused`, since t, with the words the wait was written with, in the JSON form and the person's (56, 99, 112, 130)
