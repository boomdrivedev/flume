# Verify: flumes-change-kind-works-through-openspecs-own-commands

Verified on `change/flumes-change-kind-works-through-openspecs-own-commands`
at 5abf56ba, cut from `bolt/flume-builds-changes-through-openspec` at
514f1030. The base now stands at 7563bff9, and the branch merges onto it
with no conflict.

## The whole checks

`devenv shell -- sluice exec bolt/flume-builds-changes-through-openspec`
was run once and passed: sluice ran the checks 5abf56b affects against
the base at 7563bff, that is, the base's required layer, `#check-fast`
(`.config/sluice.toml`). All 13 checks passed, the run exited 0, and
sluice recorded their results under their keys for 5abf56b:

- `root:format`, `root:changes`
- `site:build` (after `site:install`)
- `nix:typecheck`, `nix:test`
- `plugin-claude:test`
- `conformance:format`, `conformance:lint`, `conformance:coverage`, `conformance:test`
- `flume:format`, `flume:lint`, `flume:test`

`root:changes` holds every open change to `openspec validate --strict`
and to archiving cleanly onto a scratch copy of the specs (core.59), and
it passed with this change open.

`#check-tools`, which holds the walkthroughs of tasks 4.1 and 4.2, is
main's layer and not this base's. It runs at the bolt's merge onto main.
The code stage ran those two tasks' own checks (d3f8ef1d, 7d7bd513).

## OpenSpec

- `openspec validate flumes-change-kind-works-through-openspecs-own-commands --strict`:
  the change is valid.
- `tasks.md`: all 18 tasks are ticked (1.1 to 1.7, 2.1 to 2.4, 3.1 to
  3.4, 4.1, 4.2, 5.1).

## The work against the specs

### change@5 as design 6 gives it

`crates/flume/src/profiles/kinds/weir/own/change@5/`'s `kind.toml`,
`prompts/change-construct.md`, `prompts/change-code.md` and
`prompts/change-verify.md` match design 6's blocks byte for byte. Each
file ends with one newline. `OWN` lists `flume/change@5` after change@3.
No file of change@1, change@2, change@3, fix@1 or chore@1 changed.

### kinds-profile

| Requirement / scenario | Met by |
|---|---|
| The kinds built in | `flumes_own_kinds_are_built_in`: change@5 on `change/`, its code not repeating, each of its stages' one agent `full` and every other version's no setting, each last stage's `on_fail` `code` |
| Each version's prompts are its own | `each_versions_prompts_are_its_own`: change@5's directory read, its three prompts none of change@1's, change@2's or change@3's |
| No prompt asks to submit | `no_prompt_of_flumes_own_asks_to_submit`: versions 1, 2, 3 and 5 rendered |
| A place launched at change@1 / change@2 | unchanged tests, passing |
| A place launched at change@3 | `a_place_launched_at_change_3_keeps_it`: stages, prompts and no mode as its launch read them; status names `flume/change@3` |
| A code prompt sent back with a conflict | `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing`: change@5's code prompt beside the other four |
| Packages, identities and locks; Flume's own as packages | `flumes_own_as_packages`: `flume/change@5` with its hash beside the other five |
| change@5's prompts name OpenSpec alone | `change_5s_prompts_name_openspec_alone` |
| change@5's prompts say their one way to ask | `change_5s_prompts_say_their_one_way_to_ask`: record 0064's sentence after `flume needs` |
| change@5's construct prompt rendered | `change_5s_construct_prompt_writes_through_openspec` |
| The strict check before the construct's commit | `change_5s_construct_prompt_commits_once_the_strict_check_passes` |
| change@5's code prompt rendered | `change_5s_code_prompt_works_through_the_apply_instructions` |
| A retry's words come last in change@5's code prompt | `change_5s_code_prompt_puts_a_retrys_words_last` |
| An archived change sent back to change@5's code | `change_5s_code_prompt_undoes_an_archive_whole` |
| change@5's verify prompt rendered | `change_5s_verify_prompt_archives_through_openspec`: the whole checks before the apply instructions and the strict check, `verify.md` before `openspec archive plan --yes` |
| An archive OpenSpec refuses | `change_5s_verify_prompt_archives_nothing_where_the_archive_refuses` |

### kind-sources

| Scenario | Met by |
|---|---|
| The hashes recorded | `OWN_HASHES` in `built_in.rs` gains change@5's hash after change@3's; the other five are unchanged in the diff |
| A lock of flume's own at another hash | unchanged scenario, passing |

### launching

| Scenario | Met by |
|---|---|
| No version named | `engine/launching.rs` `no_version_named` (change@5, its code not repeating, `flume/change@5`) and `the_identity_status_names`; `full` from the kind read by the kinds profile's test and the walkthrough `a_change_runs_construct_code_and_verify` (design 7, 8) |
| change@3 named | `change_3_named` |
| change@2 named / change@1 named | `change_2_named`, `change_1_named`, and `the_identity_status_names` |
| A version flume does not hold | `a_version_flume_does_not_hold`: `change@6` refused naming versions 1, 2, 3, 5, nothing made |
| A version the repository's declaration does not hold; a kind of a host's source named whole | unchanged scenarios, passing |

### running-through

| Scenario | Met by |
|---|---|
| A bare change runs change@3 through | `s119_one_agent_codes_the_change`, under a lock recording `flume/change@3`, citing 22, 154, 155, S119 |
| A bare change runs change@5 through | `change_5_codes_the_change_with_one_agent`, citing 22, 154 |

### The fakes, the list, the walkthroughs and the site

- `fake_harness.rs`: `flumes_own()` gains change@5 with design 7's three prompts word for word. They name no tool and give no mode (core.1).
- The kinds seam's `flumes_own` scenario reads change@1, change@2, change@3, change@5, chore@1 and fix@1.
- `engine/list.rs` reads `change@5` for a bare change's place.
- `walkthroughs.rs`: `a_change_runs_construct_code_and_verify` reads `flume/change@5`, code started once, and each stage's agent in `full` from `kind` passing `bypassPermissions`.
- `site/src/content/docs/guides/kinds.mdx` lists `flume/change@5`, names it the newest a bare `change` takes, and says how it works. It names OpenSpec only as a product flume integrates with (core.35). `site:build` passed.
- Record 0077, proposed, is in `docs/adr/`.

## Outcome

The whole checks pass, OpenSpec's strict check passes, every task is
done, and every requirement and scenario of the change's specs is met by
the work. The change is archived with `openspec archive
flumes-change-kind-works-through-openspecs-own-commands --yes`.
