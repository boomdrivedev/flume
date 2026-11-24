//! Settling (40, 46, 111 to 113, 119 to 122, 124, 125, 133, 208): what
//! moves a place on with no process of flume's own, a caller's watch over
//! the host among it, and a landing kind's submission, made with no wait
//! and its outcomes collected at the settles that follow.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::Plan;
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name:
                "settle: a commit and a quiet agent end the stage and start the next in one settle",
            cites: &["S29", "40", "46", "121", "30"],
            run: s29_a_commit_and_a_quiet_agent,
        },
        Scenario {
            name: "settle: a change sent back by a failure its merge queue hands again at every inbox, its confirm not taken, has its code begun in the settle that sent it back and ended in the settle that reads it delivered and quiet, the failure acted on once",
            cites: &["40", "46", "112", "133", "74"],
            run: an_outcome_handed_again_holds_no_stage,
        },
        Scenario {
            name: "settle: a watch that hears its merge queue follows the repository of each place whose submission is in flight, none before the submit and none once the place landed",
            cites: &["124", "208", "225"],
            run: followed_while_in_flight,
        },
        Scenario {
            name: "settle: a delivered stage whose next waits on its caller starts nothing more",
            cites: &["S7", "47", "70", "17", "I6"],
            run: s7_two_held_places,
        },
        Scenario {
            name: "settle: with no caller keeping watch, a killed agent is read only when a command acts, which settles the places where something is pending first, the killed stage's among them",
            cites: &["S27", "119", "I8", "57", "225"],
            run: s27_killed_with_nobody_watching,
        },
        Scenario {
            name: "settle: under a caller's watch, a crashed agent whose on-quiet never runs reads lost at the caller's next settle, within its bound, nothing settling it before",
            cites: &["S28", "46", "120", "208"],
            run: s28_crashed_under_a_watch,
        },
        Scenario {
            name: "settle: under a caller's watch, the host's terminal one and a stream's another, a caller's act settles nothing first, and the stream's places move at the caller's settles",
            cites: &["119", "122", "190", "208", "S91"],
            run: a_watched_host_with_one_stream_unwatched,
        },
        Scenario {
            name: "settle: a caller's settle with a bound of two minutes holds the host's watch, when its settle began and ended and the command that ran it, flume unattended three minutes on and attended at five with no other settle between",
            cites: &["119", "208", "S118"],
            run: a_watch_kept_then_lapsed,
        },
        Scenario {
            name: "settle: a caller's settle with a bound of one minute that reads the host's places for five holds the watch throughout, flume unattended while it runs and a minute after it ended, and attended two minutes after",
            cites: &["119", "208"],
            run: a_settle_that_runs_past_its_bound,
        },
        Scenario {
            name: "settle: a watch whose settle another command began and never ended holds while that command lives, ten minutes on among it, and lapses once it died, twice its bound after the settle began",
            cites: &["119", "208", "76"],
            run: a_watch_whose_settle_died,
        },
        Scenario {
            name: "settle: no terminal keeps watch: with no caller's settle with a bound, nothing settles at an agent's change and a caller's drop settles the places where something is pending first",
            cites: &["119", "122", "208", "225"],
            run: no_terminal_keeps_watch,
        },
        Scenario {
            name: "settle: a watch's settle asks the merge queue, the terminal and the agent program nothing of a fix waiting on an answer whose agent ended, a chore whose submission its launch held or a fix whose code was read lost, reads the fix whose agent runs, and leaves the status of each waiting place as it was",
            cites: &["42", "45", "47", "225"],
            run: places_with_nothing_pending,
        },
        Scenario {
            name: "settle: with no caller keeping watch, a drop's settle asks the merge queue, the terminal and the agent program nothing of a fix waiting on an answer whose agent ended",
            cites: &["119", "225"],
            run: an_unwatched_drop,
        },
        Scenario {
            name: "settle: a fix waiting on an answer under no hold's words is read by no watch's settle four minutes on, and the watch's settle six minutes on records its stall and delivers it to the fix's recipients",
            cites: &["130", "225"],
            run: a_stalls_time_comes,
        },
        Scenario {
            name: "settle: a fix whose waiting move is held for ten minutes is read by no watch's settle nine minutes on, and the watch's settle eleven minutes on lapses the hold and records the fix stalled",
            cites: &["130", "225"],
            run: a_holds_bound_passes,
        },
        Scenario {
            name: "settle: S109, under a caller's watch a submitted chore with no agent is settled at each of the caller's settles, each stepping its base once, and once it lands the next delivers the landing and tidies it",
            cites: &["S109", "46", "124", "125", "208", "I8"],
            run: s109_a_submitted_place_with_no_agent_moves_under_a_watch,
        },
        Scenario {
            name: "settle: under a caller's watch, a chore submitted to a queue that moves on its own is stepped at each settle to no effect, and the settle after its landing tidies it",
            cites: &["124", "125", "208"],
            run: a_queue_that_moves_on_its_own,
        },
        Scenario {
            name: "settle: one settle of a caller's watch with a submission open on main and one open on a base place's branch landing on main steps each base once in the command",
            cites: &["124", "125", "46", "202", "208", "225"],
            run: two_depths_in_one_settle,
        },
        Scenario {
            name: "settle: S118, a host with no watch runs attended, its chore submitted at its agent's quiet and its base stepped only by an act settling the places where something is pending, the submitted chore's among them, until a caller keeps watch and lands it with no act, and four minutes after its last settle ended it runs attended again",
            cites: &["S118", "46", "119", "122", "124", "208", "225"],
            run: s118_a_host_with_no_watch_runs_attended,
        },
        Scenario {
            name: "settle: under a caller's watch, a launch and a retry each send the agent they opened its prompt, waiting on no receipt, which the watch's settles read",
            cites: &["6", "37", "38", "46", "49", "208"],
            run: opened_and_sent_under_a_watch,
        },
        Scenario {
            name: "settle: a settle repeated changes nothing",
            cites: &["74", "46"],
            run: a_settle_repeated,
        },
        Scenario {
            name: "settle: a place whose launching command died is undone",
            cites: &["2", "76"],
            run: a_launch_that_died,
        },
        Scenario {
            name: "settle: the agent's record is read in the place's own folder",
            cites: &["128", "77", "102"],
            run: the_record_read_in_the_places_folder,
        },
        Scenario {
            name: "submit: a fix lands, the merge queue removes nothing, and flume's tidy removes the place",
            cites: &["S1", "111", "113", "133", "66", "67", "62", "101"],
            run: s1_a_fix_lands,
        },
        Scenario {
            name: "submit: work landed inside another place's landing is not this place's",
            cites: &["S8", "63", "101"],
            run: s8_another_places_landing,
        },
        Scenario {
            name: "submit: a change's failed submission sends it back to code with the words, verify runs again, then it lands",
            cites: &["S19", "S35", "112", "131", "I7", "22", "48"],
            run: s19_a_failure_then_a_landing,
        },
        Scenario {
            name: "submit: a held submission's place landed by hand is over and never submitted",
            cites: &["S30", "63", "47", "111"],
            run: s30_a_landing_before_flume_submits,
        },
        Scenario {
            name: "submit: a held submission's chore landed by a hand is read by no settle of a caller's watch, and a settle naming no place and stating no bound reads its work over and tidies it, never submitting it",
            cites: &["S30", "63", "225"],
            run: a_held_submission_landed_by_a_hand,
        },
        Scenario {
            name: "submit: after a restart a landing is collected by the next settle, never submitted anew",
            cites: &["S34", "124", "133", "120", "77"],
            run: s34_a_restart_while_under_test,
        },
        Scenario {
            name: "submit: one settle of two places open on one base steps that base once",
            cites: &["125", "124", "111"],
            run: two_places_one_base,
        },
        Scenario {
            name: "submit: an inbox that cannot be read acts on nothing and is named in status",
            cites: &["133", "68", "57"],
            run: an_unreadable_inbox,
        },
        Scenario {
            name: "submit: a settle asks the merge queue's inbox only while the place's submission is open, never before flume submits it nor while its fix round runs, and once as flume submits it anew, an outcome pending from before deciding nothing of the new submission",
            cites: &["111", "133", "99", "101", "112", "124", "46"],
            run: an_inbox_asked_only_while_open,
        },
        Scenario {
            name: "submit: a branch the merge queue cannot read is passed over, named in status with its words, and a place's submission released on its base is still found, followed and landed",
            cites: &["99", "133", "111", "124", "47", "49", "56", "58", "63"],
            run: a_branch_passed_over,
        },
        Scenario {
            name: "submit: a place whose base the merge queue cannot read reads its own submission unread and acts on nothing until the base reads again",
            cites: &["99", "133", "15", "68", "56", "124"],
            run: its_base_passed_over,
        },
        Scenario {
            name: "submit: a place gone back as often as its kind allows waits for its caller",
            cites: &["112", "23"],
            run: the_count_reached,
        },
        Scenario {
            name: "submit: a withdrawn or refused submission leaves the place waiting, status naming why, and the settles after it ask the merge queue's inbox nothing",
            cites: &["112", "117", "133"],
            run: withdrawn_and_refused,
        },
        Scenario {
            name: "submit: held, nothing started: a fix whose landing the forge refused waits on its caller to retry code with the forge's words from the settle after the round, opening no agent, taking no verdict, counting no send-back and stepping no round, its submission held and open and its wait's since unchanged at the third settle",
            cites: &["112", "130", "133"],
            run: held_nothing_started,
        },
        Scenario {
            name: "submit: a fix's held landing pushed again lands: the next settle reads it landed, the place's work is over and the place is tidied",
            cites: &["113", "124", "112"],
            run: pushed_again,
        },
        Scenario {
            name: "submit: a fix's held landing dropped back into the queue: the settle that reads it waiting ends the wait and steps the base, and a later settle reads it landed",
            cites: &["112", "124"],
            run: back_in_the_queue,
        },
        Scenario {
            name: "submit: a retry past a held landing: a change allowed one send-back whose landing is held is retried at code with its caller's words, the submission after its stages withdraws the held one, and its failure sends it back to code with the merge queue's words, the one send-back its kind allows",
            cites: &["112", "131", "49"],
            run: a_retry_past_a_held_landing,
        },
        Scenario {
            name: "submit: a hand withdraws a fix's held landing: the next settle leaves the place waiting for its caller, status naming the withdrawal, and starts nothing",
            cites: &["112", "99"],
            run: a_held_landing_withdrawn,
        },
        Scenario {
            name: "submit: a base place whose kind names a fix, submitted at its caller's retry, whose landing the merge queue holds with a record that carries no words, waits on its caller with words saying the record gives none, launching no fix and forecasting nothing",
            cites: &["203", "112", "99"],
            run: a_base_places_held_submission,
        },
        Scenario {
            name: "submit: a fix whose landing is held stalls once five minutes pass under no hold's words, its recipient delivered once that it stalls on a retry of code",
            cites: &["130", "112", "43"],
            run: a_held_landing_stalls,
        },
    ]
}

