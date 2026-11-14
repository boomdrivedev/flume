# change-profile Specification

## Purpose
The change seam, Part B.3, through OpenSpec: a change's location,
whether it exists, whether its planning is complete, its tasks ticked
and whether it is archived, read from its files for OpenSpec's
spec-driven schema and through OpenSpec's status for any other, writing
nothing to the place.

## Requirements

### Requirement: A change read through OpenSpec's files

The change profile SHALL read a change at a revision from its files:
whether it exists, its tasks each ticked or not, and whether it is
archived. It SHALL read whether a standing change's planning is complete
through `openspec status --change <name> --json` in a checkout of the
revision, whatever the change's schema, and SHALL count none of the
change's files for it: a change `openspec status` reads no such change
of SHALL read not planned, and an archived change that no longer stands
SHALL read planned. Reading SHALL write nothing to the place (59, 86,
87, C.3).

#### Scenario: Seam scenarios over the profile

- **WHEN** the change seam's scenarios run against the profile over a scratch repository's changes
- **THEN** each passes, the same revision giving the same answer

#### Scenario: Agreement with OpenSpec's own reader

- **WHEN** a scratch change is read at each of five revisions, from created to archived
- **THEN** existence, planning complete and the tasks agree with what the pinned `openspec status` gives at the same revision (59)

#### Scenario: A change that skips specs is planned

- **WHEN** a scratch spec-driven change whose `.openspec.yaml` declares `skip_specs: true` holds its proposal, design and tasks and no delta spec
- **THEN** read-change gives its planning complete, as the pinned `openspec status` gives it at the same revision (59)

#### Scenario: A change with no delta spec and no marker is not planned

- **WHEN** the same change's `.openspec.yaml` declares no `skip_specs`
- **THEN** read-change gives its planning not complete, as the pinned `openspec status` gives it at the same revision (59)

#### Scenario: A change OpenSpec cannot read is not planned

- **WHEN** the same change's `.openspec.yaml` gives `skip_specs` a value that is no boolean, which the pinned `openspec status` refuses as invalid metadata
- **THEN** read-change gives the change standing and its planning not complete

### Requirement: Where the change lies

The profile SHALL locate a change by its name at `openspec/changes/<name>`
in the place (127).

#### Scenario: The change part of a prompt

- **WHEN** a code stage's prompt asks for `{{change}}`
- **THEN** it renders `openspec/changes/<name>`

### Requirement: Each task with its task group

The change seam SHALL give each task the task group it is in, each group
with its number and title. The OpenSpec profile SHALL read each
`## <n>. <title>` heading of `tasks.md` as a group holding the tasks
under it, and tasks above every heading as a group with no number or
title; reading SHALL stay a read of the revision alone (86, 87, C.3).

#### Scenario: Groups read from headings

- **WHEN** a scratch change's `tasks.md` holds `## 1. Parser` with two tasks and `## 2. Tests` with one
- **THEN** read-change gives two groups, `1` titled `Parser` and `2` titled `Tests`, and each task in its group, in order

#### Scenario: Tasks above every heading

- **WHEN** a `tasks.md` holds a task before its first group heading
- **THEN** that task is in a group of its own with no number or title, ahead of the others

#### Scenario: The fake change's groups

- **WHEN** the change seam's scenarios run against the fake change with groups scripted, and against the profile
- **THEN** both pass the same scenarios for groups (106)
