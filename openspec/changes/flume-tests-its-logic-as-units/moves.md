# Moves

One row for each test under `crates/flume/tests/` but the live
walkthroughs. "Planned home" is where the test's behaviour is proven
after its step. "Proven by" names the test that proves it, and its step
writes it before the step lands. A row that says "stays" is a test of
the end-to-end set.

## `walkthroughs` (42 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `a_fix_lands_through_flumes_own_submission` | stays: end-to-end | itself |
| `a_fix_lands_when_its_prompt_is_submitted_inside_paste_tags` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_fix_agents_line_is_status_line_and_the_fix_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_change_code_agents_line_names_its_group_its_run_and_its_tasks` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_failed_submission_goes_back_to_code_and_lands_on_the_second` | stays: end-to-end | itself |
| `a_conflicting_submission_merges_the_base_and_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_conflict_sluice_answers_at_the_submit_goes_back_to_code_and_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_pull_request_is_followed_from_no_entry_to_its_entry_and_its_landing` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_pull_requests_failure_goes_back_to_code_once_and_its_next_request_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_change_runs_construct_code_and_verify` | stays: end-to-end | itself |
| `verify_finding_work_sends_the_place_back_to_code` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_fixs_code_finding_work_waits_on_an_answer_and_its_retry_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_bound_host_queues_launches_and_starts_them_by_priority` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_stage_waiting_for_room_starts_at_the_next_settle_once_the_host_gives_room` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_change_of_three_task_groups_takes_three_agents_in_turn` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_failed_submission_sends_a_repeating_stage_back_for_one_repetition_with_no_group` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_spent_account_moves_the_stage_to_the_pools_next_and_the_fix_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_spent_agents_conversation_goes_on_under_the_pools_next_and_the_fix_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_most_left_pool_starts_on_the_account_with_most_left_and_none_past_its_threshold` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_start_without_its_account_is_ended_and_its_stage_started_afresh` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_fix_begun_in_another_mode_twice_is_never_begun` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `an_agent_run_after_its_start_was_read_never_begun_is_begun_and_a_retry_leaves_it` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_hosts_people_file_its_lines_and_a_repositorys_file_reach_each_streams_agents` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `own_settings_that_are_reserved_or_hold_a_credential_or_cannot_be_read_refuse_the_launch` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_forks_attribution_is_the_hosts_a_repositorys_its_own_and_another_its_accounts` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_kinds_effort_and_a_launchs_for_review_start_each_agent_at_its_own_and_the_place_lands` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_standing_agent_under_herdr_takes_a_fixs_end_through_the_binary` | stays: end-to-end | itself |
| `a_standing_agent_under_herdr_confirms_its_end_naming_no_recipient` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_talk_resumed_by_hand_waits_at_its_retry_and_goes_on_in_its_session_once_it_ends` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_lost_talk_whose_transcript_is_gone_starts_afresh_with_flumes_words` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run` | stays: end-to-end | itself |
| `s94_a_restarted_session_is_given_its_place_and_the_stall_reaches_the_door` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_fix_on_a_stream_under_rex_lands_through_the_binary` | stays: end-to-end | itself |
| `flume_open_of_a_rex_streams_session_names_the_socket_to_add` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `s126_a_role_at_work_takes_two_messages_and_an_end_as_one_prompt` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `s128_a_roles_run_launches_two_places_that_report_to_its_address` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_person_reaches_a_fixs_agent_through_a_relay` | stays: end-to-end | itself |
| `a_person_reaches_a_fixs_agent_under_rex_through_a_relay` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |
| `a_standing_roles_session_takes_a_message_through_its_deliver_part_its_draft_left_as_typed` | the conformance scenario of the behaviour, and the end-to-end test of its seam | (its step) |

## `commands` (72 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `stand_in_flume` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_same_answer_through_the_command_line_and_the_core` | stays: end-to-end | itself |
| `status_with_binding_gives_the_repositorys_binding_through_flume_host` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_launch_naming_no_base_is_cut_from_the_remotes_default_branch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_base_no_merge_queue_lands_is_cut_from_origins_copy` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_repository_with_no_origin_is_cut_from_its_own_base` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_base_the_merge_queue_lands_is_read_where_it_says` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_refusal` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `hibernation_through_the_binary_a_hibernate_of_a_fix_is_refused` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `hibernation_through_the_binary_a_resume_of_nothing_is_refused` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_watch_over_a_scratch_repository` | stays: end-to-end | itself |
| `a_watch_with_a_place_named` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `needs_from_outside_an_agent` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `needs_help` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `holds_and_recipients_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_hold_after_the_launch_and_a_stall_bound` | stays: end-to-end | itself |
| `agent_settings_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_session_for_every_stage_and_for_one_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `an_effort_for_every_stage_and_for_one_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_settings_agent_gives_are_named_in_its_refusal` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_mode_and_a_table_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `own_settings_and_a_file_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_stream_and_an_intent_at_the_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_launch_takes_its_stream_by_its_repositorys_address` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_launch_given_an_address_finds_no_clone` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `host_settings_and_an_account_for_a_kind` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `no_environment_value_is_printed_or_kept` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_place_named_from_inside_it` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `every_place_is_made_under_flumes_own_data_directory` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_place_made_in_the_repositorys_directory_is_read_and_tidied_where_it_stands` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `status_over_mcp` | stays: end-to-end | itself |
| `one_places_line_under_the_printed_command` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_list_through_the_command_line_and_the_core` | stays: end-to-end | itself |
| `a_dropped_place_is_listed_only_with_all` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_fix_under_the_printed_command_is_listed_outside_any_repository_running_no_tool` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `fifty_places_are_listed_within_a_second` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_list_for_a_person_under_the_printed_command` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_lists_help_names_status_for_a_places_every_fact` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_wait_exits_by_its_outcome` | stays: end-to-end | itself |
| `the_recipient_a_wait_and_an_inbox_default_to` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_wake_of_a_roles_address` | stays: end-to-end | itself |
| `the_wakes_help_names_a_stream_and_a_role` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `needs_to_a_role_and_to_another_streams` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_helps_of_needs_hold_retry_message_and_launch` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `kinds_lock_writes_the_hosts_lock` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `kinds_lock_writes_a_working_copys_lock` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `kinds_lock_refuses_a_hash_the_content_does_not_have` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `kinds_lock_refuses_an_identity_no_lock_records` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `kinds_lock_two_writers_at_once_and_a_removal` | stays: end-to-end | itself |
| `unlocked_then_locked_by_the_repository` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_lock_that_cannot_be_read` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_source_file_with_no_kinds_dir` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `s83_the_local_layer_through_the_binary` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `flume_writes_neither_configuration_file` | stays: end-to-end | itself |
| `open_under_the_printed_command` | stays: end-to-end | itself |
| `open_of_an_undeclared_stream` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_name_no_streams_word_gives` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_keys_and_none_in_status` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `an_unknown_terminal_app` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_streams_multiplexer_flume_does_not_carry` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_launch_from_a_rex_block_names_its_block` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_readers_place_collected_by_its_inbox` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_key_confirmed_through_the_binary` | stays: end-to-end | itself |
| `a_key_refused_through_the_binary` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `the_helps_of_confirm_and_wait` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `an_answer_naming_no_decision` | stays: end-to-end | itself |
| `the_mcp_tools_listed` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `mcp_hibernate_refused_as_the_command_line_refuses_it` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `mcp_resume_refused_as_the_command_line_refuses_it` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `message_over_mcp` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `launch_over_mcp_refused_as_the_command_line_refuses_it` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |
| `a_role_runs_launch_over_mcp_reports_to_its_address` | the conformance scenario of the act, or a unit test in `crates/flume/src/commands/` for its words and print | (its step) |

