//! Command stages (4, 49, 51, 56, 223, 224, C.2, C.5, S130): a statechart
//! kind's stage that runs one command its kind declares in place of an
//! agent, filled from the launch's parameters, through the terminal as a
//! program of a pane, never an agent; its launcher's recorded exit and the
//! place's branch end it, delivered or failed; a failure taken on to an
//! agent's stage with its words; a retry runs it again and refuses words.

use super::statecharts::settles;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "command stage: an update kind's command stage starts at once on a host whose bound is spent, through the terminal as a program and no agent, its `{{params}}` the place's copy of the launch's parameters; its command exiting with success and a commit delivers it, and the place is submitted",
            cites: &["223", "224", "S130"],
            run: a_command_stage_delivers,
        },
        Scenario {
            name: "command stage: a command that exits with an error fails its stage, status gives its exit and where its output lies, and the kind's transition starts the agent's stage with the brief and the failure as its words",
            cites: &["56", "223", "S130"],
            run: a_failure_reaches_the_agents_stage,
        },
        Scenario {
            name: "command stage: a command that exits with success and commits nothing fails its stage",
            cites: &["25", "223"],
            run: nothing_committed_fails,
        },
        Scenario {
            name: "command stage: a retry naming a command stage runs its command again from the place's head, and one with words is refused naming the stage",
            cites: &["49", "223"],
            run: a_retry_runs_it_again,
        },
        Scenario {
            name: "command stage: a launch giving no parameters while a command stage's command asks for them is refused naming the stage, nothing made",
            cites: &["4", "224"],
            run: no_parameters_refused,
        },
        Scenario {
            name: "command stage: a kind sending a place back to a command stage, or a command naming a program outside the repository, is refused naming the stage, nothing made",
            cites: &["4", "223"],
            run: kinds_refused,
        },
    ]
}

/// update@1: its first stage `move` a command stage running the release's
/// move, failing on to its agent's stage `fix`, both ending at `done`.
const UPDATE: &str = r#"[kinds.update]
title = "Take a new version of a dependency"
lands = true

[kinds.update.regions.main]
initial = "move"

[kinds.update.regions.main.states.move]
machine = "command"
transitions = [
  { when = { final = "passed" }, to = "done" },
  { when = { final = "failed" }, to = "fix" },
]

[kinds.update.regions.main.states.fix]
machine = "stage"
transitions = [
  { when = { final = "passed" }, to = "done" },
  { when = { final = "stopped" }, to = "stuck" },
]

[kinds.update.regions.main.states.done]
final = true

[kinds.update.regions.main.states.stuck]
final = true
"#;

/// The parameters a release's launch carries (224).
const PARAMS: &str = "{\"moves\":[{\"name\":\"weir\",\"from\":\"0.4.1\",\"to\":\"0.4.2\"}]}";

/// The move stage's parameters: `bin/move {{params}}`, failing on to
/// `fix` three times at most (223, C.2).
fn move_stage(command: &[&str]) -> ChartStage {
    ChartStage {
        command: Some(command.iter().map(|word| (*word).to_string()).collect()),
        agents: ChartAgents::Entries(vec![]),
        join: Join::All,
        deliverables: vec![Deliverable::Commits],
        prompt: None,
        on_fail: Some("fix".into()),
        send_back_limit: Some(3),
    }
}

/// The kind update@1 with the move stage `command` (223, S130).
fn update(command: &[&str]) -> Kind {
    let fix = chart_stage(
        listed(1),
        Deliverable::Commits,
        "Mend the move: {{brief}}{{#words}} {{words}}{{/words}}",
    );
    chart_kind(
        "update",
        UPDATE,
        vec![("main.move", move_stage(command)), ("main.fix", fix)],
    )
}

/// A launch of update@1 named `name` with the release's notes as its
/// brief and `params` (224, S130).
fn update_launch(name: &str, params: Option<&str>) -> Launch {
    Launch {
        params: params.map(str::to_string),
        ..launch("update", name, Some("weir 0.4.2's notes"))
    }
}

