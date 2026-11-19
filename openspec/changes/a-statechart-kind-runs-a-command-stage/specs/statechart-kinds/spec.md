## MODIFIED Requirements

### Requirement: A launch naming what flume does not bind is refused

A launch of a statechart kind SHALL be refused, naming it and making
nothing, where its statechart names a template other than `stage` and
`command`, a rule other than `every-file`, an evidence other than
`region.entered`, `place.submission` and `place.submissions`, or any
effect (4, 175, 178, 223).

#### Scenario: A template flume does not bind

- **WHEN** a statechart kind's state runs `session`
- **THEN** its launch is refused naming the template, and the fakes record no make and no held state (4, 175)

#### Scenario: An atom flume does not bind

- **WHEN** a statechart kind's transition reads `ev = "lamp.switch"`, and another's performs `do = "start_sessions"`
- **THEN** each launch is refused naming the atom, and nothing is made (4, 178)

#### Scenario: A rule flume does not bind

- **WHEN** a stage-template state's agents give `rule = "every-persona-definition"`
- **THEN** its launch is refused naming the rule and the state (4, 175)
