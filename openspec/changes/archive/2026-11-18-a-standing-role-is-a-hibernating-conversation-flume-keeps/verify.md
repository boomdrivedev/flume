# Verify: a-standing-role-is-a-hibernating-conversation-flume-keeps

Checked on 2026-11-18 at
`change/a-standing-role-is-a-hibernating-conversation-flume-keeps`
d765fd3. The branch holds `bolt/flumes-roles-sleep-and-wake` at its
head, 3ecc4f8, which is origin's. The check covers this change's
proposal, design, eight delta specs and tasks.

**The whole checks pass, every task is ticked, and every requirement
and scenario of the eight delta specs is met.** The change is archived
with this report.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-roles-sleep-and-wake` ran
nothing. The sluice devenv pins, 0.71.0, refused the host's
`~/.config/sluice/config.toml` before starting a task:
`unknown field credential, expected one of app_id, key, discovery`
(exit 3). The host's own sluice, 0.95.1, reads it, so the checks ran
once, through that sluice, as the earlier verifies on this bolt ran
them.

`sluice exec bolt/flumes-roles-sleep-and-wake` at d765fd3, the checks
d765fd3 affects against the bolt's head 3ecc4f8: **passed**, 25 tasks
completed, none failed, in 1m 26s (exit 0). sluice recorded 24 results
under their keys for d765fd3.

The tasks: `seams:format`, `seams:lint`, `seams:test` (121 tests),
`core:format`, `core:lint`, `core:test` (58), `fakes:format`,
`fakes:lint`, `fakes:test` (65), `conformance:format`,
`conformance:lint`, `conformance:coverage`, `conformance:test`,
`flume:format`, `flume:lint`, `flume:test` (628),
`plugin-claude:test`, `nix:typecheck`, `nix:test`, `site:install`,
`site:prose` (0 errors, 0 warnings, 151 suggestions), `site:build`,
`root:adrs`, `root:changes` and `root:format`.

`conformance:test` runs the engine's scenarios
(`the_engine_passes_its_scenarios ... ok`) and core.1's search
(`no_tool_is_named_outside_a_profile ... ok`), which finds no
program's name in `flume-core` with task 7.2 done.

`root:specs` is not among the tasks, since nothing under
`openspec/specs/` changes before the archive; `openspec validate
a-standing-role-is-a-hibernating-conversation-flume-keeps --strict`
reads the change valid, and the archive's own run of `openspec
validate --specs --strict` is recorded below. `check-tools` is main's
layer and was not run here.

## Tasks

All 22 tasks are ticked, none open. Each test or scenario a task names
stands where it says:

| Task | What it names | Where it stands |
|---|---|---|
| 1.1 | `a_standing_roles_entry_round_trips` | `crates/flume-seams/src/round_trip.rs` |
| 1.2 | `checkouts_held_round_trip`, `checkouts_held_none_in_a_state_written_before` | `round_trip.rs` |
| 2.1 | `a_standing_roles_entry_gives_its_repositories_and_kinds`, `a_standing_roles_entry_at_fault_declares_nothing` | `crates/flume/tests/host.rs` |
| 3.1 | `checkouts_made_of_a_bare_repository_and_a_clone`, `checkouts_moved_where_git_refuses`, `checkouts_moved_carrying_a_change`, `checkouts_removed_to_the_group_folder` | `crates/flume/tests/profiles/place_git_worktree.rs` |
| 3.2 | the place seam's four checkout scenarios | `crates/flume-conformance/src/seams/place.rs` |
| 3.3 | the git worktree profile passing them | `the_place_profile_passes_the_place_scenarios` |
| 4.1 to 5.2, 10.1 | standing-roles' 17 scenarios | `engine/standing_roles.rs` |
| 6.1, 6.2 | hibernation's five, role waking's two | `engine/hibernation.rs`, `engine/role_waking.rs` |
| 7.1 | `the_kinds_a_role_launches_follow_the_roles` | `crates/flume-core/src/prompt.rs`, naming no program |
| 7.2 | design 8's words read against `no_tool::hits` | "The kinds in the first prompt", `engine/standing_roles.rs` |
| 8.1, 8.2 | tidying's three | `engine/tidying.rs` |
| 9.1 | place-status' three | `engine/place_status.rs` |
| 9.2 | `a_standing_roles_place_and_its_checkouts` | `crates/flume/src/commands/print.rs` |
| 11.1 | `the_mcp_tools_listed`, `mcp_hibernate_refused_as_the_command_line_refuses_it`, `mcp_resume_refused_as_the_command_line_refuses_it` | `crates/flume/tests/commands.rs` |
| 12.1 | the site's four pages | `site:prose` and `site:build` passed |
| 13.1 | record 0069's construction choices | `root:adrs` passed |

## The specs

Every one of the eight delta specs' 50 scenarios has its test under
its own title, and each passed in the run above:

- **standing-roles**: all eight requirements, 17 scenarios, in
  `engine/standing_roles.rs`. `standing_role(kind)` is one stage, a
  conversation its kind hibernates; its wake is refused, before
  anything is made, where its kind lands, where its stage asks for
  `{{branch}}`, `{{base}}`, `{{change}}` or `{{report}}`, where its
  entry gives `repositories` to a run's kind or `repository` to a
  standing role's (`flume-core/src/standing_role.rs`). The checkouts
  are read at each repository's default branch where its merge queue
  says the base lives, the held state written before the place seam
  makes any; each start that opens an agent moves them, a move kept or
  gone or a base unread held as `unmoved` with its words and the start
  going on. "The kinds in the first prompt" reads design 8's words
  against every tool's name core.1's search holds.
- **role-waking**: the run's entry's `repositories` and `kinds`, and
  the faults of a standing role's entry, in `crates/flume/tests/host.rs`;
  a carrier's wake of a hibernating run and nothing pending in
  `engine/role_waking.rs`. The modified requirement's two earlier
  scenarios stand as they were.
- **hibernation**: a delivery resuming a role whose place hibernates,
  and a role's place holding a batch not hibernated, five scenarios in
  `engine/hibernation.rs`.
- **seam-contracts**: the place seam's four checkout scenarios in
  `seams/place.rs`, run against the fake and the git worktree profile,
  and the types' round-trip and a held state written before in
  `round_trip.rs`.
- **place-profile**: detached worktrees made, moved with no `--force`,
  and removed with no teardown, then the location and its group folder
  once empty; four scenarios over scratch repositories in
  `place_git_worktree.rs`.
- **place-status**: a hibernating standing role with nothing pending,
  its checkouts, and a run's place with none, in
  `engine/place_status.rs`; the person's form in `print.rs`.
- **tidying**: finished and removed, a checkout holding a file, the
  waking command gone, in `engine/tidying.rs`.
- **claude-code-plugin**: `flume mcp` lists six tools, `status` alone
  read-only, `hibernate` taking `place` and `force` and `resume`
  taking `place`, each run through the command line's parser and act;
  three tests against the built binary in `commands.rs`.

## The archive

The change is archived on this branch with this report, its deltas
applied to `openspec/specs/`, `standing-roles` among them with its
Purpose as the delta gives it; `openspec validate --specs --strict
--no-interactive`, the command `root:specs` runs, reads 51 of 51 specs
passing over it.
