//! Held state: one versioned document per place, written by
//! compare-and-set (73 to 77, 102, 103).

use super::repeating_stages::GROUPS;
use super::statecharts::{one, one_stage, ON_PASSED};
use super::world::*;
use crate::scenario::sample_held;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Step;
use flume_seams::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "held state: a write refused because the version moved is read and decided again",
            cites: &["75", "103"],
            run: a_refused_write_is_decided_again,
        },
        Scenario {
            name: "held state: a settle reads code's prompt received while another, run before its write, reads code begun and its agent stopped with nothing made and ends it lost; the first settle's receipt then writes nothing, and code stays lost",
            cites: &["35", "38", "75", "103"],
            run: a_receipt_read_before_another_settle_ended_the_start,
        },
        Scenario {
            name: "held state: a first write over a place's held state is refused",
            cites: &["75", "103", "3"],
            run: a_second_first_write,
        },
        Scenario {
            name: "held state: a statechart place whose stage recorded its sessions, read by an engine built anew after a restart, settles from the same engine state, starting no session twice and performing no effect already proven",
            cites: &["74", "76", "176"],
            run: a_restart_between_two_settles,
        },
        Scenario {
            name: "held state: a change@2 place whose held state was written before statechart kinds, settled after them while its code stage repeats, runs its next repetition, verify and its submission as before, holding no statechart and no session and ticked never",
            cites: &["21", "140", "177", "S77"],
            run: a_change_2_place_across_the_release,
        },
    ]
}

fn a_refused_write_is_decided_again(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    ensure!(
        world.engine.create(&sample_held(&id, REPOSITORY)).is_some(),
        "the first write was refused"
    );
    let decisions = Arc::new(AtomicUsize::new(0));
    let other = world.engine();
    let other_id = id.clone();
    world.store.before_next_write(move || {
        other.update(&other_id, |held| {
            held.place.mark = Some(format!("{}b", held.place.mark.clone().unwrap_or_default()));
            Step::Write(())
        });
    });
    let counted = decisions.clone();
    world.engine.update(&id, |held| {
        counted.fetch_add(1, Ordering::SeqCst);
        held.place.mark = Some(format!("{}a", held.place.mark.clone().unwrap_or_default()));
        Step::Write(())
    });
    ensure_eq!(decisions.load(Ordering::SeqCst), 2, "the act's decisions");
    ensure_eq!(
        world.held(&id).and_then(|held| held.place.mark),
        Some("ba".to_string()),
        "the mark the two acts left"
    );
    Ok(())
}

fn a_receipt_read_before_another_settle_ended_the_start(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    let code = World::identity("fix", "one", "code", 1);
    world.agent.receives_at(&code, Some(2));
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let id = World::id("fix", "one");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Started),
        "code once launched, its prompt sent and not read received"
    );
    // The agent takes its prompt and its session ends with nothing made;
    // a second settle, run just before the first writes the receipt it
    // read, reads code begun, then its agent stopped, and ends it lost.
    world.agent.receives_at(&code, Some(1));
    world.terminal.kill(&code);
    let (other, place) = (world.engine(), id.clone());
    world.store.before_next_write(move || {
        other.settle(Some(&place));
    });
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code once both settles read it"
    );
    ensure!(
        world
            .held(&id)
            .and_then(|held| held.stages[0].start.clone())
            .is_some_and(|start| !start.live),
        "code's start reads live"
    );
    Ok(())
}

fn a_second_first_write(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let mut held = sample_held(&id, REPOSITORY);
    ensure!(
        world.engine.create(&held).is_some(),
        "the first write was refused"
    );
    held.place.mark = Some("second".into());
    ensure!(
        world.engine.create(&held).is_none(),
        "a second first write was taken"
    );
    ensure_eq!(
        world.held(&id).and_then(|held| held.place.mark),
        None,
        "the held mark"
    );
    Ok(())
}

