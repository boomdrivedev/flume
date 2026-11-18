//! An agent read from its terminal (31, 38, 89 to 93, 134, 136, 171):
//! where an agent's profile does not build received, asking, own work,
//! ended or last words, the core reads that part from the terminal its
//! start recorded and asks the profile nothing of it; where the profile
//! builds it, the core reads it from the profile alone. Part C's profiles
//! are cited in words alone, since the suite cites as data only the
//! clauses of Parts A and B, the invariants and section 11's scenarios:
//! the common profile of C.4 and herdr's watched send and states of C.5.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "terminal readings: a begun fix's agent that reads asking from its terminal is read asking while the terminal reads it asking, a tell to it refused with nothing typed, and read neither once the terminal reads it waiting",
            cites: &["37", "90", "171"],
            run: asking_as_the_terminal_shows_it,
        },
        Scenario {
            name: "terminal readings: a begun fix's agent whose profile builds asking, holding no asking mark while the terminal reads it asking, reads unknown and not asking, and a tell to it is refused with nothing typed",
            cites: &["90"],
            run: a_profile_that_builds_them_reads_no_question,
        },
        Scenario {
            name: "terminal readings: a begun fix's agent that reads ended and last words from its terminal, the terminal showing `amp: out of credits` and then reading it stopped with nothing delivered, is reported lost with those last words",
            cites: &["43", "45", "134", "136", "171"],
            run: lost_with_its_screen,
        },
    ]
}

/// The fix `name` launched with its code agent reading `parts` from its
/// terminal, settled once: its place and its code start's identity.
pub(crate) fn fix_reading(
    world: &World,
    name: &str,
    parts: &[Part],
) -> Result<(PlaceId, Identity), String> {
    world.agent.reads_from_its_terminal(parts);
    launched(&world.engine.launch(&launch("fix", name, Some("mend it"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok((id, World::identity("fix", name, "code", 1)))
}

/// The reads of the fake agent's log of `operation` for `who`'s start.
fn reads_of(world: &World, operation: &str, who: &Identity) -> Vec<String> {
    world
        .agent
        .log
        .of(operation)
        .into_iter()
        .filter(|line| line.contains(&who.to_string()))
        .collect()
}

fn agent_status(world: &World, place: &PlaceId) -> Result<flume_core::AgentStatus, String> {
    world
        .status(place)?
        .agent
        .ok_or("no agent in status".into())
}

/// The terminal-readings spec's scenario "Asking as the terminal shows
/// it" (37, 90, 171).
fn asking_as_the_terminal_shows_it(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[Part::Asking])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Asking, "the agent's state");
    ensure_eq!(agent.asking, Reading::Yes, "the agent's asking");
    let told = world.engine().tell(&id, "go on", Some(30));
    ensure!(
        told.refusal().is_some(),
        "a tell to an asking agent was taken"
    );
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt.clone()],
        "what was typed into the agent"
    );
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Waiting, "the agent's state");
    ensure_eq!(agent.asking, Reading::No, "the agent's asking");
    Ok(())
}

/// The terminal-readings spec's scenario "A profile that builds them
/// reads no question from the terminal" (90; core.39).
fn a_profile_that_builds_them_reads_no_question(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Unknown, "the agent's state");
    ensure_eq!(agent.asking, Reading::No, "the agent's asking");
    let told = world.engine().tell(&id, "go on", Some(30));
    ensure!(
        told.refusal().is_some(),
        "a tell into a question flume cannot name was taken"
    );
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt],
        "what was typed into the agent"
    );
    Ok(())
}

/// The terminal-readings spec's scenario "Lost with its screen" (43, 45,
/// 171).
fn lost_with_its_screen(_: &()) -> Outcome {
    let world = World::new();
    let (id, code) = fix_reading(&world, "one", &[Part::Ended, Part::LastWords])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    world.agent.set_last_words(&code, "words of the record");
    world
        .terminal
        .set_screen(&code, Some("amp: out of credits\n\n"));
    world.terminal.set_state(&code, AgentState::Stopped);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("amp: out of credits"),
        "the last words the end carries"
    );
    for operation in ["ended", "last_words"] {
        ensure!(
            reads_of(&world, operation, &code).is_empty(),
            "the profile was asked {operation}"
        );
    }
    Ok(())
}
