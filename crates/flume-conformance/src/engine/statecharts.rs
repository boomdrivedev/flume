//! Statechart kinds (4, 175 to 178, C.2, S76, S77): a launch of one taken
//! or refused for what flume does not bind or run, its stage template's
//! sessions, one tick a settle over evidence read at one head, a stage's
//! end, the run's end and its submission, and a decision answered once.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::collections::BTreeMap;

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
            name: "statechart: a stage of every-file records one session per file its glob matches at the head, in path order, each starting one fresh agent whose identity names its session",
            cites: &["30", "33", "175"],
            run: one_session_per_persona,
        },
        Scenario {
            name: "statechart: a stage entered again records its sessions afresh under entry 2, and those of entry 1 stay as they stood",
            cites: &["175", "176"],
            run: a_stage_entered_again,
        },
        Scenario {
            name: "statechart: a stage's end read passed, sent back or stopped moves its template's region to that final state at that settle, and the kind's transition reading it moves at the next",
            cites: &["175", "176"],
            run: the_templates_ends,
        },
        Scenario {
            name: "statechart: of three sessions two deliver and the third works on: under all the stage works on, under any it passed at the first delivery and under count:2 at the second, each passed stage's working session ended once quiet",
            cites: &["31", "175"],
            run: a_join_of_all_of_any_and_of_a_count,
        },
        Scenario {
            name: "statechart: a session's found work under its stage's bound sends the stage back, and the stage the kind enters next starts its sessions with the found words, the finding session ended once quiet",
            cites: &["112", "131", "175"],
            run: found_work_sends_the_stage_back_with_its_words,
        },
        Scenario {
            name: "statechart: a session's found work once its stage was sent back as often as its bound ends the stage stopped",
            cites: &["112", "175"],
            run: past_its_bound,
        },
        Scenario {
            name: "statechart: a stage whose rule matches no file at the head ends stopped, and status names the rule and its pattern",
            cites: &["175"],
            run: a_rule_that_matches_no_file,
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

/// One region `stages` whose `test` runs a session per persona, each
/// delivering its own result, then final at `passed`.
const PERSONAS: &str = r#"[kinds.persona]
title = "Persona test"

[kinds.persona.regions.stages]
initial = "test"

[kinds.persona.regions.stages.states.test]
machine = "stage"
transitions = [{ when = { final = "passed" }, to = "passed" }]

[kinds.persona.regions.stages.states.passed]
final = true
"#;

/// `persona`, its `stages.test` a stage of `every-file` over
/// `personas/*.md`, each session committing `results/{{file}}` (175).
pub fn persona() -> Kind {
    chart_kind(
        "persona",
        PERSONAS,
        vec![(
            "stages.test",
            chart_stage(
                ChartAgents::Rule {
                    rule: "every-file".into(),
                    pattern: Some("personas/*.md".into()),
                },
                Deliverable::CommittedReport {
                    path: "results/{{file}}".into(),
                },
                "Test {{file}} as session {{session}}.",
            ),
        )],
    )
}

fn one_session_per_persona(_: &()) -> Outcome {
    let world = World::new();
    world.commit("main", &["personas/a.md", "personas/b.md", "notes/c.md"]);
    world.kinds.set_kinds(REPOSITORY, vec![persona()]);
    launched(&world.engine.launch(&launch("persona", "a", None)))?;
    let id = World::id("persona", "a");
    let held = world.held(&id).ok_or("the place holds no state")?;
    let sessions: Vec<(String, Option<String>, Option<String>)> = held
        .stages
        .iter()
        .map(|stage| {
            (
                stage.name.clone(),
                stage
                    .session
                    .as_ref()
                    .and_then(|session| session.file.clone()),
                stage
                    .start
                    .as_ref()
                    .filter(|start| start.live)
                    .map(|start| start.identity.stage.clone()),
            )
        })
        .collect();
    ensure_eq!(
        sessions,
        vec![
            (
                "stages.test#1/1".to_string(),
                Some("personas/a.md".to_string()),
                Some("stages.test#1/1".to_string())
            ),
            (
                "stages.test#1/2".to_string(),
                Some("personas/b.md".to_string()),
                Some("stages.test#1/2".to_string())
            ),
        ],
        "the sessions, their files and their starts' identities"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    ensure_eq!(
        held.declared(1).deliverables.clone(),
        vec![Deliverable::CommittedReport {
            path: "results/personas/b.md".into()
        }],
        "the second session's deliverable"
    );
    let identity = World::identity("persona", "a", "stages.test#1/2", 1);
    let prompt = world
        .prompt(&id, &identity)
        .ok_or("the second session was given no prompt")?;
    ensure!(
        prompt.contains("Test personas/b.md as session 2."),
        "the second session's prompt fills not its file and number: {prompt}"
    );
    Ok(())
}

fn a_stage_entered_again(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![build_test(BUILD_TEST, 2, false)]);
    let id = to_test(&world, 1)?;
    stops_short(
        &world,
        ("bt", "a"),
        "main.test#1/1",
        "Again.",
        StopKind::FoundWork,
    )?;
    // Sent back, build entered again and its second entry started.
    settles(&world, &id, 3);
    to_test(&world, 2)?;
    let held = world.held(&id).ok_or("the place holds no state")?;
    ensure_eq!(
        held.stages
            .iter()
            .map(|stage| stage.name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "main.build#1/1",
            "main.test#1/1",
            "main.build#2/1",
            "main.test#2/1"
        ],
        "the sessions of both entries"
    );
    ensure_eq!(
        held.chart
            .as_ref()
            .and_then(|chart| chart.stages.get("main.test"))
            .map(|stage| stage.entry),
        Some(2),
        "the test stage's entry recorded"
    );
    ensure_eq!(
        session(&world, &id, "main.test#1/1"),
        Some((StageState::EndedShort, false)),
        "entry 1's session, as it ended"
    );
    ensure_eq!(
        session(&world, &id, "main.build#1/1"),
        Some((StageState::Delivered, false)),
        "build's entry 1's session, as it ended"
    );
    Ok(())
}

/// The statechart's state of each region of the place, by its path.
pub fn regions(world: &World, id: &PlaceId) -> BTreeMap<String, String> {
    world
        .held(id)
        .and_then(|held| held.chart)
        .map(|chart| chart.state.regions)
        .unwrap_or_default()
}

/// The state the region at `path` holds, or none.
fn at(world: &World, id: &PlaceId, path: &str) -> Option<String> {
    regions(world, id).get(path).cloned()
}

/// Settles the place `n` times.
pub fn settles(world: &World, id: &PlaceId, n: usize) {
    for _ in 0..n {
        world.settle(id);
    }
}

/// The session `session` of the place `<kind>--<name>` stops short with
/// `words`, as `kind` says (41, 131).
pub fn stops_short(
    world: &World,
    (kind_name, name): (&str, &str),
    session: &str,
    words: &str,
    kind: StopKind,
) -> Result<(), String> {
    let id = World::id(kind_name, name);
    let held = world.held(&id).ok_or("the place holds no state")?;
    let start = held
        .stages
        .iter()
        .find(|stage| stage.name == session)
        .and_then(|stage| stage.start.clone())
        .ok_or(format!("the session {session} has no start"))?;
    let answer = world
        .engine
        .needs(&start.identity.to_string(), &start.secret.0, words, kind);
    match answer.refusal() {
        Some(refusal) => Err(format!(
            "the stop-short of {session} was refused: {refusal}"
        )),
        None => Ok(()),
    }
}

/// The session `name`'s state and whether its start is live.
pub fn session(world: &World, id: &PlaceId, name: &str) -> Option<(StageState, bool)> {
    world.held(id)?.stages.into_iter().find_map(|stage| {
        (stage.name == name).then(|| {
            let live = stage.start.as_ref().is_some_and(|start| start.live);
            (stage.state, live)
        })
    })
}

/// One region `main` whose stage `work` ends passed, sent back or stopped,
/// each taken to a final state of its own.
const ENDS: &str = r#"[kinds.ends]
title = "A stage's ends"

[kinds.ends.regions.main]
initial = "work"

[kinds.ends.regions.main.states.work]
machine = "stage"
transitions = [
  { when = { final = "passed" }, to = "done" },
  { when = { final = "sent-back" }, to = "back" },
  { when = { final = "stopped" }, to = "halt" },
]

[kinds.ends.regions.main.states.done]
final = true

[kinds.ends.regions.main.states.back]
final = true

[kinds.ends.regions.main.states.halt]
final = true
"#;

fn the_templates_ends(_: &()) -> Outcome {
    for (name, end, after) in [
        ("p", "passed", "done"),
        ("s", "sent-back", "back"),
        ("t", "stopped", "halt"),
    ] {
        let world = World::new();
        let mut stage = chart_stage(listed(1), Deliverable::Commits, "Work.");
        if end == "stopped" {
            stage.send_back_limit = Some(0);
        }
        world.kinds.set_kinds(
            REPOSITORY,
            vec![chart_kind("ends", ENDS, vec![("main.work", stage)])],
        );
        launched(&world.engine.launch(&launch("ends", name, None)))?;
        let id = World::id("ends", name);
        world.settle(&id);
        ensure_eq!(
            session(&world, &id, "main.work#1/1").map(|(state, _)| state),
            Some(StageState::Begun),
            format!("{name}'s session")
        );
        if end == "passed" {
            world.commit(&format!("ends/{name}"), &["work.rs"]);
        } else {
            stops_short(
                &world,
                ("ends", name),
                "main.work#1/1",
                "The parser drops a comment.",
                StopKind::FoundWork,
            )?;
        }
        world.settle(&id);
        ensure_eq!(
            (at(&world, &id, "main.work.run"), at(&world, &id, "main")),
            (Some(end.to_string()), Some("work".to_string())),
            format!("{name}'s regions at the settle its stage ended")
        );
        world.settle(&id);
        ensure_eq!(
            at(&world, &id, "main"),
            Some(after.to_string()),
            format!("{name}'s region at the next settle")
        );
    }
    Ok(())
}

/// One region `main` whose stage `work` runs three sessions, each
/// committing its own file, joined as `join` (175).
fn three_sessions(join: Join) -> Kind {
    let mut stage = chart_stage(
        listed(3),
        Deliverable::CommittedReport {
            path: "out/{{session}}.md".into(),
        },
        "Work.",
    );
    stage.join = join;
    chart_kind(
        "one",
        &one_stage(false, "stage", ON_PASSED),
        vec![("main.work", stage)],
    )
}

fn a_join_of_all_of_any_and_of_a_count(_: &()) -> Outcome {
    for (join, passes_at) in [
        (Join::All, None),
        (Join::Any, Some(1)),
        (Join::Count(2), Some(2)),
    ] {
        let world = World::new();
        world
            .kinds
            .set_kinds(REPOSITORY, vec![three_sessions(join)]);
        launched(&world.engine.launch(&launch("one", "a", None)))?;
        let id = World::id("one", "a");
        world.settle(&id);
        for n in 1..=2 {
            world.commit("one/a", &[&format!("out/{n}.md")]);
            world.settle(&id);
            let passed = passes_at.is_some_and(|at| n >= at);
            ensure_eq!(
                at(&world, &id, "main.work.run"),
                Some(if passed { "passed" } else { "working" }.to_string()),
                format!("the stage under {join} after {n} deliveries")
            );
            if passes_at == Some(n) {
                ensure_eq!(
                    session(&world, &id, "main.work#1/3"),
                    Some((StageState::Stopped, false)),
                    format!("the working session under {join}")
                );
            }
        }
        if passes_at.is_none() {
            ensure_eq!(
                session(&world, &id, "main.work#1/3"),
                Some((StageState::Begun, true)),
                format!("the working session under {join}")
            );
        }
    }
    Ok(())
}

/// One region `main`: `build`, then `test`, which a session's found work
/// sends back to `build` up to its bound, stopping past it.
const BUILD_TEST: &str = r#"[kinds.bt]
title = "Build and test"

[kinds.bt.regions.main]
initial = "build"

[kinds.bt.regions.main.states.build]
machine = "stage"
transitions = [{ when = { final = "passed" }, to = "test" }]

[kinds.bt.regions.main.states.test]
machine = "stage"
transitions = [
  { when = { final = "passed" }, to = "passed" },
  { when = { final = "sent-back" }, to = "build" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.bt.regions.main.states.passed]
final = true

[kinds.bt.regions.main.states.stopped]
final = true
"#;

/// `bt`, its test stage sent back up to `bound` times (112, 175).
pub fn build_test(text: &str, bound: u32, lands: bool) -> Kind {
    let build = chart_stage(
        listed(1),
        Deliverable::Commits,
        "Build.{{#words}} {{words}}{{/words}}",
    );
    let mut test = chart_stage(
        listed(1),
        Deliverable::CommittedReport {
            path: "report.md".into(),
        },
        "Test.",
    );
    test.on_fail = Some("build".into());
    test.send_back_limit = Some(bound);
    let mut kind = chart_kind("bt", text, vec![("main.build", build), ("main.test", test)]);
    kind.lands = lands;
    kind
}

/// `bt`'s place `a` run from its launch to its test stage's sessions
/// started at `entry`, build delivering at each entry before it.
pub fn to_test(world: &World, entry: u32) -> Result<PlaceId, String> {
    let id = World::id("bt", "a");
    if world.held(&id).is_none() {
        launched(&world.engine.launch(&launch("bt", "a", None)))?;
    }
    world.settle(&id);
    world.commit("bt/a", &[&format!("build-{entry}.rs")]);
    // The build stage's end, the move to test, and test's sessions started
    // and begun: a stage boundary takes three settles (176).
    settles(world, &id, 4);
    let test = format!("main.test#{entry}/1");
    ensure_eq!(
        session(world, &id, &test),
        Some((StageState::Begun, true)),
        format!("the session {test}")
    );
    Ok(id)
}

fn found_work_sends_the_stage_back_with_its_words(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![build_test(BUILD_TEST, 2, false)]);
    let id = to_test(&world, 1)?;
    let findings = "The parser drops a trailing comment.";
    stops_short(
        &world,
        ("bt", "a"),
        "main.test#1/1",
        findings,
        StopKind::FoundWork,
    )?;
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main.test.run"),
        Some("sent-back".to_string()),
        "the test stage's end"
    );
    let held = world.held(&id).ok_or("the place holds no state")?;
    let chart = held.chart.ok_or("the place holds no statechart")?;
    ensure_eq!(
        chart.stages.get("main.test").map(|stage| stage.send_backs),
        Some(1),
        "the test stage's send-backs"
    );
    ensure_eq!(
        session(&world, &id, "main.test#1/1"),
        Some((StageState::EndedShort, false)),
        "the finding session"
    );
    settles(&world, &id, 2);
    let identity = World::identity("bt", "a", "main.build#2/1", 1);
    let prompt = world
        .prompt(&id, &identity)
        .ok_or("build's second entry was given no prompt")?;
    ensure!(
        prompt.contains(findings),
        "build's second entry's prompt holds not the found words: {prompt}"
    );
    ensure_eq!(
        world
            .held(&id)
            .and_then(|held| held.chart)
            .and_then(|chart| chart.words),
        None,
        "the words held once given"
    );
    Ok(())
}