## `terminal_herdr` (62 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_herdr_profile_passes_the_terminal_scenarios` | stays: end-to-end | itself |
| `s39_the_folder_trust_question_is_answered_and_the_agent_comes_up` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s39_a_folder_trust_question_drawn_late_is_answered_from_the_screen` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s39_a_folder_trust_question_drawn_after_herdr_reads_ready_is_answered` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s39_a_folder_trust_question_drawn_after_the_open_is_answered_at_a_settle` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s39_a_folder_trust_question_herdr_does_not_see_is_answered_from_the_screen` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s40_the_bypass_warning_holds_the_start_with_its_lines` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s45_a_program_that_never_comes_up_is_not_come_up_with_the_screen` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_role_runs_pane_moved_to_another_workspace_is_reached_by_its_address` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_recipient_on_another_server_is_sent_to_through_its_own_socket` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_recipient_at_work_is_typed_nothing` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s46_a_pane_moved_to_another_workspace_is_found_by_its_agents_name` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `the_environment_is_set_on_the_pane_and_nothing_is_typed` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_nix_wrapped_claude_starts_under_herdr_from_the_agent_profile` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `instructions_of_many_lines_start_under_herdr_by_the_agent_profiles_file` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `startup_files_that_put_another_claude_first_leave_the_program_the_settings_name` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_program_that_runs_another_in_its_place_is_refused_naming_it` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_herdr_name_taken_refuses_the_open` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s47_the_last_place_on_a_base_closes_its_tab_and_workspace` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `places_on_two_bases_open_in_two_workspaces` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s48_a_persons_pane_keeps_the_workspace_named` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s49_titles_follow_the_stage_and_a_clue_that_fails_fails_nothing` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_conversation_opens_beside_a_named_pane` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `session_child` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_command_run_in_a_herdr_pane_names_its_pane` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_command_in_a_herdr_sessions_pane_is_named_as_its_agents_are_given` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `herdrs_readings_as_the_profile_reads_them` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s43_a_prompt_left_typed_is_cleared_before_each_sending` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_pane_herdr_could_not_bring_back_reads_stopped` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `instructions_of_twenty_thousand_characters_reach_the_agent_whole` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_start_the_terminal_cannot_yet_take_is_asked_again_until_it_can` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_shell_never_at_its_prompt_is_not_come_up_and_never_refused` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_herdr_call_that_never_answers_is_refused_within_its_bound` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `the_row_follows_the_line` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_quiet_pane_with_nothing_delivered_is_not_ended` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s42_a_finished_turn_nobody_looked_at_delivers_and_the_next_stage_starts` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_launch_and_a_retry_each_send_their_agents_prompt_with_no_settle` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_dialog_before_the_prompt_waits_on_a_person_with_its_words` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `an_agent_that_began_after_its_start_read_never_begun_is_begun_and_a_retry_leaves_it` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_startups_setup_is_given_by_status_while_its_server_runs` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `unattended_by_the_hosts_watch_alone` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `the_pinned_herdr_keeps_its_named_sessions_under_the_session_home` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `s82_two_streams_sessions_under_herdr` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_session_name_herdr_refuses_refuses_the_open_in_its_words` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_place_tidied_in_two_sessions_closes_its_tab_and_workspace_in_each` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_place_recorded_before_records_were_kept_by_server_is_tidied_as_before` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_streams_session_not_yet_running_is_started_for_attach` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_client_the_command_runs_attaches_to_its_session` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `the_own_session_is_attached_by_its_name_or_refused_by_its_socket` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_name_herdr_refuses_refuses_attach_in_its_words` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `herdrs_app_is_refused_and_starts_no_server` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `runs_of_a_session_not_listed_reads_not_running_and_leaves_it_so` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `runs_reads_a_running_session_and_the_own_session_by_its_name` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `runs_of_several_sessions_reads_them_in_one_listing` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `runs_of_a_herdr_that_does_not_answer_is_unread_within_two_seconds` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `runs_of_an_own_server_held_by_its_socket_reads_the_session_listed_there` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_standing_agent_is_found_by_its_name_and_reached` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_session_that_does_not_run_holds_no_agent_and_is_not_started` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `an_agent_herdr_lists_with_no_name_is_not_given` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `a_standing_agent_gone_is_not_given` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `agents_in_of_a_herdr_that_refuses_or_does_not_answer_is_unread_in_its_words` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |
| `relay_types_each_of_flumes_key_names_into_a_pane` | a unit test in `crates/flume/src/profiles/terminal/herdr/` over recorded answers | (its step) |

## `kinds_weir` (78 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_kinds_profile_passes_the_kinds_scenarios` | stays: end-to-end | itself |
| `a_declaration_weir_refuses_names_every_field_that_fails` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_prompt_the_revision_lacks_is_named` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repository_with_no_declaration_declares_no_kind` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repositorys_settings_for_a_kind_it_does_not_declare_and_a_kinds_own` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_kind_that_lands_with_no_stages_beside_one_that_does_not_and_settings` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_kind_with_no_stages_names_its_landing_fix_and_its_bound` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_landing_fix_weir_refuses_is_named_in_weirs_words` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_landing_fix_bound_past_what_flume_counts_is_refused` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `fix_and_max_fixes_are_sections_naming_no_landing_fix` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `s13_a_place_keeps_the_kind_its_launch_resolved` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `s33_a_repository_with_no_table_refuses_a_fix_naming_the_merge_queue_it_needs` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_base_places_launch_holds_the_landing_fix_the_profile_reads` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_landing_fix_at_a_version_not_declared_refuses_the_launch` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repeating_stage_is_read_through_weirs_crate` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repeat_on_a_stage_of_commits_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_environment_in_any_declaration_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_mode_for_every_stage_and_a_table_for_one` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_effort_for_every_stage_and_one_for_a_stage` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_effort_for_every_kind_of_a_repository` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_effort_that_is_no_word_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_session_in_an_agents_table_gives_no_setting` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_mode_that_is_neither_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `own_settings_for_every_stage_and_for_one` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_launchs_declaration_reads_its_settings_file_on_disk` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_entry_flume_cannot_take_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_settings_file_outside_the_repository_or_not_at_the_revision_refuses_the_launch` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repositorys_agents_for_every_kind` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repositorys_agents_table_naming_a_stage_or_an_environment_refuses_the_launch` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repositorys_keep_open_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_repositorys_hibernate_after_refuses_the_launch_naming_the_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `hibernate_through_weir_a_conversation_read` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `hibernate_through_weir_flumes_own_kinds_declare_none` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_packages_keep_open_refuses_the_launch_naming_the_stages_field` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_worktrunk_binding_is_refused_naming_the_place_profiles_there_are` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `an_elaboration_section_is_ignored` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_binding_is_read_at_origin_head_after_one_fetch` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_binding_held_is_read_at_origin_as_last_fetched_and_fetches_nothing` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_binding_held_with_no_origin_is_the_own_branch_and_may_have_no_declaration` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_binding_is_read_beside_a_bare_repository` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_binding_is_read_at_the_remotes_default_branch_beside_a_bare_repository` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `flumes_own_repository_binds_git_worktree_openspec_and_sluice` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `flumes_own_kinds_are_built_in` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `each_versions_prompts_are_its_own` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `no_prompt_of_flumes_own_asks_to_submit` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_newest_own_prompts_say_their_one_way_to_ask` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `flumes_newest_own_kinds_give_full_from_the_kind` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_3s_construct_prompt_gives_each_task_its_own_check` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_3s_code_prompt_runs_each_tasks_own_check_alone` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_3s_code_prompt_puts_a_retrys_words_last` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_3s_verify_prompt_runs_the_whole_checks_once_first` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_prompts_name_openspec_alone` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_prompts_say_their_one_way_to_ask` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_construct_prompt_writes_through_openspec` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_construct_prompt_commits_once_the_strict_check_passes` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_code_prompt_works_through_the_apply_instructions` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_code_prompt_puts_a_retrys_words_last` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_code_prompt_undoes_an_archive_whole` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_verify_prompt_archives_through_openspec` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `change_5s_verify_prompt_archives_nothing_where_the_archive_refuses` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `two_repetitions_prompts_are_equal_up_to_the_group` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_place_launched_at_change_1_keeps_it` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_place_launched_at_change_2_keeps_it` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_place_launched_at_change_3_keeps_it` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_packages_hash_wherever_it_lies` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_packages_kind_names_its_landing_fix` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_sources_listing` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_repositorys_lock_at_a_revision` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_lock_written_and_read_back` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `flumes_own_as_packages` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_package_at_another_identity_is_refused_naming_both_versions` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_part_outside_the_package_an_environment_and_a_program_by_a_location` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_packages_settings_file_is_read_within_it_and_never_outside` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_statechart_kind_is_read_through_weirs_crate` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_statechart_stages_prompt_the_revision_lacks_is_named` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `the_stage_template_names_the_atoms_flume_binds` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |
| `a_held_statechart_reads_back_the_same` | a unit test in `crates/flume/src/profiles/kinds/weir/` over declarations in a scratch folder | (its step) |

## `terminal_rex` (38 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_pinned_rex_runs_at_api_revision_twelve_or_later` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_rex_call_that_never_answers_is_refused_within_its_bound` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_call_from_inside_another_servers_block_reaches_its_own_server_alone` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_sessions_server_is_started_where_none_answers` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_server_started_again_restores_nothing` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `two_opens_of_one_identity_leave_one_block` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `places_on_two_bases_open_in_two_rex_sessions` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_second_agent_of_a_place_splits_its_window` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `an_agent_opens_beside_a_named_block` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `no_variable_of_the_agent_stands_in_any_processs_arguments` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_program_that_never_reports_is_never_begun_with_the_screen` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_program_that_exits_as_it_starts_is_never_begun_at_once` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_program_that_runs_another_in_its_place_is_refused_naming_it` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `s39_the_folder_trust_question_is_answered_and_the_agent_comes_up_under_rex` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `s39_a_folder_trust_question_drawn_after_the_open_is_answered_at_a_settle_under_rex` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `s43_a_prompt_left_typed_is_cleared_before_each_sending_under_rex` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_working_agent_is_sent_a_tell_with_no_ctrl_c` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_text_past_sixty_thousand_bytes_arrives_whole` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `rexs_readings_as_the_profile_reads_them` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `the_screen_is_the_blocks_last_rows` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `relay_gives_the_blocks_screen_and_not_its_scrollback` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `relay_types_each_of_flumes_key_names_into_a_block` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_close_rex_cannot_make_is_refused` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `s47_the_last_place_on_a_base_closes_its_window_and_rex_session` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `s48_a_persons_block_keeps_the_rex_session_named` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_recipient_block_is_sent_to_and_read_by_its_name` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_recipient_block_at_work_is_typed_nothing` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_role_runs_address_reaches_its_block` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_standing_agent_is_given_by_its_label` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `rex_session_child` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_command_in_a_labelled_block_is_named_as_its_agent_is_given` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `runs_reads_three_parts_at_once_and_starts_none` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `show_labels_the_places_window_and_its_bases_rex_session` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `show_passes_over_a_window_rex_closed_and_fails_naming_a_server_that_does_not_answer` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `attach_for_the_app_names_the_socket_to_add_where_no_app_is_a_client` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `a_done_agent_with_nothing_delivered_waits_and_no_settle_ends_it` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |
| `the_rex_profile_passes_the_terminal_scenarios` | stays: end-to-end | itself |
| `the_stand_in_reports_its_state_to_rex` | a unit test in `crates/flume/src/profiles/terminal/rex/` over recorded answers | (its step) |

