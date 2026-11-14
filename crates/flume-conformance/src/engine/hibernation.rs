//! A stage whose kind hibernates it (212, 213; record 0069): hibernated
//! once its conversation idles past its hibernate time, at its caller's
//! word or at the end of its keep-open wait, its agent ended and its
//! session kept, its room and its account freed; and resumed in its own
//! session by its caller's resume, a tell or the answer it waits on.

use super::account_pools::{account, pool, stops_at_limit};
use super::open_sessions::{stops_short, QUESTION};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, Waits, KEEP_OPEN};
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "hibernation: A conversation hibernated at its caller's word: its quiet agent is ended, and talk reads hibernating since then, why caller",
            cites: &["31", "212"],
            run: a_conversation_hibernated_at_its_callers_word,
        },
        Scenario {
            name: "hibernation: A working agent: a caller's unforced hibernate of a working talk agent is refused naming it not quiet, and a forced one ends it and talk reads hibernating",
            cites: &["31", "212"],
            run: a_working_agent,
        },
        Scenario {
            name: "hibernation: Refused: a hibernate of a fix, whose kind hibernates nothing, and of a place whose talk hibernates already, each refused naming the place, and nothing changes",
            cites: &["212"],
            run: refused_hibernates,
        },
        Scenario {
            name: "hibernation: Room freed: talk hibernates idle on a host at its bound, its starts unchanged and its end none, and another place's stage waiting for room starts",
            cites: &["48", "70", "212"],
            run: room_freed,
        },
        Scenario {
            name: "hibernation: An hour after the last turn: settles read talk quiet over an hour, and the settle at the hour ends its agent, talk hibernating since then, why idle, and the host's agents running drop by one",
            cites: &["70", "212", "S124"],
            run: an_hour_after_the_last_turn,
        },
        Scenario {
            name: "hibernation: A turn between two settles: talk read quiet at t, quiet again at t plus 50 minutes with its on-quiet settle, is not hibernated at t plus 70 minutes, and is at t plus 110",
            cites: &["44", "121", "212"],
            run: a_turn_between_two_settles,
        },
        Scenario {
            name: "hibernation: Asking, own work, a tell: no settle hibernates a talk agent past its time while it asks, its own work runs or a tell is handed to it, until none holds and it has been quiet its time again",
            cites: &["54", "90", "91", "212"],
            run: asking_own_work_a_tell_hold_a_settle_off,
        },
        Scenario {
            name: "hibernation: Zero never hibernates on idling: a launch giving talk.hibernate_after=0 leaves talk's quiet agent running for a day",
            cites: &["7", "212"],
            run: zero_never_hibernates_on_idling,
        },
        Scenario {
            name: "hibernation: A stage whose kind does not hibernate it: a conversation of a kind that declares no hibernate, quiet for a day, runs as a conversation does",
            cites: &["109", "212"],
            run: a_stage_whose_kind_does_not_hibernate_it,
        },
        Scenario {
            name: "hibernation: Two settles at once: two settles reading talk due at once close its agent once and record its hibernation once",
            cites: &["74", "75", "212"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "hibernation: Asking, own work, a tell: a caller's unforced hibernate of a talk agent asking, with its own work running, or being handed a tell, is refused, and the tell's mark is cleared once it ends",
            cites: &["54", "90", "91", "212"],
            run: asking_own_work_a_tell_refuse_a_callers_hibernate,
        },
        Scenario {
            name: "hibernation: A code stage kept open its hour: code of a kind that hibernates it stops short needing an answer, and a settle after the hour records its close hibernated, ends its agent and records it hibernated, why kept_open, code waiting on its answer",
            cites: &["42", "198", "212", "S125"],
            run: a_code_stage_kept_open_its_hour,
        },
        Scenario {
            name: "hibernation: Let go otherwise first: the same code agent's account spent within its hour lets it go by its account spent, not hibernated",
            cites: &["146", "198", "212"],
            run: let_go_otherwise_first,
        },
    ]
}

/// The research place `name`'s talk start `start`.
pub(super) fn talk_of(name: &str, start: u32) -> Identity {
    World::identity("research", name, "talk", start)
}

/// The patch place `name`'s code start `start`.
pub(super) fn patch_code(name: &str, start: u32) -> Identity {
    World::identity("patch", name, "code", start)
}

