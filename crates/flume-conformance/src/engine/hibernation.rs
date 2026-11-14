//! A stage whose kind hibernates it (212, 213; record 0069): hibernated
//! once its conversation idles past its hibernate time, at its caller's
//! word or at the end of its keep-open wait, its agent ended and its
//! session kept, its room and its account freed; and resumed in its own
//! session by its caller's resume, a tell or the answer it waits on.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Answer;
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
            name: "hibernation: Room freed: talk hibernates on a host at its bound, its starts unchanged and its end none, and another place's stage waiting for room starts",
            cites: &["48", "70", "212"],
            run: room_freed,
        },
        Scenario {
            name: "hibernation: Asking, own work, a tell: a caller's unforced hibernate of a talk agent asking, with its own work running, or being handed a tell, is refused, and the tell's mark is cleared once it ends",
            cites: &["54", "90", "91", "212"],
            run: asking_own_work_a_tell_refuse_a_callers_hibernate,
        },
    ]
}

/// The research place `name`'s talk start `start`.
pub(super) fn talk_of(name: &str, start: u32) -> Identity {
    World::identity("research", name, "talk", start)
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
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code while talk runs"
    );
    let at = world.host.now();
    taken(&world.engine().hibernate(&id, false))?;
    hibernating(&world, &id, at, HibernatedBy::Caller, 1)?;
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
