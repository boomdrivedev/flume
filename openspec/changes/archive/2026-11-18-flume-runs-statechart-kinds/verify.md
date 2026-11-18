# Verify: flume-runs-statechart-kinds

Checked on `change/flume-runs-statechart-kinds` at 1bbadc0, cut from
`bolt/flume-runs-kinds-on-weirs-engine` (e9b23ef), on 2026-11-18.

## Verdict

**Archived.** The whole checks pass, every requirement and scenario of
the change's specs is met, and every task is ticked.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-kinds-on-weirs-engine` ran
once, on 1bbadc0 as it stood: every task passed, sluice recorded 27
results under their keys for 1bbadc0, and it exited 0. A place on a bolt
runs its bolt's `check-fast`; `check-tools` waits for the bolt's merge
onto main.

| task | outcome |
|---|---|
| `seams:format`, `seams:lint`, `seams:test` (123 passed) | passed |
| `core:format`, `core:lint`, `core:test` | passed |
| `fakes:format`, `fakes:lint`, `fakes:test` (68 passed) | passed |
| `conformance:format`, `conformance:lint`, `conformance:test`, `conformance:coverage` | passed |
| `flume:format`, `flume:lint`, `flume:test` | passed |
| `plugin-claude:test`, `plugin-herdr:manifest`, `plugin-herdr:test` | passed |
| `nix:format`, `nix:lint`, `nix:typecheck`, `nix:test` | passed |
| `site:install`, `site:build` | passed |
| `root:format`, `root:changes`, `root:versions` | passed |

`nix:test`: every case holds. `nix/packages/git-crates.nix` holds each of
weir's seven crates at cc9d41f, the revision `Cargo.lock` names (task
1.2; core.22).

`conformance:coverage`: every clause of Parts A and B, I1 to I8 and S1
to S131 at 3a3b287 is cited by the suite's 1059 scenarios. The list of
unbuilt clauses no longer holds 175 to 178, S76 or S77.

`root:changes`: the one open change is valid under
`openspec validate --strict`, and archiving it writes specs that pass
it (core.59).

## Tasks

All 34 tasks of `tasks.md` are ticked `[x]`, and none is open.

## Requirements and scenarios

Each scenario of the change's nine delta specs has a test, registered
in the suite, that passed in the whole checks above. Each test's
assertions were read against its scenario's THEN at c793357; since then
only `nix/` and this change's `tasks.md` changed. All 44 scenarios are
met.

### statechart-kinds (28 scenarios)

All 28 scenarios are in `crates/flume-conformance/src/engine/statecharts.rs`
unless noted.

| scenario | test | met |
|---|---|---|
| A statechart kind launched | `a_statechart_kind_launched` | yes: the place is made and holds the kind's text, `STAGE_TEMPLATE`, `main` at `work`, `main.work.run` at `working` and entry 1, and one live session `main.work#1/1` |
| A template flume does not bind | `a_template_flume_does_not_bind` | yes: the refusal names `session` and `one@1`, with no make, no write and no open |
| An atom flume does not bind | `an_atom_flume_does_not_bind` | yes: `lamp.switch` and `start_sessions` are each refused and named, and nothing is made |
| A rule flume does not bind | `a_rule_flume_does_not_bind` | yes: the refusal names the rule and `main.work` |
| A stage with no prompt | `a_stage_with_no_prompt` | yes |
| A file no rule gives | `a_file_no_rule_gives` | yes: the refusal names `main.work` and `{{file}}` |
| A count above the agents listed | `a_count_above_the_agents_listed` | yes: the refusal names `count:3` |
| A landing fix named | `a_landing_fix_named` | yes: the refusal names `landing_fix` and `flume/fix@1` |
| The template's ends | `the_templates_ends` | yes: passed, sent-back and stopped each reach the template's final state at that settle, and the kind's region moves at the next |
| One tick over one head | `one_tick_over_one_head` | yes: the state is unchanged over two settles, with no new session and no new open |
| A sibling sees a move at the next settle | `a_sibling_sees_a_move_at_the_next_settle` | yes |
| One per persona | `one_session_per_persona` | yes: `stages.test#1/1` and `#1/2` with their files, identities naming their sessions, and `{{file}}` and `{{session}}` filled |
| A stage entered again | `a_stage_entered_again` | yes: entry 2's sessions are recorded, and entry 1's stay as they ended |
| Several agents in one place | `several_agents_in_one_place` | yes: with a bound of 2, the third session waits for room and starts once one session delivers |
| A session asks and is answered | `a_session_asks_and_is_answered` | yes |
| A lost session retried | `a_lost_session_retried` | yes: the place waits as `Lost`, and the retry naming the session starts it again on start 2 |
| Each persona's own report | `each_personas_own_report` | yes |
| A join of all, of any and of a count | `a_join_of_all_of_any_and_of_a_count` | yes: under `any` and `count:2` the working session is ended once its stage passes |
| Found work sends the stage back with its words | `found_work_sends_the_stage_back_with_its_words` | yes: `send_backs` is 1, and build's entry 2 is prompted with the found words |
| Past its bound | `past_its_bound` | yes |
| A rule that matches no file | `a_rule_that_matches_no_file` | yes: status's failures name `every-file` and `personas/*.md` |
| Persona passes and is submitted | `persona_passes_and_is_submitted` (S76) | yes: the place is submitted once, and two more settles submit nothing |
| A run that does not land | `a_run_that_does_not_land` | yes: the place waits on `Done`, and the done ends its work |
| A failure taken back to a stage | `a_failure_taken_back_to_a_stage` | yes: the region moves to `build`, whose prompt carries the merge queue's words |
| A failure nothing takes | `a_failure_nothing_takes` | yes: the place waits as `Submission` with the words, and the retry submits again |
| A stopped stage's decision answered | `a_stopped_stages_decision_answered` | yes: the place waits as `Decision` from when it stood, and `main#1` is applied, moving the region back to `work` |
| An answer refused | `an_answer_refused` | yes: the answer is refused naming `no decision`, then taken, then refused naming the answer taken, `ops` and the time |
| A change@2 place across the release | `held_state.rs`: `a_change_2_place_across_the_release` (S77) | yes: its JSON holds no `chart` or `session`, the place reads back equal, the repetition, verify and submission run, and no `read_statechart` is made |

