# Tasks

The groups are headings for reading. Each task names the narrowest check
that shows it done, run as written; no task runs the repository's whole
checks or a group's, which the change's verify stage and the merge queue
run (record 0054). The statement carries change@5 at
boomdrivedev/blueprints 2e46f6e, on the branch
`design/flume-change-kind-works-through-openspecs-own-commands`, and
record 0077, proposed, is written with this plan. Design sections are
this change's `design.md`.

## 1. change@5 built in

- [x] 1.1 Add `crates/flume/src/profiles/kinds/weir/own/change@5/kind.toml` and its `prompts/change-construct.md`, `prompts/change-code.md` and `prompts/change-verify.md`, each exactly as design 6 gives them; list `flume/change@5` in `OWN` in `crates/flume/src/profiles/kinds/weir/mod.rs` after change@3, its doc comment naming change@1, change@2, change@3 and change@5 side by side, the last the newest (records 0054, 0064, 0077); and have `each_versions_prompts_are_its_own` in `crates/flume/tests/profiles/kinds_weir.rs` read change@5's directory too, its three prompts none of an earlier version's (22, 156, 157, C.2). Check: `devenv shell -- cargo test -p flume --test profiles each_versions_prompts_are_its_own`.
- [x] 1.2 Have `flumes_own_kinds_are_built_in` in `kinds_weir.rs` read change@5 on `change/` with construct, code and verify, its code stage not repeating, each of its stages giving its one agent the mode `full` and no other version's stage giving one, landing, its verify's `on_fail` `code` (22, 140, 162, C.2). Check: `devenv shell -- cargo test -p flume --test profiles flumes_own_kinds_are_built_in`.
- [x] 1.3 Add change@5's hash to `OWN_HASHES` in `crates/flume/tests/profiles/built_in.rs`, after change@3's, the other five as they are (156). Check: `devenv shell -- cargo test -p flume --test profiles the_hashes_recorded`.
- [ ] 1.4 Have `flumes_own_as_packages` in `kinds_weir.rs` read `flume/change@5` with its hash beside the other five (156, 157). Check: `devenv shell -- cargo test -p flume --test profiles flumes_own_as_packages`.
- [ ] 1.5 Have `no_prompt_of_flumes_own_asks_to_submit` in `kinds_weir.rs` render change@5's three prompts for a place beside the other versions' (28, 71, 112). Check: `devenv shell -- cargo test -p flume --test profiles no_prompt_of_flumes_own_asks_to_submit`.
- [ ] 1.6 Have `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing` in `kinds_weir.rs` render change@5's code prompt with a conflict's words beside fix@1's, chore@1's, change@2's and change@3's (112). Check: `devenv shell -- cargo test -p flume --test profiles a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing`.
- [ ] 1.7 Add `a_place_launched_at_change_3_keeps_it` to `kinds_weir.rs`: a place launched by a flume holding change@1, change@2 and change@3 alone, settled by one holding change@5 too, keeps change@3's stages and prompts as its launch read them, its kind giving its agents no mode, and status names change@3 (21). Check: `devenv shell -- cargo test -p flume --test profiles a_place_launched_at_change_3_keeps_it`.

## 2. change@5's prompts

