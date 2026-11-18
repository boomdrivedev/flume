# Tasks

The work is on `chore/flumes-tasks-read-no-markdown`, cut from
`bolt/flumes-checks-read-no-markdown`. The principle is
boomdrivedev/blueprints `design/principles.md` 16 at `b2e8c85`.

## 1. The root and the site

- [x] 1.1 `root:adrs` and `check-adrs.sh` go, and `site:prose` with them; every task left carries one layer tag (core.21; record 0071). Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run root:layers`.
- [x] 1.2 The Pages workflow builds the site with no prose job, and the comments that named `site:prose`, the records' check or the site's prose as a check say the writers keep them; every tracked file is still claimed or excluded with its reason. Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run root:format`.
- [x] 1.3 The site builds as before. Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run site:build`.

## 2. The herdr plugin

- [x] 2.1 The manifest's check reads `herdr-plugin.toml` alone, and the setup test writes the popup's entry `lib/common.sh` gives and the sidebar's rows from `tests/config.sh` (122, C.5). Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run plugin-herdr:test`, "setup: a configuration with the popup's entry and the sidebar's rows lacks nothing".
- [x] 2.2 `tests/config.sh` is linted with the plugin's other scripts. Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run plugin-herdr:lint`.
- [x] 2.3 The install test passes the same configuration through the pinned herdr's `config check` (core.29). Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run plugin-herdr:install`.

## 3. The rule and the spec

- [x] 3.1 core.21 sets Markdown aside, citing principle 16 and record 0071, mirrored on the site's rules page. Verify with `sluice exec bolt/flumes-checks-read-no-markdown -- moon run root:format`.
- [x] 3.2 Validate with `openspec validate flumes-tasks-read-no-markdown --strict`, then archive with `openspec archive flumes-tasks-read-no-markdown --yes`.