Beyond its spec, the suite also covers a hold on a stage's path
(`a_hold_on_a_stages_path`) and sessions waiting on readiness
(`sessions_waiting_on_readiness`). These come from design 10 and from
commit b7e79890.

### kinds-profile (4)

These tests are in `crates/flume/tests/profiles/kinds_weir.rs`.

- **Persona read** and **A simple kind and a statechart kind side by
  side:** `a_statechart_kind_is_read_through_weirs_crate`. Met.
- **A prompt the revision lacks:**
  `a_statechart_stages_prompt_the_revision_lacks_is_named` names
  `kinds.persona.regions.stages.states.test.params.prompt` and ignores
  `colour`. Met.
- **Read back twice:** `a_held_statechart_reads_back_the_same`. Met.
- `the_stage_template_names_the_atoms_flume_binds` reads the template
  with weir's crate (task 3.3).

### seam-contracts (3)

- **A kinds seam that reads none:** `round_trip::the_bare_kinds_reads_no_statechart`. Met.
- **Files at two revisions:** "history: a revision's files" in
  `crates/flume-conformance/src/seams/history.rs`, run against the fake
  and against git (`the_history_scenarios_pass_against_git`). Met.
- **Round trips:** `round_trip::a_statechart_kind` and
  `round_trip::a_statechart_places_held_state`. Met.

### seam-fakes (2)

- **A scripted statechart kind read back:**
  `crates/flume-fakes/tests/fakes.rs`
  `the_fake_kinds_reads_a_held_statechart`. Met.
- **Files at two revisions:** the history scenario run against
  `FakeGit`. Met.

### held-state (1)

- **A restart between two settles:** `held_state.rs`
  `a_restart_between_two_settles`. After a restart, the engine is
  rebuilt and reads the statechart back. The state and the stages'
  entries are the same, each session has started once, and two agents
  were opened. Met.

### place-status (3)

These tests are in `crates/flume-conformance/src/engine/place_status.rs`.

- **Persona at its test stage:** `persona_at_its_test_stage`. Met.
- **A decision standing:** `a_decision_standing`. Met.
- **Two agents:** `two_agents`. Met: both agents' identities name
  their sessions, each agent's settings say where its program came
  from, and a simple kind's JSON has no `agents` and no `statechart`.
  The scenario also names each agent's account. `AgentStatus.account`
  carries it as for a simple kind's agent. The test runs with no pool
  and so does not assert it.

### the-list (1)

- **Persona's line:** `crates/flume-conformance/src/engine/list.rs`
  `personas_line`. Met.

### command-line (1)

- **An answer over a scratch repository:**
  `crates/flume/tests/commands.rs` `an_answer_naming_no_decision`. Met:
  the command exits non-zero naming `main` and `no decision`, and
  status gives the statechart with no answers.

### clause-coverage (1)

- **The clauses listed:** `the_clauses_listed` reads 3a3b287, and
  `conformance:coverage` passes, as given above. Met.

## The site

`guides/kinds.mdx` gains "Statechart kinds", covering a stage's
sessions, how a statechart kind runs, decisions, and what a statechart
kind may name. `reference/configuration.md` gains "A statechart kind",
`reference/state.md` "A statechart place", and
`reference/commands.md` `flume answer`, with a hold and a retry naming
a statechart stage. `roadmap.md` is updated too. `site:build` passes.
