//! Each of the eight seams' scenarios, and git's history's, run against
//! its fake (One interface per seam).

use flume_conformance::fake_harness::*;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::*;

#[test]
fn the_fake_place_passes_the_place_scenarios() {
    pass_all("place", &place::scenarios(), FakePlaceHarness::default);
}

#[test]
fn the_fake_kinds_pass_the_kinds_scenarios() {
    pass_all("kinds", &kinds::scenarios(), FakeKindsHarness::new);
}

#[test]
fn the_fake_change_passes_the_change_scenarios() {
    pass_all("change", &change::scenarios(), FakeChangeHarness::default);
}

#[test]
fn the_fake_agent_passes_the_agent_scenarios() {
    pass_all("agent", &agent::scenarios(), FakeAgentHarness::default);
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::carrying_none,
    );
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::taking_no_own,
    );
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::taking_no_effort,
    );
}

#[test]
fn the_fake_terminal_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), || {
        FakeTerminalHarness::new(true)
    });
    pass_all("terminal", &terminal::scenarios(), || {
        FakeTerminalHarness::new(false)
    });
}

#[test]
fn the_fake_window_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), FakeWindowHarness::default);
}

/// The seam-contracts spec's show over the fake: two shows of a command
/// under `Work Agents`, each opening a window and logged with the command
/// and the title, then the printed line and the refusal as scripted
/// (188, 189).
#[test]
fn show_over_the_fake() {
    use flume_seams::{ViewCommand, Window, WindowShown};
    let window = flume_fakes::FakeWindow::default();
    let command = ViewCommand {
        program: "/bin/fake-terminal".into(),
        arguments: vec!["attach".into(), "work".into()],
        environment: [("XDG_CONFIG_HOME".to_string(), "/scratch/home".to_string())].into(),
    };
    let title = "Work Agents";
    for _ in 0..2 {
        assert_eq!(window.show(&command, title), WindowShown::Opened);
    }
    let shown = "show /bin/fake-terminal attach work titled Work Agents";
    assert_eq!(window.log.of("show"), vec![shown; 2]);
    assert_eq!(
        window.windows(),
        vec![(command.clone(), title.to_string()); 2]
    );
    let printed = WindowShown::Printed {
        line: "XDG_CONFIG_HOME=<set> /bin/fake-terminal attach work".into(),
    };
    window.answers(Some(printed.clone()));
    assert_eq!(window.show(&command, title), printed);
    let refused = WindowShown::Refused {
        reason: "no display".into(),
    };
    window.answers(Some(refused.clone()));
    assert_eq!(window.show(&command, title), refused);
    assert_eq!(window.log.of("show"), vec![shown; 4]);
    assert_eq!(window.windows().len(), 2, "the windows opened");
}

#[test]
fn the_fake_merge_queue_passes_the_merge_queue_scenarios() {
    pass_all(
        "merge queue",
        &merge_queue::scenarios(),
        FakeMergeQueueHarness::default,
    );
}

#[test]
fn the_fake_merge_queue_passes_the_base_place_scenarios() {
    pass_all(
        "merge queue",
        &merge_queue::base_place_scenarios(),
        FakeMergeQueueHarness::default,
    );
}

