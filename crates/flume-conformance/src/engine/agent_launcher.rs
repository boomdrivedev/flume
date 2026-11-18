//! A start through flume's launcher (35, 161, C.4, S74; core.10): where
//! an agent's profile starts its program through the launcher, the core
//! takes its start only once the launcher's word names a process, the
//! terminal reads the program running in that process and the agent
//! waiting; until then it stays starting, sent nothing, and a terminal
//! that cannot read the process leaves it so, never failed. A program the
//! terminal reads running in another process than the launcher became, as
//! a person running it again by hand in the pane starts it, is no agent of
//! the start's: it is read begun otherwise, sent no prompt, ended once
//! quiet and its stage started afresh (161, S74).

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent launcher: a fix on an agent that starts through the launcher, its word naming the process 41 and the terminal reading the program running in 41 and the agent waiting, starts and is sent its prompt",
            cites: &["35"],
            run: the_word_and_its_process_read,
        },
        Scenario {
            name: "agent launcher: a start whose word names the process 41 while the terminal reads nothing running the program stays starting with nothing sent, and starts and is sent its prompt once the terminal reads the program running in 41",
            cites: &["35"],
            run: the_word_alone,
        },
        Scenario {
            name: "agent launcher: a start whose word names the process 41 under a terminal that reads no process stays starting with nothing sent, and no end is read at any settle before the stage's limit",
            cites: &["35"],
            run: a_terminal_that_reads_no_process,
        },
        Scenario {
            name: "agent launcher: a fix's agent started through the launcher in the process 41 took its prompt, and once the terminal reads its program running in 42, the agent quiet, flume ends it and starts code afresh, the start read begun in another process than the one its start's launcher became",
            cites: &["161", "S74"],
            run: s74_amp_started_again_by_hand,
        },
        Scenario {
            name: "agent launcher: a start whose word names the process 41 while the terminal first reads its program running in 42 is sent no prompt and read begun otherwise, naming the process",
            cites: &["161", "S74"],
            run: before_its_prompt,
        },
    ]
}

/// The process the launcher's word names in these scenarios.
pub(crate) const LAUNCHED: u32 = 41;

/// The fake agent starts through the launcher, and the word of the fix
/// `name`'s first code start names [`LAUNCHED`]: that start's identity.
pub(crate) fn through_the_launcher(world: &World, name: &str) -> Identity {
    world.agent.start_through_launcher();
    let who = World::identity("fix", name, "code", 1);
    world.agent.set_launched(&who, LAUNCHED);
    who
}

/// The fix `name` launched and settled once.
pub(crate) fn fix(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&launch("fix", name, Some("mend it"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

/// The agent-launcher spec's scenario "The word and its process read"
/// (35, C.4): Part C's profiles are cited in words alone, since the suite
/// cites as data only the clauses of Parts A and B, the invariants and
/// section 11's scenarios.
fn the_word_and_its_process_read(_: &()) -> Outcome {
    let world = World::new();
    let who = through_the_launcher(&world, "one");
    world
        .terminal
        .set_process(&who, ProgramProcess::Runs { pid: LAUNCHED });
    let id = fix(&world, "one")?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its program read in the launcher's process"
    );
    ensure!(
        !world.terminal.sent_to(&who).is_empty(),
        "no prompt was sent"
    );
    let (command, _) = world
        .terminal
        .opened(&who)
        .ok_or("the start opened no agent")?;
    ensure_eq!(
        command.launcher,
        vec!["/fake/flume", "ack", "launched", "--"],
        "the command's launcher"
    );
    ensure!(
        world.agent.log.count("launched") > 0,
        "the launcher's word was never read"
    );
    ensure!(
        world.terminal.log.count("process") > 0,
        "the terminal was never asked the process"
    );
    Ok(())
}

fn the_word_alone(_: &()) -> Outcome {
    let world = World::new();
    let who = through_the_launcher(&world, "one");
    world.terminal.set_process(&who, ProgramProcess::Nothing);
    let id = fix(&world, "one")?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "code, nothing read running its program"
    );
    ensure!(
        world.terminal.sent_to(&who).is_empty(),
        "a prompt was sent to a start nothing was read running"
    );
    world
        .terminal
        .set_process(&who, ProgramProcess::Runs { pid: LAUNCHED });
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its program read in the launcher's process"
    );
    ensure!(
        !world.terminal.sent_to(&who).is_empty(),
        "no prompt was sent"
    );
    Ok(())
}

