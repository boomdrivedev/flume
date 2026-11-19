//! A command stage's run (223, 224, C.2, C.5): a statechart kind's stage
//! that runs, in place of an agent, one command its kind declares. Its one
//! session is started through the stream's terminal as a program of a pane
//! or block, by a link of the start's own leading to flume's launcher,
//! which runs the command as its child in the place under the start's
//! identity, writes its exit and its whole output beside the start's
//! record and then settles the place. It takes no room, no account and no
//! points, has no prompt and no agent's settings, and is never stopped
//! short or stop-checked. Each settle reads its end from the launcher's
//! recorded exit and the place's branch: delivered once its command exited
//! with success and the branch holds a commit made since it started;
//! failed where it exited otherwise, exited with nothing committed, or
//! stopped with no exit recorded (core.9).

use crate::start::{Start, Starts};
use crate::start_record::{start_record, AGENT_VARIABLE, PLACE_VARIABLE, STAGE_VARIABLE};
use crate::{Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;

/// The variable naming a start's secret, as an agent's start carries it
/// (33, 161).
pub const SECRET_VARIABLE: &str = "FLUME_SECRET";

/// How long a run's terminal may read it stopped with no word of its
/// launcher before it is read stopped with no exit recorded: the
/// terminal's start bound (35).
pub const UNSEEN: Seconds = 60;

/// Whether the stage at `index` of `held` is a command stage's session
/// (223).
pub(crate) fn is_run(held: &HeldState, index: usize) -> bool {
    held.stages
        .get(index)
        .and_then(|stage| stage.session.as_ref())
        .is_some_and(|session| session.command.is_some())
}

/// The index of the newest session of the command stage at `path`; none
/// where `path` names no command stage or it has no session yet (49, 223).
pub(crate) fn last_run(held: &HeldState, path: &str) -> Option<usize> {
    (0..held.stages.len()).rev().find(|&index| {
        is_run(held, index)
            && held.stages[index]
                .session
                .as_ref()
                .is_some_and(|session| session.stage == path)
    })
}

impl Engine {
    /// Starts the command stage's session at `index`: its start written
    /// live under a fresh identity and secret, with no account, no
    /// settings and no prompt, its record naming the command's program and
    /// flume's variables, and the command run through the stream's
    /// terminal, its program and arguments after flume's launcher, in the
    /// stream's session for the place's kind's stage; a terminal that
    /// cannot run it fails the stage in its words (33, 70, 145, 161, 223,
    /// C.5).
    pub(crate) fn start_command(&self, id: &PlaceId, index: usize, command: &[String]) -> Start {
        let Some(read) = self.store.read(id) else {
            return Start::NoPlace;
        };
        let held = read.state;
        let Some((program, args)) = command.split_first() else {
            return Start::NotOpened("the command names no program".into());
        };
        let place = held.place.clone();
        let stage_name = held.declared(index).name.clone();
        let (terminal, named) = match self.stream_terminal(&place.stream.name) {
            Ok(found) => found,
            Err(reason) => {
                self.run_failed(id, index, &reason);
                return Start::NotOpened(reason);
            }
        };
        let opens_in = match self.queue().session_of(
            &place.stream.name,
            &place.kind.name,
            &stage_name,
            &Settings::default(),
        ) {
            Ok(opens_in) => opens_in,
            Err(reason) => {
                self.run_failed(id, index, &reason);
                return Start::NotOpened(reason);
            }
        };
        let identity = self.next_identity(&held, index);
        let secret = self.host.secret();
        let now = self.host.now();
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        let record = StartRecord {
            command: true,
            ..start_record(
                program,
                None,
                &BTreeMap::new(),
                &[],
                &identity,
                &place.location,
                &secret,
            )
        };
        let claimed = self.update(id, |held| {
            let Some(stage) = held.stages.get_mut(index) else {
                return Step::Keep(false);
            };
            if stage.start.as_ref().is_some_and(|start| start.live) {
                return Step::Keep(false);
            }
            stage.state = StageState::Starting;
            stage.starts = stage.starts.max(identity.start);
            stage.refused_part = None;
            stage.refused_setting = None;
            stage.room = None;
            stage.end = None;
            stage.start = Some(StartHeld {
                n: identity.start,
                started_at: now,
                start_head: head.clone(),
                made_from: 0,
                words: None,
                identity: identity.clone(),
                secret: secret.clone(),
                program: program.clone(),
                settings: Settings::default(),
                from: BTreeMap::new(),
                account: None,
                start_record: record.clone(),
                record: None,
                prompt: Prompt::default(),
                live: true,
                begun_at: None,
                first_quiet: None,
                last_head: head.clone(),
                last_head_move: now,
                past_limit: false,
                printed: None,
                group: None,
                may_name: vec![],
                opening: None,
                dialog: None,
                conversation: None,
                session: opens_in.clone(),
                label: None,
                terminal: Some(named.clone()),
                own: None,
                answered: 0,
                hibernated: None,
                resumes: vec![],
                handing: None,
                messages: vec![],
                counts: None,
                span: None,
                asking_since: None,
                waiting: None,
            });
            if !held
                .terminals
                .iter()
                .any(|opened| opened.profile == named.profile)
            {
                held.terminals.push(named.clone());
            }
            held.current = Some(index);
            Step::Write(true)
        });
        if claimed != Some(true) {
            return Start::AgentLive;
        }
        let folder = self.store.folder(id);
        let env = BTreeMap::from([
            (AGENT_VARIABLE.to_string(), identity.to_string()),
            (SECRET_VARIABLE.to_string(), secret.0.clone()),
            (PLACE_VARIABLE.to_string(), place.location.clone()),
            (STAGE_VARIABLE.to_string(), identity.stage.clone()),
        ]);
        let run = AgentCommand {
            program: program.clone(),
            args: args.to_vec(),
            env,
            prompt_args: vec![],
            folder_trust: None,
            kind: Some(place.kind.name.clone()),
            files: BTreeMap::new(),
            launcher: self.host.launcher(),
        };
        let at = match opens_in {
            Some(SessionHeld {
                name: Some(name), ..
            }) => OpenIn::Session { name },
            _ => OpenIn::Own,
        };
        match terminal.run(&place.place_ref(), &folder, &run, &identity, &at) {
            Opened::Opened { printed, .. } => {
                self.with_start(id, &identity, |start, _| {
                    start.printed.clone_from(&printed);
                });
                Start::Opened(identity)
            }
            Opened::Held { screen } | Opened::NotComeUp { screen } => {
                let reason = format!("the terminal did not run {identity}'s command: {screen}");
                self.run_ended(id, &identity, None, Some(reason));
                Start::NeverBegun(identity)
            }
            Opened::Refused { reason } => {
                let reason = format!("the terminal did not run {identity}'s command: {reason}");
                self.run_ended(id, &identity, None, Some(reason.clone()));
                Start::NotOpened(reason)
            }
        }
    }

    /// A command stage's session at `index` that could not be started,
    /// failed with `reason` as its words (223).
    fn run_failed(&self, id: &PlaceId, index: usize, reason: &str) {
        self.update(id, |held| {
            let Some(stage) = held.stages.get_mut(index) else {
                return Step::Keep(());
            };
            stage.state = StageState::Failed;
            stage.end = Some(StageEnd {
                words: Some(reason.to_string()),
                ..StageEnd::default()
            });
            Step::Write(())
        });
    }

    /// Steps the command stage's session at `index`: begun once its
    /// launcher's word stands; ended once its launcher recorded its exit,
    /// delivered where it exited with success and the place's branch holds
    /// a commit made since it started, failed otherwise; failed where its
    /// launcher found the start begun otherwise, or where its terminal
    /// reads it stopped with no exit recorded. True when it changed
    /// something (25, 35, 161, 223).
    pub(crate) fn step_run(&self, id: &PlaceId, held: &HeldState, index: usize) -> bool {
        let stage = &held.stages[index];
        if !matches!(
            stage.state,
            StageState::Starting | StageState::Started | StageState::Begun
        ) {
            return false;
        }
        let Some(start) = stage.start.clone().filter(|start| start.live) else {
            return false;
        };
        let folder = self.store.folder(id);
        let key = start.identity.to_string();
        let exited = self.store.read_file(id, &ran_exit(&folder, &key));
        if let Ok(Some(text)) = &exited {
            let exit = serde_json::from_str::<Exited>(text)
                .map(|exited| exited.exit)
                .unwrap_or(None);
            self.run_ended(id, &start.identity, Some(exit), None);
            return true;
        }
        let word = self
            .store
            .read_file(id, &launched_word(&folder, &key))
            .ok()
            .flatten();
        if let Some(word) = &word {
            let differed = serde_json::from_str::<serde_json::Value>(word)
                .ok()
                .is_some_and(|word| word.get("unsettled").is_some());
            if differed {
                self.run_ended(
                    id,
                    &start.identity,
                    None,
                    Some(format!(
                        "{} began otherwise than its start recorded, so its launcher ran nothing (161)",
                        start.identity
                    )),
                );
                return true;
            }
            if stage.state != StageState::Begun {
                let now = self.host.now();
                self.with_start(id, &start.identity, |start, state| {
                    start.begun_at = Some(now);
                    *state = StageState::Begun;
                });
                return true;
            }
        }
        let now = self.host.now();
        let unseen = word.is_none() && now < start.started_at + UNSEEN;
        if !unseen && self.agent_on(&start).state() == AgentState::Stopped {
            self.run_ended(
                id,
                &start.identity,
                None,
                Some(format!(
                    "{}'s command stopped with no exit recorded",
                    start.identity
                )),
            );
            return true;
        }
        false
    }

    /// The run under `identity` ended: with `exit` where its launcher
    /// recorded one, `Some(None)` where a signal ended it, delivered only
    /// where it exited 0 and the place's branch moved past its start's
    /// head; or failed with `failure` as its words. Its end names how its
    /// command exited and where its output lies (25, 43, 223).
    fn run_ended(
        &self,
        id: &PlaceId,
        identity: &Identity,
        exit: Option<Option<i32>>,
        failure: Option<String>,
    ) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let Some(index) = held.index_of(identity) else {
            return;
        };
        let Some(start) = held.stages[index].start.clone() else {
            return;
        };
        let place = &held.place;
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        let folder = self.store.folder(id);
        let output = ran_output(&folder, &identity.to_string());
        let committed = head != start.start_head;
        let (state, words) = match (exit, failure) {
            (_, Some(failure)) => (StageState::Failed, failure),
            (Some(Some(0)), None) if committed => (StageState::Delivered, String::new()),
            (Some(Some(0)), None) => (
                StageState::Failed,
                format!("the command exited with success and committed nothing; its output is at {output}"),
            ),
            (Some(Some(code)), None) => (
                StageState::Failed,
                format!("the command exited with {code}; its output is at {output}"),
            ),
            (Some(None), None) => (
                StageState::Failed,
                format!("the command was ended by a signal; its output is at {output}"),
            ),
            (None, None) => (
                StageState::Failed,
                format!("the command stopped with no exit recorded; its output is at {output}"),
            ),
        };
        let ran = Ran {
            exit: exit.flatten(),
            output,
        };
        // One write ends the start and records its end whole, so the end
        // delivered is the one read (43).
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(());
            };
            let stage = &mut held.stages[index];
            let Some(start) = stage.start.as_mut().filter(|start| start.live) else {
                return Step::Keep(());
            };
            start.live = false;
            start.first_quiet = None;
            stage.state = state;
            stage.end = Some(StageEnd {
                head: Some(head.clone()),
                words: (state == StageState::Failed).then(|| words.clone()),
                ran: Some(ran.clone()),
                ..StageEnd::default()
            });
            Step::Write(())
        });
    }
}
