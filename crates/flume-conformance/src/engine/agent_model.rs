//! An agent's model (172; record 0026): its program's own name or a
//! table of programs, given by any of the eight layers and taken whole
//! from the first, the agent's entry its program's; asked of the agent's
//! profile, which takes it or refuses, the launch or the stage refused
//! naming the model as written, the program, the stage and the layer; a
//! table naming no entry for the agent's program refused so, never
//! passing to a later layer's; and nothing asked or passed where no layer
//! gives one (4, 8, 27, 32, 88, 144).

use super::agent_modes::{change_kind, fix_kind, nothing_made};
use super::agent_settings::{for_every_stage, for_stage, opened};
use super::settling::write_change;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent model: the kind's model is kept as written and passed, the agent's settings giving it from kind",
            cites: &["172", "32", "88"],
            run: a_name_kept_as_written,
        },
        Scenario {
            name: "agent model: the launch's table of programs passes the agent's program's entry, status giving the table as written from launch",
            cites: &["172", "32", "56"],
            run: a_table_of_programs,
        },
        Scenario {
            name: "agent model: a table naming no entry for the agent's program refuses the launch naming the table, the program, the stage and the layer, the kind's model not taken, and nothing is made",
            cites: &["4", "8", "32", "169", "172"],
            run: no_entry_for_the_agents_program,
        },
        Scenario {
            name: "agent model: a model is asked at the launch and the start and passed, and a profile with no way to give one refuses the launch in its words, naming the model, the program, the stage and the layer, and nothing is made",
            cites: &["4", "8", "88", "170", "172"],
            run: a_profile_with_no_way_to_give_a_model,
        },
        Scenario {
            name: "agent model: with no model in any layer nothing is asked of the profile, the command passes none and status gives the agent none",
            cites: &["144", "172"],
            run: no_model_in_any_layer,
        },
        Scenario {
            name: "agent model: a later start whose host table gives a table naming no entry for the agent's program is refused naming it, the program, the stage and host-kind, starting no agent, until a retry after the table is put right",
            cites: &["27", "32", "172"],
            run: a_refusal_at_a_later_start,
        },
    ]
}

/// Settings giving the model `name`, one name for every program.
pub(super) fn model(name: &str) -> Settings {
    Settings {
        model: Some(name.into()),
        ..Settings::default()
    }
}

/// Settings giving a model's table of programs, each entry as written.
pub(super) fn models(entries: &[(&str, &str)]) -> Result<Settings, String> {
    Ok(Settings {
        model: Some(PerProgram::from_programs(
            entries
                .iter()
                .map(|(program, name)| (program.to_string(), name.to_string()))
                .collect::<BTreeMap<_, _>>(),
        )?),
        ..Settings::default()
    })
}

/// The model the command of the start under `identity` passed, none
/// where it passed none.
pub(super) fn model_passed(world: &World, identity: &Identity) -> Result<Option<String>, String> {
    let command = opened(world, identity)?;
    let names: Vec<&String> = command
        .args
        .windows(2)
        .filter(|pair| pair[0] == "--model")
        .map(|pair| &pair[1])
        .collect();
    match names.as_slice() {
        [] => Ok(None),
        [name] => Ok(Some(name.to_string())),
        _ => Err(format!(
            "{identity}'s command passed more than one model: {:?}",
            command.args
        )),
    }
}

fn given(layer: Layer, value: &str) -> Option<Given> {
    Some(Given {
        layer,
        value: value.into(),
        file: None,
    })
}

fn a_name_kept_as_written(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![fix_kind(for_every_stage(model("large")))?]);
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    ensure_eq!(
        model_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("large".to_string()),
        "the model passed"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        agent.settings.settings.model,
        Some(PerProgram::One("large".into())),
        "the agent's model"
    );
    ensure_eq!(
        agent.settings.from.get("model").cloned(),
        given(Layer::Kind, "large"),
        "where the model came from"
    );
    Ok(())
}

fn a_table_of_programs(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = models(&[("agent", "large"), ("other", "small")])?;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    ensure_eq!(
        model_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("large".to_string()),
        "the agent's entry passed"
    );
    ensure_eq!(
        world.agent.log.of("model"),
        vec!["model large".to_string(); 2],
        "the entry asked at the launch and the start"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        agent.settings.from.get("model").cloned(),
        given(Layer::Launch, r#"{ agent = "large", other = "small" }"#),
        "the model as the launch wrote it"
    );
    ensure_eq!(
        agent.settings.settings.model,
        models(&[("agent", "large"), ("other", "small")])?.model,
        "the start keeps the table as written"
    );
    Ok(())
}

fn no_entry_for_the_agents_program(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![fix_kind(for_every_stage(model("large")))?]);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = models(&[("other", "small")])?;
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        [
            r#"`{ other = "small" }`"#,
            "names no entry for `agent`",
            "`code`",
            "`launch`"
        ]
        .iter()
        .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    ensure!(
        !reason.contains("large"),
        "the kind's model taken in the table's stead: {reason}"
    );
    ensure_eq!(
        world.agent.log.of("model"),
        Vec::<String>::new(),
        "models asked"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_profile_with_no_way_to_give_a_model(_: &()) -> Outcome {
    let world = World::new();
    let mut one = launch("fix", "one", Some("mend it"));
    one.every_stage = model("large");
    launched(&world.engine.launch(&one))?;
    world.settle(&World::id("fix", "one"));
    // Asked of the profile at the launch, before anything is made, and
    // again at the start, before its command is built (172).
    ensure_eq!(
        world.agent.log.of("model"),
        vec!["model large".to_string(); 2],
        "models asked at the launch and the start"
    );
    ensure_eq!(
        model_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("large".to_string()),
        "the model passed"
    );
    world.agent.take_no_model();
    let mut two = launch("fix", "two", Some("mend it"));
    two.every_stage = model("large");
    let reason = refused(&world.engine.launch(&two))?;
    ensure!(
        ["`large`", "`agent`", "`code`", "`launch`"]
            .iter()
            .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    ensure!(
        reason.contains(&no_model("large")),
        "the refusal is not in the profile's words: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 1, "places made");
    ensure!(
        world.held(&World::id("fix", "two")).is_none(),
        "held state written for the refused launch"
    );
    Ok(())
}

fn no_model_in_any_layer(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(model_passed(&world, &code)?, None, "the model passed");
    ensure_eq!(
        world.agent.log.of("model"),
        Vec::<String>::new(),
        "models asked"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(agent.settings.settings.model, None, "the agent's model");
    ensure!(
        !agent.settings.from.contains_key("model"),
        "status names a layer of the model"
    );
    Ok(())
}

fn a_refusal_at_a_later_start(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![change_kind(KindSettings::default())?]);
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    world.settle(&id);
    world.host.set_settings(
        None,
        "change",
        for_stage("code", models(&[("other", "small")])?),
    );
    world.terminal.log.clear();
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Refused),
        "code"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let refusal = world.status(&id)?.stages[1]
        .refused_setting
        .clone()
        .ok_or("no setting named")?;
    ensure!(
        [
            r#"`{ other = "small" }`"#,
            "`agent`",
            "`code`",
            "`host-kind`"
        ]
        .iter()
        .all(|named| refusal.contains(named)),
        "the refusal: {refusal}"
    );
    world
        .host
        .set_settings(None, "change", KindSettings::default());
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code once the table is put right"
    );
    ensure_eq!(
        model_passed(&world, &World::identity("change", "plan", "code", 1))?,
        None,
        "code's model passed once the table is put right"
    );
    Ok(())
}