## `agent_claude` (45 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_profile_passes_the_agent_scenarios` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_code_stage` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_passes_the_mode_it_is_given` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_stage_with_no_effort` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_stage_at_xhigh` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `each_of_the_five_and_two_the_release_reads_otherwise` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_profile_names_claude_config_dir_as_its_home` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_stage_in_edits` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_stage_with_own_settings` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_for_a_stage_with_none` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `each_reserved_key` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_peoples_settings_file` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `effort_level_written_in_a_table` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `effort_level_in_a_named_file` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `an_effort_the_accounts_variable_outranks` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `keys_taken_as_written` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_repositorys_tagline_written_as_given` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `each_of_the_four` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `its_own_value_under_its_name` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_mode_read_back` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_words_a_fixs_code_is_told` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_resolver_binds_the_program_by_name_or_path` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_plugin_an_agent_loads` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_plugin_carries_flumes_version` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_prompt_queued_while_the_agent_worked` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_prompt_inside_paste_tags_is_the_prompt_sent` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_slash_command_prompt` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `own_work_ends_with_its_tasks_notification` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_question_answered_in_the_transcript_is_no_longer_asked` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_trust_screen` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_command_of_a_carried_start` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_conversation_carried_to_a_home_whose_projects_links_the_firsts` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `two_homes_of_their_own` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `the_conversation_is_the_session_its_started_word_names` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_start_that_never_said_it_started` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_background_task_the_stopped_agent_left` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `last_words_before_the_carried_agent_wrote_any` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_question_and_a_slash_command_of_the_stopped_agent` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_fresh_start_reads_its_whole_transcript` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_transcript_read_from_its_length` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_lost_starts_conversation_carried_to_its_own_home` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_session_record_and_its_process` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_reused_process_id` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_resume_under_a_home_not_given` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |
| `a_sessions_folder_that_cannot_be_read` | a unit test in `crates/flume/src/profiles/agent/claude/` over fixture files | (its step) |

