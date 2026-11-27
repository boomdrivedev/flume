## ADDED Requirements

### Requirement: The setting-up-flume skill

The plugin SHALL carry the skill `setting-up-flume`, which reads
`flume status --binding`, writes the `[profiles]` table of
`.config/flume.toml`, and hands the merge queue's own setup to sluice's
skill `setting-up-sluice`. The binary SHALL write the skill into the
plugin of the agents that it starts. The MCP tool `status` SHALL take
`binding`, which asks for the binding of the repository that the server
runs in.

#### Scenario: The plugin validates with the skill

- **WHEN** the pinned Claude Code validates the plugin strictly
- **THEN** it passes, and the plugin that the binary writes holds each file of `plugins/claude`, the skill among them
