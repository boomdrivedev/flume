## ADDED Requirements

### Requirement: launch gives a model and an effort per program by --agent

`flume launch --agent [<stage>.]model.<program>=<name>` SHALL give one
entry of a model's table of programs, and `--agent
[<stage>.]effort.<program>=<word>` one entry of an effort's, several
for one stage, or for every stage, building one table; a name and a
table for the same stage's model, or a word and a table for its effort,
SHALL be refused naming both, and an entry naming no program SHALL be
refused naming the form (7, 32, 172, 197).

#### Scenario: A model's table and an effort's at the launch

- **WHEN** `flume launch --kind fix --agent model.claude=opus --agent model.codex=gpt-5.5 --agent code.effort.claude=xhigh` runs over a scratch repository
- **THEN** the launch's settings give every stage the model `{ claude = "opus", codex = "gpt-5.5" }` and code the effort `{ claude = "xhigh" }`, each at the `launch` layer (7, 32)

#### Scenario: A name and a table for one stage

- **WHEN** `flume launch --kind fix --agent code.model=opus --agent code.model.claude=sonnet` runs, and then with `--agent effort=high --agent effort.claude=max`
- **THEN** each is refused naming both, and nothing is launched (7, 172, 197)