/// The research place `name` of the kind whose talk hibernates, launched
/// with no settings and begun.
pub(super) fn research(world: &World, name: &str) -> Result<PlaceId, String> {
    world.researching(launch("research", name, Some("weigh the options")))
}

/// The stage `stage` of the place `id` as its held state gives it.
pub(super) fn stage_of(world: &World, id: &PlaceId, stage: &str) -> Result<StageHeld, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .find(|held| held.name == stage)
        .ok_or_else(|| format!("no stage {stage}"))
}

/// The start the stage `stage` of the place `id` holds now.
pub(super) fn start_now(world: &World, id: &PlaceId, stage: &str) -> Result<StartHeld, String> {
    stage_of(world, id, stage)?
        .start
        .ok_or_else(|| format!("{stage} holds no start"))
}

/// The hibernation the start of the stage `stage` records.
pub(super) fn hibernation(
    world: &World,
    id: &PlaceId,
    stage: &str,
) -> Result<Option<HibernationHeld>, String> {
    Ok(start_now(world, id, stage)?.hibernated)
}

/// The agents the fake terminal was asked to close.
pub(super) fn closed(world: &World) -> Vec<String> {
    world.terminal.log.of("close")
}

pub(super) fn taken(answer: &Answer) -> Outcome {
    match answer.refusal() {
        Some(reason) => Err(format!("the act was refused: {reason}")),
        None => Ok(()),
    }
}

/// talk reads hibernating since `since` for `why`, its start not live and
/// its end none, its starts still `starts` (48, 212).
pub(super) fn hibernating(
    world: &World,
    id: &PlaceId,
    since: Seconds,
    why: HibernatedBy,
    starts: u32,
) -> Outcome {
    let stage = stage_of(world, id, "talk")?;
    let start = stage.start.clone().ok_or("talk holds no start")?;
    ensure_eq!(
        (
            stage.state,
            stage.starts,
            stage.end.is_none(),
            start.live,
            start.hibernated.map(|held| (held.since, held.why))
        ),
        (
            StageState::Hibernating,
            starts,
            true,
            false,
            Some((since, why))
        ),
        "talk's state, its starts, whether its end is none, whether its start is live, and its hibernation"
    );
    Ok(())
}

fn a_conversation_hibernated_at_its_callers_word(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    world.host.advance(10 * 60);
    let at = world.host.now();
    taken(&world.engine().hibernate(&id, false))?;
    ensure_eq!(
        closed(&world),
        vec![format!("close {}", talk_of("one", 1))],
        "the agents closed"
    );
    hibernating(&world, &id, at, HibernatedBy::Caller, 1)
}

fn a_working_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    let talk = talk_of("one", 1);
    world.terminal.set_state(&talk, AgentState::Working);
    let reason = refused(&world.engine().hibernate(&id, false))?;
    ensure!(
        reason.contains(&talk.to_string()) && reason.contains("not quiet"),
        "the refusal names not the agent not quiet: {reason}"
    );
    ensure_eq!(closed(&world), Vec::<String>::new(), "the agents closed");
    ensure_eq!(
        (
            world.stage_state(&id, "talk"),
            hibernation(&world, &id, "talk")?
        ),
        (Some(StageState::Begun), None),
        "talk after the refusal"
    );
    let at = world.host.now();
    taken(&world.engine().hibernate(&id, true))?;
    ensure_eq!(
        closed(&world),
        vec![format!("close {talk}")],
        "the agents closed"
    );
    hibernating(&world, &id, at, HibernatedBy::Caller, 1)
}

fn refused_hibernates(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let fix = World::id("fix", "one");
    world.settle(&fix);
    let before = world.held(&fix);
    let reason = refused(&world.engine().hibernate(&fix, false))?;
    ensure!(
        reason.contains(&fix.to_string()) && reason.contains("does not hibernate"),
        "the refusal names not the place and that its kind does not hibernate its stage: {reason}"
    );
    ensure_eq!(world.held(&fix), before, "the fix after the refusal");
    let id = research(&world, "one")?;
    taken(&world.engine().hibernate(&id, false))?;
    let closes = closed(&world).len();
    let before = world.held(&id);
    let reason = refused(&world.engine().hibernate(&id, true))?;
    ensure!(
        reason.contains(&id.to_string()) && reason.contains("no live agent"),
        "the refusal names not the place and that it has no live agent: {reason}"
    );
    ensure_eq!(
        world.held(&id),
        before,
        "the research place after the refusal"
    );
    ensure_eq!(closed(&world).len(), closes, "the agents closed");
    Ok(())
}

