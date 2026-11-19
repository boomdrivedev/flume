## MODIFIED Requirements

### Requirement: The live walkthroughs pass under the pinned Claude Code

The `check-live` layer's walkthroughs SHALL carry a fix and a change
from their launch to their landing with the pinned Claude Code under
herdr, signed in by a test account's token, SHALL leave a stage whose
pane goes quiet with no deliverable not ended (39, S3), and SHALL have
a role's session take a batch through flume's plugin's deliver part,
its input left as typed (215); the operator runs them by hand, outside
`sluice exec`.

#### Scenario: The operator's run

- **WHEN** the operator runs `moon run flume:live` from the devenv shell with a test account's token
- **THEN** both walkthroughs land, and the quiet stage with no deliverable is not ended

#### Scenario: A role's session takes a batch with words half typed

- **WHEN** the live walkthrough wakes the standing role `w/ops`, whose talk reads its inbox and waits, types words into the talk's pane without submitting them, and gives `w/ops` two messages each asking for one word
- **THEN** the talk's transcript holds each message in a user turn and the word each asks for in its answers, holds the words typed in no user turn, the pane's input holds them once the talk waits again, and status gives each message handed to `w/ops` by `message` alone (54, 214, 215, C.4)
