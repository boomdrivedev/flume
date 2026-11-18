//! An agent read from its terminal (31, 38, 89 to 93, 134, 136, 171):
//! where an agent's profile does not build received, asking, own work,
//! ended or last words, the core reads that part from the terminal its
//! start recorded and asks the profile nothing of it; where the profile
//! builds it, the core reads it from the profile alone. Part C's profiles
//! are cited in words alone, since the suite cites as data only the
//! clauses of Parts A and B, the invariants and section 11's scenarios:
//! the common profile of C.4 and herdr's watched send and states of C.5.

use super::agent_launcher::{through_the_launcher, LAUNCHED};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{DegradedPart, OWN_WORK_AFTER};
use flume_fakes::FakeAgent;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "terminal readings: a begun fix's agent that reads asking from its terminal is read asking while the terminal reads it asking, a tell to it refused with nothing typed, and read neither once the terminal reads it waiting",
            cites: &["37", "90", "171"],
            run: asking_as_the_terminal_shows_it,
        },
        Scenario {
            name: "terminal readings: a begun fix's agent whose profile builds asking, holding no asking mark while the terminal reads it asking, reads unknown and not asking, and a tell to it is refused with nothing typed",
            cites: &["90"],
            run: a_profile_that_builds_them_reads_no_question,
        },
        Scenario {
            name: "terminal readings: a begun fix's agent that reads ended and last words from its terminal, the terminal showing `amp: out of credits` and then reading it stopped with nothing delivered, is reported lost with those last words",
            cites: &["43", "45", "134", "136", "171"],
            run: lost_with_its_screen,
        },
        Scenario {
            name: "terminal readings: a fix's agent that reads received and asking from its terminal, the terminal answering that it saw the turn its prompt began, is begun after one sending by the watched send and none by a plain send, its prompt received when the terminal answered and its start naming no record",
            cites: &["38", "77", "89", "171"],
            run: the_turn_seen,
        },
        Scenario {
            name: "terminal readings: a prompt whose turn the terminal did not see, the agent read working ten seconds after the sending, is begun at that reading with one sending made",
            cites: &["38", "171"],
            run: the_turn_not_seen_then_read_working,
        },
        Scenario {
            name: "terminal readings: a prompt whose turn the terminal never sees, the agent read waiting throughout, is sent once more 60 seconds after the first, its held input cleared first, and reported never begun with what the terminal shows once 60 seconds pass after the second",
            cites: &["38", "95", "171"],
            run: the_turn_never_seen,
        },
        Scenario {
            name: "terminal readings: a terminal that reads no turn refuses the watched send, and code is reported never begun, its failure naming the terminal's words, nothing typed",
            cites: &["38", "95"],
            run: a_terminal_that_reads_no_turn,
        },
        Scenario {
            name: "terminal readings: a tell to a begun fix's agent, waiting and reading received and asking from its terminal, is handed on the turn the terminal saw, sent once by the watched send",
            cites: &["53", "54", "171"],
            run: a_tell_handed_on_the_turn,
        },
        Scenario {
            name: "terminal readings: a begun fix's agent that reads the five from its terminal, its deliverable committed and the terminal reading it waiting, settled now and 30 seconds later reads its own work running, and is quiet and delivers at the settle 60 seconds later",
            cites: &["31", "91", "171"],
            run: quiet_after_a_minutes_waiting,
        },
        Scenario {
            name: "terminal readings: an agent read waiting at a settle, working at one 30 seconds later, and waiting at those 60, 90 and 120 seconds later delivers at the settle 120 seconds later and at none before it",
            cites: &["31", "171"],
            run: a_reading_between_starts_the_minute_again,
        },
        Scenario {
            name: "terminal readings: status asked 90 seconds after a settle read the agent waiting, no settle between, reads its own work running and leaves the place's held state unchanged",
            cites: &["31", "171"],
            run: status_counts_no_settle,
        },
        Scenario {
            name: "terminal readings: a fix's agent read begun in another process than its launcher became, reading the five from its terminal, is read quiet once the terminal reads it stopped, and code is started afresh with no minute's wait",
            cites: &["31", "161", "171"],
            run: a_stopped_agent_runs_no_own_work,
        },
        Scenario {
            name: "terminal readings: an agent that reads the five from its terminal, read waiting past the stage's limit with nothing delivered, the terminal showing `nothing to do`, is reported stuck with those last words",
            cites: &["43", "44", "171"],
            run: stuck_with_its_screen,
        },
        Scenario {
            name: "terminal readings: a fix over an agent that reads the five from its terminal, its prompt's turn seen, then read asking, then waiting at two settles a minute apart with its deliverable committed, status asked at each settle, delivers with none of the five read of its start from its profile",
            cites: &["93", "171"],
            run: nothing_of_the_five_asked,
        },
        Scenario {
            name: "S73: a fix over an agent that starts through the launcher, reads the five from its terminal and offers no hooks, its program read in the launcher's process 41 and its prompt's turn seen, starts and begins, status naming the eight parts degraded with how each is read, and once it commits and is read waiting at two settles a minute apart code delivers and the place is submitted",
            cites: &["35", "38", "171", "S73"],
            run: s73_over_the_fakes,
        },
    ]
}

