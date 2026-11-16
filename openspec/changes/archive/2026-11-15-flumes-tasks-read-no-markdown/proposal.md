# flume's tasks read no Markdown

## Why

No check, test or gate task in any kit reads a Markdown file: a doc, a
README, a skill, a site page, an instruction, a spec or a record. What a
page says is kept by its writers (boomdrivedev/blueprints principle 16,
the user's words of 2026-11-11: "We do not fucking write code tests
against Markdown files."). sluice's core.21 says so since its 4295ca8.

flume's gate still read four: `root:adrs` read every record's
frontmatter for its number, `site:prose` ran Vale over the site's
pages, the Pages workflow ran Vale before it built the site, and the
herdr plugin's manifest, test and install tasks read the plugin's
README's TOML blocks.

## What Changes

- **`root:adrs` and `check-adrs.sh` go.** A record's number is kept by
  its writer, who takes the next free one.
- **`site:prose` goes, and the Pages workflow's prose job with it.** The
  site's build stays, and Vale stays in the shell, `prose`, for the
  pages' writers to run; no task or workflow runs it.
- **The herdr plugin's tests read no README.** The manifest's check
  reads `herdr-plugin.toml` alone; the setup test and the install test
  write the popup's entry `lib/common.sh` gives and the sidebar's rows
  into their scratch configuration (`tests/config.sh`), and the README
  gives a person the same, kept by its writers.
- **core.21 sets Markdown aside**, citing principle 16, and record 0071
  says what stays.

What stays: OpenSpec's validation of the specs and the open changes,
the site's build, Claude Code's `plugin validate` of the plugin, the
kinds' prompts and the plugin's files the binary carries, and tests
that write a `README.md` or a change folder into a scratch repository
as data.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: the root project holds no `adrs` task.
- `herdr-plugin`: the popup's entry is checked as `lib/common.sh` gives
  it, and no check reads the README.
