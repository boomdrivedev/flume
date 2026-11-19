## MODIFIED Requirements

### Requirement: Claude Code's map of flume's modes and its own values

The Claude Code profile SHALL map `plan` to `plan`, `ask` to `manual`,
`edits` to `acceptEdits` and `full` to `bypassPermissions`, and SHALL
take its own values under `claude` in a table of programs, the pinned
2.1.322's `--permission-mode` choices `acceptEdits`, `auto`,
`bypassPermissions`, `manual`, `dontAsk` and `plan`, each passed as
written; any other own value SHALL be refused naming it, `claude` and
the values it names; and it SHALL read the mode back (161, 169, C.4).

#### Scenario: Each of the four

- **WHEN** the profile maps `plan`, `ask`, `edits` and `full`
- **THEN** it gives `plan`, `manual`, `acceptEdits` and `bypassPermissions` (169, C.4)

#### Scenario: Its own value under its name

- **WHEN** the profile maps `{ claude = "auto" }`'s entry, and then `{ claude = "yolo" }`'s
- **THEN** the first gives `auto`, and the second is refused naming `yolo`, `claude` and its six values (169, C.4)

#### Scenario: The mode read back

- **WHEN** the profile reads back `default`, `acceptEdits` and `plan`
- **THEN** it reads `manual`, `acceptEdits` and `plan` (161, C.4)

### Requirement: Claude Code takes the pinned release's five efforts

The Claude Code profile SHALL take `low`, `medium`, `high`, `xhigh` and
`max`, the pinned 2.1.322's `--effort` words, and SHALL refuse any
other, naming it, `claude` and the five; and SHALL name
`CLAUDE_CODE_EFFORT_LEVEL` as the variable its program reads over
`--effort` (197, C.4).

#### Scenario: Each of the five, and two the release reads otherwise

- **WHEN** the profile is asked of `low`, `medium`, `high`, `xhigh`, `max`, `med` and `ultracode`
- **THEN** it takes the first five, and refuses `med` and `ultracode`, each naming it, `claude` and the five (197, C.4)

### Requirement: The credentials pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's `withheld`
rests on to stand byte for byte in the pinned Claude Code: its lists of
credentials, the memory service's pair, the test of the environment
against the list, the names its child environment drops and the drop,
the hook runner's environment built on that child environment, and the
environment an organization's hook inherits, built by the same builder
(core.22, C.4).

#### Scenario: What the pinned release keeps from its hooks

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment `withheld` rests on is found in it, and the test names any that is not (161, C.4)

### Requirement: Claude Code's deliver part at the pinned release

Claude Code's profile SHALL answer deliver offered at the pinned
2.1.322, which runs a plugin's hooks module in an interactive session,
and not offered where it is built for a release that runs none; its
command SHALL set `FLUME_DELIVER=1` in the agent's environment only
where it answers offered (54, 171, 215, C.4; core.39; record 0069).

#### Scenario: The pinned release

- **WHEN** `deliver` is asked of Claude Code's profile as the binary builds it
- **THEN** it answers offered

#### Scenario: A profile that offers it

- **WHEN** a command is built for a start of the profile as the binary builds it, offering deliver
- **THEN** the command's environment gives `FLUME_DELIVER=1` beside `FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`

#### Scenario: A profile that does not offer it

- **WHEN** the profile is built answering deliver not offered, as for a release that runs no hooks module, and a command is built for a start
- **THEN** the command's environment names no `FLUME_DELIVER`

## ADDED Requirements

### Requirement: The hooks' inputs pinned to the release the tree holds

A `check-tools` test SHALL require the source the plugin's hooks read
of their inputs to stand byte for byte in the pinned Claude Code: the
input every hook is built on, with its session and its transcript, and
each event's input with the fields the hooks read, SessionStart's,
StopFailure's and SessionEnd's naming no mode (116, 134, 161, C.4;
core.22, core.39).

#### Scenario: What the pinned release hands its hooks

- **WHEN** the test reads the program devenv.nix pins
- **THEN** it finds SessionStart's input with its source, PreToolUse's, PermissionRequest's and PostToolUse's each with its tool and the tool's input, Stop's with whether a stop hook already blocked it, StopFailure's with its error and its words, and SessionEnd's with its reason, and names any it does not (C.4)

### Requirement: The hooks' answers and the plan's tool pinned to the release the tree holds

A `check-tools` test SHALL require the source of how the pinned Claude
Code reads a SessionStart hook's context and a PreToolUse hook's
refusal, and of `ExitPlanMode`'s input, its plan and its plan's file,
which the asking mark keeps, to stand byte for byte in the pinned
release (43, 90, 108, C.4; core.22, core.39).

#### Scenario: What the pinned release reads of the hooks' answers

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment is found in it, and the test names any that is not (C.4)

### Requirement: The command's options pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's command
rests on to stand byte for byte in the pinned Claude Code:
`--plugin-dir`, which loads a plugin for the session alone; `--model`;
and the file `--append-system-prompt-file` names read as
`--append-system-prompt`'s text is, with the words the release ends
with where no such file is (35, 148, C.4; core.22, core.39).

#### Scenario: What the pinned release takes of the command

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment the command rests on is found in it, and the test names any that is not (C.4)

### Requirement: The folder-trust dialog pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's folder-trust
question rests on to stand byte for byte in the pinned Claude Code: the
dialog's choices, its refusal listed first and in focus and
`Yes, I trust this folder` its confirming choice, and the mark of the
choice in focus; and the screens the profile's own tests read SHALL be
the pinned release's, with and without its warning that the folder's
settings pre-approve tools (34, C.4; core.22, core.39; record 0018).

#### Scenario: What the pinned release asks in a folder new to it

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment the folder-trust question rests on is found in it, and the test names any that is not (34, C.4)