/// A change's construct stage begun, its agent waiting.
pub fn construct_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("change", name);
    launched(
        &world
            .engine
            .launch(&launch("change", name, Some("plan it"))),
    )?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Begun),
        "construct"
    );
    Ok(id)
}

/// The change written on the place's branch, its planning complete.
pub fn write_change(world: &World, name: &str) {
    let revision = world.commit(
        &format!("change/{name}"),
        &[&format!("plans/{name}/design.md")],
    );
    world.change.set(
        REPOSITORY,
        &revision,
        name,
        ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1 do it".into(),
                ticked: false,
                group: None,
            }],
            archived: false,
            groups: vec![],
        },
    );
}

fn s29_a_commit_and_a_quiet_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.terminal.log.clear();
    world.settle(&id);
    let calls: Vec<String> = world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| call.starts_with("close") || call.starts_with("open"))
        .collect();
    ensure_eq!(
        calls,
        vec![
            format!(
                "close {}",
                World::identity("change", "plan", "construct", 1)
            ),
            format!(
                "open {} {}",
                World::identity("change", "plan", "code", 1),
                id
            ),
        ],
        "the settle's closes and opens"
    );
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start"
    );
    Ok(())
}

fn an_outcome_handed_again_holds_no_stage(_: &()) -> Outcome {
    // On deneb on 2026-11-15 a change's code, sent back by a failure its
    // merge queue kept owed, stayed started with its tasks ticked and its
    // agent waiting for hours: each settle's steps went to collecting that
    // failure again, so none read the stage's prompt received or its end
    // (40, 46, 133). A merge queue that takes no confirm keeps the
    // failure owed and hands it again at every inbox, as one did before
    // flume confirmed what it acted on (133).
    let world = World::new();
    world
        .queue
        .refuse_confirms(Some("the merge queue could not be read"));
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    world.tick("change/plan", "plan", 1, 1);
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not start in code's settle"
    );
    let words = "check test-x failed";
    world.queue.plan_submits(vec![Plan::Fail(words.into())]);
    archive(&world, "plan");
    world.settle(&id);
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("code did not start again")?;
    ensure!(prompt.contains(words), "{words} is not in {prompt}");
    let location = world.held(&id).ok_or("no place")?.place.location;
    ensure_eq!(
        world.queue.pending(&location),
        vec![SubmissionOutcome::Failed {
            words: words.into()
        }],
        "the failure owed, handed again at the next inbox"
    );
    world.queue.log.clear();
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, sent back, in the settle that sent it back"
    );
    world.tick("change/plan", "plan", 1, 1);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, delivered and its agent quiet, at the first settle"
    );
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not start again in code's settle"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        (held.submission.submits, held.submission.send_backs),
        (1, 1),
        "submissions and goings-back, the failure handed again"
    );
    // The inbox is asked only while the submission is open: no settle
    // asked it again while flume held that failure and code ran (111,
    // 133; record 0012).
    ensure_eq!(
        world.queue.log.count("inbox"),
        0,
        "inboxes while flume held the failure"
    );
    Ok(())
}

fn the_record_read_in_the_places_folder(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    let folder = world.store.folder(&id);
    world.agent.keep_record_in(&code, &folder);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its record read in the place's folder"
    );
    let elsewhere: Vec<String> = world
        .agent
        .log
        .calls()
        .into_iter()
        .filter(|call| !call.starts_with("command") && !call.ends_with(&format!(" in {folder}")))
        .collect();
    ensure_eq!(
        elsewhere,
        Vec::<String>::new(),
        "readings in another folder"
    );
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the place did not land");
    Ok(())
}