## `hooks_claude` (44 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_started_and_received_acks` | stays: end-to-end | itself |
| `a_variable_with_another_value` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_credential_the_program_keeps_from_its_hooks` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `another_program` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `another_mode` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `manual_read_back_as_default` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `no_mode_in_the_record` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `no_mode_in_the_input` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_later_prompt_and_a_compaction_after_the_mode_moved` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `an_ack_by_hand_with_no_mode` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_prompt_submitted_as_a_paste_is_acked_as_sent` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_subagents_input_acks_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_bare_resume_acks_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_plugin_of_another_version` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `an_agent_tries_to_push` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_subagents_push_is_refused` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `an_agent_merges_the_base_for_a_conflict` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_dismissed_question_clears_its_mark` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_permission_prompt_is_asking` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_stop_failure_as_last_words` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `the_stop_check_sends_back_then_the_stop_settles` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `the_stop_asks_flume_host_and_leaves_no_process` | stays: end-to-end | itself |
| `with_no_flume_host_the_stop_leaves_no_process` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_conversations_agent_stops` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_session_the_operator_started` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_restarted_pane` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `an_end_pending_for_the_pane_collected_at_its_start` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `nothing_waits_on_the_session` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_recipient_in_the_environment` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `flume_unreachable_at_a_sessions_start` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `the_status_line_of_a_code_agent` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `the_status_line_outside_an_agent_flume_started` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `the_status_line_writes_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_carried_start_in_another_session` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_start_that_resumes_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_carried_starts_length_kept_at_a_compaction` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `hook_deliver_prints_the_message_waiting` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `hook_deliver_prints_nothing_for_another_secret` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `hook_deliver_prints_nothing_while_the_agent_asks` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_report_of_the_windows_is_the_accounts_reading` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_window_in_no_form_keeps_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `a_report_under_no_account_writes_no_accounts_document` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `hook_usage_outside_a_session_flume_started_writes_nothing` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |
| `each_claude_code_accounts_home_in_status` | a unit test in `crates/flume/src/profiles/agent/claude/hook.rs` over the hook's input | (its step) |

