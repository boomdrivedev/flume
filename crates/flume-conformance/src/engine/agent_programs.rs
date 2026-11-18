//! The programs flume runs (170, C.4, C.5; record 0026): a program bound
//! by a profile that names no way to give it a model, an effort or a
//! mode is taken where its settings give none of them, and refused
//! naming the program, the stage and the layer where they give any; and
//! the stream's terminal asked whether it starts each stage's program's
//! kind, at the launch before anything is made and at each start under
//! the chosen account's environment, a refusal in its words refusing the
//! launch, or leaving the stage refused until a retry (4, 8, 27, 32).

use super::agent_modes::nothing_made;
use super::agent_settings::{opened, to_code};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::FakeAgent;
use flume_seams::*;
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent programs: a program whose profile names no way to give a model, an effort or a mode is taken given none, its command passing none, and refused given any of them, naming the program, the stage and the layer, nothing made",
            cites: &["4", "8", "170"],
            run: a_profile_with_no_ways,
        },
        Scenario {
            name: "agent programs: a kind the stream's terminal cannot start refuses the launch in the terminal's words, nothing made, and the launch is taken once the terminal starts it",
            cites: &["4", "170", "S75"],
            run: a_kind_the_terminal_cannot_start,
        },
        Scenario {
            name: "agent programs: a later start whose kind the terminal refuses is refused in the terminal's words, starting no agent, until a retry once the terminal starts it",
            cites: &["27", "32", "170"],
            run: a_later_start_the_terminal_refuses,
        },
    ]
}

/// The words the fake terminal refuses the kind `agent` with, naming an
/// install as herdr's do (C.5).
const INSTALL: &str =
    "the agent's integration is not installed: run `fake-terminal integration install agent`";

/// Settings naming the program `program`.
fn program(program: &str) -> Settings {
    Settings {
        program: Some(program.into()),
        ..Settings::default()
    }
}

fn a_profile_with_no_ways(_: &()) -> Outcome {
    let world = World::new();
    let other = Arc::new(FakeAgent::new(world.terminal.clone()));
    other.take_no_model();
    world.resolver.bind_agent("other", other.clone());
    let mut one = launch("fix", "one", Some("mend it"));
    one.every_stage = program("other");
    launched(&world.engine.launch(&one))?;
    world.settle(&World::id("fix", "one"));
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure!(
        !command
            .args
            .iter()
            .any(|arg| ["--model", "--effort", "--mode"].contains(&arg.as_str())),
        "the command passes a setting none gave: {:?}",
        command.args
    );
    for (name, settings) in [
        (
            "two",
            Settings {
                model: Some("large".into()),
                ..program("other")
            },
        ),
        (
            "three",
            Settings {
                effort: Some("high".into()),
                ..program("other")
            },
        ),
        (
            "four",
            Settings {
                mode: Some(ModeSetting::Flume(Mode::Edits)),
                ..program("other")
            },
        ),
    ] {
        let mut given = launch("fix", name, Some("mend it"));
        given.every_stage = settings;
        let reason = refused(&world.engine.launch(&given))?;
        ensure!(
            ["`other`", "`code`", "`launch`"]
                .iter()
                .all(|named| reason.contains(named)),
            "the refusal of {name}: {reason}"
        );
        ensure!(
            world.held(&World::id("fix", name)).is_none(),
            "held state written for {name}"
        );
    }
    ensure_eq!(world.place.log.count("make"), 1, "places made");
    Ok(())
}

fn a_kind_the_terminal_cannot_start(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.refuse_kind("agent", INSTALL);
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    ensure!(reason.contains(INSTALL), "the refusal: {reason}");
    ensure_eq!(
        world.terminal.log.of("starts"),
        vec!["starts agent".to_string()],
        "the terminal asked under no account's environment"
    );
    nothing_made(&world, &World::id("fix", "one"))?;
    world.terminal.start_kind("agent");
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    world.settle(&World::id("fix", "one"));
    ensure_eq!(
        world.stage_state(&World::id("fix", "one"), "code"),
        Some(StageState::Begun),
        "code once the terminal starts its kind"
    );
    Ok(())
}

fn a_later_start_the_terminal_refuses(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    world.terminal.refuse_kind("agent", INSTALL);
    world.terminal.log.clear();
    world.tick("change/plan", "plan", 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Refused),
        "verify"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let refusal = world.status(&id)?.stages[2]
        .refused_setting
        .clone()
        .ok_or("no refusal named")?;
    ensure!(
        refusal.contains(INSTALL) && refusal.contains("`verify`"),
        "the refusal: {refusal}"
    );
    world.terminal.start_kind("agent");
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("verify".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify once the terminal starts its kind"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}