#[test]
fn the_fake_merge_queue_holds_a_landing_the_forge_refused() {
    // seam-fakes "Held, pushed again, dropped and replaced": a held
    // landing stays held through a round, lands pushed again, waits once
    // dropped, and is withdrawn by a later submit of its branch (106, 112,
    // 124).
    use flume_fakes::{Plan, HELD_WITHOUT_WORDS};
    use flume_seams::{
        MergeQueue, PlaceRef, SubmissionOutcome, SubmissionState, SubmitHead, Submitted,
    };
    let harness = FakeMergeQueueHarness::default();
    let (git, queue) = (&harness.git, &harness.queue);
    let place = PlaceRef {
        repository: "/repository".into(),
        location: "/places/fix--x".into(),
        branch: "fix/x".into(),
        base: "main".into(),
    };
    let base = git.branch_head(&place.repository, "main").expect("main");
    git.set_branch(&place.repository, &place.branch, &base);
    let head = git.commit(&place.repository, &place.branch, &["work.rs"]);
    let words = "push declined: the push holds what looks like a secret";
    let held = |recorded| Plan::Held {
        words: words.into(),
        recorded,
    };
    let submit = || {
        let Submitted::Submitted { submission } =
            queue.submit(&place, &SubmitHead::Worktree { head: head.clone() })
        else {
            panic!("submitted")
        };
        submission
    };
    let state_of = |id: &str| {
        queue
            .submissions(&place.repository, &place.branch)
            .into_iter()
            .find(|submission| submission.id == id)
            .expect("recorded")
            .state
    };

    queue.plan_submits(vec![held(true)]);
    let first = submit();
    queue.run_round(&place.repository, "main");
    let state = state_of(&first.id);
    assert_eq!(
        state,
        SubmissionState::Held {
            words: words.into()
        }
    );
    assert!(!state.open(), "a held landing is not open");
    let flume_seams::Inbox::Outcomes { outcomes, .. } = queue.inbox(&place) else {
        panic!("read")
    };
    assert_eq!(
        outcomes,
        vec![SubmissionOutcome::Held {
            words: words.into()
        }]
    );
    assert_eq!(queue.pending(&place.location), vec![], "given once");
    queue.run_round(&place.repository, "main");
    assert_eq!(
        state_of(&first.id),
        SubmissionState::Held {
            words: words.into()
        },
        "a round leaves it held"
    );
    let commit = queue
        .push_again(&place.repository, &place.branch)
        .expect("a held landing pushed again");
    assert_eq!(
        state_of(&first.id),
        SubmissionState::Landed {
            commit: commit.clone()
        }
    );
    assert_eq!(
        queue.pending(&place.location),
        vec![SubmissionOutcome::Landed { commit }]
    );
    queue.inbox(&place);

    queue.plan_submits(vec![held(false)]);
    let second = submit();
    queue.run_round(&place.repository, "main");
    assert_eq!(
        state_of(&second.id),
        SubmissionState::Held {
            words: HELD_WITHOUT_WORDS.into()
        },
        "a record that carries no words"
    );
    queue.drop_held(&place.repository, &place.branch);
    assert_eq!(state_of(&second.id), SubmissionState::Waiting);
    // Kept waiting, so the round that holds the third leaves it be.
    queue.decide(&place.repository, &place.branch, Plan::Hold);

    queue.plan_submits(vec![held(true)]);
    let third = submit();
    queue.run_round(&place.repository, "main");
    assert!(matches!(state_of(&third.id), SubmissionState::Held { .. }));
    let fourth = submit();
    assert_eq!(
        state_of(&third.id),
        SubmissionState::Withdrawn,
        "a later submit of its branch withdraws it"
    );
    assert_eq!(state_of(&fourth.id), SubmissionState::Waiting);
}

#[test]
fn the_fake_store_passes_the_store_scenarios() {
    pass_all("store", &store::scenarios(), FakeStoreHarness::default);
}

#[test]
fn the_fake_history_passes_the_history_scenarios() {
    pass_all(
        "history",
        &history::scenarios(),
        FakeHistoryHarness::default,
    );
}

/// An agent of the fix `one`'s code stage open under the fake terminal
/// of `harness`, waiting.
fn an_open_agent(harness: &FakeTerminalHarness) -> flume_seams::Identity {
    use flume_conformance::seams::terminal::TerminalHarness;
    use flume_seams::{Identity, OpenIn, Opened, Terminal};
    let agent = Identity {
        repository: "0123456789ab".into(),
        place: "fix--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let opened = harness.terminal.open(
        &harness.place(),
        &harness.folder(),
        &harness.command(),
        &agent,
        &OpenIn::Own,
        None,
    );
    assert!(matches!(opened, Opened::Opened { .. }), "{opened:?}");
    agent
}

