## ADDED Requirements

### Requirement: launch carries parameters from a file

`flume launch --params-file <file>` SHALL read the file whole and give
it to the core as the launch's parameters, as written; a file that
cannot be read SHALL refuse the launch naming it (224).

#### Scenario: Parameters given by a file

- **WHEN** `flume launch update a --params-file moves.json` runs where update@1's move asks for `{{params}}`
- **THEN** the place's own folder holds `params` with the file's text whole (224, C.7)