## `merge_queue_sluice` (20 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_sluice_profile_passes_the_merge_queue_scenarios` | stays: end-to-end | itself |
| `the_sluice_profile_passes_the_base_place_scenarios` | stays: end-to-end | itself |
| `a_base_places_branch_over_a_scratch_repository` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_base_places_branch_forecast_over_a_scratch_repository` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `an_open_of_a_branch_the_policy_names_no_table_for_is_refused_in_sluices_words` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_close_sluice_refuses_keeps_the_branch_in_its_words` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_submission_from_a_herdr_panes_hook_names_the_place_and_no_session` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_submission_from_a_rex_block_names_no_session` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_failed_entry_is_given_with_sluices_notice_until_it_is_confirmed` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `every_branch_asked_at_once_is_read_from_sluice_hosts_socket_with_no_sluice_command` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_queue_the_settles_never_step_is_decided_by_sluices_host_process` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_conflicts_base_revision_and_words_are_sluices` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_conflict_sluice_answers_at_the_submit_is_its_outcome_never_a_refusal` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_settle_beside_the_stand_ins_submit_collects_at_most_the_outcome_its_answer_gives` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_pull_request_with_no_entry_is_held_and_the_entry_taken_in_from_it_is_read_to_its_landing` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_branch_whose_records_sluice_cannot_read_is_passed_over_and_the_landing_collected` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `a_fetch_that_fails_is_answered_with_its_words` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `what_the_policy_says_of_a_place` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |
| `s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone` | stays: end-to-end | itself |
| `s109_a_host_process_ended_with_work_in_play_is_started_again_by_a_settles_inbox` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` over recorded answers, or a case of the sluice scenarios | (its step) |

