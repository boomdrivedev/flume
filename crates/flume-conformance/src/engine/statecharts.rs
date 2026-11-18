//! Statechart kinds (4, 175 to 178, C.2, S76, S77): a launch of one taken
//! or refused for what flume does not bind or run, its stage template's
//! sessions, one tick a settle over evidence read at one head, a stage's
//! end, the run's end and its submission, and a decision answered once.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answering, Retry, Waits};
use flume_fakes::Plan;
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
            name: "statechart: a stage of three sessions on a host with room for two runs two agents in one place, the third waiting for room and starting once one of the two ends",
            cites: &["30", "70"],
            run: several_agents_in_one_place,
        },
        Scenario {
            name: "statechart: a session that stops short needing an answer leaves the place waiting on it, an answer naming its key reaches the session kept open, and the other session works on",
            cites: &["42", "198"],
            run: a_session_asks_and_is_answered,
        },
        Scenario {
            name: "statechart: a session whose agent goes with nothing delivered and nothing said leaves the place waiting on its caller, whose retry naming the session starts it again with one fresh agent",
            cites: &["45", "49"],
            run: a_lost_session_retried,
        },
        Scenario {
            name: "statechart: each session reads its own report, `{{file}}` filled with its file, delivered only where its own is committed",
            cites: &["24", "175"],
            run: each_personas_own_report,
        },
        Scenario {
            name: "statechart: a launch's hold naming a stage's path holds each of its sessions, the place waiting on a retry, and a retry naming the path starts them",
            cites: &["47", "130", "175"],
            run: a_hold_on_a_stages_path,
        },
        Scenario {
            name: "statechart: a kind whose build delivers and whose test passes, its top region reaching passed, is submitted through the merge queue once, as after a last stage's delivery",
            cites: &["111", "176", "S76"],
            run: persona_passes_and_is_submitted,
        },
        Scenario {
            name: "statechart: a kind that does not land, each top region final, waits on its caller, whose done makes its work over",
            cites: &["114", "130"],
            run: a_run_that_does_not_land,
        },
        Scenario {
            name: "statechart: a failed submission a final state's transition takes moves the run back to build at the next settle, build's sessions prompted with the merge queue's words",
            cites: &["112", "176"],
            run: a_failure_taken_back_to_a_stage,
        },
        Scenario {
            name: "statechart: a failed submission no transition takes leaves the place waiting on its caller with the merge queue's words, and its retry submits it again",
            cites: &["112", "130"],
            run: a_failure_nothing_takes,
        },
        Scenario {
            name: "statechart: a region at a stopped stage's decision leaves the place waiting on its caller from its standing, and the caller's answer naming the region is applied at the next settle, moving the region to the stage it names",
            cites: &["130", "176"],
            run: a_stopped_stages_decision_answered,
        },
        Scenario {
            name: "statechart: an answer naming a region that carries no decision is refused naming it, and a second answer to a decision standing before a settle is refused naming the answer taken, who gave it and when",
            cites: &["176", "core.54"],
            run: an_answer_refused,
        },
        Scenario {
            name: "statechart: a place whose readiness failed holds its first stage's sessions waiting on readiness, the place waiting on a retry naming one, which runs readiness again and, once it passes, starts every session",
            cites: &["12", "49", "175"],
            run: sessions_waiting_on_readiness,
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
/// delivering its own result, then final at `passed`, or, stopped, at a
/// decision answered `retry` or `drop`.
const PERSONAS: &str = r#"[kinds.persona]
title = "Persona test"

[kinds.persona.regions.stages]
initial = "test"

[kinds.persona.regions.stages.states.test]
machine = "stage"
transitions = [
  { when = { final = "passed" }, to = "passed" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.passed]
final = true

[kinds.persona.regions.stages.states.stopped]
decision = { kind = "stalled", answers = ["retry", "drop"] }
transitions = [
  { when = { response = "retry" }, to = "test" },
  { when = { response = "drop" }, to = "dropped" },
]

[kinds.persona.regions.stages.states.dropped]
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

/// `one`, its `main.work` a stage of `n` listed sessions, each committing
/// `out/<n>.md` (175).
fn listed_sessions(n: usize) -> Kind {
    let stage = chart_stage(
        listed(n),
        Deliverable::CommittedReport {
            path: "out/{{session}}.md".into(),
        },
        "Work.",
    );
    chart_kind(
        "one",
        &one_stage(false, "stage", ON_PASSED),
        vec![("main.work", stage)],
    )
}

fn several_agents_in_one_place(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    world.kinds.set_kinds(REPOSITORY, vec![listed_sessions(3)]);
    launched(&world.engine.launch(&launch("one", "a", None)))?;
    let id = World::id("one", "a");
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    ensure_eq!(
        session(&world, &id, "main.work#1/3"),
        Some((StageState::WaitingForRoom, false)),
        "the third session"
    );
    world.commit("one/a", &["out/1.md"]);
    world.settle(&id);
    ensure_eq!(
        session(&world, &id, "main.work#1/1"),
        Some((StageState::Delivered, false)),
        "the first session"
    );
    ensure_eq!(world.terminal.log.count("open"), 3, "agents opened");
    ensure!(
        session(&world, &id, "main.work#1/3").is_some_and(|(_, live)| live),
        "the third session did not start once room was made"
    );
    Ok(())
}

fn a_session_asks_and_is_answered(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(REPOSITORY, vec![listed_sessions(2)]);
    launched(&world.engine.launch(&launch("one", "a", None)))?;
    let id = World::id("one", "a");
    world.settle(&id);
    let question = "Which parser do I mend?";
    stops_short(
        &world,
        ("one", "a"),
        "main.work#1/1",
        question,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.words.as_deref()
        ),
        (
            Act::Retry,
            Some("main.work#1/1"),
            Waits::Answer,
            Some(question)
        ),
        "the move the place waits on"
    );
    ensure!(waiting.open_until.is_some(), "the session is not kept open");
    let key = EndKey::Stage {
        stage: "main.work#1/1".into(),
        start: 1,
        state: StageState::EndedShort,
        answered: 0,
    };
    let answer = world.engine.tell_answering(
        &id,
        "The fast one.",
        None,
        &Answering {
            key: Some(key),
            by: Some("ops".into()),
        },
    );
    ensure!(
        answer.refusal().is_none(),
        "the answer was refused: {answer:?}"
    );
    let first = World::identity("one", "a", "main.work#1/1", 1);
    ensure!(
        world
            .terminal
            .sent_to(&first)
            .iter()
            .any(|sent| sent.contains("The fast one.")),
        "the answer did not reach the session"
    );
    ensure_eq!(
        session(&world, &id, "main.work#1/1"),
        Some((StageState::Begun, true)),
        "the session answered"
    );
    ensure_eq!(
        session(&world, &id, "main.work#1/2"),
        Some((StageState::Begun, true)),
        "the other session"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_lost_session_retried(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(REPOSITORY, vec![listed_sessions(2)]);
    launched(&world.engine.launch(&launch("one", "a", None)))?;
    let id = World::id("one", "a");
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("one", "a", "main.work#1/1", 1));
    world.settle(&id);
    ensure_eq!(
        session(&world, &id, "main.work#1/1"),
        Some((StageState::Lost, false)),
        "the lost session"
    );
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("main.work#1/1"), Waits::Lost),
        "the move the place waits on"
    );
    ensure_eq!(
        at(&world, &id, "main.work.run"),
        Some("working".to_string()),
        "the stage"
    );
    let answer = world
        .engine
        .retry(&id, Retry::Stage("main.work#1/1".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    let held = world.held(&id).ok_or("the place holds no state")?;
    let retried = held
        .stages
        .iter()
        .find(|stage| stage.name == "main.work#1/1")
        .ok_or("the session is gone")?;
    ensure_eq!(retried.starts, 2, "the session's starts");
    ensure!(
        retried
            .start
            .as_ref()
            .is_some_and(|start| start.live && start.n == 2),
        "the session's second start is not live"
    );
    ensure_eq!(
        session(&world, &id, "main.work#1/2"),
        Some((StageState::Begun, true)),
        "the other session"
    );
    ensure_eq!(world.terminal.log.count("open"), 3, "agents opened");
    Ok(())
}

fn each_personas_own_report(_: &()) -> Outcome {
    let world = World::new();
    world.commit("main", &["personas/a.md", "personas/b.md"]);
    world.kinds.set_kinds(REPOSITORY, vec![persona()]);
    launched(&world.engine.launch(&launch("persona", "a", None)))?;
    let id = World::id("persona", "a");
    world.settle(&id);
    world.commit("persona/a", &["results/personas/a.md"]);
    world.settle(&id);
    ensure_eq!(
        session(&world, &id, "stages.test#1/1").map(|(state, _)| state),
        Some(StageState::Delivered),
        "the session for personas/a.md"
    );
    ensure_eq!(
        session(&world, &id, "stages.test#1/2"),
        Some((StageState::Begun, true)),
        "the session for personas/b.md"
    );
    Ok(())
}

fn a_hold_on_a_stages_path(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(REPOSITORY, vec![listed_sessions(2)]);
    let mut held = launch("one", "a", None);
    held.hold = vec!["main.work".into()];
    held.hold_words =
        BTreeMap::from([("main.work".to_string(), "Wait for the review.".to_string())]);
    launched(&world.engine.launch(&held))?;
    let id = World::id("one", "a");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    for n in 1..=2 {
        ensure_eq!(
            session(&world, &id, &format!("main.work#1/{n}")),
            Some((StageState::OnHold, false)),
            format!("session {n}")
        );
    }
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.why, waiting.words.as_deref()),
        (Act::Retry, Waits::OnHold, Some("Wait for the review.")),
        "the move the place waits on"
    );
    let mut refused_launch = launch("one", "b", None);
    refused_launch.hold = vec!["main.nowhere".into()];
    let reason = refused(&world.engine.launch(&refused_launch))?;
    ensure!(
        reason.contains("main.nowhere") && reason.contains("main.work"),
        "the refusal names not the hold and the stages: {reason}"
    );
    let answer = world
        .engine
        .retry(&id, Retry::Stage("main.work".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

/// `bt` landing, its final state `passed` taking a failed submission back
/// to `build` where `back` (111, 112, 176).
fn landing(back: bool) -> Kind {
    let mut text = BUILD_TEST.replace(
        "title = \"Build and test\"\n",
        "title = \"Build and test\"\nlands = true\n",
    );
    if back {
        text = text.replace(
            "[kinds.bt.regions.main.states.passed]\nfinal = true\n",
            "[kinds.bt.regions.main.states.passed]\nfinal = true\ntransitions = [{ when = { ev = \"place.submission\", is = \"failed\" }, to = \"build\" }]\n",
        );
    }
    build_test(&text, 2, true)
}

/// `bt`'s place `a` run to its test stage's delivery and the run's end at
/// `passed`, its submission made in that settle.
fn to_passed(world: &World) -> Result<PlaceId, String> {
    let id = to_test(world, 1)?;
    world.commit("bt/a", &["report.md"]);
    world.settle(&id);
    ensure_eq!(
        at(world, &id, "main.test.run"),
        Some("passed".to_string()),
        "the test stage's end"
    );
    world.settle(&id);
    Ok(id)
}

fn persona_passes_and_is_submitted(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    world.kinds.set_kinds(REPOSITORY, vec![landing(false)]);
    let id = to_passed(&world)?;
    ensure_eq!(
        at(&world, &id, "main"),
        Some("passed".to_string()),
        "the top region"
    );
    ensure!(
        world
            .held(&id)
            .and_then(|held| held.chart)
            .is_some_and(|chart| chart.over),
        "the run is not over"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    settles(&world, &id, 2);
    ensure_eq!(
        world.queue.log.count("submit"),
        1,
        "submissions after two more settles"
    );
    Ok(())
}

fn a_run_that_does_not_land(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![one(&one_stage(false, "stage", ON_PASSED), listed(1))],
    );
    launched(&world.engine.launch(&launch("one", "a", Some("the work"))))?;
    let id = World::id("one", "a");
    world.settle(&id);
    world.commit("one/a", &["work.rs"]);
    settles(&world, &id, 2);
    ensure_eq!(
        at(&world, &id, "main"),
        Some("done".to_string()),
        "the region"
    );
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why),
        (Act::Done, None, Waits::Delivered),
        "the move the place waits on"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions");
    let answer = world.engine.done(&id, false);
    ensure!(
        answer.refusal().is_none(),
        "the done was refused: {answer:?}"
    );
    ensure!(
        world.held(&id).is_none_or(|held| matches!(
            held.phase,
            Phase::Over {
                over: Over::Done,
                ..
            } | Phase::Gone { .. }
                | Phase::Tidied { .. }
        )),
        "the place's work is not over"
    );
    Ok(())
}

fn a_failure_taken_back_to_a_stage(_: &()) -> Outcome {
    let world = World::new();
    let words = "check test-x failed";
    world
        .queue
        .plan_submits(vec![Plan::Fail(words.into()), Plan::Hold]);
    world.kinds.set_kinds(REPOSITORY, vec![landing(true)]);
    let id = to_passed(&world)?;
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main"),
        Some("build".to_string()),
        "the region the failure took"
    );
    world.settle(&id);
    let identity = World::identity("bt", "a", "main.build#2/1", 1);
    let prompt = world
        .prompt(&id, &identity)
        .ok_or("build's second entry was given no prompt")?;
    ensure!(
        prompt.contains(words),
        "build's second entry's prompt holds not the merge queue's words: {prompt}"
    );
    ensure!(
        world.status(&id)?.waiting_on.is_none(),
        "the place waits on its caller"
    );
    Ok(())
}

fn a_failure_nothing_takes(_: &()) -> Outcome {
    let world = World::new();
    let words = "check test-x failed";
    world
        .queue
        .plan_submits(vec![Plan::Fail(words.into()), Plan::Hold]);
    world.kinds.set_kinds(REPOSITORY, vec![landing(false)]);
    let id = to_passed(&world)?;
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main"),
        Some("passed".to_string()),
        "the region"
    );
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why),
        (Act::Retry, None, Waits::Submission),
        "the move the place waits on"
    );
    ensure!(
        waiting
            .words
            .as_deref()
            .is_some_and(|said| said.contains(words)),
        "the wait gives not the merge queue's words: {:?}",
        waiting.words
    );
    let answer = world.engine.retry(&id, Retry::Submission, None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    Ok(())
}

