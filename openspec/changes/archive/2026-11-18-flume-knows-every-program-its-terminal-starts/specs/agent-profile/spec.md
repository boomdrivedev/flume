## ADDED Requirements

### Requirement: Claude Code takes every model name

Claude Code's profile SHALL take every name as its model, since its
release's model names are open, and its command SHALL pass the agent's
entry by `--model` as written; its program's kind SHALL be `claude`
(88, 170, 172, C.4).

#### Scenario: A table's entry passed by --model

- **WHEN** the profile is asked of `opus` and of `claude-opus-4-5[1m]`, and gives the command for a stage whose settings hand it the entry `opus` of `{ claude = "opus", codex = "gpt-5.5" }`
- **THEN** it takes each name, and runs `claude` with `--model opus` and no other model (172, C.4)