fn s7_two_held_places(_: &()) -> Outcome {
    let world = World::new();
    let mut ids = vec![];
    for name in ["one", "two"] {
        let mut held = launch("change", name, Some("plan it"));
        held.hold = vec!["code".into(), "verify".into()];
        held.hold_submission = true;
        held.mark = Some("team-a".into());
        launched(&world.engine.launch(&held))?;
        let id = World::id("change", name);
        world.settle(&id);
        write_change(&world, name);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "construct"),
            Some(StageState::Delivered),
            "construct"
        );
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::OnHold),
            "code"
        );
        ids.push(id);
    }
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    let answer = world
        .engine
        .retry(&ids[0], Retry::Stage("code".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure!(
        world
            .stage_state(&ids[0], "code")
            .is_some_and(StageState::running),
        "the first place's code stage"
    );
    ensure_eq!(
        world.stage_state(&ids[1], "code"),
        Some(StageState::OnHold),
        "the second's"
    );
    world.tick("change/one", "one", 1, 1);
    world.settle(&ids[0]);
    ensure_eq!(
        world.stage_state(&ids[0], "verify"),
        Some(StageState::OnHold),
        "the first place's verify once its code delivered"
    );
    let opened = world.terminal.log.count("open");
    let answer = world.engine.retry(&ids[0], Retry::Submission, None, false);
    ensure!(
        answer.refusal().is_none(),
        "the submission's retry was refused: {answer:?}"
    );
    ensure_eq!(world.queue.log.of("submit").len(), 1, "submissions");
    ensure!(
        world.queue.log.of("submit")[0].contains("change/one"),
        "another place was submitted"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        opened,
        "agents a submission's retry opened"
    );
    ensure_eq!(world.place.log.count("make"), 2, "places made");
    Ok(())
}

/// The settling spec's "Killed with nobody watching", S27: no caller keeps
/// watch, a code stage's agent stops with nothing delivered, and nothing
/// reads the stage until a caller drops another place, which settles the
/// places where something is pending first, the killed stage's among
/// them, reading it lost (S27, 119, 225).
fn s27_killed_with_nobody_watching(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let other = World::id("fix", "two");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(3600);
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Begun,
        "the stage before a command acts"
    );
    let dropped = world.engine.drop_place(&other, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Lost,
        "the stage once a command acted, settling the places where something is pending first"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn s28_crashed_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    world.agent.no_on_quiet();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a launch under a watch settles not"
    );
    world.host.advance(30);
    world.engine().settle_every(30);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage the watch read"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_watched_host_with_one_stream_unwatched(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    let trial = world.other_terminal("other");
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_display("trial", Some("other"), None, None);
    let id = World::id("fix", "one");
    let other = World::id("fix", "two");
    let tried = World::id("fix", "three");
    for (stream, name) in [("work", "one"), ("work", "two"), ("trial", "three")] {
        launched(&world.engine.launch(&Launch {
            stream: Some(stream.into()),
            ..launch("fix", name, Some("x"))
        }))?;
    }
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    trial.kill(&World::identity("fix", "three", "code", 1));
    let dropped = world.engine.drop_place(&other, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    for (place, stream) in [(&id, "work"), (&tried, "trial")] {
        ensure_eq!(
            world.status(place)?.stages[0].state,
            StageState::Begun,
            format!("the {stream} stage once the drop acted, which settles nothing first")
        );
    }
    world.host.advance(30);
    world.engine().settle_every(30);
    for (place, stream) in [(&id, "work"), (&tried, "trial")] {
        ensure_eq!(
            world.status(place)?.stages[0].state,
            StageState::Lost,
            format!("the {stream} stage at the caller's settle")
        );
    }
    Ok(())
}

fn opened_and_sent_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    let id = World::id("fix", "one");
    let first = World::identity("fix", "one", "code", 1);
    let at = world.host.now();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.terminal.sent_to(&first).len(),
        1,
        "the launch's sendings"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage after the launch"
    );
    let second = World::identity("fix", "one", "code", 2);
    world.agent.receives_at(&second, Some(2));
    // The begun stage's agent is the caller's to end first (50).
    let stopped = world.engine().stop(&id, false);
    ensure!(
        stopped.refusal().is_none(),
        "the stop was refused: {stopped:?}"
    );
    let retried = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(
        retried.refusal().is_none(),
        "the retry was refused: {retried:?}"
    );
    ensure_eq!(
        world.terminal.sent_to(&second).len(),
        1,
        "the retry's sendings"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Started),
        "the code stage after the retry, its sending not yet received"
    );
    ensure_eq!(
        world.host.now(),
        at,
        "the time the launch and the retry waited"
    );
    world.host.advance(30);
    world.engine().settle_every(30);
    ensure_eq!(
        world.terminal.sent_to(&second).len(),
        2,
        "sendings once the watch read the first not received within 60 seconds"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(())
}

fn a_settle_repeated(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    let held = world.held(&id).ok_or("no place")?;
    let writes = world.store.log.count("write");
    world.settle(&id);
    ensure_eq!(world.held(&id), Some(held), "the held state a repeat left");
    ensure_eq!(
        world.store.log.count("write"),
        writes,
        "writes a repeat made"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_launch_that_died(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let mut held = world.held(&id).ok_or("no place")?;
    held.phase = Phase::Making {
        by: world.host.another_command(),
    };
    world.store.put(&id, held);
    world.settle(&id);
    ensure!(
        world.held(&id).is_some(),
        "a place its live launch makes was undone"
    );
    if let Some(Phase::Making { by }) = world.held(&id).map(|held| held.phase) {
        world.host.end_command(&by);
    }
    world.engine().settle(None);
    ensure!(
        world.held(&id).is_none(),
        "the dead launch's held state is left"
    );
    ensure!(
        !world.place.exists(&id.0),
        "the dead launch's worktree is left"
    );
    Ok(())
}

/// A change's place driven to its verify stage, begun.
pub fn change_at_verify(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = construct_begun(world, name)?;
    write_change(world, name);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    world.tick(&format!("change/{name}"), name, 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify"
    );
    Ok(id)
}

/// The change archived on the place's branch, its report with it.
pub fn archive(world: &World, name: &str) {
    let revision = world.commit(
        &format!("change/{name}"),
        &[&format!("plans/archive/{name}/verify.md")],
    );
    world.change.set(
        REPOSITORY,
        &revision,
        name,
        ChangeFacts {
            archived: true,
            ..ChangeFacts::default()
        },
    );
}

fn s1_a_fix_lands(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    launched(
        &world
            .engine
            .launch(&launch("fix", "one", Some("the release's brief"))),
    )?;
    world.settle(&id);
    world.commit("fix/one", &["Cargo.toml"]);
    world.agent.set_own_work(&code, Reading::Yes);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code while its test runs"
    );
    world.agent.set_own_work(&code, Reading::No);
    let answer = world.engine().settle(Some(&id));
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(
        world.held(&id).is_none(),
        "held state of a landed place is kept"
    );
    ensure!(!world.place.exists(&id.0), "the place stands");
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_none(),
        "a branch holding nothing its base lacks is kept"
    );
    ensure_eq!(world.place.log.count("remove"), 1, "removes flume made");
    ensure_eq!(world.queue.log.count("inbox"), 2, "inbox reads");
    match answer {
        flume_core::Answer::Settled { tidy, .. } => {
            ensure_eq!(tidy.removed, vec![id.clone()], "the places removed")
        }
        other => return Err(format!("settle answered {other:?}")),
    }
    Ok(())
}

fn s8_another_places_landing(_: &()) -> Outcome {
    let world = World::new();
    let mut held = launch("change", "one", Some("plan it"));
    held.hold = vec!["code".into()];
    launched(&world.engine.launch(&held))?;
    let id = World::id("change", "one");
    world.settle(&id);
    write_change(&world, "one");
    world.settle(&id);
    let head = world
        .git
        .branch_head(REPOSITORY, "change/one")
        .ok_or("no branch")?;
    world.git.set_branch(REPOSITORY, "change/other", &head);
    world.commit("change/other", &["other.rs"]);
    world.queue.hand_landing(REPOSITORY, "change/other", "main");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code"
    );
    Ok(())
}

fn s19_a_failure_then_a_landing(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    let words = "check test-x failed: 2 of 46";
    world.queue.plan_submits(vec![Plan::Fail(words.into())]);
    archive(&world, "plan");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs");
    ensure_eq!(held.stages[1].starts, 2, "code's starts");
    ensure_eq!(held.stages[2].starts, 1, "verify's starts");
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("no second code")?;
    ensure!(prompt.contains(words), "the words are not in {prompt}");
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::NotReached),
        "verify, before code delivers again"
    );
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, delivered again"
    );
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not run again"
    );
    archive(&world, "plan");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(())
}

fn s30_a_landing_before_flume_submits(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code"
    );
    ensure_eq!(
        world.queue.log.count("submit"),
        0,
        "a held submission was made"
    );
    world.queue.hand_landing(REPOSITORY, "fix/one", "main");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions flume made");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(())
}