/// The fix `name` begun with its code agent reading the five from its
/// terminal, and read working at a settle since, so no settle has read it
/// waiting: its place and its code start's identity.
fn begun_working(world: &World, name: &str) -> Result<(PlaceId, Identity), String> {
    let (id, code) = fix_reading(world, name, &FakeAgent::FROM_TERMINAL)?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    world.terminal.set_state(&code, AgentState::Working);
    world.settle(&id);
    ensure_eq!(
        code_start(world, &id)?.waiting,
        None,
        "the settles' reading of the agent waiting, while it works"
    );
    Ok((id, code))
}

/// The fix `name` launched with its code agent reading `parts` from its
/// terminal, opened but not yet read waiting, so nothing is sent until the
/// scenario sets its state and settles: its place and its code start's
/// identity.
fn opened_reading(
    world: &World,
    name: &str,
    parts: &[Part],
) -> Result<(PlaceId, Identity), String> {
    world.agent.reads_from_its_terminal(parts);
    world.terminal.comes_up(AgentState::Unknown);
    launched(&world.engine.launch(&launch("fix", name, Some("mend it"))))?;
    Ok((
        World::id("fix", name),
        World::identity("fix", name, "code", 1),
    ))
}

/// The code start of the place `id` as it is held.
fn code_start(world: &World, id: &PlaceId) -> Result<StartHeld, String> {
    world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start of code".into())
}

/// The fix `name` launched with its code agent reading `parts` from its
/// terminal, settled once: its place and its code start's identity.
pub(crate) fn fix_reading(
    world: &World,
    name: &str,
    parts: &[Part],
) -> Result<(PlaceId, Identity), String> {
    world.agent.reads_from_its_terminal(parts);
    launched(&world.engine.launch(&launch("fix", name, Some("mend it"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok((id, World::identity("fix", name, "code", 1)))
}

/// The reads of the fake agent's log of `operation` for `who`'s start.
fn reads_of(world: &World, operation: &str, who: &Identity) -> Vec<String> {
    world
        .agent
        .log
        .of(operation)
        .into_iter()
        .filter(|line| line.contains(&who.to_string()))
        .collect()
}

fn agent_status(world: &World, place: &PlaceId) -> Result<flume_core::AgentStatus, String> {
    world
        .status(place)?
        .agent
        .ok_or("no agent in status".into())
}

/// The terminal-readings spec's scenario "Asking as the terminal shows
/// it" (37, 90, 171).
fn asking_as_the_terminal_shows_it(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[Part::Asking])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Asking, "the agent's state");
    ensure_eq!(agent.asking, Reading::Yes, "the agent's asking");
    let told = world.engine().tell(&id, "go on", Some(30));
    ensure!(
        told.refusal().is_some(),
        "a tell to an asking agent was taken"
    );
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt.clone()],
        "what was typed into the agent"
    );
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Waiting, "the agent's state");
    ensure_eq!(agent.asking, Reading::No, "the agent's asking");
    Ok(())
}