/// The run of the move's session `n` under its launcher: its word that
/// nothing differed, a commit where `commits`, and its exit `exit`
/// recorded beside the start's record (223, C.2).
fn ran(world: &World, id: &PlaceId, n: u32, exit: i32, commits: bool) -> Result<Identity, String> {
    let held = world.held(id).ok_or("the place holds no state")?;
    let session = format!("main.move#{n}/1");
    let start = held
        .stages
        .iter()
        .find(|stage| stage.name == session)
        .and_then(|stage| stage.start.clone())
        .ok_or(format!("the session {session} has no start"))?;
    let folder = world.store.folder(id);
    let key = start.identity.to_string();
    world
        .store
        .write_file(
            id,
            &launched_word(&folder, &key),
            "{\"pid\":41,\"program\":\"/r/bin/move\"}",
        )
        .map_err(|why| format!("the launcher's word: {why}"))?;
    world.settle(id);
    if commits {
        world.commit("update/a", &["flake.lock"]);
    }
    world
        .store
        .write_file(
            id,
            &ran_exit(&folder, &key),
            &format!("{{\"exit\":{exit}}}"),
        )
        .map_err(|why| format!("the exit: {why}"))?;
    world
        .terminal
        .set_state(&start.identity, AgentState::Stopped);
    world.settle(id);
    Ok(start.identity)
}

fn a_command_stage_delivers(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(0));
    world.queue.plan_submits(vec![Plan::Hold]);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![update(&["bin/move", "{{params}}"])]);
    launched(&world.engine.launch(&update_launch("a", Some(PARAMS))))?;
    let id = World::id("update", "a");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    ensure_eq!(world.terminal.log.count("run"), 1, "commands run");
    let folder = world.store.folder(&id);
    let kept = world
        .store
        .read_file(&id, &params_path(&folder))
        .map_err(|why| format!("the parameters: {why}"))?;
    ensure_eq!(
        kept.as_deref(),
        Some(PARAMS),
        "the place's copy of the parameters"
    );
    let identity = World::identity("update", "a", "main.move#1/1", 1);
    let (command, _) = world
        .terminal
        .opened(&identity)
        .ok_or("the terminal ran no command under the move's identity")?;
    ensure!(
        command.program.ends_with("bin/move"),
        "the program run: {}",
        command.program
    );
    ensure_eq!(
        command.args,
        vec![params_path(&folder)],
        "the command's arguments"
    );
    ensure!(
        !command.launcher.is_empty(),
        "the command was run with no launcher"
    );
    ensure!(
        command.kind.as_deref() == Some("update"),
        "the link's name: {:?}",
        command.kind
    );
    let held = world.held(&id).ok_or("the place holds no state")?;
    let start = held.stages[0]
        .start
        .clone()
        .ok_or("the move has no start")?;
    ensure!(start.account.is_none(), "the run took an account");
    ensure!(
        start.start_record.command,
        "the start's record is no command's"
    );
    ran(&world, &id, 1, 0, true)?;
    ensure_eq!(
        world.stage_state(&id, "main.move#1/1"),
        Some(StageState::Delivered),
        "the move's state"
    );
    settles(&world, &id, 3);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn a_failure_reaches_the_agents_stage(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![update(&["bin/move", "{{params}}"])]);
    launched(&world.engine.launch(&update_launch("a", Some(PARAMS))))?;
    let id = World::id("update", "a");
    let identity = ran(&world, &id, 1, 2, false)?;
    ensure_eq!(
        world.stage_state(&id, "main.move#1/1"),
        Some(StageState::Failed),
        "the move's state"
    );
    let status = world.status(&id)?;
    let end = status
        .stages
        .iter()
        .find(|stage| stage.name == "main.move#1/1")
        .and_then(|stage| stage.end.clone())
        .ok_or("status gives the move no end")?;
    let output = ran_output(&world.store.folder(&id), &identity.to_string());
    ensure_eq!(
        end.ran.clone(),
        Some(Ran {
            exit: Some(2),
            output: output.clone()
        }),
        "the move's exit and output in status"
    );
    let words = end.words.unwrap_or_default();
    ensure!(
        words.contains("exited with 2") && words.contains(&output),
        "the failure's words: {words}"
    );
    settles(&world, &id, 3);
    let fix = World::identity("update", "a", "main.fix#1/1", 1);
    let prompt = world
        .prompt(&id, &fix)
        .ok_or("the fix stage's agent was given no prompt")?;
    ensure!(
        prompt.contains("weir 0.4.2's notes") && prompt.contains("exited with 2"),
        "the fix's prompt holds not the brief and the failure: {prompt}"
    );
    Ok(())
}

