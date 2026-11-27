## MODIFIED Requirements

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, a place submitted with no agent in it SHALL
land by sluice's host process and SHALL be read and tidied by the
settles of a caller keeping watch, with no other caller's act, no
`sluice wait` and no sluice command that moves a queue. Where that
host process ended before the submission was decided, the settle's
inbox, finding no sluice host on its socket and running `sluice inbox
--json`, SHALL start it again on a host where no service manager owns
it, and the place SHALL land the same (46, 124, 125, 208, C.6, S109;
sluice 135; the-service-manager-starts-each-resident).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, whose host process runs as `sluice daemon`, its check held until the watch's first settle has read it open, and the test then runs `flume settle --every 1m` until it lands
- **THEN** no settle ran a `sluice step`, `run`, `daemon` or `wait`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

#### Scenario: A host process started again by a settle's read

- **WHEN** a chore is submitted to the pinned sluice with its check held, sluice's host process is ended while the check waits, and the test then releases the check and runs `flume settle --every 1m` until the chore lands
- **THEN** the settle's inbox started a host process with another pid, the chore landed and was tidied, and no settle ran a `sluice step` (124, 208, S109; sluice 135)

### Requirement: Outcomes collected from sluice's inbox until confirmed

The profile SHALL collect a place's pending outcomes by asking sluice
host's socket the `inbox` read that `sluice inbox --json` in the place
asks, its folder and worktree the place's and its caller the
`SLUICE_AGENT` the command would carry, starting no sluice process,
and by `sluice inbox --json` in the place only where no sluice host
answers the ask; never with `--confirm`, each with its entry's `id`
and its `delivery`'s key where owed, given again until confirmed:
landed with its commit, or failed or in conflict with sluice's `notice`
as its words, a conflict's base revision its `tested.on`; and SHALL read
a withdrawal from read-submission (99, 101, 133, C.6; sluice 51;
flume-reads-sluices-records-on-sluice-hosts-socket;
residents-ask-each-other-by-socket).

#### Scenario: The inbox asked of sluice host

- **WHEN** a stand-in sluice host on the profile's socket answers the inbox with a `landed` outcome of #6 under `6:landed`, and the seam's `sluice` is no program
- **THEN** the inbox gives the landing under `6:landed`, and the host was asked `inbox` with the place's folder as its folder and worktree, the JSON answer and the caller's `SLUICE_AGENT`, confirming nothing (133)

#### Scenario: No sluice host answers

- **WHEN** no sluice host answers on the profile's socket, and the seam's `sluice` is no program
- **THEN** the inbox is unread in the words of `sluice inbox --json`, which could not be run, and not of the socket (133, S109)

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry given until it is confirmed

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice, the outcome confirmed by its key, and the inbox collected again
- **THEN** the first two collects give the failure with sluice's notice, its entry's id and the key `<id>:failed`, the confirm answers confirmed, the third collect gives nothing, and a second confirm of the key answers confirmed (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

### Requirement: A refused outcome is held

The profile SHALL read an inbox's `refused` record, a landing the forge
refused that sluice holds, as held with its `notice` whole as its words,
or what a record with none says, owed under its key until confirmed as
any outcome is (112, 133, C.6 at boomdrivedev/blueprints `8b73d11` and
`fc4323b`; sluice 152; core.58).

#### Scenario: A refused outcome

- **WHEN** a stand-in sluice host answers the inbox with sluice `44e33d4`'s documents, a `refused` outcome of #7 under `7:refused` beside a `landed` one of #6 under `6:landed`, and `7:refused` is then confirmed
- **THEN** the inbox gives the landing and the outcome held with its `notice` whole, each with its entry's id and key, asks the inbox with no `--confirm` and confirms nothing, and the confirm runs `sluice confirm 7:refused --json` (112, 133, C.6)

### Requirement: A settle runs no round of sluice's queue

The sluice profile's step SHALL run no sluice command and SHALL answer
that the base's queue moves on its own, whichever of sluice's ways runs
it. The profile SHALL ask sluice of an open submission only its reads:
the inbox, which asks sluice host's socket and, where no sluice host
answers, runs `sluice inbox --json`, which starts sluice's host process
where work waits, none runs and no service manager owns it, and
read-submission, which asks its socket and starts none (124, 125, C.6;
sluice 135; sluice record 0093 as amended; core.6;
flume-reads-sluices-records-on-sluice-hosts-socket;
the-service-manager-starts-each-resident).

#### Scenario: A step over a stand-in sluice

- **WHEN** the profile's step is asked of `main` with a stand-in `sluice` first on its `PATH` that leaves a mark whenever it runs
- **THEN** it answers that `main`'s queue moves on its own, and the stand-in left no mark (125, C.6)

#### Scenario: A queue the settles never step

- **WHEN** a place's entry is submitted to `main` of a scratch repository through the pinned sluice, sluice's host process runs as `sluice daemon` and no `sluice run` is live, and the profile's step, read-submission and inbox are asked as a settle asks them until the entry is decided
- **THEN** sluice's host process lands the entry, each step answers that the queue moves on its own, and every sluice command the profile ran after the submit was `inbox` or `confirm` (124, 125, C.6; sluice 135)
