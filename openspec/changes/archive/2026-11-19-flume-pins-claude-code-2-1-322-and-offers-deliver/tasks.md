# Tasks

The work is on `chore/flume-pins-claude-code-2-1-322-and-offers-deliver`,
cut from `bolt/flume-is-tested-against-the-claude-code-hosts-run`, after
`bolt/flumes-roles-sleep-and-wake` landed. Each task names the narrowest
check that shows it done; `<base>` is that bolt. The release is read as
its program stands in the nix store the pin builds; no agent runs but
the walkthroughs'.

## 1. The pin and the fragments

- [x] 1.1 Pin Claude Code 2.1.322 in `devenv.nix` by the four checksums its release's `manifest.zst.json` gives, its comment naming what a bump reads again (core.22, core.39). Check: `devenv shell -- claude --version` gives `2.1.322`.
- [x] 1.2 Read again each fragment of `limit`, `withheld`, `mode`, `carry`, `live`, `settings` and `effort` in the pinned program, each meaning what it meant in 2.1.296, and write each as 2.1.322 holds it, with the unit tests that pin their text; give `WITHHELD` `CLAUDE_CODE_SESSIONS_OAUTH_TOKEN` in the release's place, and `withheld::fragments` the environment an organization's hook inherits (161, 169, 179, 195, 197, 207, C.4). Check: `sluice exec <base> -- moon run flume:tools` passes `claude_words`.
- [x] 1.3 Add `hook::fragments`, `command_fragments` and `trust_fragments` in `crates/flume/src/profiles/agent/claude/` and their check-tools tests in `crates/flume/tests/profiles/tools/claude_words.rs`, the agent-profile spec's "What the pinned release hands its hooks", "What the pinned release reads of the hooks' answers", "What the pinned release takes of the command" and "What the pinned release asks in a folder new to it" (34, 90, 116, 134, 148, 161, C.4). Check: `sluice exec <base> -- moon run flume:tools` passes `claude_words`.
- [x] 1.4 Capture 2.1.322's folder-trust screens, with and without the warning that the folder's settings pre-approve tools, as `crates/flume/tests/profiles/fixtures/claude/trust-screen-2.1.322.txt` and `trust-screen-settings-2.1.322.txt`, in place of 2.1.296's (34; record 0018). Check: `devenv shell -- cargo test -p flume --test profiles the_trust_screen`.
- [x] 1.5 Copy the engine types 2.1.322 writes over `plugins/claude/engine-types/claude-code/index.d.ts`, the claude-code-plugin spec's "A release that moves the engine" (C.4; core.22). Check: `sluice exec <base> -- moon run plugin-claude:engine-types plugin-claude:typecheck plugin-claude:validate plugin-claude:test`.
- [x] 1.6 Name 2.1.322 wherever the profile, its tests, the stand-in, the fixtures, the plugin's tests and the site name the pinned release, each claim read again in its program (C.4; core.35). Check: `git grep -n 2.1.296 -- crates plugins site` finds nothing.

## 2. Deliver offered

- [x] 2.1 Have `DELIVER` answer offered and `Claude::without_deliver` build the profile that answers not offered, with `deliver_is_offered_at_the_pinned_release` and `the_command_names_no_flume_deliver_where_deliver_is_not_offered`, the agent-profile spec's "The pinned release", "A profile that offers it" and "A profile that does not offer it", and `the_command_for_a_code_stage` holding `FLUME_DELIVER=1` (215, C.4; record 0069). Check: `devenv shell -- cargo test -p flume --lib profiles::agent::claude`.
- [x] 2.2 Have the stand-in run the plugin's module's deliver part where its session names `FLUME_PLACE`, `FLUME_AGENT` and `FLUME_DELIVER`: `flume hook deliver` at its session's start, at each turn's end where another run follows, and every ten seconds while it waits for a prompt, bounded at ten seconds, what a run that exits 0 prints its next prompt, no paste, what was typed and not submitted kept in its input (54, 215, C.4). Check: `devenv shell -- cargo clippy -p flume --all-targets --all-features -- -D warnings`.
- [x] 2.3 Add `a_standing_roles_session_takes_a_message_through_its_deliver_part_its_draft_left_as_typed` in `crates/flume/tests/profiles/tools/walkthroughs.rs`, the standing role's source shared from `tools/walk.rs`, the claude-code-plugin spec's "A message while words are half typed" (54, 193, 214, 215). Check: `sluice exec <base> -- moon run flume:tools`.
- [x] 2.4 Add `a_standing_roles_session_takes_a_batch_through_its_deliver_part_its_draft_left_as_typed` in `crates/flume/tests/profiles/live/walkthroughs.rs`, with the herdr helpers on any session's server in `live/main.rs`, the-gate spec's "A role's session takes a batch with words half typed" (54, 214, 215, C.4). Check: `devenv shell -- cargo clippy -p flume --all-targets --all-features -- -D warnings`; the operator's `moon run flume:live` on deneb runs it.
- [x] 2.5 Say on the site that Claude Code's profile offers the deliver part on 2.1.322 and what flume checks in that release (core.35). Check: `sluice exec <base> -- moon run site:build`.

## 3. The spec

- [x] 3.1 Validate with `openspec validate flume-pins-claude-code-2-1-322-and-offers-deliver --strict`, then archive with `openspec archive flume-pins-claude-code-2-1-322-and-offers-deliver --yes`. Check: `sluice exec <base> -- moon run root:specs root:changes`.
