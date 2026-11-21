## ADDED Requirements

### Requirement: Claude Code's configuration home is CLAUDE_CONFIG_DIR

The Claude Code profile SHALL name `CLAUDE_CONFIG_DIR` as the variable
whose value is an account's configuration home, the folder Claude Code
keeps its settings and sign-in in, so status gives each Claude Code
account the `CLAUDE_CONFIG_DIR` its variables give, and none where they
give none (56, 144, C.4).

#### Scenario: The variable the profile names

- **WHEN** the Claude Code profile is asked which variable names an account's configuration home
- **THEN** it names `CLAUDE_CONFIG_DIR` (C.4)

#### Scenario: Claude Code accounts' homes in the built binary's status

- **WHEN** a host's pool declares the Claude Code accounts work-1, whose `env` gives `CLAUDE_CONFIG_DIR = "~/.claude-work-1"` and `CLAUDE_CODE_OAUTH_TOKEN`, and work-2, whose `env` gives neither, and `flume status --json` and `flume status` are run with the host user's `HOME` a scratch folder
- **THEN** the JSON form's `host.pools[0].accounts` give work-1 `home` the scratch folder's `.claude-work-1` and work-2 `home` `null`, the person's form gives `configuration home` with that folder under work-1's line, and neither form gives the token's value (56, C.4; core.30)
