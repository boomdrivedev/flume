## MODIFIED Requirements

### Requirement: UserPromptSubmit compares the mode at the start's first prompt

At the start's first prompt, the first with no received ack where no
word that the agent began otherwise stands, the UserPromptSubmit hook
SHALL compare its input's `permission_mode`, `default` read as
`manual`, with the start's record before it acks received, where the
record names a mode, since 2.1.322 gives a SessionStart input none
(161, 162, C.4).

#### Scenario: SessionStart acks on the program and the variables

- **WHEN** the SessionStart hook runs for a start whose record names `bypassPermissions`, its input naming no mode, as 2.1.322's never does, and its program and variables as the record names them
- **THEN** the started ack is written (35, 161)

#### Scenario: manual read back as default

- **WHEN** the hook runs at the first prompt of a start whose record names `manual`, its input's `permission_mode` reading `default`
- **THEN** the received ack is written and nothing is blocked (161, C.4)

#### Scenario: No mode in the record

- **WHEN** the hook runs at the first prompt of a start whose record names no mode, its input's `permission_mode` reading `acceptEdits`
- **THEN** the received ack is written, no mode compared (161, 162)

### Requirement: The engine the module rests on pinned to the release the tree holds

The plugin SHALL keep the engine types the pinned Claude Code writes, a
`check-fast` task SHALL fail where they differ from the pinned
release's, and its modules and tests SHALL be typed against them, so
`$.ui.status`, `$.clock.every`, `$.process.run` with its `stdin`,
`$.env.get`, `$.prompt.submit` with its `asUser`, `session.measure`
with its `changed` and `rateLimits`, and the other events the modules
register on are the pinned release's (C.4; core.22, core.39).

#### Scenario: A release that moves the engine

- **WHEN** the vendored engine types differ from those the pinned Claude Code writes
- **THEN** `plugin-claude:engine-types` fails naming the file and the release (core.22)

## ADDED Requirements

### Requirement: A role's session takes its deliveries through the deliver part, its draft kept

Under herdr, a standing role's session whose agent's profile offers
deliver SHALL take each message for its address through the plugin's
deliver part, `flume hook deliver` run while the session waits, as its
next prompt, nothing typed into its pane by flume, each message
recorded handed to the address by `message`; and words typed into its
input and not submitted SHALL stay there, the person's to submit (54,
193, 214, 215, C.4).

#### Scenario: A message while words are half typed

- **WHEN** under the pinned herdr and the stand-in, whose session runs the deliver part as the pinned release runs the plugin's module, the standing role `w/ops` of a scratch source is woken, its talk takes its first prompt and waits, words are typed into its pane and not submitted, and `flume message w/ops` gives it a message
- **THEN** the talk's next prompt holds the message with its confirm and not the words typed, the stand-in logs that the plugin's module submitted it and that its input still holds the words, status gives the message handed to `w/ops` by `message` alone, and the enter typed after it submits the words as typed (54, 214, 215)
