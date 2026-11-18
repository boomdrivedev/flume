## MODIFIED Requirements

### Requirement: A role's entry gives its run

An entry SHALL give its address's run: a kind of its own source's, its
version the locks'; its repository by its location on the host, or a
standing role's `repositories`; its priority; its base, else the
repository's default branch; and an optional brief, name, intent,
`agents`, `kinds`, `answers` and `answerer`, its settings taken as a
launch's. An entry giving `points` SHALL declare nothing for its
address, as this flume takes points for no place (7, 32, 70, 147, 155,
165, 193, C.7).

#### Scenario: The run's intent and settings

- **WHEN** the entry gives `east/planner` the intent `east-planner` and, under `agents`, `model = "opus"`
- **THEN** the woken place serves the intent `east-planner`, and its agent starts with the model `opus`, which status gives as from the launch (32, 147)

#### Scenario: Points in a role's entry

- **WHEN** `acme`'s file gives `east/planner` a run's keys and `points = 3`
- **THEN** `east/planner` wakes nothing, and status names the entry and that `points` is no word of a role's run, since this flume takes a place's points from nothing (165)

#### Scenario: A standing role's repositories and kinds

- **WHEN** `acme`'s file gives `east/ops` the kind `desk`, `repositories = ["~/code/flume", "~/code/blueprints"]` and `[roles."east/ops".kinds]` with `fix` and `change`, each with its words
- **THEN** the host's roles give `east/ops` a run whose repositories are both locations read under the home, whose repository is the first, and whose kinds are `change` and `fix` with their words (193, C.7)

## ADDED Requirements

### Requirement: A standing role's entry at fault declares nothing

An entry SHALL declare nothing for its address, status naming why,
where its `repositories` is empty, names one location twice, gives two
whose folders are one, or stands beside `repository` or `base`; where a
value of its `kinds` is not words; or where an entry giving `agent` or
`carried` gives `kinds` (191, 193, C.7).

#### Scenario: Two folders of one name

- **WHEN** `acme`'s file gives `east/ops` `repositories = ["~/a/flume", "~/b/flume"]`
- **THEN** `east/ops` wakes nothing, and status names the entry and the folder `flume` both would take

#### Scenario: A base beside repositories

- **WHEN** the entry gives `repositories` and `base = "next"`
- **THEN** `east/ops` wakes nothing, and status names that each checkout is at its repository's default branch, so a standing role's entry gives no base

#### Scenario: Kinds for a standing agent

- **WHEN** an entry gives `agent = "ops"` and `kinds`
- **THEN** it declares nothing for its address, and status names `kinds` as no word of a standing agent's entry

### Requirement: A delivery to a hibernating role's run resumes it

Where a batch is pending at a role's address whose run stands with its
current stage hibernating, the address's wake, by a settle, a carrier,
a message or the host's settle, SHALL resume that stage by `delivery`
and launch nothing; a run whose stage waits on its answer SHALL take
nothing (42, 132, 213, 214).

#### Scenario: A carrier's wake of a hibernating run

- **WHEN** `east/ops`'s run hibernates, an end is pending for it, and `flume wake east/ops` runs
- **THEN** nothing is launched, the run's stage is resumed by `delivery`, and the answer gives the run that stands (132, 213)

#### Scenario: Nothing pending

- **WHEN** `east/ops`'s run hibernates with nothing pending at its address, and `flume wake east/ops` runs
- **THEN** nothing is launched or resumed, and the answer gives the run that stands, hibernating