/// One region `main` whose stage `work`, stopped, stands at a decision
/// answered `retry`, back to `work`, or `drop` (130, 176).
const DECIDE: &str = r#"[kinds.decide]
title = "A stopped stage's decision"

[kinds.decide.regions.main]
initial = "work"

[kinds.decide.regions.main.states.work]
machine = "stage"
transitions = [
  { when = { final = "passed" }, to = "done" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.decide.regions.main.states.stopped]
decision = { kind = "stalled", answers = ["retry", "drop"] }
transitions = [
  { when = { response = "retry" }, to = "work" },
  { when = { response = "drop" }, to = "dropped" },
]

[kinds.decide.regions.main.states.done]
final = true

[kinds.decide.regions.main.states.dropped]
final = true
"#;

/// `decide`, its one stage stopped by the first work it finds (175).
pub fn decide() -> Kind {
    let mut stage = chart_stage(listed(1), Deliverable::Commits, "Work.");
    stage.send_back_limit = Some(0);
    chart_kind("decide", DECIDE, vec![("main.work", stage)])
}

/// `decide`'s place `a` launched and its stage stopped, its region at the
/// decision.
pub fn to_decision(world: &World) -> Result<PlaceId, String> {
    world.kinds.set_kinds(REPOSITORY, vec![decide()]);
    let id = World::id("decide", "a");
    if world.held(&id).is_none() {
        launched(&world.engine.launch(&launch("decide", "a", None)))?;
    }
    world.settle(&id);
    stops_short(
        world,
        ("decide", "a"),
        "main.work#1/1",
        "Stuck.",
        StopKind::FoundWork,
    )?;
    settles(world, &id, 2);
    ensure_eq!(
        at(world, &id, "main"),
        Some("stopped".to_string()),
        "the region"
    );
    Ok(id)
}

fn a_stopped_stages_decision_answered(_: &()) -> Outcome {
    let world = World::new();
    let id = to_decision(&world)?;
    let held = world.held(&id).ok_or("the place holds no state")?;
    let standing = held
        .chart
        .as_ref()
        .and_then(|chart| chart.state.entered_at.get("main").copied());
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.since
        ),
        (Act::Decide, Some("main"), Waits::Decision, standing),
        "the move the place waits on, and since when"
    );
    ensure!(
        waiting
            .words
            .as_deref()
            .is_some_and(|words| words.contains("stalled") && words.contains("retry")),
        "the wait names not the decision and its answers: {:?}",
        waiting.words
    );
    let answer = world.engine.answer_decision(&id, "main", "retry", "ops");
    ensure!(
        answer.refusal().is_none(),
        "the answer was refused: {answer:?}"
    );
    world.settle(&id);
    ensure_eq!(
        at(&world, &id, "main"),
        Some("work".to_string()),
        "the region"
    );
    let chart = world
        .held(&id)
        .and_then(|held| held.chart)
        .ok_or("the place holds no statechart")?;
    ensure_eq!(
        chart
            .answers
            .iter()
            .map(|answer| answer.id.as_str())
            .collect::<Vec<_>>(),
        vec!["main#1"],
        "the answers given"
    );
    ensure!(
        chart.state.applied.contains(&"main#1".to_string()),
        "the answer was not applied"
    );
    ensure!(chart.decisions.is_empty(), "a decision still stands");
    world.settle(&id);
    ensure!(
        session(&world, &id, "main.work#2/1").is_some_and(|(_, live)| live),
        "the stage the answer named did not start again"
    );
    Ok(())
}