/// The terminal-readings spec's scenario "A profile that builds them
/// reads no question from the terminal" (90; core.39).
fn a_profile_that_builds_them_reads_no_question(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Unknown, "the agent's state");
    ensure_eq!(agent.asking, Reading::No, "the agent's asking");
    let told = world.engine().tell(&id, "go on", Some(30));
    ensure!(
        told.refusal().is_some(),
        "a tell into a question flume cannot name was taken"
    );
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt],
        "what was typed into the agent"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Lost with its screen" (43, 45,
/// 171).
fn lost_with_its_screen(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[Part::Ended, Part::LastWords])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    world.agent.set_last_words(&code, "words of the record");
    world
        .terminal
        .set_screen(&code, Some("amp: out of credits\n\n"));
    world.terminal.set_state(&code, AgentState::Stopped);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("amp: out of credits"),
        "the last words the end carries"
    );
    for operation in ["ended", "last_words"] {
        ensure!(
            reads_of(&world, operation, &code).is_empty(),
            "the profile was asked {operation}"
        );
    }
    Ok(())
}

/// The terminal-readings spec's scenario "The turn seen" (38, 89, 171).
fn the_turn_seen(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = opened_reading(&world, "one", &[Part::Received, Part::Asking])?;
    world.terminal.set_state(&code, AgentState::Waiting);
    let at = world.host.now();
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let start = code_start(&world, &id)?;
    ensure_eq!(start.prompt.sendings, vec![at], "the sendings");
    ensure_eq!(start.prompt.received_at, Some(at), "received at");
    ensure_eq!(start.begun_at, Some(at), "begun at");
    ensure_eq!(start.record, None, "the record the start names");
    ensure_eq!(
        world.terminal.log.count("send-watching"),
        1,
        "watched sends"
    );
    ensure_eq!(world.terminal.log.count("send"), 0, "plain sends");
    ensure!(
        reads_of(&world, "received", &code).is_empty(),
        "the profile was asked whether its agent received the prompt"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "The turn not seen, then read
/// working" (38, 171).
fn the_turn_not_seen_then_read_working(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = opened_reading(&world, "one", &[Part::Received, Part::Asking])?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.terminal.watch_as(
        &code,
        SentWatching::NotSeen {
            why: "no turn began".into(),
        },
    );
    let at = world.host.now();
    world.terminal.schedule(&code, at + 10, AgentState::Working);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let start = code_start(&world, &id)?;
    ensure_eq!(start.prompt.sendings, vec![at], "the sendings");
    ensure_eq!(start.begun_at, Some(at + 10), "begun at");
    ensure_eq!(world.terminal.sent_to(&code).len(), 1, "the sendings typed");
    Ok(())
}

/// The terminal-readings spec's scenario "The turn never seen" (38, 95,
/// 171).
fn the_turn_never_seen(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = opened_reading(&world, "one", &[Part::Received, Part::Asking])?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.terminal.watch_as(
        &code,
        SentWatching::NotSeen {
            why: "no turn began".into(),
        },
    );
    world.terminal.leaves_next_typed(&code);
    world.terminal.set_screen(&code, Some("amp> "));
    let at = world.host.now();
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "code"
    );
    let start = code_start(&world, &id)?;
    ensure_eq!(
        start.prompt.sendings,
        vec![at, at + flume_core::RECEIVED_WITHIN],
        "the sendings"
    );
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt.clone(), prompt],
        "the sendings typed"
    );
    ensure_eq!(
        world.terminal.typed(&code),
        Some((String::new(), 1)),
        "what is left typed, and how often a send cleared it"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.screen.as_deref(),
        Some("amp> "),
        "what the terminal shows"
    );
    ensure!(
        world.host.now() >= at + 2 * flume_core::RECEIVED_WITHIN,
        "read never begun before 60 seconds passed after the second sending"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "A terminal that reads no turn"
/// (38, 95).
fn a_terminal_that_reads_no_turn(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = opened_reading(&world, "one", &[Part::Received, Part::Asking])?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.terminal.watch_as(
        &code,
        SentWatching::Refused {
            reason: NO_TURN.into(),
        },
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "code"
    );
    let held = world.held(&id).ok_or("no held state")?;
    ensure!(
        held.failures
            .iter()
            .any(|failure| failure.contains(NO_TURN)),
        "no failure names the terminal's words: {:?}",
        held.failures
    );
    ensure!(
        world.terminal.sent_to(&code).is_empty(),
        "something was typed"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "A tell handed on the turn" (53,
/// 54, 171).
fn a_tell_handed_on_the_turn(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[Part::Received, Part::Asking])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let watched = world.terminal.log.count("send-watching");
    let told = world.engine().tell(&id, "look again", Some(30));
    ensure!(
        told.refusal().is_none(),
        "the tell was refused: {:?}",
        told.refusal()
    );
    ensure_eq!(
        world.terminal.log.count("send-watching"),
        watched + 1,
        "watched sends of the tell"
    );
    ensure_eq!(world.terminal.log.count("send"), 0, "plain sends");
    ensure_eq!(
        world.terminal.sent_to(&code).last().cloned(),
        Some("look again".to_string()),
        "the words typed"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Quiet after a minute's waiting"
/// (31, 91, 171).
fn quiet_after_a_minutes_waiting(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = begun_working(&world, "one")?;
    world.commit("fix/one", &["fix.rs"]);
    world.terminal.set_state(&code, AgentState::Waiting);
    let at = world.host.now();
    for after in [0, 30] {
        world.host.advance(after);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Begun),
            "code, {after} seconds after the first settle read it waiting"
        );
        let agent = agent_status(&world, &id)?;
        ensure_eq!(
            agent.own_work,
            Reading::Yes,
            "its own work, {after} seconds after"
        );
    }
    ensure_eq!(
        code_start(&world, &id)?.waiting,
        Some(WaitingRead {
            since: at,
            at: at + 30
        }),
        "the settles' readings of the agent waiting"
    );
    world.host.advance(OWN_WORK_AFTER - 30);
    world.settle(&id);
    delivered(
        &world,
        "code, a minute after the first settle read it waiting",
    )?;
    for operation in ["own_work", "ended"] {
        ensure!(
            reads_of(&world, operation, &code).is_empty(),
            "the profile was asked {operation}"
        );
    }
    Ok(())
}

/// The terminal-readings spec's scenario "A reading between starts the
/// minute again" (31, 171).
fn a_reading_between_starts_the_minute_again(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = begun_working(&world, "one")?;
    world.commit("fix/one", &["fix.rs"]);
    for (after, state) in [
        (0, AgentState::Waiting),
        (30, AgentState::Working),
        (30, AgentState::Waiting),
        (30, AgentState::Waiting),
    ] {
        world.host.advance(after);
        world.terminal.set_state(&code, state);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Begun),
            "code, read {state:?}"
        );
    }
    world.host.advance(30);
    world.settle(&id);
    delivered(&world, "code, at the settle 120 seconds after the first")
}

/// A fix whose code delivered: its agent ended and the place submitted,
/// which the fake merge queue lands (31, 47).
fn delivered(world: &World, what: &str) -> Outcome {
    ensure_eq!(
        world.terminal.log.count("close"),
        1,
        format!("{what}: agents ended")
    );
    ensure_eq!(
        world.queue.log.count("submit"),
        1,
        format!("{what}: submissions")
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Status counts no settle" (31,
/// 171; core.52).
fn status_counts_no_settle(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = begun_working(&world, "one")?;
    world.keep_watch();
    world.commit("fix/one", &["fix.rs"]);
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    let version = world.store.read(&id).ok_or("no place")?.version;
    world.host.advance(90);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.own_work, Reading::Yes, "its own work, in status");
    ensure_eq!(
        world.store.read(&id).ok_or("no place")?.version,
        version,
        "the place's version after status"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "A stopped agent runs no own
/// work" (31, 161, 171).
fn a_stopped_agent_runs_no_own_work(_: &()) -> Outcome {
    let world = World::new();
    let first = through_the_launcher(&world, "one");
    world
        .terminal
        .set_process(&first, ProgramProcess::Runs { pid: LAUNCHED });
    let (id, code) = fix_reading(&world, "one", &FakeAgent::FROM_TERMINAL)?;
    ensure_eq!(code, first, "the code start");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its prompt taken in the launcher's process"
    );
    world.terminal.set_state(&first, AgentState::Working);
    world
        .terminal
        .set_process(&first, ProgramProcess::Runs { pid: LAUNCHED + 1 });
    world.settle(&id);
    ensure_eq!(
        code_start(&world, &id)?.n,
        1,
        "code's start, while its agent works"
    );
    world.terminal.set_state(&first, AgentState::Stopped);
    world.settle(&id);
    ensure_eq!(
        code_start(&world, &id)?.n,
        2,
        "code's start, started afresh at once"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Stuck with its screen" (43, 44,
/// 171).
fn stuck_with_its_screen(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = begun_working(&world, "one")?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.terminal.set_screen(&code, Some("nothing to do"));
    world.settle(&id);
    world.host.advance(OWN_WORK_AFTER);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, quiet a minute after the first settle read it waiting"
    );
    world.host.advance(flume_core::QUIET_LIMIT);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Stuck),
        "code, quiet past its limit"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("nothing to do"),
        "the last words the end carries"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Nothing of the five asked of a
/// profile that does not build them" (93, 171).
fn nothing_of_the_five_asked(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = opened_reading(&world, "one", &FakeAgent::FROM_TERMINAL)?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    world.status(&id)?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its prompt's turn seen"
    );
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    world.status(&id)?;
    world.commit("fix/one", &["fix.rs"]);
    world.terminal.set_state(&code, AgentState::Waiting);
    for after in [0, OWN_WORK_AFTER] {
        world.host.advance(after);
        world.settle(&id);
        if world.held(&id).is_some() {
            world.status(&id)?;
        }
    }
    delivered(&world, "code")?;
    for operation in ["received", "asking", "own_work", "ended", "last_words"] {
        ensure!(
            reads_of(&world, operation, &code).is_empty(),
            "the profile was asked {operation}: {:?}",
            reads_of(&world, operation, &code)
        );
    }
    Ok(())
}

/// S73 over the fakes, the terminal-readings spec's scenario of that name:
/// the common profile of C.4 over a program herdr starts, as C.5 reads it
/// (35, 38, 171, S73).
fn s73_over_the_fakes(_: &()) -> Outcome {
    let world = World::new();
    let first = through_the_launcher(&world, "one");
    world
        .terminal
        .set_process(&first, ProgramProcess::Runs { pid: LAUNCHED });
    world.agent.read_mode_back();
    world.agent.carry_to(
        FakeAgent::LOGIN,
        Carry::Yes {
            session: "s-1".into(),
        },
    );
    world.agent.take_own();
    world.agent.no_send_back();
    world.agent.no_guard();
    world.agent.no_on_quiet();
    let (id, code) = fix_reading(&world, "one", &FakeAgent::FROM_TERMINAL)?;
    ensure_eq!(code, first, "the code start");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its program read in the launcher's process and its prompt's turn seen"
    );
    ensure_eq!(
        world.terminal.log.count("send-watching"),
        1,
        "watched sends"
    );
    let agent = agent_status(&world, &id)?;
    let eight = [
        Part::Received,
        Part::Asking,
        Part::OwnWork,
        Part::Ended,
        Part::LastWords,
        Part::SendBack,
        Part::Guard,
        Part::OnQuiet,
    ];
    ensure_eq!(
        agent.degraded,
        eight
            .map(|part| DegradedPart {
                part,
                read: part.read().into(),
            })
            .to_vec(),
        "the agent's degraded parts"
    );
    world.commit("fix/one", &["fix.rs"]);
    for after in [0, OWN_WORK_AFTER] {
        world.host.advance(after);
        world.settle(&id);
    }
    delivered(&world, "code")
}