## `host` (33 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_history_scenarios_pass_against_git` | stays: end-to-end | itself |
| `a_rewritten_origin_is_read_as_configured` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_head_reads_none_for_a_branch_that_does_not_exist` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_default_branch_is_the_remotes` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_common_git_directory_names_the_repository` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_thousand_secrets_never_repeat` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_command_lives_while_its_process_does` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_files_write_time_by_the_hosts_clock` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_bound_the_configuration_states` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_source_written_beside_and_renamed` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `entries_that_are_no_source` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `no_sources_directory` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `each_fault_is_its_sources_own` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_sources_directory_that_cannot_be_listed_fails_the_configuration` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_local_layer_declares_no_source` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_sources_roles_name_each_run` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_roles_file_read_as_it_stands` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_standing_agent_and_a_carried_address` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `an_entry_at_fault_declares_nothing_for_its_address` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_standing_roles_entry_gives_its_repositories_and_kinds` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_standing_roles_entry_at_fault_declares_nothing` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_roles_file_that_cannot_be_read_declares_no_role` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `what_two_entries_claim_is_neithers` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_answerer_behind_each_stream` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_roles_a_host_answers_with` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_sources_roles_path_at_fault` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_roles_table_in_the_hosts_configuration_is_unreadable` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_child_process_reads_the_hosts_layers` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_host_reads_both_layers_and_writes_neither` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `a_child_process_holds_the_room` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_room_lock_is_exclusive_between_processes` | stays: end-to-end | itself |
| `the_modules_files_read_whole` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |
| `the_modules_sources_read_whole` | a unit test in `crates/flume/src/host/` over files in a scratch folder | (its step) |

## `place_git_worktree` (26 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_place_profile_passes_the_place_scenarios` | stays: end-to-end | itself |
| `s10_an_untracked_file_and_a_failed_teardown_keep_their_places` | a case of the place scenarios against git | (its step) |
| `s9_a_branch_is_kept_while_it_holds_a_change_its_base_lacks` | a case of the place scenarios against git | (its step) |
| `a_branch_is_judged_against_the_base_where_it_lands` | a case of the place scenarios against git | (its step) |
| `s16_readiness_that_fails_at_its_toolchain` | a case of the place scenarios against git | (its step) |
| `a_place_is_readied_at_the_revision_its_launch_read` | a case of the place scenarios against git | (its step) |
| `nothing_is_kept_in_the_places_git_directory` | a case of the place scenarios against git | (its step) |
| `with_no_merge_queue_bound_nothing_runs` | a case of the place scenarios against git | (its step) |
| `make_runs_no_hook` | a case of the place scenarios against git | (its step) |
| `a_bare_repositorys_places_go_under_flumes_own_data_directory` | a case of the place scenarios against git | (its step) |
| `a_working_trees_places_go_under_flumes_own_data_directory` | a case of the place scenarios against git | (its step) |
| `a_place_made_inside_the_main_worktree_is_read_and_removed_where_it_stands` | a case of the place scenarios against git | (its step) |
| `a_place_made_beside_a_bare_repositorys_worktrees_is_read_and_removed_where_it_stands` | a case of the place scenarios against git | (its step) |
| `the_main_worktree_is_never_removed` | a case of the place scenarios against git | (its step) |
| `what_git_left_of_a_failed_removal_goes_at_the_next_remove` | a case of the place scenarios against git | (its step) |
| `what_a_process_wrote_at_a_gone_places_location_goes` | a case of the place scenarios against git | (its step) |
| `a_group_folder_holding_another_place_stays` | a case of the place scenarios against git | (its step) |
| `a_worktree_another_repository_holds_at_a_places_location_is_kept` | a case of the place scenarios against git | (its step) |
| `only_a_folder_flume_made_goes_from_a_places_location` | a case of the place scenarios against git | (its step) |
| `a_base_place_is_given_its_location_and_nothing_is_made_there` | a case of the place scenarios against git | (its step) |
| `checkouts_made_of_a_bare_repository_and_a_clone` | a case of the place scenarios against git | (its step) |
| `checkouts_moved_where_git_refuses` | a case of the place scenarios against git | (its step) |
| `checkouts_moved_carrying_a_change` | a case of the place scenarios against git | (its step) |
| `checkouts_removed_to_the_group_folder` | a case of the place scenarios against git | (its step) |
| `a_fresh_clones_pins_are_installed_by_the_pinned_sluice` | stays: end-to-end | itself |
| `without_the_install_moon_is_not_there_to_answer_its_version` | a case of the place scenarios against git | (its step) |

