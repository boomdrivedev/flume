## ADDED Requirements

### Requirement: The popup's key is the entry lib/common.sh gives

The manifest SHALL declare no key, since herdr 0.9.5's action has none, and SHALL describe the entry that binds one; the `[[keys.command]]` entry `lib/common.sh` gives, the same as the startup's words, and the sidebar's rows SHALL pass the pinned herdr's `config check`. The README gives a person that entry apart from the rows, kept by its writers, and no check SHALL read it (C.5; core.21, core.29; record 0071).

#### Scenario: The manifest

- **WHEN** the manifest's check reads `herdr-plugin.toml`
- **THEN** the `places` action holds a description naming `plugin_action` and `agentplot.flume.places`

#### Scenario: The entry against the pinned herdr

- **WHEN** `herdr config check` of the pinned herdr reads a scratch configuration holding the entry `lib/common.sh` gives and the sidebar's rows
- **THEN** it answers `config: ok`

## MODIFIED Requirements

### Requirement: The startup records what herdr's configuration lacks

At herdr's startup the plugin SHALL read herdr's configuration file
from its server's environment and write `setup.json` in its server's
state directory, with its server's socket: what is missing, the
popup's `[[keys.command]]` entry and the sidebar rows' `$flume`, each
in the plugin's words naming the file, or none. It SHALL log them and
tell no person (72, C.5).

#### Scenario: A configuration with neither

- **WHEN** the startup runs with `HERDR_CONFIG_PATH` naming a scratch file that binds no key to `agentplot.flume.places` and names no `$flume`
- **THEN** `setup.json` holds two entries, one giving the `[[keys.command]]` entry `lib/common.sh` gives and naming the file, the plugin's log holds them, and the stand-in herdr recorded no notification

#### Scenario: A configuration with both

- **WHEN** the scratch file holds the entry `lib/common.sh` gives and the sidebar's rows, and the startup runs
- **THEN** `setup.json` holds an empty array

#### Scenario: No configuration file

- **WHEN** `HERDR_CONFIG_PATH` names no file
- **THEN** `setup.json` holds both entries, naming that path

## REMOVED Requirements

### Requirement: The popup's key is the host's entry

**Reason**: Its check read the README for the popup's entry, and no
check reads a Markdown file (boomdrivedev/blueprints principle 16;
core.21; record 0071).

**Migration**: "The popup's key is the entry lib/common.sh gives"
holds it, the README kept by its writers.
