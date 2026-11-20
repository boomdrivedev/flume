## ADDED Requirements

### Requirement: herdr reads the folder-trust question with the reading it is given

After `agent_started` the herdr profile SHALL read a screen still as it
was when herdr answered as drawn only after three seconds, and SHALL
read the screen for the folder-trust question for at most ten; a
profile given a draw and a settle of its own, for a program known to
draw at once, SHALL read with those in their place (34, 35, C.5; record
0018).

#### Scenario: flume's own starts

- **WHEN** flume's commands open an agent under herdr
- **THEN** the profile reads the folder-trust question with a draw of three seconds and a settle of ten (34; record 0018)

#### Scenario: A stand-in that draws at once

- **WHEN** the herdr tests' profile, given no draw and a settle of four seconds, opens the stand-in, which draws nothing and asks no question
- **THEN** the open answers once the screen has held still a second, not three seconds after herdr answered (34; record 0018)
