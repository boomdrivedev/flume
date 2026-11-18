//! Statechart kinds (4, 175 to 178, C.2, S76, S77): a launch of one taken
//! or refused for what flume does not bind or run, its stage template's
//! sessions, one tick a settle over evidence read at one head, a stage's
//! end, the run's end and its submission, and a decision answered once.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "statechart: a statechart kind launched makes its place, holding its statechart, the stage template and its first state",
            cites: &["2", "21", "175", "176"],
            run: a_statechart_kind_launched,
        },
        Scenario {
            name: "statechart: a place settled twice with nothing changed ticks once to start its stage's session, then fires nothing and performs no effect",
            cites: &["74", "176"],
            run: one_tick_over_one_head,
        },
        Scenario {
            name: "statechart: a region reading a sibling's state moves at the settle after the one the sibling moved in",
            cites: &["176"],
            run: a_sibling_sees_a_move_at_the_next_settle,
        },
        Scenario {
            name: "statechart: a state running a template flume does not bind refuses the launch, nothing made",
            cites: &["4", "175"],
            run: a_template_flume_does_not_bind,
        },
        Scenario {
            name: "statechart: an evidence or an effect flume does not bind refuses the launch naming the atom, nothing made",
            cites: &["4", "178"],
            run: an_atom_flume_does_not_bind,
        },
        Scenario {
            name: "statechart: a rule flume does not bind refuses the launch naming the rule and the state",
            cites: &["4", "175"],
            run: a_rule_flume_does_not_bind,
        },
        Scenario {
            name: "statechart: a stage with no prompt refuses the launch naming the state and its prompt",
            cites: &["4", "C.2"],
            run: a_stage_with_no_prompt,
        },
        Scenario {
            name: "statechart: a deliverable naming {{file}} on a stage of listed agents refuses the launch naming the state and the part",
            cites: &["4", "C.2"],
            run: a_file_no_rule_gives,
        },
        Scenario {
            name: "statechart: a join by a count above the agents listed refuses the launch naming the state and its join",
            cites: &["4", "175"],
            run: a_count_above_the_agents_listed,
        },
        Scenario {
            name: "statechart: a statechart kind naming its landing fix is refused naming the field, nothing made",
            cites: &["4", "175"],
            run: a_landing_fix_named,
        },
    ]
}

/// One region `main` running the stage template at `work` with
/// `params`, then final at `done` once it passes; `lands` as given.
pub fn one_stage(lands: bool, machine: &str, transitions: &str) -> String {
    format!(
        r#"[kinds.one]
title = "One stage"
lands = {lands}

[kinds.one.regions.main]
initial = "work"

[kinds.one.regions.main.states.work]
machine = "{machine}"
transitions = [{transitions}]

[kinds.one.regions.main.states.done]
final = true
"#
    )
}

/// The one stage's transitions: on to `done` once it passes.
pub const ON_PASSED: &str = r#"{ when = { final = "passed" }, to = "done" }"#;

/// `one`, its `main.work` a stage of `agents` delivering commits.
pub fn one(text: &str, agents: ChartAgents) -> Kind {
    chart_kind(
        "one",
        text,
        vec![(
            "main.work",
            chart_stage(agents, Deliverable::Commits, "Work on {{brief}}."),
        )],
    )
}

/// A launch of `one` refused, its words, and nothing made: no place, no
/// held state, no agent opened (4; core.8).
fn refused_whole(world: &World, kind: Kind) -> Result<String, String> {
    world.kinds.set_kinds(REPOSITORY, vec![kind]);
    let reason = refused(&world.engine.launch(&launch("one", "a", Some("the work"))))?;
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure_eq!(world.store.log.count("write"), 0, "held states written");
    ensure!(world.store.places().is_empty(), "held state was left");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(reason)
}

