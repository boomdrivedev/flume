# Verify: a-pool-of-several-programs-picks-each-stages-program

Verified on `change/a-pool-of-several-programs-picks-each-stages-program`
at `1c3eaa33`. The branch's merge base with
`bolt/flume-runs-every-agent-its-terminal-can-start` is `502bc6d1`, the
bolt's tip, so the branch is the bolt with this change on it.

## Outcome

**Archived.** The whole checks pass, all 16 tasks are ticked, the change
passes `openspec validate --strict`, and every scenario of the change's
specs has a test that reads as the scenario says.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`
ran once and exited **0**. sluice ran the checks that `1c3eaa3` affects
against the bolt at `502bc6d`, waiting first for room under the host's
bound. All 23 tasks finished in 3m 2s. 22 passed and were recorded under
their keys for `1c3eaa3`. The 23rd, `site:install`, only installs the
site's packages.

The tasks run were the format, lint and test of `seams`, `core`,
`fakes`, `conformance` and `flume`; `conformance:coverage`;
`site:build`; `plugin-claude:test`; `nix:typecheck` and `nix:test`; and
`root:format` and `root:changes`. Among them:

- `conformance:test`: `the_engine_passes_its_scenarios` passed, and so
  did `tests/no_tool.rs`'s `no_tool_is_named_outside_a_profile`
  (core.1).
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
  S1 to S131 at `48a94da` is cited by the suite's 1187 scenarios. 173
  and S71 are no longer on the unbuilt list, and S72 is left to record
  0024's plan stage's change beside 163, 164 and S68.
- `root:changes`: one open change, valid under
  `openspec validate --strict`, and it archives to specs that pass.
- `flume:test`: the library's 344 tests passed, `tests/commands.rs` 73
  and the profiles' tests 246, each with 0 failed.
- `seams:test` 149 passed, `fakes:test` 74 passed, `core:test` 80
  passed, each with 0 failed.

`check-tools` runs at the bolt's merge onto main, not on this branch.

## OpenSpec

- `openspec instructions apply --change a-pool-of-several-programs-picks-each-stages-program --json`
  reads 16 tasks, 16 complete and 0 remaining, its state `all_done`.
  Tasks 1.1 to 6.1 are all ticked `[x]`.
- `openspec validate a-pool-of-several-programs-picks-each-stages-program --strict`
  reads the change valid.

## The change's specs against the work

Each scenario below has a test that reads as the scenario says.

### account-pools

- **A pool names the program of a stage whose layers name none.**
  `engine/account_pools.rs` holds five tests for it:
  - "A mode on a pool of two programs" is
    `s71_a_mode_on_a_pool_of_two_programs`, citing S71. Review a starts
    on agent-1 passed `edit-files`, and b on other-1 passed
    `other-edits`. Status names `edits` from `kind` for both. Once the
    host gives `{ agent = "own-auto" }`, review c waits while agent-1
    is full, since other-1 cannot run it. c then starts on agent-1
    passed `own-auto`, from `host-kind`.
  - "A model only one program takes" starts on agent-1, passed
    `--model large`.
  - "A program no profile binds" starts past nobody-1, on agent-1.
  - "Effort and own settings only one program takes" starts on
    agent-1, passed `--effort high`, with the own setting from `kind`.
  - "No pool" runs flume's default, from `default`, under no account.
- **A stage whose layers name no program waits for an account that can
  run it.** "The only account that can run it is spent" waits with
  nothing opened. Its want is the pool `work` until agent-1's reset, and
  it starts on agent-1 once the reset passes.
- **A pool that holds no account that can run a stage refuses it.**
  - "A launch no account can run" is refused naming code, the pool and
    `other` with the table's missing entry. Nothing is made, no held
    state is written and the terminal is asked nothing.
  - "The account that could run it dropped before a later start"
    refuses review with nothing opened, the place waiting on a retry.
    The retry, once agent-1 is back, starts review on it.
- **An intent keeps the program its last agent ran.** Three tests read
  the affinity's accounts, `last` and moves:
  - "A stage that names no program keeps to the last program": review
    starts on other-1 from the affinity, with no move.
  - "The last program cannot run the stage": review starts on agent-1
    from the strategy, with no move. The affinity keeps both programs'
    accounts, and its last program is `agent`.
  - "The last program's account is spent": review starts on agent-1
    from the strategy. other-1 is kept for `other` with no move,
    agent-1 is written for `agent`, and the last program is `agent`.
- **A conversation stays with its program.**
  `engine/carried_conversations.rs` holds two tests for it:
  - "A spent agent's stage moves to another program": the agent is
    ended and code starts afresh on agent-1, its program `agent` from
    `account`. Neither profile is asked to carry, and the move reads
    other-1 to agent-1, `spent`, carried `false`.
  - "A lost conversation's retry" starts on other-1 from `conversation`,
    its program `other` from `account`, and the affinity records no move.
- **A stage's prompt is previewed on the account a start would choose.**
  "The preview of a stage on the last program" holds `other`'s scripted
  way to stop short, and the store logs no write.

`choose_account` in `crates/flume-core/src/accounts.rs` takes the
conversation's account first, while it is usable and of a program that
can run the stage. It then takes the affinity's account for the last
program, while that program can run the stage, and then the strategy's
choice among the able accounts. A pool with no able account refuses in
`cannot_run`'s words.

### launching

"flume's default and a pool" is `flumes_default_and_a_pool` in
`engine/agent_settings.rs`. On `solo`, with no pool, the first fix's
code runs `agent` from `default` under no account. On `pooled`, whose
pool holds other-1 alone, the second runs `other` from `account` under
other-1. `s44_settings_from_four_layers` now reads its program from
`account`, since its agent has a pool.

### place-status

"A review on the account's program" has two tests:

- In JSON, `a_review_on_the_accounts_program` in
  `engine/place_status.rs` reads the program `other`, its layer
  `account`, and the account other-1 from `strategy`.
- In the person's form, `an_agent_whose_account_names_its_program` in
  `crates/flume/src/commands/print.rs` gives `(other, from its account)`
  and `program = other (from account)`. A program from `kind` still
  reads `(other)`.

### seam-contracts

"An affinity with its last program" is `an_affinity_with_its_last_program`
in `crates/flume-seams/src/round_trip.rs`. `last` reads back as `other`.
An affinity written before reads with none and is written again with no
field for it, as `#[serde(default, skip_serializing_if =
"Option::is_none")]` on `Affinity::last` gives. The other two scenarios
of the modified requirement keep their tests, which pass.

### seam-fakes

"Two fake programs' ways" is `two_fake_programs_ways` in
`crates/flume-fakes/tests/fakes.rs`. For starts 1 and 2, `agent` gives
the fixed words and `other` the scripted words.

### clause-coverage

- "The clauses listed": `clauses.rs` keeps `REVISION` at `48a94da`.
  `unbuilt` no longer holds 173 or S71, `PLAN` names S72 beside 163,
  164 and S68, and `PROFILES` is gone.
  `the_clauses_unbuilt_are_named_by_the_changes_that_build_them` reads
  them so, with a count two fewer.
- "The suite as this change leaves it": `conformance:coverage` passes,
  and 173 and S71 are cited by the engine's scenarios of a pool's
  program.
- "A clause left out" keeps its test, which passes.

## The site

The site says to any host's operator, naming `claude` and `codex` only
as products flume integrates with (core.35):

- `guides/agent-settings.md` says flume's default names the program only
  for an agent with no pool, and has the sections "A pool chooses the
  program" and "a pool that cannot run a stage".
- `reference/configuration.md` says the same in an account's `program`
  row and the pools' prose.
- `reference/state.md` gives `last`, a program from `account`, and a
  queued stage's want.

`site:build` passes.
