## MODIFIED Requirements

### Requirement: The popup's key is the entry lib/common.sh gives

The manifest SHALL declare no key, since herdr 0.9.5's action has none, and SHALL describe the entry that binds one; the `[[keys.command]]` entry `lib/common.sh` gives, the same as the startup's words, and the sidebar's rows SHALL pass the pinned herdr's `config check`. The README gives a person that entry apart from the rows, kept by its writers, and no check SHALL read it (C.5; core.21, core.29; record 0074).

#### Scenario: The manifest

- **WHEN** the manifest's check reads `herdr-plugin.toml`
- **THEN** the `places` action holds a description naming `plugin_action` and `agentplot.flume.places`

#### Scenario: The entry against the pinned herdr

- **WHEN** `herdr config check` of the pinned herdr reads a scratch configuration holding the entry `lib/common.sh` gives and the sidebar's rows
- **THEN** it answers `config: ok`