fn room_freed(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    let id = research(&world, "one")?;
    world.settle(&id);
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code while talk runs"
    );
    world.host.advance(HOUR);
    let at = world.host.now();
    world.settle(&id);
    hibernating(&world, &id, at, HibernatedBy::Idle, 1)?;
    ensure!(
        world
            .stage_state(&two, "code")
            .is_some_and(StageState::running),
        "two's code did not start in the room talk freed: {:?}",
        world.stage_state(&two, "code")
    );
    Ok(())
}

/// What a hibernate asked during a tell answered, and the start's mark
/// that the tell is being handed as it then stood.
type During = (Answer, Option<Seconds>);

fn asking_own_work_a_tell_refuse_a_callers_hibernate(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    let talk = talk_of("one", 1);
    world.agent.set_asking(&talk, Reading::Yes);
    let reason = refused(&world.engine().hibernate(&id, false))?;
    ensure!(
        reason.contains("not quiet"),
        "the refusal names not the agent asking: {reason}"
    );
    world.agent.set_asking(&talk, Reading::No);
    world.agent.set_own_work(&talk, Reading::Yes);
    let reason = refused(&world.engine().hibernate(&id, false))?;
    ensure!(
        reason.contains("not quiet"),
        "the refusal names not the agent's own work: {reason}"
    );
    world.agent.set_own_work(&talk, Reading::No);
    // A tell handed to the agent while it works: the caller's hibernate
    // asked while it waits for the agent to take it.
    world.terminal.set_state(&talk, AgentState::Working);
    let during: Arc<Mutex<Option<During>>> = Arc::default();
    let into = during.clone();
    let other = world.engine();
    let (place, store, terminal, who) = (
        id.clone(),
        world.store.clone(),
        world.terminal.clone(),
        talk.clone(),
    );
    world.host.at_next_sleep(move || {
        // The agent is quiet while the hibernate is asked, so the mark
        // alone refuses it.
        terminal.set_state(&who, AgentState::Waiting);
        let answer = other.hibernate(&place, false);
        let mark = store.held(&place).and_then(|held| {
            held.stages
                .into_iter()
                .find_map(|stage| stage.start)
                .and_then(|start| start.handing)
        });
        *into.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some((answer, mark));
    });
    taken(&world.engine().tell(&id, "go on", None))?;
    let (answer, mark) = during
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take()
        .ok_or("the hibernate was not asked during the tell")?;
    ensure!(mark.is_some(), "the start was not marked handing");
    let reason = refused(&answer)?;
    ensure!(
        reason.contains("a tell is being handed") && reason.contains(&talk.to_string()),
        "the refusal names not the tell being handed: {reason}"
    );
    ensure_eq!(
        (
            start_now(&world, &id, "talk")?.handing,
            world.stage_state(&id, "talk"),
            closed(&world).len()
        ),
        (None, Some(StageState::Begun), 0),
        "the mark once the tell ended, talk, and the agents closed"
    );
    taken(&world.engine().hibernate(&id, false))
}

/// flume's own hibernate time, an hour (212).
const HOUR: Seconds = flume_core::HIBERNATE_AFTER;

/// The host's agents running, as status gives them (56, 70).
fn running(world: &World) -> Result<u32, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host.running),
        other => Err(format!("status answered {other:?}")),
    }
}

/// Settles the place each `step` seconds until `until` seconds have
/// passed, each reading talk not yet hibernated.
fn settle_through(
    world: &World,
    id: &PlaceId,
    step: Seconds,
    until: Seconds,
    stage: &str,
) -> Outcome {
    let mut passed = 0;
    while passed + step <= until {
        world.host.advance(step);
        passed += step;
        world.settle(id);
        ensure!(
            world.stage_state(id, stage) != Some(StageState::Hibernating),
            "{stage} hibernated {passed} seconds on"
        );
    }
    Ok(())
}