/// The settling spec's "A held submission landed by a hand": a chore's
/// launch held its submission, a hand lands the chore's branch through the
/// fake merge queue, and a caller keeps watch; the watch's settles ask the
/// merge queue nothing of the chore, and a settle naming no place and
/// stating no bound reads its work over and tidies it, never making the
/// held submission. The chore's caller's pane waits, so its ends are
/// handed to it by notice (S30, 43, 63, 225; design 3).
fn a_held_submission_landed_by_a_hand(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session(CALLER, AgentState::Waiting);
    let id = World::id("chore", "one");
    let mut chore = launch("chore", "one", Some("bump it"));
    chore.hold_submission = true;
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    world.commit("chore/one", &["chore.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "the chore's code"
    );
    world.queue.hand_landing(REPOSITORY, "chore/one", "main");
    world.queue.log.clear();
    for _ in 0..2 {
        world.engine.settle_every(60);
        world.host.advance(60);
    }
    let asked = world.queue.log.calls();
    ensure!(
        !asked.iter().any(|call| call.contains("chore/one")),
        "the watch's settles asked the merge queue of the chore: {asked:?}"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.phase),
        Some(Phase::Live),
        "the chore under the watch"
    );
    world.engine.settle(None);
    ensure!(world.held(&id).is_none(), "the landed chore is kept");
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions flume made");
    Ok(())
}

fn s34_a_restart_while_under_test(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    world.queue.plan_submits(vec![Plan::Hold]);
    archive(&world, "plan");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        held.submission.id.is_some(),
        "no submission of flume's is recorded"
    );
    ensure_eq!(held.phase, Phase::Live, "the place under test");
    world.host.restart();
    world.terminal.forget_all();
    world.queue.decide(REPOSITORY, "change/plan", Plan::Land);
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(!world.place.exists(&id.0), "the landed place stands");
    Ok(())
}

/// A fix launched as `name` with its code stage begun.
pub fn fix_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

/// The fix's code stage delivered, unsettled.
pub fn deliver_fix(world: &World, id: &PlaceId) {
    let name = id.0.rsplit("--").next().unwrap_or_default();
    world.commit(&format!("fix/{name}"), &["fix.rs"]);
}

fn two_places_one_base(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    let one = fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    deliver_fix(&world, &one);
    deliver_fix(&world, &two);
    world.queue.log.clear();
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "steps one settle made"
    );
    for id in [&one, &two] {
        ensure_eq!(
            world.held(id).map(|held| held.phase),
            Some(Phase::Live),
            "a place under test"
        );
    }
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions made anew");
    ensure_eq!(world.queue.log.count("step"), 2, "steps two settles made");
    Ok(())
}

fn an_unreadable_inbox(_: &()) -> Outcome {
    let world = World::new();
    world
        .queue
        .plan_submits(vec![Plan::Fail("check x failed".into())]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.queue.moves_on_its_own();
    world.settle(&id);
    world.queue.run_round(REPOSITORY, "main");
    world.queue.unreadable("the merge queue is down");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 0, "send-backs");
    ensure_eq!(held.stages[0].starts, 1, "code's starts");
    ensure!(
        world
            .status(&id)?
            .cannot_read
            .iter()
            .any(|what| what.contains("inbox") && what.contains("down")),
        "status names not the unread inbox"
    );
    world.queue.readable();
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs once read");
    ensure_eq!(
        held.submission.unread,
        None,
        "the inbox's reason, once read"
    );
    Ok(())
}

fn an_inbox_asked_only_while_open(_: &()) -> Outcome {
    let world = World::new();
    world
        .queue
        .plan_submits(vec![Plan::Fail("check x failed".into()), Plan::Hold]);
    world.queue.moves_on_its_own();
    let id = fix_begun(&world, "one")?;
    let location = world.held(&id).ok_or("no place")?.place.location;
    world.settle(&id);
    ensure_eq!(world.queue.log.count("inbox"), 0, "inboxes before a submit");

    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.queue.log.clear();
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.queue.log.count("inbox"),
        2,
        "inboxes of two settles while the submission is open"
    );

    world.queue.run_round(REPOSITORY, "main");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs");
    ensure_eq!(held.stages[0].starts, 2, "code's starts");
    let first = held.submission.id.ok_or("no submission")?;

    // An outcome pending for the place once flume holds its verdict, as
    // a merge queue leaves one it could not deliver to its reader.
    world.queue.leave_pending(
        &location,
        &first,
        SubmissionOutcome::Failed {
            words: "an earlier failure".into(),
        },
    );
    world.queue.log.clear();
    world.settle(&id);
    world.settle(&id);
    world.engine().settle(None);
    ensure_eq!(
        world.queue.log.count("inbox"),
        0,
        "inboxes while the fix round runs"
    );

    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions anew");
    ensure!(
        world.queue.log.count("inbox") >= 1,
        "no inbox as flume submitted anew"
    );
    ensure!(
        world.queue.pending(&location).is_empty(),
        "an outcome left pending from before the submission"
    );
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs");
    ensure_eq!(held.stages[0].starts, 2, "code's starts");
    ensure_eq!(
        held.submission.verdict,
        None,
        "the new submission's verdict"
    );

    world.queue.decide(REPOSITORY, "fix/one", Plan::Land);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions anew");
    Ok(())
}

/// The words the merge queue refused a branch with on deneb, 2026-11-04.
const REFUSED: &str =
    "bolt/a-run-compiles-through-a-shared-cache's policy at 4f6fe55f: [checkout] declares a cache";

fn a_branch_passed_over(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    let other = "bolt/a-run-compiles-through-a-shared-cache";
    world.queue.refuse_branch(REPOSITORY, other, REFUSED);
    world.queue.plan_submits(vec![Plan::Hold]);
    let released = world
        .engine
        .retry(&id, flume_core::Retry::Submission, None, false);
    if let Some(reason) = released.refusal() {
        return Err(format!("the submission's retry was refused: {reason}"));
    }
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    let status = world.status(&id)?;
    let read = status.submission.map(|submission| submission.read);
    ensure!(
        matches!(&read, Some(SubmissionRead::Found { submission }) if submission.state.open()),
        "the place's submission past {other} read {read:?}"
    );
    ensure_eq!(
        status
            .cannot_read
            .iter()
            .filter(|what| what.contains(other) && what.contains(REFUSED))
            .count(),
        2,
        format!(
            "the branch passed over, by read-submission and the inbox, in {:?}",
            status.cannot_read
        )
    );
    world.queue.decide(REPOSITORY, "fix/one", Plan::Land);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(!world.place.exists(&id.0), "the landed place stands");
    Ok(())
}

fn its_base_passed_over(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.queue.decide(REPOSITORY, "fix/one", Plan::Land);
    world.queue.refuse_branch(REPOSITORY, "main", REFUSED);
    world.queue.log.clear();
    world.settle(&id);
    ensure_eq!(world.queue.log.count("step"), 0, "steps of an unread base");
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    let status = world.status(&id)?;
    let read = status.submission.map(|submission| submission.read);
    ensure!(
        matches!(&read, Some(SubmissionRead::CouldNotRead { reason }) if reason.contains(REFUSED)),
        "the place's submission on its unread base read {read:?}"
    );
    ensure!(
        status
            .cannot_read
            .iter()
            .any(|what| what.contains("main") && what.contains(REFUSED)),
        "status names not the base passed over: {:?}",
        status.cannot_read
    );
    world.queue.read_branch(REPOSITORY, "main");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions made anew");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(())
}