fn a_statechart_kind_launched(_: &()) -> Outcome {
    let world = World::new();
    let text = one_stage(false, "stage", ON_PASSED);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![one(&text, listed(1))]);
    launched(&world.engine.launch(&launch("one", "a", Some("the work"))))?;
    ensure_eq!(world.place.log.count("make"), 1, "places made");
    let id = World::id("one", "a");
    let held = world.held(&id).ok_or("the place holds no state")?;
    ensure_eq!(held.phase, Phase::Live, "the place's phase");
    ensure_eq!(
        held.place
            .kind
            .statechart
            .as_ref()
            .map(|chart| chart.text.as_str()),
        Some(text.as_str()),
        "the kind's statechart held"
    );
    let chart = held.chart.ok_or("the place holds no statechart")?;
    ensure_eq!(
        chart.template.as_str(),
        flume_core::atoms::STAGE_TEMPLATE,
        "the stage template held"
    );
    ensure_eq!(
        chart.state.regions.get("main").map(String::as_str),
        Some("work"),
        "the region's state"
    );
    ensure_eq!(
        chart.state.regions.get("main.work.run").map(String::as_str),
        Some("working"),
        "the stage template's region's state"
    );
    ensure!(!chart.over, "the run reads over at its launch");
    ensure_eq!(
        chart.stages.get("main.work").map(|stage| stage.entry),
        Some(1),
        "the stage's entry recorded"
    );
    let session = held
        .stages
        .iter()
        .find(|stage| stage.name == "main.work#1/1")
        .ok_or("the stage's one session was not recorded")?;
    ensure_eq!(held.stages.len(), 1, "sessions recorded");
    let start = session
        .start
        .as_ref()
        .filter(|start| start.live)
        .ok_or("the session's start is not live")?;
    ensure_eq!(
        start.identity.stage.as_str(),
        "main.work#1/1",
        "the identity's stage"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn one_tick_over_one_head(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![one(&one_stage(false, "stage", ON_PASSED), listed(1))],
    );
    launched(&world.engine.launch(&launch("one", "a", Some("the work"))))?;
    let id = World::id("one", "a");
    let first = world.held(&id).ok_or("the place holds no state")?;
    ensure_eq!(first.stages.len(), 1, "sessions after the launch's tick");
    for settle in 1..=2 {
        world.settle(&id);
        let held = world.held(&id).ok_or("the place holds no state")?;
        ensure_eq!(
            held.chart.as_ref().map(|chart| &chart.state),
            first.chart.as_ref().map(|chart| &chart.state),
            format!("the statechart's state after settle {settle}")
        );
        ensure_eq!(
            held.stages.len(),
            1,
            format!("sessions after settle {settle}")
        );
        ensure_eq!(
            held.stages[0].starts,
            1,
            format!("the session's starts after settle {settle}")
        );
        ensure_eq!(
            world.terminal.log.count("open"),
            1,
            format!("agents opened after settle {settle}")
        );
    }
    Ok(())
}

/// Two regions side by side: `a` moves on at once, and `b` moves once it
/// reads `a` moved.
const SIBLINGS: &str = r#"[kinds.pair]
title = "Two regions"

[kinds.pair.regions.a]
initial = "x"

[kinds.pair.regions.a.states.x]
transitions = [{ when = { always = true }, to = "y" }]

[kinds.pair.regions.a.states.y]
final = true

[kinds.pair.regions.b]
initial = "w"

[kinds.pair.regions.b.states.w]
transitions = [{ when = { region = { name = "a", in = ["y"] } }, to = "z" }]

[kinds.pair.regions.b.states.z]
final = true
"#;

fn a_sibling_sees_a_move_at_the_next_settle(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![chart_kind("pair", SIBLINGS, vec![])]);
    launched(&world.engine.launch(&launch("pair", "a", None)))?;
    let id = World::id("pair", "a");
    let regions = |world: &World| -> Result<(String, String), String> {
        let held = world.held(&id).ok_or("the place holds no state")?;
        let chart = held.chart.ok_or("the place holds no statechart")?;
        let at = |region: &str| chart.state.regions.get(region).cloned().unwrap_or_default();
        Ok((at("a"), at("b")))
    };
    ensure_eq!(
        regions(&world)?,
        ("y".to_string(), "w".to_string()),
        "the regions after the launch's settle"
    );
    world.settle(&id);
    ensure_eq!(
        regions(&world)?,
        ("y".to_string(), "z".to_string()),
        "the regions after the next settle"
    );
    Ok(())
}