## `store_state_dir` (13 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_store_profile_passes_the_store_scenarios` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `a_removed_place_leaves_nothing_in_the_state_directory` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `a_child_process_writes_at_a_version` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `two_processes_writing_at_one_version_leave_one_write` | stays: end-to-end | itself |
| `s12_a_restart_over_the_state_directory_loses_nothing` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `the_hosts_deliveries_are_seen_by_another_process` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `two_processes_record_one_account_spent` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `a_spent_time_is_seen_by_another_process_and_kept_from_other_users` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `an_accounts_reading_beside_its_spent_time` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `an_accounts_document_an_earlier_release_wrote` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `an_intents_document_an_earlier_release_wrote` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `a_name_reaches_nothing_outside_its_directory` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |
| `a_file_of_a_places_folder_is_written_whole_for_the_hosts_user_alone` | a unit test in `crates/flume/src/profiles/store/state_dir/` | (its step) |

## `command_line` (12 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `a_hold_on_the_waiting_move_then_a_retry_naming_its_key` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_tell_naming_its_key` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_message_naming_its_key` | a unit test in `crates/flume/src/commands/` | (its step) |
| `tell_runs_as_message` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_bound_and_a_key_in_a_wrong_form` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_bound_on_a_stage_hold` | a unit test in `crates/flume/src/commands/` | (its step) |
| `settle_on_quiet_starts_a_conversations_quiet_again` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_message_to_an_address_answers_with_its_key` | a unit test in `crates/flume/src/commands/` | (its step) |
| `a_target_names_an_address_or_a_place` | a unit test in `crates/flume/src/commands/` | (its step) |
| `relay_writes_screens_and_types_each_line_of_input` | a unit test in `crates/flume/src/commands/` | (its step) |
| `relay_gives_back_what_it_cannot_type_and_a_refusal_alone` | a unit test in `crates/flume/src/commands/` | (its step) |
| `relays_help_names_its_lines_and_no_caller` | a unit test in `crates/flume/src/commands/` | (its step) |

## `telemetry` (6 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `a_collector_that_never_answers_holds_no_answer` | a unit test in `crates/flume/src/telemetry.rs` over an in-memory exporter | (its step) |
| `nothing_is_sent_without_a_table` | a unit test in `crates/flume/src/telemetry.rs` over an in-memory exporter | (its step) |
| `a_callers_collector_is_never_flumes` | stays: end-to-end | itself |
| `otels_variables_turn_signals_off_and_never_on` | a unit test in `crates/flume/src/telemetry.rs` over an in-memory exporter | (its step) |
| `a_refusing_collector_changes_nothing` | a unit test in `crates/flume/src/telemetry.rs` over an in-memory exporter | (its step) |
| `status_and_the_list_send_nothing` | a unit test in `crates/flume/src/telemetry.rs` over an in-memory exporter | (its step) |

## `built_in` (10 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_engine_is_built_over_a_scratch_state_and_configuration` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `the_resolver_names_the_profiles_built_in` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `a_terminal_the_host_names_is_bound_or_refused` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `terminals_and_windows_are_given_by_name` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `a_place_is_readied_as_the_merge_queue_equips_it` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `a_branch_is_judged_against_the_base_where_the_policy_lands_it` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `a_recipient_unnamed_is_the_herdr_pane_the_command_runs_in` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `the_hashes_recorded` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `flumes_own_kinds_declare_no_label` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |
| `a_repositorys_binding_names_what_is_missing_until_its_merge_queue_lands_it` | a unit test in `crates/flume/src/profiles.rs`, or the end-to-end test of launch | (its step) |

## `terminal_printed` (11 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_printed_command_passes_the_terminal_scenarios` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `relay_under_the_printed_command_is_refused_and_runs_nothing` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `attach_under_the_printed_command_is_refused_for_either_view` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `attach_under_the_printed_command_runs_nothing` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `runs_under_the_printed_command_holds_no_part_and_runs_nothing` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `agents_under_the_printed_command_are_not_read_and_nothing_is_run` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `the_command_status_gives_carries_the_prompt` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `a_session_names_nothing_in_the_printed_command` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `the_printed_command_hands_a_running_agent_no_words` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `a_message_under_the_printed_command_is_refused_with_nothing_typed` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |
| `an_agent_whose_record_says_its_session_ended_reads_stopped` | a unit test in `crates/flume/src/profiles/terminal/printed/` | (its step) |

## `open` (4 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `s90_a_streams_view_through_the_binary` | stays: end-to-end | itself |
| `s90_the_printed_window_through_the_binary` | the conformance scenario of the behaviour, and a unit test of its profile | (its step) |
| `s91_a_stream_under_the_printed_command_beside_one_under_herdr` | the conformance scenario of the behaviour, and a unit test of its profile | (its step) |
| `s107_a_design_session_through_the_binary` | the conformance scenario of the behaviour, and a unit test of its profile | (its step) |