fn the_count_reached(_: &()) -> Outcome {
    let world = World::new();
    let mut code = stage(
        "code",
        Some(Deliverable::Commits),
        "{{brief}}{{#words}} {{words}}{{/words}}",
    );
    code.send_back_limit = Some(1);
    code.send_back_to = Some("code".into());
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("bump", true, vec![code])]);
    world.queue.plan_submits(vec![
        Plan::Fail("first".into()),
        Plan::Fail("second".into()),
    ]);
    launched(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    let id = World::id("bump", "one");
    world.settle(&id);
    world.commit("bump/one", &["a.rs"]);
    world.settle(&id);
    world.settle(&id);
    world.commit("bump/one", &["b.rs"]);
    world.settle(&id);
    let status = world.status(&id)?;
    let submission = status.submission.ok_or("no submission")?;
    ensure_eq!(submission.submits, 2, "submissions");
    ensure_eq!(submission.send_backs, 1, "send-backs");
    ensure!(
        submission
            .waiting
            .as_deref()
            .is_some_and(|why| why.contains("second")),
        "status names not why it waits: {:?}",
        submission.waiting
    );
    ensure_eq!(status.stages[0].starts, 2, "code's starts");
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn withdrawn_and_refused(_: &()) -> Outcome {
    for (plan, why) in [
        (Plan::Withdraw, "withdrew"),
        (
            Plan::Refuse("a conflict with main".into()),
            "a conflict with main",
        ),
    ] {
        let world = World::new();
        world.queue.plan_submits(vec![plan]);
        launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
        let id = World::id("fix", "one");
        world.settle(&id);
        world.commit("fix/one", &["fix.rs"]);
        world.settle(&id);
        let status = world.status(&id)?;
        let waiting = status.submission.and_then(|submission| submission.waiting);
        ensure!(
            waiting.as_deref().is_some_and(|text| text.contains(why)),
            "status names not why it waits: {waiting:?}"
        );
        ensure_eq!(status.phase, Phase::Live, "the place");
        ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
        world.queue.log.clear();
        world.settle(&id);
        ensure_eq!(
            world.queue.log.count("inbox"),
            0,
            format!("inboxes once the submission is decided, {why}")
        );
    }
    Ok(())
}

/// The host's watch as the store holds it.
fn host_watch(world: &World) -> Option<Watched> {
    match world.store.host_document(&HostKey::Watch)? {
        HostDocument::Watch(watched) => Some(watched),
        _ => None,
    }
}

/// The host's part of status.
fn host_status(world: &World) -> Result<flume_core::HostStatus, String> {
    match world.engine().status(None) {
        flume_core::Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

/// The repositories a watch that hears its merge queue follows: none
/// while a chore works, its repository's while its submission is in
/// flight, and none once it landed and was tidied (124, 208, 225; record
/// 0082).
fn followed_while_in_flight(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = World::id("chore", "one");
    let mut chore = launch("chore", "one", Some("bump it"));
    chore.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    ensure!(
        world.engine.followed().is_empty(),
        "a repository followed while the chore works"
    );
    world.commit("chore/one", &["chore.rs"]);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "the chore submitted");
    ensure_eq!(
        world.engine.followed().into_iter().collect::<Vec<_>>(),
        vec![REPOSITORY.to_string()],
        "the repositories followed while its submission is in flight"
    );
    world.queue.decide(REPOSITORY, "chore/one", Plan::Land);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed chore is kept");
    ensure!(
        world.engine.followed().is_empty(),
        "a repository followed once the chore landed"
    );
    Ok(())
}

/// A chore named `name` for the recipient `ops`, its code stage delivered
/// and its agent ended, the place submitted at its agent's settle.
fn chore_submitted(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("chore", name);
    let mut chore = launch("chore", name, Some("bump it"));
    chore.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    world.commit(&format!("chore/{name}"), &["chore.rs"]);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "the chore submitted");
    ensure!(no_agent_live(world, &id), "an agent of the chore is live");
    Ok(id)
}

/// Whether no agent of the place is live.
fn no_agent_live(world: &World, id: &PlaceId) -> bool {
    world.held(id).is_some_and(|held| {
        held.stages
            .iter()
            .filter_map(|stage| stage.start.as_ref())
            .all(|start| !start.live)
    })
}

fn a_watch_kept_then_lapsed(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    for name in ["one", "two", "three"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    let began = world.host.now();
    world.engine.settle_every(120);
    let watched = Watched {
        began,
        bound: 120,
        by: Some(world.host.command()),
        ended: Some(began),
    };
    ensure_eq!(
        host_watch(&world),
        Some(watched.clone()),
        "the host's watch: when its settle began, the command that ran it, when it ended and its bound"
    );
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(3 * 60);
    let dropped = world.engine.drop_place(&World::id("fix", "two"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the stage once a drop acted three minutes on, unattended"
    );
    world.host.advance(2 * 60);
    let dropped = world.engine.drop_place(&World::id("fix", "three"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage once a drop acted five minutes on, attended"
    );
    ensure_eq!(
        host_watch(&world),
        Some(watched),
        "the host's watch, which only a caller's settle with a bound writes"
    );
    Ok(())
}

/// The settling spec's "A settle that runs past its bound": a caller
/// settles with a bound of one minute, and while that settle reads the
/// host's places the fake clock passes five minutes; flume runs unattended
/// throughout that settle and a minute after it ended, and attended once
/// two minutes have passed since it ended (119, 208).
fn a_settle_that_runs_past_its_bound(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let during = Arc::new(Mutex::new(None));
    {
        let host = world.host.clone();
        let engine = world.engine();
        let during = during.clone();
        world.store.during_next_read_every(move || {
            host.advance(5 * 60);
            *during.lock().expect("the reading") = Some(engine.attended());
        });
    }
    let began = world.host.now();
    let settled = world.engine.settle_every(60);
    ensure!(
        settled.refusal().is_none(),
        "the settle was refused: {settled:?}"
    );
    ensure_eq!(
        *during.lock().expect("the reading"),
        Some(false),
        "whether flume ran attended while the settle read the places, five minutes on"
    );
    let ended = began + 5 * 60;
    ensure_eq!(
        host_watch(&world).map(|watched| (watched.began, watched.ended)),
        Some((began, Some(ended))),
        "when the watch's settle began and ended"
    );
    world.host.advance(60);
    ensure!(
        !world.engine.attended(),
        "flume ran attended a minute after the settle ended"
    );
    world.host.advance(60);
    ensure!(
        world.engine.attended(),
        "flume ran unattended two minutes after the settle ended"
    );
    ensure_eq!(
        host_status(&world)?.watch_lapsed,
        Some(ended + 2 * 60),
        "when the watch lapsed"
    );
    Ok(())
}

/// The settling spec's "A watch whose settle died": the host's watch names
/// a settle with a bound of one minute begun by another command, which has
/// not ended it; flume runs unattended while that command lives, ten
/// minutes on among it, and attended once the command has died, two
/// minutes having passed since that settle began (76, 119, 208).
fn a_watch_whose_settle_died(_: &()) -> Outcome {
    let world = World::new();
    let other = world.host.another_command();
    let began = world.host.now();
    let watched = Watched {
        began,
        bound: 60,
        by: Some(other.clone()),
        ended: None,
    };
    world
        .store
        .write_host(&HostKey::Watch, None, Some(&HostDocument::Watch(watched)));
    ensure!(
        !world.engine.attended(),
        "flume ran attended while the other command's settle runs"
    );
    world.host.advance(10 * 60);
    ensure!(
        !world.engine.attended(),
        "flume ran attended ten minutes on, the other command living"
    );
    ensure_eq!(
        host_status(&world)?.watch_lapsed,
        None,
        "when the watch lapsed while its settle runs"
    );
    world.host.end_command(&other);
    ensure!(
        world.engine.attended(),
        "flume ran unattended once the command running the watch's settle died"
    );
    ensure_eq!(
        host_status(&world)?.watch_lapsed,
        Some(began + 2 * 60),
        "when the watch whose settle died lapsed"
    );
    Ok(())
}

/// The settling spec's "No terminal keeps watch": no caller has settled
/// with a bound, and the fake terminal's agents change state, one killed
/// and one working; nothing settles at those changes, and a caller's drop
/// settles the places where something is pending first (119, 122, 208,
/// 225).
fn no_terminal_keeps_watch(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    for name in ["one", "two", "three"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.terminal.set_state(
        &World::identity("fix", "three", "code", 1),
        AgentState::Working,
    );
    world.host.advance(10 * 60);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the stage after the agents changed, nothing settling"
    );
    let dropped = world.engine.drop_place(&World::id("fix", "two"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage once the drop settled the places where something is pending first"
    );
    ensure_eq!(host_watch(&world), None, "a watch held");
    Ok(())
}

/// A fix launched as `name` whose code stage stopped short needing an
/// answer, its agent kept open for no time and ended, so it waits only on
/// its caller's answer (42, 198).
fn fix_waiting_on_an_answer(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("fix", name);
    launched(&world.engine.launch(&Launch {
        every_stage: Settings {
            keep_open: Some("0".into()),
            ..Settings::default()
        },
        ..launch("fix", name, Some("x"))
    }))?;
    world.settle(&id);
    let start = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start of the fix's code")?;
    let answer = world.engine.needs(
        &start.identity.to_string(),
        &start.secret.0,
        "which way?",
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short: {answer:?}");
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::EndedShort),
        "the fix's code"
    );
    ensure!(no_agent_live(world, &id), "an agent of the fix is live");
    Ok(id)
}

/// Every call the fake merge queue, terminal and agent program logged.
fn seams_asked(world: &World) -> Vec<String> {
    [
        world.queue.log.calls(),
        world.terminal.log.calls(),
        world.agent.log.calls(),
    ]
    .concat()
}

/// Clears the fake merge queue's, terminal's and agent program's logs.
fn clear_seams(world: &World) {
    world.queue.log.clear();
    world.terminal.log.clear();
    world.agent.log.clear();
}

/// The settling spec's "Places with nothing pending": a fix waiting on an
/// answer whose agent has ended, a chore whose launch held its
/// submission, a fix whose code stage was read lost and a fix whose code
/// agent runs; a caller's settle with a bound asks the fake merge queue,
/// terminal and agent program nothing of the first three, by their names,
/// branches or identities, reads the running fix, and leaves the status
/// of each of the first three as it was. Their caller's pane waits, so
/// each end of theirs was handed to it by notice (42, 43, 45, 47, 225).
fn places_with_nothing_pending(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session(CALLER, AgentState::Waiting);
    let one = fix_waiting_on_an_answer(&world, "one")?;
    let two = World::id("chore", "two");
    let mut chore = launch("chore", "two", Some("bump it"));
    chore.hold_submission = true;
    launched(&world.engine.launch(&chore))?;
    world.settle(&two);
    world.commit("chore/two", &["chore.rs"]);
    world.settle(&two);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::Delivered),
        "the chore's code"
    );
    let three = fix_begun(&world, "three")?;
    world
        .terminal
        .kill(&World::identity("fix", "three", "code", 1));
    world.settle(&three);
    ensure_eq!(
        world.stage_state(&three, "code"),
        Some(StageState::Lost),
        "the third fix's code"
    );
    fix_begun(&world, "four")?;
    let waiting = [&one, &two, &three];
    let before = waiting
        .iter()
        .map(|id| world.status(id))
        .collect::<Result<Vec<_>, _>>()?;
    clear_seams(&world);
    let settled = world.engine.settle_every(120);
    ensure!(
        settled.refusal().is_none(),
        "the settle was refused: {settled:?}"
    );
    let asked = seams_asked(&world);
    for word in [
        "fix--one",
        "fix/one",
        "chore--two",
        "chore/two",
        "fix--three",
        "fix/three",
    ] {
        ensure!(
            !asked.iter().any(|call| call.contains(word)),
            "the watch's settle asked of {word}: {asked:?}"
        );
    }
    ensure!(
        asked.iter().any(|call| call.contains("fix--four")),
        "the watch's settle did not read the running fix: {asked:?}"
    );
    let after = waiting
        .iter()
        .map(|id| world.status(id))
        .collect::<Result<Vec<_>, _>>()?;
    ensure_eq!(after, before, "the status of each waiting place");
    Ok(())
}

/// The settling spec's "An unwatched drop": no caller keeps watch, a fix
/// waits on an answer whose agent has ended, and a caller drops another
/// place; the drop's settle asks the fake merge queue, terminal and agent
/// program nothing of the waiting fix (119, 225).
fn an_unwatched_drop(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session(CALLER, AgentState::Waiting);
    let one = fix_waiting_on_an_answer(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    clear_seams(&world);
    ensure!(world.engine.attended(), "a caller keeps watch");
    let dropped = world.engine.drop_place(&two, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    let asked = seams_asked(&world);
    for word in ["fix--one", "fix/one"] {
        ensure!(
            !asked.iter().any(|call| call.contains(word)),
            "the drop's settle asked of {word}: {asked:?}"
        );
    }
    ensure_eq!(
        world.stage_state(&one, "code"),
        Some(StageState::EndedShort),
        "the waiting fix's code"
    );
    Ok(())
}

/// Whether `calls` name the fix `name` by its place or its branch.
fn names_fix(calls: &[String], name: &str) -> bool {
    let (place, branch) = (format!("fix--{name}"), format!("fix/{name}"));
    calls
        .iter()
        .any(|call| call.contains(&place) || call.contains(&branch))
}

/// The stalls the place's held state records.
fn stalls_of(world: &World, id: &PlaceId) -> Vec<EndHeld> {
    world
        .held(id)
        .map(|held| held.ends)
        .unwrap_or_default()
        .into_iter()
        .filter(|end| matches!(end.of, EndOf::Stall { .. }))
        .collect()
}

/// The settling spec's "A stall's time comes": a fix waits on an answer
/// whose agent has ended, under no hold's words, and a caller settles with
/// a bound of one minute four minutes after the fix began to wait and
/// again six minutes after; the first settle reads nothing of the fix, and
/// the second records its stall and delivers it to the fix's recipients
/// (130, 225).
fn a_stalls_time_comes(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session(CALLER, AgentState::Waiting);
    let id = fix_waiting_on_an_answer(&world, "one")?;
    world.host.advance(4 * 60);
    clear_seams(&world);
    world.engine.settle_every(60);
    ensure!(
        !names_fix(&seams_asked(&world), "one"),
        "the settle four minutes on asked of the fix: {:?}",
        seams_asked(&world)
    );
    ensure!(stalls_of(&world, &id).is_empty(), "a stall four minutes on");
    world.host.advance(2 * 60);
    world.engine.settle_every(60);
    ensure_eq!(stalls_of(&world, &id).len(), 1, "the stalls six minutes on");
    ensure!(world.status(&id)?.stalled, "status does not say stalled");
    ensure!(
        world
            .terminal
            .notices(CALLER)
            .iter()
            .any(|notice| notice.contains("stalled")),
        "the stall was not delivered to the fix's recipient: {:?}",
        world.terminal.notices(CALLER)
    );
    Ok(())
}

/// The settling spec's "A hold's bound passes": a fix's waiting move is
/// held for ten minutes, and a caller settles with a bound of one minute
/// nine minutes after the hold and again eleven minutes after; the first
/// settle reads nothing of the fix, and the second lapses the hold and
/// records the fix stalled (130, 225).
fn a_holds_bound_passes(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session(CALLER, AgentState::Waiting);
    let id = fix_waiting_on_an_answer(&world, "one")?;
    let held = world
        .engine
        .hold(&id, HoldOn::Waiting, "on the user's card", Some(10 * 60));
    ensure!(held.refusal().is_none(), "the hold was refused: {held:?}");
    world.host.advance(9 * 60);
    clear_seams(&world);
    world.engine.settle_every(60);
    ensure!(
        !names_fix(&seams_asked(&world), "one"),
        "the settle nine minutes on asked of the fix: {:?}",
        seams_asked(&world)
    );
    ensure!(
        world
            .held(&id)
            .is_some_and(|held| held.waiting_hold().is_some()),
        "the hold lapsed nine minutes on"
    );
    world.host.advance(2 * 60);
    let lapsed_at = world.host.now();
    world.engine.settle_every(60);
    let after = world.held(&id).ok_or("the fix is gone")?;
    ensure!(
        after
            .holds
            .iter()
            .any(|hold| hold.on == HoldOn::Waiting && hold.lapsed == Some(lapsed_at)),
        "the hold did not lapse eleven minutes on: {:?}",
        after.holds
    );
    ensure!(world.status(&id)?.stalled, "status does not say stalled");
    ensure_eq!(
        stalls_of(&world, &id).len(),
        1,
        "the stalls eleven minutes on"
    );
    Ok(())
}

fn s109_a_submitted_place_with_no_agent_moves_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    world.engine.settle_every(120);
    let id = chore_submitted(&world, "one")?;
    for pass in 1..=3 {
        world.host.advance(120);
        world.queue.log.clear();
        world.engine().settle_every(120);
        ensure_eq!(
            world.queue.log.of("step"),
            vec![format!("step {REPOSITORY} main")],
            format!("the steps of the caller's settle {pass}")
        );
        ensure_eq!(
            world.held(&id).map(|held| held.phase),
            Some(Phase::Live),
            format!("the chore at the caller's settle {pass}")
        );
        ensure!(
            no_agent_live(&world, &id),
            "an agent of the chore is live at the caller's settle {pass}"
        );
    }
    // The third round lands it.
    world.queue.decide(REPOSITORY, "chore/one", Plan::Land);
    world.host.advance(120);
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(!world.place.exists(&id.0), "the chore's worktree stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn a_queue_that_moves_on_its_own(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.moves_on_its_own();
    world.queue.plan_submits(vec![Plan::Land]);
    world.engine.settle_every(120);
    let id = chore_submitted(&world, "one")?;
    for pass in 1..=2 {
        let before = world.held(&id).ok_or("no chore")?;
        world.host.advance(120);
        world.queue.log.clear();
        world.engine().settle_every(120);
        ensure_eq!(
            world.queue.log.of("step"),
            vec![format!("step {REPOSITORY} main")],
            format!("the steps of the caller's settle {pass}")
        );
        ensure_eq!(
            world.held(&id).map(|held| (held.phase, held.submission)),
            Some((before.phase, before.submission)),
            format!("the chore after the caller's settle {pass}")
        );
    }
    world.queue.run_round(REPOSITORY, "main");
    world.host.advance(120);
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    Ok(())
}

/// The settling spec's "A host with no runner", S118 as amended: no
/// caller keeps watch, a chore's code stage delivers and the settle its
/// agent's quiet runs submits the place; nothing settles for ten minutes,
/// the terminal settling nothing; a caller's drop settles the places where
/// something is pending first, the submitted chore's among them, stepping
/// its base; a caller then keeps watch, and the chore lands at a later
/// settle of its with no act; four minutes after its last settle ended
/// none keeps watch, and a drop settles the places where something is
/// pending first again (46, 119, 121, 122, 124, 208, 225).
fn s118_a_host_with_no_watch_runs_attended(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::None, None, None),
        "status before any caller keeps watch"
    );
    launched(&world.engine.launch(&launch("fix", "other", Some("x"))))?;
    let id = World::id("chore", "one");
    let mut chore = launch("chore", "one", Some("bump it"));
    chore.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    world.commit("chore/one", &["chore.rs"]);
    // The settle the agent's quiet runs, naming its place.
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "the chore submitted");
    ensure!(no_agent_live(&world, &id), "an agent of the chore is live");
    world.queue.log.clear();
    world.host.advance(10 * 60);
    ensure_eq!(
        world.queue.log.count("step"),
        0,
        "steps in ten minutes with no act"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.phase),
        Some(Phase::Live),
        "the chore after ten minutes with no act"
    );
    let dropped = world.engine.drop_place(&World::id("fix", "other"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "the drop's steps, settling the places where something is pending first"
    );
    // The runner starts, each pass settling the places where something is
    // pending with a bound.
    world.queue.log.clear();
    let first = world.host.now();
    world.engine().settle_every(120);
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "the caller's settle's steps"
    );
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::Bound { seconds: 120 }, Some(first), None),
        "status under the runner's watch"
    );
    world.queue.decide(REPOSITORY, "chore/one", Plan::Land);
    world.host.advance(120);
    let last = world.host.now();
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    // The runner stops; places launched meanwhile are not settled first.
    for name in ["three", "four"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    let four = World::id("fix", "four");
    world
        .terminal
        .kill(&World::identity("fix", "four", "code", 1));
    world.host.advance(4 * 60);
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::None, Some(last), Some(last + 4 * 60)),
        "status four minutes after the last pass ended"
    );
    ensure_eq!(
        world.stage_state(&four, "code"),
        Some(StageState::Begun),
        "the stage before the operator acts"
    );
    let dropped = world.engine.drop_place(&World::id("fix", "three"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&four, "code"),
        Some(StageState::Lost),
        "the stage once the operator's drop settled the places where something is pending first"
    );
    Ok(())
}

