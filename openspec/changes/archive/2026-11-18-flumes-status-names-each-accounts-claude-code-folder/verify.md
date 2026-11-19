# Verify: flumes-status-names-each-accounts-claude-code-folder

Verified at a0e81192 on `change/flumes-status-names-each-accounts-claude-code-folder`,
cut from `bolt/flume-names-each-accounts-claude-code-folder` (e9b23ef).

**Outcome: holds. The gate's checks pass and every requirement, scenario
and task of the change is met, so the change is archived.**

## The whole checks

`devenv shell -- sluice exec bolt/flume-names-each-accounts-claude-code-folder`,
run once: the checks a0e8119 affects against e9b23ef. All 23 tasks
completed and the run exited 0, in 1m 18s.

- **Passed:** `conformance:coverage`, `:format`, `:lint` and `:test`;
  `core:format`, `:lint` and `:test`; `fakes:format`, `:lint` and
  `:test`; `flume:format`, `:lint` and `:test`; `seams:format`, `:lint`
  and `:test`; `nix:typecheck` and `:test`; `plugin-claude:test`;
  `root:changes` and `root:format`; `site:install` and `:build`.
- `conformance:test` ran every binary: `tests/engine.rs`
  (`the_engine_passes_its_scenarios`), `tests/no_tool.rs`
  (`no_tool_is_named_outside_a_profile`, core.1), `tests/seams.rs` and
  `tests/suite.rs`.
- moon warned that it could not archive some tasks' outputs into its
  cache under `.moon/cache/outputs/`. These warnings are about moon's
  cache only, and no task failed for them.

## The work against the change

The tasks are 1.1 to 3.1, and all nine are ticked. Each task's named
test ran in the gate and passed:

| Spec | Requirement / scenario | Held by | Outcome |
|---|---|---|---|
| seam-contracts | A profile that keeps the seam's defaults | `round_trip::a_profile_that_names_no_home` | passed |
| seam-fakes | A scripted home's variable | `the_agents_home_is_scripted` | passed |
| agent-profile | The variable the profile names | `agent_claude::the_profile_names_claude_config_dir_as_its_home` | passed |
| agent-profile | Claude Code accounts' homes in the built binary's status | `hooks_claude::each_claude_code_accounts_home_in_status`, under `crates/flume/tests/profiles/` | passed |
| place-status | Status gives the host's pools (modified) | the existing pool scenarios, which now give `home: null` | passed (engine) |
| place-status | Each account's home | engine scenario `each_accounts_home`; person's form `an_accounts_configuration_home` | passed |
| place-status | The home among the fields the form holds | `FIELDS` in `answers.rs`, with the sample's work-1 `home` and work-2 `home_unread` | passed (engine) |
| place-status | A home the login environment cannot read | engine scenario `a_home_the_login_environment_cannot_read`; person's form `an_accounts_configuration_home` | passed |
| place-status | The home given and its variable masked | engine scenario `the_home_given_and_its_variable_masked` | passed |

Task 2.5 holds as well. `crates/flume/tests/commands.rs` names no
`claude`, and the gate's `no_tool_is_named_outside_a_profile` passes
(core.1).

The code follows the design:

- `Agent::home_variable` is none by default, and Claude Code's profile
  gives `carry::CONFIG_DIR`.
- `Engine::account_home` reads only the profile's variable and treats an
  empty value as none. It reads the value against `Host::login` with the
  `expand` a start uses. Its `home_unread` words come from the same
  `lacks` helper as a start's refusal, and they name the variables but
  no value.
- `AccountStatus.home` is always serialized, and `home_unread` is left
  out while none.
- `pool_lines` prints the home, or why it cannot be read, under the
  account's line and before its spent words.

The site's state reference (`pools` row), its agent settings guide and
its Claude Code guide say what task 3.1 asks, and they name Claude Code
only as a product flume integrates with (core.35).
