# Verify: a-stage-whose-kind-hibernates-it-sleeps-and-resumes

Checked on 2026-11-18 at
`change/a-stage-whose-kind-hibernates-it-sleeps-and-resumes` 28a05f9.
The branch was cut from `bolt/flumes-roles-sleep-and-wake` at 2d180f0.
The bolt's head is now 5d93507, 46 commits past that, with
`every-message-to-a-role-waits-at-its-address` landed (#76). The check
covers this change's proposal, design, ten delta specs and tasks.

**The work matches its specs and the whole checks pass, but the change
is not archived.** Every task is ticked. Every requirement is built,
and every scenario of the ten specs has a test that the gate ran and
passed. The archive, though, gives specs that fail `root:specs`, a
check-fast task of the gate: four modified requirements are longer than
500 characters, and the new `hibernation` capability's Purpose is
openspec's placeholder. Archiving would put a failing gate on the
branch. The change goes back to its code stage with what follows under
"What goes back".

## The whole checks

`devenv shell -- sluice exec bolt/flumes-roles-sleep-and-wake` ran
nothing. The sluice that devenv pins, 0.71.0, refused the host's
`~/.config/sluice/config.toml` before starting a task:
`unknown field credential, expected one of app_id, key, discovery`
(exit 3). This is the same refusal the bolt's verify of
`every-message-to-a-role-waits-at-its-address` met (46ea2ec). The host's
own sluice, 0.95.1, read it, so the checks ran once, through that
sluice:

`sluice exec bolt/flumes-roles-sleep-and-wake` at 28a05f9: **passed**,
39 of 39 checks across 40 tasks in 3m 54s, recorded under their keys for
28a05f9. These include `conformance:test` (the engine's scenarios),
`core:test`, `seams:test`, `fakes:test`, `flume:test`, `nix:test`,
`site:prose`, `site:build`, `root:specs` and `root:changes`. Each crate's
format and lint also passed. `check-tools` is main's layer and was not
run here.

sluice reported that 28a05f9 would conflict with the bolt's head
5d93507 in seven files:
`crates/flume-conformance/src/engine/place_status.rs`,
`crates/flume-core/src/acts.rs`, `crates/flume-core/src/start.rs`,
`crates/flume-seams/src/held.rs`, `crates/flume-seams/src/round_trip.rs`,
`crates/flume/tests/command_line.rs` and
`site/src/content/docs/reference/commands.md`. It ran the checks on the
branch as it stands, not merged.

## Tasks

All 28 tasks of the nine groups are ticked, `[x]`. Each test a task
names stands where the task puts it, and the gate ran it:

- **1.1–1.3:** `an_identity_names_its_resume`,
  `a_stage_that_hibernates_and_its_hibernate_time`,
  `a_hibernation_held_and_resumed`, `a_hibernations_types_round_trip`
  and `a_hibernation_absent_from_a_state_written_before`, in
  `flume-seams/src/round_trip.rs`.
- **1.4:** `the_clauses_listed`. 212 and 213 are in Part A, S124, S125
  and S127 are read at `HIBERNATION` 30dce35, and `REVISION` stays
  31f9f1c.
- **2.2–2.4:** the host's, the launch's and the declaration's
  `hibernate_after` tests, in `host/config.rs`, `commands/launch.rs`
  and `tests/profiles/kinds_weir.rs`.
- **3.1–3.2:** `an_identity_read_back_as_flume_spells_it`,
  `a_resumes_herdr_name_differs_from_its_starts` and
  `a_resumes_rex_name_differs_from_its_starts`.
- **4.4, 6.2, 7.2:** `settle_on_quiet_starts_a_conversations_quiet_again`,
  `hibernation_through_the_binary_*` and
  `a_hibernating_stage_and_its_resumes`.
- **8.1:** `weir-kinds`, `weir-delivery` and `weir-pack` are pinned at
  cc9d41f. `map_agent` reads `stage.hibernate.unwrap_or(false)` and
  never a section, and the two `hibernate_through_weir_*` tests pass.
- **8.2:** `nix:test` passes.
- **9.1:** `site:prose` passes, and each of the six site pages the task
  names gives hibernation.
- **The engine tasks (2.1, 4.x, 5.x, 6.1, 7.1, 7.3):** each scenario
  they add is registered (`engine/mod.rs` extends `hibernation::scenarios()`),
  and `conformance:test` passes.

## Specs

Each scenario the change adds or modifies has a test of the same name.
The modified requirements' earlier scenarios keep the tests they had.

| spec | requirements | scenarios | where |
|---|---|---|---|
| hibernation | 10 added | 25 | `engine/hibernation.rs`, all 25. "Asking, own work, a tell" is split in two, one for a settle and one for a caller's hibernate |
| caller-acts | Retry, Stop, Finish, Tell modified | 5 new | `engine/caller_acts.rs` |
| carried-conversations | 1 added, 1 modified | 2 new | `engine/carried_conversations.rs` |
| command-line | 2 added, 1 modified | 4 new | `tests/commands.rs` (`hibernation_through_the_binary_*`), `commands/launch.rs` (`a_launchs_hibernate_after_*`) |
| kinds-profile | 2 added | 3 | `tests/profiles/kinds_weir.rs` |
| launching | 1 added | 3 | `engine/agent_settings.rs` |
| place-line | 1 modified | 2 new | `engine/place_line.rs` |
| place-status | 1 added | 3 | `engine/place_status.rs` |
| seam-contracts | 3 added | 5 | `flume-seams/src/round_trip.rs` |
| stage-runs | 1 added, 2 modified | 3 new | `engine/stage_runs.rs`, `engine/open_sessions.rs` |

The code follows design 6 to 8 where the specs leave the choice open.
`Engine::hibernate` refuses a stage its kind does not hibernate, and a
place with no live agent, naming the place. `Engine::resume` refuses a
stage that hibernated waiting on its answer, naming the tell or the
retry with words that resumes it. A hibernation is claimed on its start
in one write before the agent is closed, so two settles close it once.
`openspec validate a-stage-whose-kind-hibernates-it-sleeps-and-resumes
--strict` passes.

**A warning, not a shortfall.** The scenario "A time in another form"
says "and nothing is made". The unit test checks the refusal of
`--agent` parsing, which comes before the launch makes anything, so it
holds by order. No test launches end to end with that refusal.

## The archive fails root:specs

`openspec archive` was run in the working tree and then undone, so
nothing of it is committed. It applied the specs (+21, ~9). Then
`openspec validate --specs --strict --no-interactive`, which
`root:specs` runs, failed three of 50 specs:

- **`caller-acts`:** three requirements are longer than 500 characters:
  Retry, 740 (494 at the base); Stop, 506 (259); and Tell, 735 (485).
- **`stage-runs`:** one requirement is longer than 500 characters: "An
  agent let go is ended", 590 (428 at the base).
- **`hibernation`:** the Purpose is openspec's placeholder,
  `TBD - created by archiving change …`. The delta spec
  `specs/hibernation/spec.md` gives no `## Purpose`, and openspec reads
  a delta's Purpose only when it creates the capability. Earlier changes
  that added a capability give one at the delta's head, as
  `2026-11-07-every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`'s
  `base-places` does.

## What goes back

1. Bring each of caller-acts' Retry, Stop and Tell, and stage-runs' "An
   agent let go is ended", to 500 characters or fewer in the delta
   specs. Move what each adds for hibernation into a requirement of its
   own, or into its scenarios. The bolt's aa8d2c5 did the same for its
   change.
2. Give `specs/hibernation/spec.md` a `## Purpose` saying what the
   capability is for.
3. Take the bolt's head. sluice forecasts conflicts in seven files.
   Also, `caller-acts` on the bolt no longer holds "Tell": it holds "A
   message to a place's agent" and four other message requirements.
   This change's MODIFIED "Tell" then has nothing to modify, so the tell
   that resumes a hibernating stage has to be carried onto the
   message's requirement.

Check each point by running the archive in the working tree and then
`openspec validate --specs --strict` before the next verify.