/// The settling spec's "Two depths in one settle": a chore submitted to
/// `main` and another submitted to `train/x`, a branch that lands on
/// `main`, open together; one settle of a caller keeping watch steps each
/// base once (124, 125, 202, 225).
fn two_depths_in_one_settle(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("train", true, vec![])]);
    launched(&world.engine.launch(&launch("train", "x", None)))?;
    for (name, base) in [("one", "main"), ("two", "train/x")] {
        let id = World::id("chore", name);
        launched(&world.engine.launch(&Launch {
            base: base.into(),
            ..launch("chore", name, Some("bump it"))
        }))?;
        world.settle(&id);
        world.commit(&format!("chore/{name}"), &["chore.rs"]);
        world.settle(&id);
    }
    ensure_eq!(world.queue.log.count("submit"), 2, "the chores submitted");
    world.queue.log.clear();
    world.engine().settle_every(120);
    let mut steps = world.queue.log.of("step");
    steps.sort();
    ensure_eq!(
        steps,
        vec![
            format!("step {REPOSITORY} main"),
            format!("step {REPOSITORY} train/x"),
        ],
        "the steps of one settle of the watch's"
    );
    Ok(())
}

/// The forge's words for a landing it refused, as a push protection
/// gives them.
pub const FORGE: &str =
    "push declined: the push holds what looks like a secret in src/config.rs:12";