- [ ] 2.1 Add to `kinds_weir.rs` a `change_5_rendered` helper beside `change_3_rendered`, a place named `plan` carried to its verify by a flume holding change@5 with its three prompts opened, and the tests `change_5s_prompts_name_openspec_alone` and `change_5s_prompts_say_their_one_way_to_ask`, the kinds-profile spec's two scenarios of "change@5's prompts run OpenSpec and ask only by stopping short" (28, 158, C.3; record 0064). Check: `devenv shell -- cargo test -p flume --test profiles change_5s_prompts_`.
- [ ] 2.2 Add `change_5s_construct_prompt_writes_through_openspec` and `change_5s_construct_prompt_commits_once_the_strict_check_passes` to `kinds_weir.rs`, the spec's two scenarios of change@5's construct, the second finding `openspec validate plan --strict` before the commit it asks for (22, C.2, C.3). Check: `devenv shell -- cargo test -p flume --test profiles change_5s_construct_prompt_`.
- [ ] 2.3 Add `change_5s_code_prompt_works_through_the_apply_instructions`, `change_5s_code_prompt_puts_a_retrys_words_last` and `change_5s_code_prompt_undoes_an_archive_whole` to `kinds_weir.rs`, the spec's three scenarios of change@5's code, the second as `change_3s_code_prompt_puts_a_retrys_words_last` renders change@3's (22, 131, C.2, C.3; core.33). Check: `devenv shell -- cargo test -p flume --test profiles change_5s_code_prompt_`.
- [ ] 2.4 Add `change_5s_verify_prompt_archives_through_openspec` and `change_5s_verify_prompt_archives_nothing_where_the_archive_refuses` to `kinds_weir.rs`, the spec's two scenarios of change@5's verify, the first finding the whole checks before `openspec instructions apply` and `openspec archive plan --yes` after `verify.md` (22, 131, C.2, C.3). Check: `devenv shell -- cargo test -p flume --test profiles change_5s_verify_prompt_`.

## 3. The fakes and the engine's scenarios

- [ ] 3.1 Give `flumes_own()` in `crates/flume-conformance/src/fake_harness.rs` change@5 after change@3, as design 7 gives it, its doc comment naming the five versions side by side; and have the kinds seam's `flumes_own` scenario in `crates/flume-conformance/src/seams/kinds.rs` read fix@1, chore@1, change@1, change@2, change@3 and change@5, change@2's code alone repeating (22, 140, 157; core.1). Check: `devenv shell -- cargo test -p flume-conformance --test seams the_fake_kinds_pass_the_kinds_scenarios`.
- [ ] 3.2 Have the kinds profile pass the kinds seam's scenarios with change@5 built in (22, 157). Check: `devenv shell -- cargo test -p flume --test profiles the_kinds_profile_passes_the_kinds_scenarios`.
- [ ] 3.3 Have `crates/flume-conformance/src/engine/launching.rs` hold the launching spec's scenarios of "A launch may name its kind's version": a bare `change` runs change@5, its code stage one agent, and status names `flume/change@5`; `change@3`, `change@2` and `change@1` named run their versions; and `change@6` is refused naming the versions held, 1, 2, 3 and 5, nothing made (4, 19, 22, 154); and have `engine/list.rs` read `change@5` for a bare change's place, as design 8 has it. Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [ ] 3.4 Have the change@3 scenario of `crates/flume-conformance/src/engine/running_through.rs` launch `change` in a repository whose lock records `flume/change@3`, citing 22, 154, 155 and S119; and add the running-through spec's scenario of a bare `change` running change@5 through, citing 22 and 154: its construct writing four task groups, its code stage one start whose status and line give `code` with no group count, and the place submitted once verify archives the change (design 8). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.

## 4. The walkthroughs

- [ ] 4.1 Have `a_change_runs_construct_code_and_verify` in `crates/flume/tests/profiles/tools/walkthroughs.rs` keep its bare `change` and read that the place runs `flume/change@5`, its code stage started once and its agents started in the mode `full` from `kind`, and have the module's doc comment name change@5 as flume's newest (S2, S35, 162). Check: `devenv shell -- cargo test -p flume --features tools --test tools a_change_runs_construct_code_and_verify`.
- [ ] 4.2 Keep `verify_finding_work_sends_the_place_back_to_code`, `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start` and `s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run` in `walkthroughs.rs`, which launch a bare `change`, passing with change@5 and its agents in `full` (131, 145, 162, S93). Check: `devenv shell -- cargo test -p flume --features tools --test tools -- verify_finding_work_sends_the_place_back_to_code a_pool_with_no_account_of_a_stages_program s93_a_question_for_easts_ops`.

## 5. The site

- [ ] 5.1 In `site/src/content/docs/guides/kinds.mdx`, list `flume/change@5` among flume's own kinds, say a bare `change` takes the newest, `change@5`, unless a lock records another, and say in a sentence how change@5 works, as design 9 has it, to any host's operator (22, 154; core.35). Check: `sluice exec <base> -- moon run site:build`.
