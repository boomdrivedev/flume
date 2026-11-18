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