/// A fix named `name` delivered and submitted to a queue that moves on
/// its own, whose next round holds its landing with the forge's words,
/// on its record where `recorded`: the round run, unsettled.
pub fn fix_held(world: &World, name: &str, recorded: bool) -> Result<PlaceId, String> {
    world.queue.plan_submits(vec![Plan::Held {
        words: FORGE.into(),
        recorded,
    }]);
    let id = fix_begun(world, name)?;
    world.queue.moves_on_its_own();
    deliver_fix(world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.queue.run_round(REPOSITORY, "main");
    Ok(id)
}

fn held_nothing_started(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_held(&world, "one", true)?;
    world.host.advance(60);
    let at = world.host.now();
    world.queue.log.clear();
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    let waiting = held.waiting.clone().ok_or("the place waits on nobody")?;
    ensure_eq!(waiting.on.act, Act::Retry, "the move waited on");
    ensure_eq!(
        waiting.on.stage.as_deref(),
        Some("code"),
        "the stage retried"
    );
    ensure_eq!(waiting.on.why, Waits::ForgeRefused, "why it waits");
    ensure!(
        waiting
            .on
            .words
            .as_deref()
            .is_some_and(|words| words.contains(FORGE)),
        "the wait's words are not the forge's: {:?}",
        waiting.on.words
    );
    ensure_eq!(
        waiting.on.since,
        Some(at),
        "since the settle after the round"
    );
    for _ in 0..2 {
        world.host.advance(60);
        world.settle(&id);
    }
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    ensure_eq!(held.submission.verdict, None, "a verdict taken");
    ensure_eq!(held.submission.send_backs, 0, "send-backs");
    ensure_eq!(held.submission.submits, 1, "submissions");
    ensure_eq!(held.stages[0].starts, 1, "code's starts");
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    ensure_eq!(world.queue.log.count("step"), 0, "rounds stepped");
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions made anew");
    ensure_eq!(
        held.waiting.map(|waiting| waiting.on.since),
        Some(Some(at)),
        "the wait's since at the third settle"
    );
    let status = world.status(&id)?;
    let submission = status.submission.ok_or("no submission")?;
    ensure!(
        matches!(
            &submission.read,
            SubmissionRead::Found { submission }
                if submission.state == SubmissionState::Held { words: FORGE.into() }
        ),
        "the submission read {:?}",
        submission.read
    );
    ensure!(
        submission.submits == 1 && submission.verdict.is_none(),
        "the submission is not open: {submission:?}"
    );
    Ok(())
}

/// The fix `one` whose landing the queue holds, its wait read at a settle.
fn fix_waiting_on_a_held_landing(world: &World) -> Result<PlaceId, String> {
    let id = fix_held(world, "one", true)?;
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        held.submission.landing_refused,
        "the place waits on no held landing: {:?}",
        held.submission
    );
    Ok(id)
}