fn an_hour_after_the_last_turn(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    world.settle(&id);
    ensure_eq!(running(&world)?, 1, "agents running while talk runs");
    settle_through(&world, &id, 10 * 60, HOUR - 10 * 60, "talk")?;
    world.host.advance(10 * 60);
    let at = world.host.now();
    world.settle(&id);
    ensure_eq!(
        closed(&world),
        vec![format!("close {}", talk_of("one", 1))],
        "the agents closed"
    );
    hibernating(&world, &id, at, HibernatedBy::Idle, 1)?;
    ensure_eq!(running(&world)?, 0, "agents running once talk hibernated");
    Ok(())
}

fn a_turn_between_two_settles(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    let talk = talk_of("one", 1);
    world.settle(&id);
    let t = world.host.now();
    ensure_eq!(
        start_now(&world, &id, "talk")?.first_quiet,
        Some(t),
        "talk's quiet from flume's first reading of it"
    );
    // A turn no settle reads: the agent works and is quiet again by the
    // time its own quiet starts a settle, at t plus 50 minutes.
    world.terminal.set_state(&talk, AgentState::Working);
    world.host.advance(50 * 60);
    world.terminal.set_state(&talk, AgentState::Waiting);
    taken(&world.engine().settle_on_quiet(&id))?;
    world.host.advance(20 * 60);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Begun),
        "talk at t plus 70 minutes"
    );
    world.host.advance(40 * 60);
    world.settle(&id);
    hibernating(&world, &id, t + 110 * 60, HibernatedBy::Idle, 1)
}

fn asking_own_work_a_tell_hold_a_settle_off(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    let talk = talk_of("one", 1);
    world.settle(&id);
    for (what, set, clear) in [
        (
            "asking",
            Box::new(|world: &World, talk: &Identity| world.agent.set_asking(talk, Reading::Yes))
                as Box<dyn Fn(&World, &Identity)>,
            Box::new(|world: &World, talk: &Identity| world.agent.set_asking(talk, Reading::No))
                as Box<dyn Fn(&World, &Identity)>,
        ),
        (
            "its own work",
            Box::new(|world: &World, talk: &Identity| world.agent.set_own_work(talk, Reading::Yes)),
            Box::new(|world: &World, talk: &Identity| world.agent.set_own_work(talk, Reading::No)),
        ),
    ] {
        world.host.advance(HOUR - 60);
        set(&world, &talk);
        world.host.advance(60);
        world.settle(&id);
        ensure_eq!(
            (world.stage_state(&id, "talk"), closed(&world).len()),
            (Some(StageState::Begun), 0),
            format!("talk past its time while {what}")
        );
        world.host.advance(HOUR);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "talk"),
            Some(StageState::Begun),
            format!("talk an hour on while {what}")
        );
        clear(&world, &talk);
        world.settle(&id);
        settle_through(&world, &id, 20 * 60, HOUR - 20 * 60, "talk")?;
    }
    // A tell being handed to the agent once it is past its time: the
    // settle beside it passes it over, and its quiet starts again. A
    // caller keeps watch, so the tell settles nothing first.
    world.keep_watch();
    world.host.advance(HOUR);
    world.agent.receives_at(&talk, Some(2));
    let other = world.engine();
    let place = id.clone();
    world.host.at_next_sleep(move || {
        other.settle(Some(&place));
    });
    taken(&world.engine().tell(&id, "go on", None))?;
    ensure_eq!(
        (world.stage_state(&id, "talk"), closed(&world).len()),
        (Some(StageState::Begun), 0),
        "talk once a settle read it while a tell was handed"
    );
    world.settle(&id);
    settle_through(&world, &id, 20 * 60, HOUR - 20 * 60, "talk")?;
    world.host.advance(20 * 60);
    let at = world.host.now();
    world.settle(&id);
    hibernating(&world, &id, at, HibernatedBy::Idle, 1)
}

fn zero_never_hibernates_on_idling(_: &()) -> Outcome {
    let world = World::new();
    let mut given = launch("research", "one", Some("weigh the options"));
    given
        .settings
        .insert("talk".into(), super::agent_settings::hibernates_after("0"));
    let id = world.researching(given)?;
    settle_through(&world, &id, HOUR, 24 * HOUR, "talk")?;
    ensure_eq!(
        (world.stage_state(&id, "talk"), closed(&world).len()),
        (Some(StageState::Begun), 0),
        "talk a day on"
    );
    Ok(())
}