fn past_its_bound(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![build_test(BUILD_TEST, 1, false)]);
    let id = to_test(&world, 1)?;
    stops_short(
        &world,
        ("bt", "a"),
        "main.test#1/1",
        "Once.",
        StopKind::FoundWork,
    )?;
    // Sent back, build entered again and its second entry started.
    settles(&world, &id, 3);
    to_test(&world, 2)?;
    stops_short(
        &world,
        ("bt", "a"),
        "main.test#2/1",
        "Twice.",
        StopKind::FoundWork,
    )?;
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main.test.run"),
        Some("stopped".to_string()),
        "the test stage's end past its bound"
    );
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main"),
        Some("stopped".to_string()),
        "the region"
    );
    Ok(())
}

fn a_rule_that_matches_no_file(_: &()) -> Outcome {
    let world = World::new();
    world.commit("main", &["notes/c.md"]);
    world.kinds.set_kinds(REPOSITORY, vec![persona()]);
    launched(&world.engine.launch(&launch("persona", "a", None)))?;
    let id = World::id("persona", "a");
    ensure!(
        world.held(&id).is_some_and(|held| held.stages.is_empty()),
        "a session was recorded"
    );
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "stages.test.run"),
        Some("stopped".to_string()),
        "the stage's end"
    );
    let status = world.status(&id)?;
    ensure!(
        status
            .failures
            .iter()
            .any(|failure| failure.contains("every-file") && failure.contains("personas/*.md")),
        "status names not the rule and its pattern: {:?}",
        status.failures
    );
    Ok(())
}