fn pushed_again(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_waiting_on_a_held_landing(&world)?;
    world
        .queue
        .push_again(REPOSITORY, "fix/one")
        .ok_or("no held landing to push again")?;
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(!world.place.exists(&id.0), "the landed place stands");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    Ok(())
}

fn back_in_the_queue(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_waiting_on_a_held_landing(&world)?;
    world.queue.drop_held(REPOSITORY, "fix/one");
    world.queue.log.clear();
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        !held.submission.landing_refused && held.submission.waiting.is_none(),
        "the wait stands: {:?}",
        held.submission
    );
    ensure_eq!(held.waiting, None, "the move waited on");
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "the base stepped"
    );
    world.queue.run_round(REPOSITORY, "main");
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions made anew");
    Ok(())
}

fn a_retry_past_a_held_landing(_: &()) -> Outcome {
    let world = World::with_every_own_version();
    let mut own = crate::fake_harness::flumes_own();
    for kind in own.iter_mut().filter(|kind| kind.name == "change") {
        if let Some(verify) = kind.stages.last_mut() {
            verify.send_back_limit = Some(1);
        }
    }
    world.kinds.set_own(own);
    let failed = "check test-y failed: 1 of 46";
    world.queue.plan_submits(vec![
        Plan::Held {
            words: FORGE.into(),
            recorded: true,
        },
        Plan::Fail(failed.into()),
    ]);
    let id = change_at_verify(&world, "plan")?;
    world.queue.moves_on_its_own();
    archive(&world, "plan");
    world.settle(&id);
    world.queue.run_round(REPOSITORY, "main");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(held.submission.landing_refused, "no held landing's wait");
    let first = held.submission.id.clone().ok_or("no submission")?;
    let words = "take the token out of src/config.rs";
    let retried = world
        .engine
        .retry(&id, Retry::Stage("code".into()), Some(words.into()), false);
    if let Some(reason) = retried.refusal() {
        return Err(format!("the retry was refused: {reason}"));
    }
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("no second code")?;
    ensure!(
        prompt.contains(words),
        "the caller's words are not in {prompt}"
    );
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        !held.submission.landing_refused && held.submission.waiting.is_none(),
        "the held landing's wait outlives the retry: {:?}",
        held.submission
    );
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify, run again"
    );
    archive(&world, "plan");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    ensure!(
        world
            .queue
            .submissions(REPOSITORY, "change/plan")
            .iter()
            .any(|submission| submission.id == first
                && submission.state == SubmissionState::Withdrawn),
        "the held submission is not withdrawn"
    );
    world.queue.run_round(REPOSITORY, "main");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs");
    ensure_eq!(
        held.submission.waiting,
        None,
        "a wait past the kind's bound"
    );
    ensure_eq!(held.stages[1].starts, 3, "code's starts");
    let code = World::identity("change", "plan", "code", 3);
    let prompt = world.prompt(&id, &code).ok_or("no third code")?;
    ensure!(
        prompt.contains(failed),
        "the merge queue's words are not in {prompt}"
    );
    Ok(())
}

fn a_held_landing_withdrawn(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_waiting_on_a_held_landing(&world)?;
    world.queue.decide(REPOSITORY, "fix/one", Plan::Withdraw);
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    ensure!(
        !held.submission.landing_refused,
        "the held landing's wait stands"
    );
    ensure_eq!(
        held.submission.verdict,
        Some(Verdict::Withdrawn),
        "the verdict"
    );
    let status = world.status(&id)?;
    let waiting = status.submission.and_then(|submission| submission.waiting);
    ensure!(
        waiting
            .as_deref()
            .is_some_and(|text| text.contains("withdrew")),
        "status names not the withdrawal: {waiting:?}"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    Ok(())
}

fn a_base_places_held_submission(_: &()) -> Outcome {
    let world = World::new();
    let spring = super::base_places::spring_mended(&world, None)?;
    world.queue.plan_submits(vec![Plan::Held {
        words: FORGE.into(),
        recorded: false,
    }]);
    match world
        .engine()
        .retry(&spring, Retry::Submission, None, false)
    {
        flume_core::Answer::Acted { .. } => {}
        other => return Err(format!("the retry answered {other:?}")),
    }
    world.engine().settle(None);
    world.engine().settle(None);
    ensure!(
        world
            .queue
            .submissions(REPOSITORY, "train/spring")
            .last()
            .is_some_and(|submission| matches!(submission.state, SubmissionState::Held { .. })),
        "the merge queue holds no landing of train/spring"
    );
    world.queue.log.clear();
    world.engine().settle(None);
    let held = world.held(&spring).ok_or("the base place is gone")?;
    ensure!(held.submission.landing_refused, "no held landing's wait");
    ensure!(held.fixes.is_empty(), "fixes launched: {:?}", held.fixes);
    ensure_eq!(held.submission.verdict, None, "a verdict taken");
    let status = world.status(&spring)?;
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom the base place waits on"
    );
    let words = status
        .waiting_on
        .and_then(|waiting| waiting.words)
        .unwrap_or_default();
    ensure!(
        words.contains(flume_fakes::HELD_WITHOUT_WORDS),
        "the words say not that the record gives none: {words}"
    );
    ensure!(
        world
            .terminal
            .log
            .of("open")
            .iter()
            .all(|call| !call.contains("mend--")),
        "a fix's agent opened"
    );
    ensure_eq!(world.queue.log.count("forecast"), 0, "forecasts");
    Ok(())
}

fn a_held_landing_stalls(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Held {
        words: FORGE.into(),
        recorded: true,
    }]);
    let mut fix = launch("fix", "one", Some("x"));
    fix.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.queue.moves_on_its_own();
    deliver_fix(&world, &id);
    world.settle(&id);
    world.queue.run_round(REPOSITORY, "main");
    world.host.advance(60);
    let since = world.host.now();
    world.settle(&id);
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    for _ in 0..2 {
        world.host.advance(30);
        world.settle(&id);
    }
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        held.waiting.as_ref().is_some_and(|waiting| waiting.stalled),
        "the place is not stalled: {:?}",
        held.waiting
    );
    let stalls: Vec<EndOf> = held
        .ends
        .into_iter()
        .map(|end| end.of)
        .filter(|of| matches!(of, EndOf::Stall { .. }))
        .collect();
    ensure!(
        matches!(
            stalls.as_slice(),
            [EndOf::Stall { since: at, waiting, .. }] if *at == since && waiting == "a retry of code"
        ),
        "the stalls recorded: {stalls:?}"
    );
    let notices: Vec<String> = world
        .terminal
        .notices("ops")
        .into_iter()
        .filter(|text| text.contains(" stalled: "))
        .collect();
    ensure!(
        matches!(notices.as_slice(), [one] if one.contains("a retry of code")),
        "the stall notices to ops: {notices:?}"
    );
    Ok(())
}