## `stand_in` (5 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_stand_in_drives_one_stage` | stays: end-to-end | itself |
| `the_stand_in_refuses_a_session_it_finds_no_transcript_of` | `stand_in::the_stand_in_drives_one_stage` | (its step) |
| `the_stand_in_under_herdr_takes_its_later_prompt_and_reports_its_state` | `stand_in::the_stand_in_drives_one_stage` | (its step) |
| `each_start_reads_its_own_script_from_the_scripts_folder` | `stand_in::the_stand_in_drives_one_stage` | (its step) |
| `the_stand_in_begins_at_the_effort_passed_and_at_its_default_for_a_word_it_does_not_know` | `stand_in::the_stand_in_drives_one_stage` | (its step) |

## `open_sessions` (4 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `s4_an_answer_within_the_hour_by_a_retry_and_a_message_reaches_the_same_session` | stays: end-to-end | itself |
| `s104_the_time_passing_lets_the_agent_go_and_a_late_answer_starts_a_fresh_one` | the conformance scenario of the behaviour | (its step) |
| `s105_the_agents_session_ended_within_the_hour_and_a_retry_answers` | the conformance scenario of the behaviour | (its step) |
| `s105_the_agents_account_spent_within_the_hour_and_a_retry_answers` | the conformance scenario of the behaviour | (its step) |

## `change_openspec` (6 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_change_profile_passes_the_change_scenarios` | stays: end-to-end | itself |
| `the_change_lies_at_its_folder` | a case of the change scenarios against openspec | (its step) |
| `reading_writes_nothing` | a case of the change scenarios against openspec | (its step) |
| `the_file_reading_agrees_with_openspec_at_five_revisions` | a case of the change scenarios against openspec | (its step) |
| `a_change_of_another_schema_is_read_through_openspec` | a case of the change scenarios against openspec | (its step) |
| `a_change_that_skips_specs_is_planned_as_openspec_reads_it` | a case of the change scenarios against openspec | (its step) |

## `window_ghostty` (8 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `ghostty_on_macos_passes_the_window_scenarios` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `ghostty_elsewhere_passes_the_window_scenarios` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `ghostty_elsewhere_slow_to_start_passes_the_window_scenarios` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `the_words_open_is_given` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `ghostty_not_installed` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `an_open_that_never_answers` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `the_words_and_environment_ghostty_is_given` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |
| `no_ghostty_to_run` | a unit test in `crates/flume/src/profiles/window/ghostty/` over a recorded call | (its step) |

## `window_terminal` (5 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `terminal_on_macos_passes_the_window_scenarios` | a unit test in `crates/flume/src/profiles/window/terminal/` over a recorded call | (its step) |
| `the_words_osascript_is_given` | a unit test in `crates/flume/src/profiles/window/terminal/` over a recorded call | (its step) |
| `a_command_that_needs_a_variable` | a unit test in `crates/flume/src/profiles/window/terminal/` over a recorded call | (its step) |
| `terminals_automation_not_allowed` | a unit test in `crates/flume/src/profiles/window/terminal/` over a recorded call | (its step) |
| `not_macos` | a unit test in `crates/flume/src/profiles/window/terminal/` over a recorded call | (its step) |

## `claude_words` (10 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_pinned_release_holds_the_usage_limit_words` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_keeps_its_credentials_from_its_hooks` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_holds_the_modes_the_map_rests_on` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_resumes_as_the_carry_reads_it` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_names_its_running_sessions_as_live_reads_them` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_takes_own_settings_as_the_profile_passes_them` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_hands_its_hooks_the_inputs_they_read` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_takes_the_command_the_profile_builds` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_asks_its_folder_trust_as_the_terminal_answers_it` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |
| `the_pinned_release_takes_the_efforts_the_profile_passes` | `claude_words::the_pinned_release_holds_what_the_profile_reads` | (its step) |

## `merge_queue_sluice_documents` (5 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `a_refused_outcome_is_held_with_its_notice_and_owed_until_confirmed` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` | (its step) |
| `an_outcome_in_a_state_flume_does_not_read_is_left_owed_and_named` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` | (its step) |
| `a_refused_record_is_held_with_its_refused_words` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` | (its step) |
| `a_refused_record_without_them_is_held_saying_sluices_record_gives_none` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` | (its step) |
| `a_bases_records_are_asked_again_only_once_their_version_moved` | a unit test in `crates/flume/src/profiles/merge_queue/sluice/` | (its step) |

## `window_printed` (2 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `the_printed_window_passes_the_window_scenarios` | a unit test in `crates/flume/src/profiles/window/printed/` | (its step) |
| `a_line_a_person_runs` | a unit test in `crates/flume/src/profiles/window/printed/` | (its step) |

## `scratch` (2 tests)

| Test | Planned home | Proven by |
|---|---|---|
| `a_scratch_repository_reads_no_configuration_of_the_host` | the end-to-end tests that use the scratch repository | (its step) |
| `a_clone_names_its_bare_remote_as_origin` | the end-to-end tests that use the scratch repository | (its step) |