fn a_restart_between_two_settles(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![one(&one_stage(false, "stage", ON_PASSED), listed(2))],
    );
    launched(&world.engine.launch(&launch("one", "a", Some("the work"))))?;
    let id = World::id("one", "a");
    let before = world.held(&id).ok_or("the place holds no state")?;
    ensure_eq!(before.stages.len(), 2, "sessions recorded");
    let chart = before
        .chart
        .clone()
        .ok_or("the place holds no statechart")?;
    world.host.restart();
    let rebuilt = world.engine();
    let read = world.kinds.log.count("read_statechart");
    rebuilt.settle(Some(&id));
    ensure!(
        world.kinds.log.count("read_statechart") > read,
        "the settle after the restart did not read the statechart back"
    );
    let after = world.held(&id).ok_or("the place's held state was lost")?;
    let next = after.chart.clone().ok_or("the statechart was lost")?;
    ensure_eq!(next.state, chart.state, "the engine's state");
    ensure_eq!(next.stages, chart.stages, "the stages' entries");
    ensure_eq!(after.stages.len(), 2, "sessions recorded");
    ensure_eq!(
        after
            .stages
            .iter()
            .map(|stage| stage.starts)
            .collect::<Vec<_>>(),
        vec![1, 1],
        "each session's starts"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

/// The place holds no statechart and no session, and no tick read a
/// statechart back for it (177).
fn as_before(world: &World, id: &PlaceId, when: &str) -> Outcome {
    let held = world.held(id).ok_or("the place holds no state")?;
    ensure!(held.chart.is_none(), "{when}: the place holds a statechart");
    ensure!(
        held.stages.iter().all(|stage| stage.session.is_none()),
        "{when}: a stage holds a session"
    );
    ensure_eq!(
        world.kinds.log.count("read_statechart"),
        0,
        format!("{when}: statecharts read back")
    );
    Ok(())
}

fn a_change_2_place_across_the_release(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    launched(
        &world
            .engine
            .launch(&launch("change@2", "plan", Some("plan it"))),
    )?;
    let id = World::id("change", "plan");
    world.settle(&id);
    world.tick_groups("change/plan", "plan", GROUPS, 0);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's first repetition"
    );
    // Its held state as a release before statechart kinds wrote it: none
    // of their fields, and read back the same (21, 177).
    let held = world.held(&id).ok_or("the place holds no state")?;
    let mut json = serde_json::to_value(&held).map_err(|error| error.to_string())?;
    let document = json.as_object_mut().ok_or("the held state is no object")?;
    ensure!(
        !document.contains_key("chart"),
        "the held state writes a chart"
    );
    let stages = document
        .get_mut("stages")
        .and_then(|stages| stages.as_array_mut())
        .ok_or("the held state has no stages")?;
    for stage in stages.iter_mut() {
        let stage = stage.as_object_mut().ok_or("a stage is no object")?;
        ensure!(!stage.contains_key("session"), "a stage writes a session");
    }
    let read: HeldState = serde_json::from_value(json).map_err(|error| error.to_string())?;
    ensure_eq!(read, held, "the held state written before, read back");
    world.host.restart();
    as_before(&world, &id, "after its launch")?;
    world.tick_groups("change/plan", "plan", GROUPS, 2);
    world.engine().settle(Some(&id));
    as_before(&world, &id, "after group 1")?;
    let group = world.held(&id).ok_or("the place holds no state")?.stages[1]
        .start
        .clone()
        .and_then(|start| start.group)
        .and_then(|group| group.number);
    ensure_eq!(
        group,
        Some("2".to_string()),
        "code's next repetition's group"
    );
    world.tick_groups("change/plan", "plan", GROUPS, 5);
    world.engine().settle(Some(&id));
    as_before(&world, &id, "after every group")?;
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify"
    );
    super::settling::archive(&world, "plan");
    world.engine().settle(Some(&id));
    as_before(&world, &id, "after verify")?;
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    Ok(())
}
