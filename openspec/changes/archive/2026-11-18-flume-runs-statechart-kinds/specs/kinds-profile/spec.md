## ADDED Requirements

### Requirement: A statechart kind read through weir's crate

The kinds profile SHALL read a statechart kind of a repository's or a
launch's declaration through `weir-kinds` at the revision the workspace
pins for weir's four crates, giving the seam's kind its title, label,
branch, whether it lands, its landing fix, the kind's table spelled as a
declaration of it alone, and each state's stage parameters (19, 84, 85,
175, core.4, core.22).

#### Scenario: Persona read

- **WHEN** a scratch repository's `.config/flume.toml` declares weir C.4's persona kind with a `prompt` beside each stage's parameters
- **THEN** read-kinds gives persona with its title, its branch, its statechart's text and the parameters of `stages.build` and `stages.test`, the test stage's agents its rule and pattern and its join all (84, 175, S76)

#### Scenario: A simple kind and a statechart kind side by side

- **WHEN** one declaration holds a simple kind and a statechart kind, and another a table holding both `stages` and `regions`
- **THEN** the first gives both kinds, each in its format, and the second is answered as could not be read, naming the field (85, 177; weir 95)

### Requirement: A stage's prompt is read beside its declaration

The kinds profile SHALL read the `prompt` a stage-template state gives
as a simple stage's prompt is read, beside the declaration at the base's
revision or beside the launch's file, naming `<path>.params.prompt` where
it cannot be read, and SHALL ignore every other key of `params` weir does
not name (20, 23, C.2).

#### Scenario: A prompt the revision lacks

- **WHEN** a statechart kind's stage names `prompt = "prompts/test.md"` and the base's revision holds no such file
- **THEN** read-kinds answers could not be read, naming `kinds.persona.regions.stages.states.test.params.prompt` (20, C.2)

### Requirement: A held statechart kind is read back the same

The kinds profile SHALL read a held statechart kind's text and the stage
template's text back into the engine's types through `weir-kinds`, the
same texts always giving the same statechart and templates, or refuse
naming why (21, 85, 176).

#### Scenario: Read back twice

- **WHEN** persona's held text and the stage template's are read back twice
- **THEN** each read gives the same statechart and the template `stage`, equal to the statechart read-kinds gave at the launch (21, 85)
