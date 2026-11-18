# Verify: a-stage-whose-kind-hibernates-it-sleeps-and-resumes

Checked on 2026-11-18 at
`change/a-stage-whose-kind-hibernates-it-sleeps-and-resumes` e5fca8d.
The branch holds `bolt/flumes-roles-sleep-and-wake` at its head,
5d93507, by the merge 81514d1. The check covers this change's proposal,
design, ten delta specs and tasks.

**The whole checks pass, the work matches its specs, every task is
ticked, and the archive gives specs that pass.** The change is
archived with this report.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-roles-sleep-and-wake` ran
nothing. The sluice that devenv pins, 0.71.0, refused the host's
`~/.config/sluice/config.toml` before starting a task:
`unknown field credential, expected one of app_id, key, discovery`
(exit 3). The host's own sluice, 0.95.1, read it, so the checks ran
once, through that sluice, as the change's earlier verify ran them
(f7b30f1).

`sluice exec bolt/flumes-roles-sleep-and-wake` at e5fca8d: **passed**,
28 of 28 tasks, the checks e5fca8d affects against the bolt's head
5d93507, in 1m 2s, recorded under their keys for e5fca8d. They are
each crate's format, lint and test (`seams`, `core`, `fakes`,
`conformance`, `flume`), `conformance:coverage`, `nix:format`,
`nix:lint`, `nix:typecheck`, `nix:test`, `plugin-claude:test`,
`plugin-herdr:manifest`, `plugin-herdr:test`, `site:install`,
`site:prose`, `site:build`, `root:changes`, `root:format` and
`root:versions`. sluice forecast no conflict with the bolt's head.

`root:specs` is not among them, since nothing under `openspec/specs/`
changed. The archive changes it, so the archive was run in the working
tree and `openspec validate --specs --strict --no-interactive`, the
command `root:specs` runs, was run over it: **50 of 50 specs passed**
(see "The archive"). `check-tools` is main's layer and was not run
here.

## Tasks

All 30 tasks of the ten groups are ticked, `[x]`, and none is open.
Each test a task names stands where the task puts it, and the gate ran
it and it passed:

- **1.1–1.3:** `an_identity_names_its_resume`,
  `a_stage_that_hibernates_and_its_hibernate_time`,
  `a_hibernation_held_and_resumed`, `a_hibernations_types_round_trip`
  and `a_hibernation_absent_from_a_state_written_before`, in
  `flume-seams/src/round_trip.rs`.
- **1.4:** `the_clauses_listed`.
- **2.2–2.4:** `a_hosts_hibernate_after_for_a_kind_and_for_one_stage`,
  `a_hosts_hibernate_after_in_another_form_is_refused`,
  `a_launchs_hibernate_after_for_a_stage`,
  `a_launchs_hibernate_after_in_another_form_is_refused`,
  `a_launchs_hibernate_after_for_a_program_is_refused` and
  `a_repositorys_hibernate_after_refuses_the_launch_naming_the_field`.
- **3.1–3.2:** `an_identity_read_back_as_flume_spells_it`,
  `a_resumes_herdr_name_differs_from_its_starts` and
  `a_resumes_rex_name_differs_from_its_starts`.
- **4.4, 6.2, 7.2:** `settle_on_quiet_starts_a_conversations_quiet_again`,
  `hibernation_through_the_binary_a_hibernate_of_a_fix_is_refused`,
  `hibernation_through_the_binary_a_resume_of_nothing_is_refused` and
  `a_hibernating_stage_and_its_resumes`.
- **8.1:** `hibernate_through_weir_a_conversation_read` and
  `hibernate_through_weir_flumes_own_kinds_declare_none`.
- **8.2:** `nix:test` passed.
- **9.1:** `site:prose` and `site:build` passed. Each of the six pages
  the task names gives hibernation: `flume hibernate` and `resume` and
  a message resuming a hibernating place in `reference/commands.md`,
  `hibernate_after` in `reference/configuration.md` and
  `guides/agent-settings.md`, a stage's `hibernate` in
  `guides/kinds.mdx`, `hibernating` and a start's hibernation and
  resumes in `reference/state.md`, and an answer resuming a hibernated
  agent in `guides/callers-and-holds.md`.
- **The engine tasks (2.1, 4.1–4.3, 4.5, 4.6, 5.x, 6.1, 7.1, 7.3):**
  `engine/mod.rs` extends `hibernation::scenarios()`, each scenario
  they add is registered, and `the_engine_passes_its_scenarios` passed.
- **10.1:** the merge 81514d1 takes the bolt at 5d93507. Hibernation
  stands on the message: `Act::Message`, `Engine::message` and
  `message_answering`, `ResumedBy::Message` written `message`, a
  message to a hibernating stage resuming it (`acts.rs`,
  `message_hibernating`), and `hand` marking the start handing before
  it chooses the deliver part's take or the terminal, so the mark covers
  both. The change's proposal, design, specs and the site say message;
  `told` in design.md is the function's name.
- **10.2:** every requirement the delta specs add or modify is 500
  characters or fewer, and `specs/hibernation/spec.md` has a
  `## Purpose`. The archive's specs pass (see "The archive").

## Specs

Each scenario the change adds or modifies has a test of the same name,
and the gate ran each. The modified requirements' earlier scenarios
stand on the base and keep the tests they had.

| spec | requirements | new scenarios | where |
|---|---|---|---|
| hibernation | 10 added | 25 | `engine/hibernation.rs`. "Asking, own work, a message" is split in two, one for a settle and one for a caller's hibernate |
| caller-acts | Finish and "A message to a place's agent" modified; 3 added | 5 | `engine/caller_acts.rs` |
| carried-conversations | 1 added, 1 modified | 2 | `engine/carried_conversations.rs` |
| command-line | 2 added, 1 modified | 4 | `tests/commands.rs` (`hibernation_through_the_binary_*`), `commands/launch.rs` (`a_launchs_hibernate_after_*`) |
| kinds-profile | 2 added | 3 | `tests/profiles/kinds_weir.rs` |
| launching | 1 added | 3 | `engine/agent_settings.rs` |
| place-line | 1 modified | 2 | `engine/place_line.rs` |
| place-status | 1 added | 3 | `engine/place_status.rs` |
| seam-contracts | 3 added | 5 | `flume-seams/src/round_trip.rs` |
| stage-runs | 1 added, 2 modified | 3 | `engine/stage_runs.rs`, `engine/open_sessions.rs` |

The code follows design 6 to 8 where the specs leave the choice open.
`Engine::hibernate` refuses a stage its kind does not hibernate, and a
place with no live agent, naming the place, and refuses unforced while
the start's handing mark stands. A message to a place whose stage
hibernates resumes it at once; one to a place whose hibernating stage
was stopped finds no live agent and is refused with nothing typed. A
hibernation is claimed on its start in one write before the agent is
closed, so two settles close it once.
`openspec validate a-stage-whose-kind-hibernates-it-sleeps-and-resumes
--strict` passes.

**A warning, not a shortfall.** The scenario "A time in another form"
says "and nothing is made". The unit test checks the refusal of
`--agent` parsing, which comes before the launch makes anything, so it
holds by order. No test launches end to end with that refusal.

## The archive

`openspec archive a-stage-whose-kind-hibernates-it-sleeps-and-resumes
--yes` applied the specs, + 24 and ~ 7, the new capability
`hibernation` taking its Purpose from the delta. Then
`openspec validate --specs --strict --no-interactive` passed 50 of 50
specs. The archive is committed with this report.