fn a_template_flume_does_not_bind(_: &()) -> Outcome {
    let world = World::new();
    let reason = refused_whole(
        &world,
        one(&one_stage(false, "session", ON_PASSED), listed(1)),
    )?;
    ensure!(
        reason.contains("`session`") && reason.contains("one@1"),
        "the refusal names not the kind and the template: {reason}"
    );
    Ok(())
}

fn an_atom_flume_does_not_bind(_: &()) -> Outcome {
    let world = World::new();
    let lamp = r#"{ when = { ev = "lamp.switch", is = "on" }, to = "done" }"#;
    let reason = refused_whole(&world, one(&one_stage(false, "stage", lamp), listed(1)))?;
    ensure!(
        reason.contains("`lamp.switch`"),
        "the refusal names not the evidence: {reason}"
    );
    let world = World::new();
    let effect =
        r#"{ when = { final = "passed" }, to = "done", effects = [{ do = "start_sessions" }] }"#;
    let reason = refused_whole(&world, one(&one_stage(false, "stage", effect), listed(1)))?;
    ensure!(
        reason.contains("`start_sessions`"),
        "the refusal names not the effect: {reason}"
    );
    Ok(())
}

fn a_rule_flume_does_not_bind(_: &()) -> Outcome {
    let world = World::new();
    let rule = ChartAgents::Rule {
        rule: "every-persona-definition".into(),
        pattern: Some("personas/*.md".into()),
    };
    let reason = refused_whole(&world, one(&one_stage(false, "stage", ON_PASSED), rule))?;
    ensure!(
        reason.contains("`every-persona-definition`") && reason.contains("main.work"),
        "the refusal names not the rule and the state: {reason}"
    );
    Ok(())
}

fn a_stage_with_no_prompt(_: &()) -> Outcome {
    let world = World::new();
    let mut kind = one(&one_stage(false, "stage", ON_PASSED), listed(1));
    if let Some(chart) = kind.statechart.as_mut() {
        chart
            .stages
            .values_mut()
            .for_each(|stage| stage.prompt = None);
    }
    let reason = refused_whole(&world, kind)?;
    ensure!(
        reason.contains("main.work") && reason.contains("prompt"),
        "the refusal names not the state and its prompt: {reason}"
    );
    Ok(())
}

fn a_file_no_rule_gives(_: &()) -> Outcome {
    let world = World::new();
    let mut kind = one(&one_stage(false, "stage", ON_PASSED), listed(2));
    if let Some(chart) = kind.statechart.as_mut() {
        chart.stages.values_mut().for_each(|stage| {
            stage.deliverables = vec![Deliverable::CommittedReport {
                path: "out/{{file}}".into(),
            }]
        });
    }
    let reason = refused_whole(&world, kind)?;
    ensure!(
        reason.contains("main.work") && reason.contains("{{file}}"),
        "the refusal names not the state and the part: {reason}"
    );
    Ok(())
}

fn a_count_above_the_agents_listed(_: &()) -> Outcome {
    let world = World::new();
    let mut kind = one(&one_stage(false, "stage", ON_PASSED), listed(2));
    if let Some(chart) = kind.statechart.as_mut() {
        chart
            .stages
            .values_mut()
            .for_each(|stage| stage.join = Join::Count(3));
    }
    let reason = refused_whole(&world, kind)?;
    ensure!(
        reason.contains("main.work") && reason.contains("count:3"),
        "the refusal names not the state and its join: {reason}"
    );
    Ok(())
}

fn a_landing_fix_named(_: &()) -> Outcome {
    let world = World::new();
    let mut kind = one(&one_stage(true, "stage", ON_PASSED), listed(1));
    kind.fix = Some("flume/fix@1".into());
    let reason = refused_whole(&world, kind)?;
    ensure!(
        reason.contains("landing_fix") && reason.contains("flume/fix@1"),
        "the refusal names not the field: {reason}"
    );
    Ok(())
}
