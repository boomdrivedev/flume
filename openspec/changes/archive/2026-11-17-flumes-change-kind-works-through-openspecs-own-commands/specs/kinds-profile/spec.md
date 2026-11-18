## MODIFIED Requirements

### Requirement: flume's own three kinds and their prompts

flume's own kinds SHALL be built into the binary one declaration per
version, each with its prompts beside it, and each read like any other:
fix@1, chore@1, change@1, change@2, change@3 and change@5 as C.2 gives
them, change@2's code stage repeating per task group and the others'
not, change@5's stages giving their agents `full`, each prompt naming
its deliverable and the way to stop short, and none asking for a
submit, a push or a rebase (22, 28, 71, 140, 157, 162, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own kinds are read
- **THEN** they are fix@1 on `fix/`, chore@1 on `chore/`, and change@1, change@2, change@3 and change@5 on `change/`, each change with construct, code and verify, change@2's code repeating per task group and the others' not, change@5's three stages each giving its one agent the mode `full` and no other version's stages giving one, all landing, each last stage's `on_fail` `code`

#### Scenario: Each version's prompts are its own

- **WHEN** flume's own declarations are read
- **THEN** each version's prompts are read from its own directory, change@1's code prompt is the one flume held before change@2, change@2's construct and verify are change@1's, change@3's three prompts are none of change@1's or change@2's, and change@5's three are none of an earlier version's (156, 157)

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a push or a rebase, nor a merge but the base's in a conflict's words (28, 71, 112)

#### Scenario: A place launched at change@1

- **WHEN** a place launched at change@1 is settled by a flume that holds change@1 and change@2
- **THEN** it keeps change@1's stages as its launch read them, its code stage not repeating, and status names change@1 (21)

#### Scenario: A place launched at change@2

- **WHEN** a place launched at change@2 is settled by a flume that holds change@1, change@2 and change@3
- **THEN** it keeps change@2's stages as its launch read them, its code stage repeating per task group, and status names change@2 (21)

#### Scenario: A place launched at change@3

- **WHEN** a place launched at change@3 is settled by a flume that holds change@1, change@2, change@3 and change@5
- **THEN** it keeps change@3's stages and prompts as its launch read them, its kind giving its agents no mode, and status names change@3 (21)

### Requirement: What flume's own prompts ask

change@2's, change@3's and change@5's verify prompts SHALL commit
`verify.md` in the change's directory and archive the change, or stop
short having found work; their code prompts sent with words SHALL
restore an archived change and record the fix as a ticked task; every
code prompt sent back with a conflict's words SHALL have its agent `git
merge` the base's revision named, resolve the files contested and
commit, first, and never rebase (22, 112, C.2).

#### Scenario: A code prompt sent back with a conflict

- **WHEN** fix@1's, chore@1's, change@2's, change@3's and change@5's code prompts are rendered with a conflict's words naming the files contested and the base's revision
- **THEN** each tells the agent to `git merge` that revision into the place's branch, resolve those files and commit, before anything else, and never to rebase (112)

### Requirement: Packages, identities and locks through weir's crate

The kinds profile SHALL read a kind as a launch writes it, a package with
its content hash, a source's listing, the repository's lock at a revision
and a lock on disk, and SHALL write a lock, each through `weir-kinds` at
its pinned revision alone; it SHALL spell every identity it answers as
weir's crate spells it, and flume's own code SHALL parse, spell or hash
none (core.4; weir 88 to 93, C.4).

#### Scenario: A package's hash wherever it lies

- **WHEN** one package is copied to two scratch directories, one under a link, and each is read at its identity
- **THEN** both read the same kind and the same hash, and the hash changes when one prompt's byte does (156; weir 90)

#### Scenario: A source's listing

- **WHEN** a scratch `kinds_dir` holds `review@1/` and `review@2/`, each with a `kind.toml`, and `notes/`, `lint@x/` and `lint@1/` with no `kind.toml`
- **THEN** the listing answers acme/review@1 and acme/review@2 alone (159)

#### Scenario: The repository's lock at a revision

- **WHEN** a scratch repository commits `.config/flume.lock` recording `flume/change@1`, then a second revision recording nothing
- **THEN** the lock at the first revision records flume/change@1 with its hash, the one at the second records nothing, and one at a revision with no lock is answered as none (20, 155)

#### Scenario: A lock written and read back

- **WHEN** the profile writes a lock of acme/review@1 and acme/review@2 to a scratch file and reads it
- **THEN** it reads both entries as written, and a write that fails leaves the file as it was (155; weir 91)

#### Scenario: Flume's own as packages

- **WHEN** flume's own kinds are read
- **THEN** each is a package with its identity, `flume/fix@1`, `flume/chore@1`, `flume/change@1`, `flume/change@2`, `flume/change@3` and `flume/change@5`, and its hash (156, 157)

## ADDED Requirements

### Requirement: change@5's prompts run OpenSpec and ask only by stopping short

Each of change@5's prompts SHALL name OpenSpec's commands, run as the
repository's instructions run its tools, and no other tool, runner or
agent program's skill or command for OpenSpec; and each SHALL say,
after its way to stop short, that an agent that needs an answer stops
short with its words and never asks through anything else (22, 28, 158,
C.2, C.3; record 0064).

#### Scenario: change@5's prompts name OpenSpec alone

- **WHEN** change@5's construct, code and verify prompts are rendered for a place named `plan`
- **THEN** each names `openspec` and says to run it as the repository's instructions run its tools, and none names `sluice`, `moon`, `cargo`, `devenv` or `/opsx` (158, C.3)

#### Scenario: change@5's prompts say their one way to ask

- **WHEN** change@5's construct, code and verify prompts are rendered for a place
- **THEN** each holds, after `flume needs`, that an agent that needs an answer stops short with its words and never asks through a card, an inbox, a message or a question in its pane, whatever a hook, a plugin, a file or a person's words for every agent tell it (28)

### Requirement: change@5's construct writes its change through OpenSpec

change@5's construct prompt SHALL have its agent make the change with
`openspec new change` where it does not stand, write each part from
`openspec instructions <part> --change <name> --json` in the order
`openspec status` gives until it reads the change's planning complete,
give each task the narrowest check that shows it done, and commit the
change only once `openspec validate <name> --strict` passes on it (22,
C.2, C.3).

#### Scenario: change@5's construct prompt rendered

- **WHEN** change@5's construct prompt is rendered for a place named `plan`
- **THEN** it names `openspec new change plan`, `openspec status --change plan --json` and `openspec instructions <part> --change plan --json`, asks each task for the narrowest check that shows it done, and rules out a task running the repository's whole checks or a task group's (22, C.2, C.3)

#### Scenario: The strict check before the construct's commit

- **WHEN** change@5's construct prompt is rendered for a place named `plan`
- **THEN** it names `openspec validate plan --strict` before the change's commit, and gives its deliverable as the change with its planning complete and passing that check (22, C.2)

### Requirement: change@5's code works through OpenSpec's apply instructions

change@5's code prompt SHALL have its one agent take the tasks `openspec
instructions apply --change <name> --json` gives, in order, run each
task's own check and tick it at the line those instructions name once
the check passes, run no other check, commit any other write to the
change only once `openspec validate <name> --strict` passes, and put a
retry's words last (22, 131, C.2, C.3; core.33).

#### Scenario: change@5's code prompt rendered

- **WHEN** change@5's code prompt is rendered for a place named `plan`
- **THEN** it names `openspec instructions apply --change plan --json`, each task's own check before its tick at the line the instructions give, and `openspec validate plan --strict` before any other write to the change is committed, and rules out the repository's whole checks and a task group's (22, C.2, C.3)

#### Scenario: A retry's words come last in change@5's code prompt

- **WHEN** change@5's code prompt is rendered for one place without words and with a verify's words
- **THEN** the two texts are equal up to the first character of the words' section, and the second holds the words and asks the fix be recorded as a task naming its own check (131; core.33)

#### Scenario: An archived change sent back to change@5's code

- **WHEN** change@5's code prompt is rendered for a place with a verify's words
- **THEN** it asks that a change archived on the branch have its archive undone first, in one commit, the change moved back to its directory and every spec the archive wrote returned to what it held before the archive (22, 131)

### Requirement: change@5's verify archives through OpenSpec's own archive

change@5's verify prompt SHALL run the repository's whole checks once
before it checks the work, read every task done through `openspec
instructions apply` and the change passing `openspec validate <name>
--strict`, and archive only by `openspec archive <name> --yes`,
committing the specs it writes; where the checks fail, the work falls
short or the archive refuses, it SHALL archive nothing and stop short
having found work (22, 131, C.2, C.3).

#### Scenario: change@5's verify prompt rendered

- **WHEN** change@5's verify prompt is rendered for a place named `plan`
- **THEN** it runs the repository's whole checks once before it reads `openspec instructions apply --change plan --json` and `openspec validate plan --strict`, names the whole checks' outcome and OpenSpec's in `verify.md`, and archives by `openspec archive plan --yes`, never by moving the change or writing its specs itself (22, 131, C.2)

#### Scenario: An archive OpenSpec refuses

- **WHEN** change@5's verify prompt is rendered for a place
- **THEN** it asks that where the whole checks fail, the work falls short or the archive refuses, nothing be archived and the agent stop short having found work, naming what fails (131)
