## MODIFIED Requirements

### Requirement: Metrics by kind and stage, never by place

flume's metrics SHALL name a repository, a kind, a stage, a stream, how
a start ended, and whom a wait is on and for which move, and no place,
intent, mark, conversation or account: each start's time to its end;
each wait's time; the stalls and the stuck stages counted; and, at a
settle of the watch's, the places standing by kind, state and whom
they wait on, and each stream's agents running and room. A command
SHALL send what changed in them since it began, once, as it ends (208,
220).

#### Scenario: A start's time by its kind and stage

- **WHEN** a settle reads a code stage's delivery an hour after its start
- **THEN** the command sends one measure of an hour of the start's time, named by its repository, kind, stage, stream and `delivered`, and by nothing that names its place (220)

#### Scenario: The host's places at a settle of every place

- **WHEN** a watch's settle runs over two places waiting on their caller and one running, in two streams
- **THEN** the command sends the places standing by kind, state and whom they wait on, the two waiting among them, and each stream's agents running and room; a settle of one place, a settle of every place a caller asks with no bound and the settle an unwatched command runs first send neither (220, 121, 208, 225)

#### Scenario: No metric names a place

- **WHEN** S129 runs on a host that sends
- **THEN** no attribute of any metric received names the place's name, its intent, its mark, its conversation or its account (220, S129)