fn an_answer_refused(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    world.kinds.set_kinds(REPOSITORY, vec![decide()]);
    launched(&world.engine.launch(&launch("decide", "a", None)))?;
    let id = World::id("decide", "a");
    let reason = refused(&world.engine.answer_decision(&id, "main", "retry", "ops"))?;
    ensure!(
        reason.contains("main") && reason.contains("no decision"),
        "the refusal names not the region: {reason}"
    );
    to_decision(&world)?;
    let at = world.host.now();
    let answer = world.engine.answer_decision(&id, "main", "retry", "ops");
    ensure!(
        answer.refusal().is_none(),
        "the answer was refused: {answer:?}"
    );
    let reason = refused(
        &world
            .engine
            .answer_decision(&id, "main", "drop", "east/ops"),
    )?;
    ensure!(
        reason.contains("retry") && reason.contains("ops") && reason.contains(&at.to_string()),
        "the refusal names not the answer taken, who gave it and when: {reason}"
    );
    ensure_eq!(
        world
            .held(&id)
            .and_then(|held| held.chart)
            .map(|chart| chart.answers.len()),
        Some(1),
        "the answers given"
    );
    Ok(())
}

fn sessions_waiting_on_readiness(_: &()) -> Outcome {
    let world = World::new();
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", false, "offline")]);
    world.kinds.set_kinds(REPOSITORY, vec![listed_sessions(2)]);
    launched(&world.engine.launch(&launch("one", "a", None)))?;
    let id = World::id("one", "a");
    for n in 1..=2 {
        ensure_eq!(
            session(&world, &id, &format!("main.work#1/{n}")),
            Some((StageState::WaitingOnReadiness, false)),
            format!("session {n}")
        );
    }
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("main.work#1/1"), Waits::Unready),
        "the move the place waits on"
    );
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", true, "installed")]);
    let answer = world
        .engine
        .retry(&id, Retry::Stage("main.work#1/1".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    for n in 1..=2 {
        ensure!(
            session(&world, &id, &format!("main.work#1/{n}")).is_some_and(|(_, live)| live),
            "session {n} did not start once readiness passed"
        );
    }
    Ok(())
}