fn a_stage_whose_kind_does_not_hibernate_it(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "chat",
            false,
            vec![stage("talk", None, "Talk {{name}} over.\n\n{{brief}}")],
        )],
    );
    launched(&world.engine.launch(&launch("chat", "one", Some("x"))))?;
    let id = World::id("chat", "one");
    world.settle(&id);
    settle_through(&world, &id, HOUR, 24 * HOUR, "talk")?;
    let start = start_now(&world, &id, "talk")?;
    ensure_eq!(
        (
            world.stage_state(&id, "talk"),
            closed(&world).len(),
            start.live,
            start.hibernated
        ),
        (Some(StageState::Begun), 0, true, None),
        "talk a day on"
    );
    taken(&world.engine().finish(&id, false))?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Finished),
        "talk once its caller finished it"
    );
    Ok(())
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    let id = research(&world, "one")?;
    world.settle(&id);
    world.host.advance(HOUR);
    let at = world.host.now();
    let other = world.engine();
    let place = id.clone();
    world.store.before_next_write(move || {
        other.settle(Some(&place));
    });
    world.settle(&id);
    ensure_eq!(
        closed(&world),
        vec![format!("close {}", talk_of("one", 1))],
        "the agents closed"
    );
    hibernating(&world, &id, at, HibernatedBy::Idle, 1)
}

/// The patch place `name` whose code stopped short needing an answer at
/// the time given, settled, its agent kept open (42, 198).
pub(super) fn patch_asking(world: &World, name: &str) -> Result<(PlaceId, Seconds), String> {
    let id = world.patching(name)?;
    let at = stops_short(
        world,
        &id,
        0,
        &patch_code(name, 1),
        QUESTION,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::EndedShort),
        "code once it stopped short"
    );
    Ok((id, at))
}

/// The place waits on its caller's answer to `stage`, since `since` (42,
/// 130).
pub(super) fn waits_on_its_answer(
    world: &World,
    id: &PlaceId,
    stage: &str,
    since: Seconds,
) -> Outcome {
    let waiting = world
        .status(id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.since
        ),
        (Act::Retry, Some(stage), Waits::Answer, Some(since)),
        "the move the place waits on"
    );
    Ok(())
}

fn a_code_stage_kept_open_its_hour(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = patch_asking(&world, "one")?;
    let who = patch_code("one", 1);
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
    let then = at + KEEP_OPEN;
    let code = stage_of(&world, &id, "code")?;
    ensure_eq!(
        code.end.as_ref().and_then(|end| end.closed),
        Some(SessionClosed {
            at: then,
            by: ClosedBy::Hibernated,
            ended_at: Some(then),
            left_in_pane: false,
        }),
        "code's close"
    );
    ensure_eq!(
        closed(&world),
        vec![format!("close {who}")],
        "the agents closed"
    );
    let start = code.start.ok_or("code holds no start")?;
    ensure_eq!(
        (
            code.state,
            start.live,
            start.hibernated.map(|held| (held.since, held.why))
        ),
        (
            StageState::EndedShort,
            false,
            Some((then, HibernatedBy::KeptOpen))
        ),
        "code's state, whether its start is live, and its hibernation"
    );
    waits_on_its_answer(&world, &id, "code", at)
}

fn let_go_otherwise_first(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let (id, at) = patch_asking(&world, "one")?;
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    world.host.advance(20 * 60);
    stops_at_limit(&world, "two", 1, Some(at + 5 * 3600));
    world.settle(&two);
    world.settle(&id);
    let code = stage_of(&world, &id, "code")?;
    ensure_eq!(
        code.end
            .as_ref()
            .and_then(|end| end.closed)
            .map(|closed| closed.by),
        Some(ClosedBy::AccountSpent),
        "code's close"
    );
    ensure_eq!(
        code.start.and_then(|start| start.hibernated),
        None,
        "code's hibernation"
    );
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
    ensure_eq!(
        hibernation(&world, &id, "code")?,
        None,
        "code's hibernation once its hour passed"
    );
    Ok(())
}
