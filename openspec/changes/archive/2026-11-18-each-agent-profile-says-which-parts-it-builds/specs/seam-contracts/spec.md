## ADDED Requirements

### Requirement: The agent seam says which parts its profile builds

The agent seam SHALL give, for its program, which of the parts a
profile may degrade, spent, windows, send-back, guard, on-quiet, the
mode, carry and own settings, its profile builds from the program's own
record, and for each other that it is degraded, with how flume reads
it (171); none built by default. Whether it offers send-back and
on-quiet, reports windows and reads its mode back SHALL be read from
them, and a part not built SHALL be answered the lesser way (93, 174).

#### Scenario: A profile that builds no part

- **WHEN** an agent profile that keeps the seam's defaults is asked its parts
- **THEN** each of the eight is degraded, in 171's order, each with how flume reads it: spent and windows unknown, send-back, guard, on-quiet and carry not offered, the mode recorded and not compared, and own settings neither reserved nor taken; and it offers neither send-back nor on-quiet, reports no windows and reads no mode back (174)

#### Scenario: A profile's answers agree with its parts

- **WHEN** the agent seam's scenarios run against a profile, the fake agent among them building every part, carrying none, taking no own settings, reporting no windows and offering no hooks
- **THEN** its parts name each of the eight once; send-back, on-quiet, windows and the mode's read-back are offered exactly where its parts build them; and each part it does not build answers the lesser way, spent and windows unknown, guard and carry not offered, own settings given refused and no key reserved (93, 174)

### Requirement: The type of a part

The seams SHALL pass one type for a part a profile may degrade, which
SHALL serialize to its word, `spent`, `windows`, `send_back`, `guard`,
`on_quiet`, `mode`, `carry` or `own_settings`, and back unchanged, a
word that is no part refused naming it; each part SHALL give its name
as the statement writes it, and how flume reads it where its profile
does not build it (171, 174).

#### Scenario: Each part written and read

- **WHEN** each of the eight parts is written to JSON and read back, and the word `colour` is read as a part
- **THEN** each reads back unchanged from its word, and `colour` is refused naming it (174)

#### Scenario: A part's name and its lesser reading

- **WHEN** the name of `send_back` and of `own_settings` is read, and how flume reads `carry` where its profile does not build it
- **THEN** the names are `send-back` and `own settings`, and carry reads as not offered, a stage whose agent stopped at its account's usage limit starting afresh (171, 174)
