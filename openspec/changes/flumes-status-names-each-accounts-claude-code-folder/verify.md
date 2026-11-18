# Verify: flumes-status-names-each-accounts-claude-code-folder

Verified at 0724ebde on `change/flumes-status-names-each-accounts-claude-code-folder`,
cut from `bolt/flume-names-each-accounts-claude-code-folder` (e9b23ef).

**Outcome: falls short. The gate's checks fail, so the change is not
archived.** It goes back to its code stage.

## The whole checks

`devenv shell -- sluice exec bolt/flume-names-each-accounts-claude-code-folder`,
run once: the checks 0724ebd affects against e9b23ef. 22 tasks
completed and 1 failed; the run exited 1.

- **Failed: `conformance:test`.** `tests/no_tool.rs`'s
  `no_tool_is_named_outside_a_profile` (core.1) finds the tool name
  `claude` outside a profile, at `crates/flume/tests/commands.rs` lines
  3803, 3808, 3809, 3811 (three times), 3813, 3814 and 3833. These are
  all in the new test `each_claude_code_accounts_home_in_status` (task
  2.4): its comment, its host's name `claude-homes`, its accounts'
  `program = "claude"`, and its `work-1 (claude)` line. A source is
  exempt only under a `profiles` directory or a `profiles.rs` module
  (`crates/flume-conformance/src/no_tool.rs`'s `in_profiles`), and
  `crates/flume/tests/commands.rs` is in neither.
- **Did not run: the rest of `conformance:test`.** cargo stopped at the
  failing binary, so `crates/flume-conformance/tests/seams.rs` and
  `tests/suite.rs` ran neither before nor after it. `tests/engine.rs`
  ran and passed (`the_engine_passes_its_scenarios`), and so did the
  library's unit tests.
- **Passed:** `conformance:coverage`, `:format` and `:lint`;
  `core:format`, `:lint` and `:test`; `fakes:format`, `:lint` and
  `:test`; `flume:format`, `:lint` and `:test`; `seams:format`, `:lint`
  and `:test`; `nix:typecheck` and `:test`; `plugin-claude:test`;
  `root:changes` and `root:format`; `site:install` and `:build`.

## The work against the change

The tasks are 1.1 to 3.1, and all eight are ticked. Each task's named
test ran in the gate and passed:

| Spec | Requirement / scenario | Held by | Outcome |
|---|---|---|---|
| seam-contracts | A profile that keeps the seam's defaults | `round_trip::a_profile_that_names_no_home` | passed |
| seam-fakes | A scripted home's variable | `the_agents_home_is_scripted` | passed |
| agent-profile | The variable the profile names | `agent_claude::the_profile_names_claude_config_dir_as_its_home` | passed |
| agent-profile | Claude Code accounts' homes in the built binary's status | `each_claude_code_accounts_home_in_status` | passed, but it is the test that breaks core.1 |
| place-status | Status gives the host's pools (modified) | the existing pool scenarios, `a_pool_with_readings` now giving `home: null` | passed (engine) |
| place-status | Each account's home | engine scenario `each_accounts_home`; person's form `an_accounts_configuration_home` | passed |
| place-status | The home among the fields the form holds | `FIELDS` in `answers.rs` with the sample's work-1 `home` and work-2 `home_unread` | passed (engine) |
| place-status | A home the login environment cannot read | engine scenario `a_home_the_login_environment_cannot_read`; person's form `an_accounts_configuration_home` | passed |
| place-status | The home given and its variable masked | engine scenario `the_home_given_and_its_variable_masked` | passed |

The code follows the design:

- `Agent::home_variable` is none by default, and Claude Code's profile
  gives `carry::CONFIG_DIR`.
- `Engine::account_home` reads only the profile's variable, treats an
  empty value as none, and expands the value against `Host::login`. Its
  `home_unread` words come from the same `lacks` helper as a start's
  refusal.
- `AccountStatus.home` is always serialized, and `home_unread` is left
  out while none.
- `pool_lines` prints the home, or why it cannot be read, before the
  spent words.

The site's state reference, agent settings guide and Claude Code guide
say what 3.1 asks.

## What the code stage must do

Make `no_tool_is_named_outside_a_profile` pass (core.1) while keeping
the agent-profile scenario "Claude Code accounts' homes in the built
binary's status". Move `each_claude_code_accounts_home_in_status` out of
`crates/flume/tests/commands.rs` into the profile tests under
`crates/flume/tests/profiles/`, where a test may name `claude`. Update
task 2.4 and design 6 to name the place it moves to. Then the whole
checks run again, `conformance:test`'s `seams.rs` and `suite.rs` among
them.