/// The seam-fakes scenario "A relay of the fake": an open agent's screen
/// read, the text `hi` typed onto it and shown, `enter` submitting it,
/// and every relay call logged without `hi` (106, 222).
#[test]
fn the_fake_terminals_relay_echoes_what_is_typed() {
    use flume_seams::{Key, KeyName, RelayScreen, Terminal, Typed};
    let harness = FakeTerminalHarness::new(true);
    let agent = an_open_agent(&harness);
    let fake = &harness.terminal;
    fake.set_screen(&agent, Some("ready"));
    fake.log.clear();
    {
        let mut relay = fake.relay(&agent).expect("the fake relays an open agent");
        assert_eq!(relay.every(), std::time::Duration::from_millis(250));
        assert_eq!(
            relay.screen(),
            RelayScreen::Shows {
                screen: "ready".into()
            }
        );
        assert_eq!(relay.typed(&Typed::Text("hi".into())), Ok(()));
        match relay.screen() {
            RelayScreen::Shows { screen } => assert!(screen.contains("hi"), "{screen}"),
            other => panic!("the second read gave {other:?}"),
        }
        assert_eq!(
            relay.typed(&Typed::Keys(vec![Key::bare(KeyName::Enter)])),
            Ok(())
        );
        assert_eq!(
            relay.screen(),
            RelayScreen::Shows {
                screen: "ready".into()
            },
            "what was typed is submitted, so the screen holds it no more"
        );
    }
    assert_eq!(fake.sent_to(&agent).last().map(String::as_str), Some("hi"));
    assert_eq!(
        fake.typed(&agent),
        Some((String::new(), 0)),
        "nothing cleared"
    );
    let logged = fake.log.calls();
    assert_eq!(
        logged,
        vec![
            format!("relay {agent}"),
            "relay read".into(),
            "relay text".into(),
            "relay read".into(),
            "relay keys enter".into(),
            "relay read".into(),
        ]
    );
    assert!(logged.iter().all(|line| !line.contains("hi")), "{logged:?}");
    assert!(fake.log.of("send").is_empty(), "the relay is no send");
    {
        let mut held = fake.relay(&agent).expect("relayed");
        fake.close(&agent);
        assert_eq!(
            held.screen(),
            RelayScreen::Gone,
            "a closed agent reads gone"
        );
    }
    assert!(
        fake.relay(&agent).is_err(),
        "a closed agent is relayed no more"
    );
    let killed = FakeTerminalHarness::new(true);
    let agent = an_open_agent(&killed);
    let mut held = killed.terminal.relay(&agent).expect("relayed");
    killed.terminal.kill(&agent);
    assert_eq!(
        held.screen(),
        RelayScreen::Gone,
        "a killed agent reads gone"
    );
}

/// The seam-fakes scenario "Scripted refusals": a relay refused as
/// scripted, then answered, its reads giving the unread and the screens
/// scripted in turn, and its first input refused as scripted and its
/// second typed (106, 222).
#[test]
fn the_fake_terminal_scripts_its_relay() {
    use flume_seams::{Key, KeyName, RelayScreen, Terminal, Typed};
    let harness = FakeTerminalHarness::new(true);
    let agent = an_open_agent(&harness);
    let fake = &harness.terminal;
    fake.refuse_relay(Some("no relay here"));
    match fake.relay(&agent) {
        Err(refused) => assert_eq!(refused, "no relay here"),
        Ok(_) => panic!("relayed while scripted to refuse"),
    }
    fake.refuse_relay(None);
    fake.relay_unread("slow");
    fake.relay_screens(&["one", "two"]);
    fake.refuse_next_typed("no such key");
    let mut relay = fake.relay(&agent).expect("answered once scripted so");
    assert_eq!(
        relay.screen(),
        RelayScreen::Unread {
            reason: "slow".into()
        }
    );
    for screen in ["one", "two"] {
        assert_eq!(
            relay.screen(),
            RelayScreen::Shows {
                screen: screen.into()
            }
        );
    }
    assert_eq!(
        relay.typed(&Typed::Keys(vec![Key::bare(KeyName::F(12))])),
        Err("no such key".into())
    );
    assert_eq!(relay.typed(&Typed::Text("yes".into())), Ok(()));
    assert_eq!(fake.typed(&agent), Some(("yes".into(), 0)));
    assert_eq!(
        relay.screen(),
        RelayScreen::Shows {
            screen: "the agent's screen\nyes".into()
        },
        "the agent's own screen once the script is spent"
    );
}