fn a_terminal_that_reads_no_process(_: &()) -> Outcome {
    let world = World::new();
    let who = through_the_launcher(&world, "one");
    let id = fix(&world, "one")?;
    // Settled at once, then three times more before the stage's limit.
    for settle in 0..4 {
        if settle > 0 {
            world.host.advance(flume_core::QUIET_LIMIT / 4);
            world.settle(&id);
        }
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Starting),
            "code, under a terminal that reads no process, at settle {settle}"
        );
        ensure!(
            world.terminal.sent_to(&who).is_empty(),
            "a prompt was sent to a start whose process was never read"
        );
        let held = world.held(&id).ok_or("no held state")?;
        ensure!(held.ends.is_empty(), "an end was read: {:?}", held.ends);
        ensure!(
            held.stages[0].end.is_none(),
            "the stage's end was read: {:?}",
            held.stages[0].end
        );
        ensure!(held.failures.is_empty(), "failures: {:?}", held.failures);
    }
    ensure_eq!(
        world.terminal.program_process(&who),
        ProgramProcess::Unknown {
            why: NO_PROCESS.into()
        },
        "the terminal's reading"
    );
    Ok(())
}

/// The words a start read begun in another process gives (161, S74).
const ANOTHER_PROCESS: &str = "in another process than the one its start's launcher became";

fn s74_amp_started_again_by_hand(_: &()) -> Outcome {
    let world = World::new();
    let first = through_the_launcher(&world, "one");
    world
        .terminal
        .set_process(&first, ProgramProcess::Runs { pid: LAUNCHED });
    let id = fix(&world, "one")?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its prompt taken in the launcher's process"
    );
    world
        .terminal
        .set_process(&first, ProgramProcess::Runs { pid: LAUNCHED + 1 });
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {first}")],
        "the agents ended"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let start = held.stages[0].start.clone().ok_or("no start of code")?;
    ensure_eq!(start.n, 2, "code's start, started afresh");
    ensure!(
        held.ends.is_empty(),
        "an end recorded for the start begun in another process: {:?}",
        held.ends
    );
    ensure_eq!(
        held.submission.send_backs,
        0,
        "the kind's bound, counted by the place's send-backs"
    );
    let status = world.status(&id)?;
    let read: Vec<(u32, String)> = status
        .unsettled
        .iter()
        .map(|read| (read.start, read.unsettled.words()))
        .collect();
    ensure_eq!(
        read,
        vec![(1, ANOTHER_PROCESS.to_string())],
        "the start read begun otherwise"
    );
    Ok(())
}

fn before_its_prompt(_: &()) -> Outcome {
    let world = World::new();
    let who = through_the_launcher(&world, "one");
    world
        .terminal
        .set_process(&who, ProgramProcess::Runs { pid: LAUNCHED + 1 });
    // At work, so it is not yet ended.
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "code, its program read in another process"
    );
    ensure!(
        world.terminal.sent_to(&who).is_empty(),
        "a prompt was sent to a program in another process than the launcher became"
    );
    let status = world.status(&id)?;
    ensure_eq!(
        status.agent.map(|agent| agent.started),
        Some(Reading::No),
        "started"
    );
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.unsettled.clone()))
            .collect::<Vec<_>>(),
        vec![(
            "code",
            1,
            Unsettled {
                process: true,
                ..Unsettled::default()
            }
        )],
        "the start read begun otherwise"
    );
    ensure!(
        status.unsettled[0]
            .unsettled
            .words()
            .contains("another process"),
        "the words name no process: {}",
        status.unsettled[0].unsettled.words()
    );
    Ok(())
}
