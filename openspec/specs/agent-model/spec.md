# agent-model Specification

## Purpose
An agent's model, its program's own name or a table of programs given
in every layer of clause 32 and taken whole, the agent's entry its
program's; asked of its profile, which takes it or refuses, at the
launch and each start; nothing passed where no layer gives one (172).

## Requirements

### Requirement: A model is its program's own name or a table of programs

An agent's model SHALL be its program's own name, kept as written, or
a table of programs, each entry its program's own name, kept as
written; flume SHALL map no model's name from one program to another.
A table that names no program SHALL be refused where it is read (32,
172, C.2).

#### Scenario: A name kept as written

- **WHEN** the kind's agents table gives `model = "large"` and a fix's agent runs the fake program `agent`
- **THEN** the agent's command passes `large` as written, and status gives the model `large` from `kind` (172)

#### Scenario: A table of programs

- **WHEN** the launch gives `--agent model.agent=large --agent model.other=small` and a fix's agent runs the fake program `agent`
- **THEN** its command passes `large`, and status gives the model as the table was written, from `launch`, the entry `agent` and `large` passed (56, 172)

### Requirement: A table's entry is the agent's program's, never a later layer's

The agent's entry of a model's table SHALL be the entry for its
program's name. A table that names no entry for the agent's program
SHALL refuse the launch, or the stage at a later start, naming the
table as written, the program, the stage and the layer, and SHALL never
pass to a later layer's model (4, 32, 169, 172).

#### Scenario: No entry for the agent's program

- **WHEN** the kind's declaration gives `model = "large"` and the launch gives `--agent model.other=small`
- **THEN** the launch is refused naming `{ other = "small" }`, the program `agent`, the stage and `launch`, the kind's `large` is not taken, and nothing is made (4, 172)

### Requirement: A profile takes a model or refuses it

The core SHALL ask the agent's profile of its entry before any command
is built, at the launch and at each start, and a profile that names no
way to give its program a model SHALL refuse every name; where no layer
gives a model, nothing SHALL be asked and the command SHALL pass none,
the account's own configuration deciding (88, 144, 170, 172).

#### Scenario: A profile with no way to give a model

- **WHEN** the fake agent is scripted to take no model, and a fix launches with `--agent model=large`
- **THEN** the launch is refused naming `large`, `agent`, the stage and `launch`, in the profile's words that it names no way to give a model, and nothing is made (4, 172)

#### Scenario: No model in any layer

- **WHEN** a fix launches with no model in any layer
- **THEN** the fake's log shows no `model` asked, its command passes none, and status gives the agent no model (172)

### Requirement: A model its profile refuses refuses the launch or the stage

`Engine::launch` SHALL resolve every stage's model and ask its agent's
profile before anything is made, a refusal refusing the launch whole; a
model refused at a later start, from the host's table as it stands
then, SHALL leave the stage refused in the same words, starting no
agent, until a retry (4, 8, 27, 32, 172).

#### Scenario: A refusal at a later start

- **WHEN** the host's `[kinds.change.agents]` gives no model at a change's launch and `model = { other = "small" }` before its code starts
- **THEN** code is refused naming the table, `agent`, the stage and `host-kind`, no agent starts, and a retry after the host's table is put right starts it (27, 32, 172)