fn nothing_committed_fails(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![update(&["bin/move", "{{params}}"])]);
    launched(&world.engine.launch(&update_launch("a", Some(PARAMS))))?;
    let id = World::id("update", "a");
    ran(&world, &id, 1, 0, false)?;
    ensure_eq!(
        world.stage_state(&id, "main.move#1/1"),
        Some(StageState::Failed),
        "the move's state"
    );
    Ok(())
}

fn a_retry_runs_it_again(_: &()) -> Outcome {
    let world = World::new();
    let mut kind = update(&["bin/move", "{{params}}"]);
    // A failure past the move's bound leaves the place on its caller.
    if let Some(chart) = kind.statechart.as_mut() {
        if let Some(stage) = chart.stages.get_mut("main.move") {
            stage.send_back_limit = Some(0);
        }
    }
    world.kinds.set_kinds(REPOSITORY, vec![kind]);
    launched(&world.engine.launch(&update_launch("a", Some(PARAMS))))?;
    let id = World::id("update", "a");
    ran(&world, &id, 1, 1, false)?;
    let refusal = refused(&world.engine.retry(
        &id,
        Retry::Stage("main.move".into()),
        Some("try harder".into()),
        false,
    ))?;
    ensure!(
        refusal.contains("main.move#1/1") && refusal.contains("command stage"),
        "the refusal: {refusal}"
    );
    ensure_eq!(world.terminal.log.count("run"), 1, "commands run");
    let answer = world
        .engine
        .retry(&id, Retry::Stage("main.move".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure_eq!(world.terminal.log.count("run"), 2, "commands run");
    ensure_eq!(
        world.stage_state(&id, "main.move#1/1"),
        Some(StageState::Starting),
        "the move's state once retried"
    );
    let held = world.held(&id).ok_or("the place holds no state")?;
    let start = held.stages[0].start.clone().ok_or("no start")?;
    ensure_eq!(start.n, 2, "the move's start");
    Ok(())
}

fn no_parameters_refused(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![update(&["bin/move", "{{params}}"])]);
    let reason = refused(&world.engine.launch(&update_launch("a", None)))?;
    ensure!(
        reason.contains("main.move") && reason.contains("parameters"),
        "the refusal: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(world.store.places().is_empty(), "held state was left");
    // A command that asks for none launches with none.
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![update(&["make-move"])]);
    launched(&world.engine.launch(&update_launch("a", None)))?;
    Ok(())
}

fn kinds_refused(_: &()) -> Outcome {
    let world = World::new();
    let mut back = update(&["bin/move"]);
    if let Some(chart) = back.statechart.as_mut() {
        if let Some(fix) = chart.stages.get_mut("main.fix") {
            fix.on_fail = Some("move".into());
        }
    }
    world.kinds.set_kinds(REPOSITORY, vec![back]);
    let reason = refused(&world.engine.launch(&update_launch("a", None)))?;
    ensure!(
        reason.contains("main.move") && reason.contains("command stage"),
        "the refusal of a send-back: {reason}"
    );
    for outside in ["/usr/local/bin/move", "../bin/move"] {
        let world = World::new();
        world.kinds.set_kinds(REPOSITORY, vec![update(&[outside])]);
        let reason = refused(&world.engine.launch(&update_launch("a", None)))?;
        ensure!(
            reason.contains("main.move") && reason.contains(outside),
            "the refusal of {outside}: {reason}"
        );
        ensure_eq!(world.place.log.count("make"), 0, "places made");
    }
    Ok(())
}
