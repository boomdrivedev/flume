//! A start through flume's launcher (35, 161, C.4, S74; core.10): where
//! an agent's profile starts its program through the launcher, the core
//! takes its start only once the launcher's word names a process, the
//! terminal reads the program running in that process and the agent
//! waiting; until then it stays starting, sent nothing, and a terminal
//! that cannot read the process leaves it so, never failed.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent launcher: a fix on an agent that starts through the launcher, its word naming the process 41 and the terminal reading the program running in 41 and the agent waiting, starts and is sent its prompt",
            cites: &["35", "C.4"],
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
