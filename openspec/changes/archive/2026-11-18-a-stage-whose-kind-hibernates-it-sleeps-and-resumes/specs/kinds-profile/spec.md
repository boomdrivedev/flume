## ADDED Requirements

### Requirement: A stage's hibernate read through weir's crate

The kinds profile SHALL read whether a stage hibernates from
weir-kinds' `Stage`'s `hibernate` alone, at the revision the tree pins,
and never from its unread sections; a stage that leaves it out SHALL
not hibernate, and flume's own kinds SHALL declare none (23, 212, C.2;
weir 53, 58; core.4, core.22).

#### Scenario: A hibernating conversation read

- **WHEN** a scratch repository's declaration gives its conversation stage `hibernate = true`
- **THEN** the kind is read with that stage hibernating and every other stage not (212)

#### Scenario: flume's own kinds

- **WHEN** each of flume's own kinds is read
- **THEN** none of its stages hibernates (212)

### Requirement: A declaration gives no hibernate time

A declaration's agents table, a kind's, a stage's, or a repository's
for every kind or for a kind it does not declare, that gives
`hibernate_after` SHALL fail to read, naming the field and that the
host or the launch gives it, so the launch is refused whole (4, 158,
198, 212, C.2).

#### Scenario: A repository's hibernate time

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.research.agents] hibernate_after = "2h"`
- **THEN** a launch of research is refused naming `kinds.research.agents.hibernate_after`, and nothing is made (4, 212)
