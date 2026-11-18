# Verify: a-standing-role-is-a-hibernating-conversation-flume-keeps

Checked on 2026-11-18 at
`change/a-standing-role-is-a-hibernating-conversation-flume-keeps`
7307f55. The branch holds `bolt/flumes-roles-sleep-and-wake` at its
head, 3ecc4f8, which is origin's. The check covers this change's
proposal, design, eight delta specs and tasks.

**The whole checks fail: `conformance:test` finds a program named
outside a profile (core.1).** The work otherwise matches its specs and
every task is ticked, but the change is not archived: the failure goes
back to the code stage.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-roles-sleep-and-wake` ran
nothing. The sluice devenv pins, 0.71.0, refused the host's
`~/.config/sluice/config.toml` before starting a task:
`unknown field credential, expected one of app_id, key, discovery`
(exit 3). The host's own sluice, 0.95.1, read it, so the checks ran
once, through that sluice, as the earlier verifies on this bolt ran
them.

`sluice exec bolt/flumes-roles-sleep-and-wake` at 7307f55, the checks
7307f55 affects against the bolt's head 3ecc4f8: **failed**, 24 of 25
tasks completed and 1 failed, in 2m 48s (exit 1). sluice recorded 23
results under their keys for 7307f55.

Passed: `seams:format`, `seams:lint`, `seams:test`, `core:format`,
`core:lint`, `core:test`, `fakes:format`, `fakes:lint`, `fakes:test`,
`conformance:format`, `conformance:lint`, `conformance:coverage`,
`flume:format`, `flume:lint`, `flume:test` (628 tests),
`plugin-claude:test`, `nix:typecheck`, `nix:test`, `site:install`,
`site:prose`, `site:build`, `root:adrs`, `root:changes` and
`root:format`.

Failed: **`conformance:test`**. The engine's scenarios pass
(`the_engine_passes_its_scenarios ... ok`), but
`tests/no_tool.rs`'s `no_tool_is_named_outside_a_profile` fails:

```
a tool is named outside a profile (core.1):
crates/flume-conformance/../flume-core/src/prompt.rs:435: claude
```

The line is in task 7.1's unit test
`the_kinds_a_role_launches_follow_the_roles`, added by 13bff0d:

```rust
assert!(!KINDS_WORDS.contains("claude"), "a program named");
```

The assertion that flume's words name no program itself names one in
a core module, which core.1 rules out (core.1's details: the string
`claude` in any module outside a profile directory). The search reads
test modules as it reads the rest of the file.

sluice reported the failed task as one that could not run, "sluice's
sandbox refused the run file-write-data /Users/chuck/.cargo/.package-cache
(by cargo) … and 40 more", and recorded nothing for it. The task's own
output shows the test ran and failed as above, so the failure stands
whatever the sandbox's refusals.

`root:specs` is not among the tasks, since nothing under
`openspec/specs/` changed; `openspec validate
a-standing-role-is-a-hibernating-conversation-flume-keeps --strict`
reads the change valid. So the archive's own check is known before the
change comes back, the change was archived in a throwaway worktree at
7307f55, outside this branch and since removed, and `openspec validate
--specs --strict --no-interactive`, the command `root:specs` runs, read
**51 of 51 specs passing** over it, `standing-roles` among them with
its Purpose as the delta gives it. `check-tools` is main's layer and
was not run here.

## Tasks

All 21 tasks are ticked, none open. Each test or scenario a task names
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
| 7.1 | `the_kinds_a_role_launches_follow_the_roles` | `crates/flume-core/src/prompt.rs`, the failing line among it |
| 8.1, 8.2 | tidying's three | `engine/tidying.rs` |
| 9.1 | place-status' three | `engine/place_status.rs` |
| 9.2 | `a_standing_roles_place_and_its_checkouts` | `crates/flume/src/commands/print.rs` |
| 11.1 | `the_mcp_tools_listed`, `mcp_hibernate_refused_as_the_command_line_refuses_it`, `mcp_resume_refused_as_the_command_line_refuses_it` | `crates/flume/tests/commands.rs` |
| 12.1 | the site's four pages | `site:prose` and `site:build` passed |
| 13.1 | record 0069's construction choices | `root:adrs` passed |

## The specs

Every requirement and scenario of the eight delta specs has its test,
and each passed in the run above but for core.1's search, which no
scenario covers:

- **standing-roles**: all seven requirements, 17 scenarios, in
  `engine/standing_roles.rs`. `standing_role(kind)` and its wake's
  refusals are in `flume-core/src/standing_role.rs`, refused before
  anything is made; the checkouts are read at each repository's default
  branch where its merge queue says the base lives, the held state
  written before the place seam makes any; each start that opens an
  agent moves them in `Engine::open`, after the room is claimed and
  before the agent's command is built, one write recording every
  outcome, a move kept or gone or a base unread held as `unmoved` with
  its words and the start going on.
- **role-waking**: the run's entry's `repositories` and `kinds`, and
  the faults of a standing role's entry, in `crates/flume/tests/host.rs`
  over `host/roles.rs`; a carrier's wake of a hibernating run and
  nothing pending in `engine/role_waking.rs`. The modified
  requirement's two earlier scenarios stand as they were.
- **hibernation**: a delivery resuming a role whose place hibernates,
  `ResumedBy::Delivery`, and a role's place holding a batch not
  hibernated, five scenarios in `engine/hibernation.rs`.
- **seam-contracts**: the place seam's `make_checkouts`,
  `move_checkout` and `remove_checkouts`, run against the fake and the
  git worktree profile, and the types' round-trips.
- **place-profile**: `git -c core.hooksPath=/dev/null worktree add
  --quiet --detach`, `git -c core.hooksPath=/dev/null checkout --quiet
  --detach` with no `--force`, `git worktree remove` with no `--force`
  and no teardown, then the location and its group folder once empty;
  four scenarios over scratch repositories.
- **place-status**: `checkouts` and `AddressStatus::standing_role`,
  every field added; three scenarios, and the person's form in
  `print.rs`.
- **tidying**: a standing role's place tidied by its checkouts, kept
  whole while one holds anything, no branch removed; a wake undone by
  `remove_checkouts`; three scenarios.
- **claude-code-plugin**: `flume mcp` lists six tools, `hibernate` and
  `resume` not read-only, each run through the command line's parser
  and act; three scenarios against the built binary.

## What goes back to the code stage

1. `crates/flume-core/src/prompt.rs:435` names `claude` in a core
   module, failing `no_tool_is_named_outside_a_profile` (core.1). The
   test's intent, that flume's kinds words name no program, is what
   core.1's search already checks of `KINDS_WORDS`, so the assertion
   can go, or be made in a way that names no program in a core module.
   Verify with `sluice exec bolt/flumes-roles-sleep-and-wake --
   moon run conformance:test`.

The change is not archived.
