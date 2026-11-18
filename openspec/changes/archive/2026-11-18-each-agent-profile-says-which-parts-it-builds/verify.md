# Verify: each-agent-profile-says-which-parts-it-builds

Verified on `change/each-agent-profile-says-which-parts-it-builds` at
16a5fab1, cut from `bolt/flume-runs-every-agent-its-terminal-can-start`
at e9b23ef9. The base still stands at e9b23ef9, so the branch is ahead
of it with nothing to merge.

## The whole checks

`devenv shell -- sluice exec bolt/flume-runs-every-agent-its-terminal-can-start`
was run once and passed. sluice ran the checks that 16a5fab affects
against the base at e9b23ef, which is the base's required layer,
`#check-fast` (`.config/sluice.toml`). All 23 moon tasks completed, the
run exited 0, and sluice recorded 22 results under their keys for
16a5fab:

- `root:format`, `root:changes`
- `site:install`, `site:build`
- `nix:typecheck`, `nix:test`
- `plugin-claude:test`
- `seams:format`, `seams:lint`, `seams:test`
- `core:format`, `core:lint`, `core:test`
- `fakes:format`, `fakes:lint`, `fakes:test`
- `conformance:format`, `conformance:lint`, `conformance:coverage`, `conformance:test`
- `flume:format`, `flume:lint`, `flume:test`

`root:changes` holds every open change to `openspec validate --strict`
and checks that each archives cleanly onto a scratch copy of the specs
(core.59). It passed with this change open. `conformance:coverage`
passed with 174 cited and 170 to 173 and S71 to S75 named unbuilt.
moon logged warnings that it could not archive task outputs into
`.moon/cache` under the sandbox. These cost the run only the cache and
failed no task.

`#check-tools` is main's layer, not this base's. It runs at the bolt's
merge onto main. Task 4.2 only needs its stand-ins to compile, under
`--features tools`.

## OpenSpec

- `openspec validate each-agent-profile-says-which-parts-it-builds --strict`:
  the change is valid.
- `tasks.md`: all 10 tasks are ticked (1.1, 2.1, 3.1, 3.2, 4.1 to 4.3,
  5.1 to 5.3).

## The work against the specs

### seam-contracts

| Requirement / scenario | Met by |
|---|---|
| The agent seam says which parts its profile builds | `crates/flume-seams/src/agent.rs`: `Agent::parts()` defaults to `Parts::none()`. `send_back`, `on_quiet`, `reports_windows` and `mode_read_back` are provided readings of `parts()`, offered where it builds `SendBack`, `OnQuiet`, `Windows` and `Mode`. No profile in the tree overrides them. |
| A profile that builds no part | `round_trip::a_profile_that_builds_no_part` over `Bare`: all eight parts are degraded in 171's order, each with its reading, and none of the four is offered. Spent and windows read unknown, guard and carry are not offered, `own` refuses and `reserved` is empty. |
| A profile's answers agree with its parts | `seams/agent.rs` `answers_agree_with_the_parts`, citing 174 and 93. It runs under the fake's default, `carrying_none`, `taking_no_own`, `reporting_no_windows` and `offering_no_hooks` harnesses (`tests/seams.rs`) and under Claude Code's profile. The windows scenario cites 174 beside 211. |
| The type of a part | `Part`, serialized in `snake_case`, with `Part::ALL`, `name()` and `read()`, and the design's eight sentences word for word |
| Each part written and read | `round_trip::each_part_written_and_read`: each part round-trips through its word, and `colour` is refused naming it |
| A part's name and its lesser reading | `round_trip::a_parts_name_and_its_lesser_reading`: `send-back` and `own settings` are named, and carry's reading is checked |

### seam-fakes

| Requirement / scenario | Met by |
|---|---|
| The fake agent's parts follow its scripts | `FakeAgent::parts()` in `crates/flume-fakes/src/agent.rs` reads each part from its script, as design 5 says. The fake no longer answers the four itself. |
| The fake scripted as each harness scripts it | `fakes.rs` `each_script_degrades_its_own_part`: every part is built by default, and each script degrades only its own part |
| The fake as a new fake makes it | `fakes.rs` `a_new_fakes_parts`: mode, carry and own settings are degraded |

### agent-profile

| Requirement / scenario | Met by |
|---|---|
| Claude Code's profile builds every part | `profiles/agent/claude/mod.rs` `parts()` names each of the eight in turn, never `Part::ALL`. Its own four answers are dropped. |
| Claude Code's parts | `agent_claude::the_profile_builds_every_part`: none is degraded, and all four are offered |
| The seam's parts scenarios | `agent_claude::the_profile_passes_the_agent_scenarios`, which now includes `answers_agree_with_the_parts` |

### place-status

| Requirement / scenario | Met by |
|---|---|
| Status names each degraded part of an agent | `AgentStatus::degraded`, a list of `DegradedPart { part, read }`. It is filled in `status.rs` from the bound profile's `parts().degraded()` and left empty where no profile binds the program. Serde's default reads it, and it is skipped in the JSON when empty (core.14). The answers' `FIELDS` list gains `agent.degraded`, `degraded[].part` and `degraded[].read` for status and launched. |
| A profile that offers no hooks | `engine/place_status.rs` `a_profile_that_offers_no_hooks`, citing 56 and 174. The JSON gives `send_back`, `guard` and `on_quiet` in order, each with its reading. In the person's form, `print.rs` `an_agents_degraded_parts` gives each part a line, `    send-back degraded: not offered, so it may go quiet with its deliverable missing` and the next two, after the agent's notes. |
| A profile that builds every part | `a_profile_that_builds_every_part`: the JSON holds no `degraded`. `an_agents_degraded_parts` shows that an agent with no degraded part gets no such line. |

### clause-coverage

| Requirement / scenario | Met by |
|---|---|
| The clauses listed | `clauses.rs`: `REVISION` is 48a94da and the scenarios run to S131. `PROFILES` names 170 to 173 and S71 to S75, record 0026's common profile, and no longer 174. `the_clauses_unbuilt_are_named_by_the_changes_that_build_them` counts nine for it and lists 174 among the clauses built. |
| A clause left out | Unchanged scenario, passing |
| The suite as this change leaves it | `conformance:coverage` passed. 174 is cited by the parts scenarios, the windows scenario and status's two scenarios. |

### The stand-ins, the site and the record

- The stand-in agents in `tests/commands.rs`, `terminal_herdr.rs` and
  `terminal_rex.rs` drop their own `send_back` and `on_quiet`, so each
  builds no part. `flume:test` and `flume:lint` passed.
- `site/src/content/docs/guides/claude-code.mdx` says Claude Code's
  profile builds every part, so status names none of its agents
  degraded. `reference/state.md`'s `agent` row gains `degraded`, with
  its eight words, `part` and `read`, left out where none is. It names
  Claude Code only as a product flume integrates with (core.35).
  `site:build` passed with every internal link valid.
- Record 0026 is amended on 2026-11-18 with the construct's choices,
  each with the path not taken. It is still accepted, and rule core.39
  already holds the declaration.

## Outcome

The whole checks pass, OpenSpec's strict check passes, every task is
done, and the work meets every requirement and scenario of the change's
specs. The change is archived with `openspec archive
each-agent-profile-says-which-parts-it-builds --yes`.
