//! Reading a stage's deliverable at one head of the place's branch (24,
//! 25), an agent's state from the terminal and the agent seam (31, 77,
//! 90, 134, 135), and what an end that read no deliverable carries (43).
//! A part of the agent seam its profile does not build, of received,
//! asking, own work, ended and last words, is read from the terminal its
//! start recorded, and the profile is asked nothing of it; one its profile
//! builds is read from the profile alone, never from the terminal (93,
//! 171; core.2, core.39).

use crate::start::Starts;
use crate::{Bound, Engine, Step, OWN_WORK_AFTER};
use flume_seams::*;

/// The facts of a place read at one head.
pub(crate) struct Facts {
    pub head: Revision,
    /// Whether the stage's deliverable exists, made since it started,
    /// and every thing it declares it makes has a line (129).
    pub delivered: bool,
    /// Whether the deliverable alone exists, made since it started.
    pub deliverable: bool,
    /// The report it delivered, for a report form.
    pub report: Option<String>,
    pub change: Option<ChangeRead>,
    /// Each thing the stage declares it makes that no line its run wrote
    /// since its start names: while one is, the stage is not delivered
    /// (129, 44).
    pub unmade: Vec<String>,
}

/// An agent's state as flume reads it.
pub(crate) struct AgentReading {
    pub state: AgentState,
    /// Whether the agent said it started under its start's identity
    /// (135).
    pub started: Reading,
    pub asking: Reading,
    pub own_work: Reading,
    /// Whether the agent under the start's identity keeps the record its
    /// start began with: no where it keeps another (77).
    pub own_record: Reading,
}

impl AgentReading {
    /// Neither working, asking, nor waiting on own work; anything flume
    /// cannot read is not quiet (31).
    pub fn quiet(&self) -> bool {
        matches!(self.state, AgentState::Waiting | AgentState::Stopped)
            && self.asking == Reading::No
            && self.own_work == Reading::No
            && self.own_record != Reading::Unknown
    }

    /// What flume could not read of the agent (31).
    pub fn unread(&self) -> Vec<String> {
        let mut unread = vec![];
        if self.state == AgentState::Unknown {
            unread.push("the agent's state".to_string());
        }
        if self.asking == Reading::Unknown {
            unread.push("whether the agent is asking".to_string());
        }
        if self.own_work == Reading::Unknown {
            unread.push("whether the agent's own work runs".to_string());
        }
        if self.own_record == Reading::Unknown {
            unread.push("the agent's own record".to_string());
        }
        unread
    }
}

/// Whether the change holds the repetition's group, found by its number
/// and title, with no task of it unticked; a group no longer found is
/// not delivered (140).
pub(crate) fn group_ticked(facts: &ChangeFacts, group: &GroupHeld) -> bool {
    find_group(facts, group).is_some_and(|at| facts.tasks_of(at).all(|task| task.ticked))
}

/// The index of the repetition's group among the change's groups, found
/// by its number and title (140).
pub(crate) fn find_group(facts: &ChangeFacts, group: &GroupHeld) -> Option<usize> {
    facts
        .groups
        .iter()
        .position(|found| found.number == group.number && found.title == group.title)
}

/// Whether a delivered start of the stage at `index` is a repetition with
/// a task of the change still unticked at the head its facts were read
/// at: then the stage repeats, and has not ended (140).
pub(crate) fn repeats_on(held: &HeldState, index: usize, facts: &Facts) -> bool {
    held.place.kind.stages[index].repeats
        && matches!(&facts.change, Some(ChangeRead::Read { facts }) if facts.tasks.iter().any(|task| !task.ticked))
}

/// Whether a kind reads a change (3, 4).
pub(crate) fn reads_change(kind: &Kind) -> bool {
    kind.stages.iter().any(|stage| {
        stage.deliverables.iter().any(|deliverable| {
            matches!(
                deliverable,
                Deliverable::ChangeWritten | Deliverable::TasksTicked | Deliverable::ChangeArchived
            )
        })
    })
}

impl Engine {
    /// The place's head, the deliverable of the stage at `index` and the
    /// place's change, every fact read at one head: if the head moved by
    /// the end of the read, every fact is read again (25).
    pub(crate) fn facts(&self, held: &HeldState, bound: &Bound, index: Option<usize>) -> Facts {
        let place = &held.place;
        let head_now = || {
            self.history
                .head(&place.repository, &place.branch)
                .unwrap_or_else(|| place.base_revision.clone())
        };
        let mut head = head_now();
        loop {
            let change = match (&bound.change, reads_change(&place.kind)) {
                (Some(change), true) => {
                    Some(change.read_change(&place.repository, &head, &place.name))
                }
                _ => None,
            };
            let (delivered, report) = match index {
                Some(index) => self.delivered(held, bound, index, &head, change.as_ref()),
                None => (false, None),
            };
            let again = head_now();
            if again == head {
                let unmade = index
                    .map(|index| self.unmade(held, index))
                    .unwrap_or_default();
                return Facts {
                    head,
                    delivered: delivered && unmade.is_empty(),
                    deliverable: delivered,
                    report,
                    change,
                    unmade,
                };
            }
            head = again;
        }
    }

    /// The path of `made.jsonl` in the place's own folder, which
    /// `{{made}}` renders (129).
    pub(crate) fn made_path(&self, place: &PlaceId) -> String {
        format!("{}/{MADE}", self.store.folder(place))
    }

    /// The lines written to `made.jsonl` since the start its deliverable
    /// counts from, each as written (25, 129).
    pub(crate) fn made_since(&self, place: &PlaceId, start: &StartHeld) -> Vec<Made> {
        self.store
            .made(place)
            .iter()
            .skip(start.counted().made_from)
            .map(|line| Made::read(line))
            .collect()
    }

    /// The lines a stage's end carries, every line written to
    /// `made.jsonl` since its stage started: from where the stage's count
    /// began, else from where a document written before kept them, else
    /// from the start's own count; each as written (129, 140).
    pub(crate) fn stage_made(&self, held: &HeldState, start: &StartHeld) -> Vec<Made> {
        let stage = held.stages.iter().find(|stage| {
            stage
                .start
                .as_ref()
                .is_some_and(|held| held.identity == start.identity)
        });
        self.store
            .made(&held.place.id)
            .iter()
            .skip(lines_from(stage, start))
            .map(|line| Made::read(line))
            .collect()
    }

    /// Each name the stage at `index` declares it makes that no line
    /// written since its current start's count began names (25, 129).
    pub(crate) fn unmade(&self, held: &HeldState, index: usize) -> Vec<String> {
        let makes = &held.place.kind.stages[index].makes;
        let Some(start) = held.stages[index].start.as_ref() else {
            return makes.clone();
        };
        if makes.is_empty() {
            return vec![];
        }
        let made = self.made_since(&held.place.id, start);
        makes
            .iter()
            .filter(|name| !made.iter().any(|line| line.name() == Some(name.as_str())))
            .cloned()
            .collect()
    }

    /// Whether the stage's deliverable exists at `head`, made since the
    /// start it counts from, the first of the stage's starts since it last
    /// delivered, and the report it is, for a report form (24, 25).
    fn delivered(
        &self,
        held: &HeldState,
        bound: &Bound,
        index: usize,
        head: &Revision,
        change: Option<&ChangeRead>,
    ) -> (bool, Option<String>) {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let Some(start) = held.stages[index].start.as_ref() else {
            return (false, None);
        };
        let counted = start.counted();
        let since = &counted.head;
        let facts = match change {
            Some(ChangeRead::Read { facts }) => Some(facts),
            _ => None,
        };
        let touched = || {
            bound.change.as_ref().is_some_and(|change| {
                self.history
                    .newest_touching(&place.repository, head, since, &change.path(&place.name))
                    .is_some_and(|touch| touch.since)
            })
        };
        match stage.deliverables.first() {
            None | Some(Deliverable::Unreadable { .. }) => (false, None),
            Some(Deliverable::Commits) => (
                self.history.holds_since(&place.repository, head, since),
                None,
            ),
            Some(Deliverable::ChangeWritten) => (
                facts.is_some_and(|facts| facts.exists && facts.planning_complete) && touched(),
                None,
            ),
            Some(Deliverable::TasksTicked) => {
                let ticked = match start.group.as_ref().filter(|_| stage.repeats) {
                    Some(group) => facts.is_some_and(|facts| group_ticked(facts, group)),
                    None => facts.is_some_and(|facts| {
                        !facts.tasks.is_empty() && facts.tasks.iter().all(|task| task.ticked)
                    }),
                };
                (ticked && touched(), None)
            }
            Some(Deliverable::ChangeArchived) => (
                facts.is_some_and(|facts| facts.archived)
                    && self.history.holds_since(&place.repository, head, since),
                None,
            ),
            Some(Deliverable::Report { path }) => {
                let path = self.report_path(place, path);
                let written = self
                    .host
                    .write_time(&path)
                    .is_some_and(|written| written > counted.at);
                (written, written.then_some(path))
            }
            Some(Deliverable::CommittedReport { path }) => {
                let committed = self
                    .history
                    .in_commit_since(&place.repository, head, since, path);
                (committed, committed.then(|| path.clone()))
            }
        }
    }

    /// Whether the agent of `start` started, its record read in the
    /// place's own `folder`: the profile's own answer where it starts its
    /// program itself, or where that answer is not yes; for one that starts
    /// it through flume's launcher, otherwise, yes only where the start's
    /// terminal reads the program running in the process the launcher's
    /// word names and, for a start not yet sent its prompt nor begun, reads
    /// the agent waiting; no where it reads nothing running the program, it
    /// running in another process, the word naming none, or the agent not
    /// yet waiting; and unknown where the terminal cannot read the process,
    /// so the start stays starting and is never read failed for it (35,
    /// 135, 171, C.4; core.10). Every reading of a start's started is this
    /// one, so a settle, a message and the stop check read it alike.
    pub(crate) fn started(&self, agent: &dyn Agent, folder: &str, start: &StartHeld) -> Reading {
        let said = agent.started(folder, &start.identity);
        if !agent.launches() || said != Reading::Yes {
            return said;
        }
        let Some(launched) = agent.launched(folder, &start.identity) else {
            return Reading::No;
        };
        match self.agent_on(start).program_process() {
            ProgramProcess::Runs { pid } if pid == launched => {
                let taken = !start.prompt.sendings.is_empty() || start.begun_at.is_some();
                if taken || self.state_of(Some(agent), folder, start) == AgentState::Waiting {
                    Reading::Yes
                } else {
                    Reading::No
                }
            }
            ProgramProcess::Runs { .. } | ProgramProcess::Nothing => Reading::No,
            ProgramProcess::Unknown { .. } => Reading::Unknown,
        }
    }

    /// How the agent of `start` began otherwise than its start recorded,
    /// its record read in the place's own `folder`: the profile's own
    /// answer; else, for a profile that starts its program through flume's
    /// launcher whose start's word names a process, that it runs in
    /// another process where the terminal reads the program running in
    /// another than the word names, as a person running it again by hand
    /// in the pane starts it; none otherwise (161, S74; core.10). Every
    /// reading of how a start began is this one, so a settle and the stop
    /// check read it alike.
    pub(crate) fn began_otherwise(
        &self,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
    ) -> Option<Unsettled> {
        if let Some(unsettled) = agent.started_without(folder, &start.identity) {
            return Some(unsettled);
        }
        if !agent.launches() {
            return None;
        }
        let launched = agent.launched(folder, &start.identity)?;
        match self.agent_on(start).program_process() {
            ProgramProcess::Runs { pid } if pid != launched => Some(Unsettled {
                process: true,
                ..Unsettled::default()
            }),
            _ => None,
        }
    }

    /// How the agent of `start` began otherwise, as
    /// [`Engine::began_otherwise`] reads it now, else as `held` recorded a
    /// reading of its program in another process than its launcher became,
    /// which a later reading of the terminal need not repeat: the start is
    /// still no start of its stage's agent, and is ended once quiet (161,
    /// S74).
    pub(crate) fn began_otherwise_held(
        &self,
        held: &HeldState,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
    ) -> Option<Unsettled> {
        self.began_otherwise(agent, folder, start).or_else(|| {
            held.unsettled
                .iter()
                .find(|read| {
                    read.stage == start.identity.stage
                        && read.start == start.n
                        && read.unsettled.process
                })
                .map(|read| read.unsettled.clone())
        })
    }

    /// The agent of a place's start, as the terminal and the agent seam
    /// read it, the agent's record read in the place's own folder (128):
    /// its asking as [`Engine::read_asking`] reads it, from the state read
    /// here where its profile does not build asking, and its own work as
    /// [`Engine::read_own_work`] reads it (31, 90, 171).
    pub(crate) fn read_agent(&self, place: &PlaceId, start: &StartHeld) -> AgentReading {
        let agent = self.agent(&start.program);
        let folder = self.store.folder(place);
        let state = self.state_of(agent.as_deref(), &folder, start);
        let own_record = own_record(agent.as_deref(), &folder, start);
        match agent {
            Some(agent) => AgentReading {
                state,
                started: self.started(agent.as_ref(), &folder, start),
                asking: if agent.parts().builds(Part::Asking) {
                    agent.asking(&folder, &start.identity)
                } else {
                    asking_of(state)
                },
                own_work: self.read_own_work(agent.as_ref(), &folder, start, state),
                own_record,
            },
            None => AgentReading {
                state,
                started: Reading::Unknown,
                asking: Reading::Unknown,
                own_work: Reading::Unknown,
                own_record,
            },
        }
    }

    /// The agent's state: the reading of the terminal its start opened
    /// it in, but asking only where the agent profile's mark says it
    /// asks, or where its profile does not build asking, so the terminal's
    /// reading is its asking, and unknown otherwise, never waiting, so
    /// nothing is typed into a dialog flume cannot name (90, 171, 190,
    /// core.2); and stopped where the terminal cannot read the agent and
    /// its own record says its session ended, read only where its profile
    /// builds ended, so an agent whose profile does not is read stopped by
    /// its terminal alone (134, 171).
    pub(crate) fn state_of(
        &self,
        agent: Option<&dyn Agent>,
        folder: &str,
        start: &StartHeld,
    ) -> AgentState {
        let identity = &start.identity;
        match (self.agent_on(start).state(), agent) {
            (AgentState::Asking, Some(agent))
                if !agent.parts().builds(Part::Asking)
                    || agent.asking(folder, identity) == Reading::Yes =>
            {
                AgentState::Asking
            }
            (AgentState::Asking, _) => AgentState::Unknown,
            (AgentState::Unknown, Some(agent))
                if agent.parts().builds(Part::Ended)
                    && agent.ended(folder, identity) == Reading::Yes =>
            {
                AgentState::Stopped
            }
            (state, _) => state,
        }
    }

    /// Whether the agent of `start` waits on a question it put to a
    /// person: its profile's answer from its own record where it builds
    /// asking; otherwise the terminal's reading, asking as yes, working,
    /// waiting and stopped as no and unknown as unknown, the profile asked
    /// nothing (90, 171; core.2, core.39). Every reading of an agent's
    /// asking is this one, so a settle, a message and the stop check read it
    /// alike.
    pub(crate) fn read_asking(
        &self,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
    ) -> Reading {
        if agent.parts().builds(Part::Asking) {
            return agent.asking(folder, &start.identity);
        }
        asking_of(self.agent_on(start).state())
    }

    /// Whether own work the agent of `start` started still runs: its
    /// profile's answer from its own record where it builds own work;
    /// otherwise from `state`, the agent's state as the core read it, and
    /// the settles' readings of it waiting its start holds: none where it is
    /// read stopped, or waiting with the last settle's reading a minute or
    /// more after the first; running where it is read working, asking, or
    /// waiting short of that; unknown where its state is unknown, the
    /// profile asked nothing (31, 91, 171; core.39). Every reading of an
    /// agent's own work is this one, so a settle, status and the stop check
    /// read it alike, and only a settle writes what it reads.
    pub(crate) fn read_own_work(
        &self,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
        state: AgentState,
    ) -> Reading {
        if agent.parts().builds(Part::OwnWork) {
            return agent.own_work(folder, &start.identity);
        }
        match state {
            AgentState::Stopped => Reading::No,
            AgentState::Waiting => match start.waiting {
                Some(WaitingRead { since, at }) if at.saturating_sub(since) >= OWN_WORK_AFTER => {
                    Reading::No
                }
                _ => Reading::Yes,
            },
            AgentState::Working | AgentState::Asking => Reading::Yes,
            AgentState::Unknown => Reading::Unknown,
        }
    }

    /// A settle's reading of the place's live start whose agent's profile
    /// does not build own work: where its agent is read waiting, when a
    /// settle first read it so, kept, or now where none did, and now as
    /// the last; any other reading clears it; nothing is written where
    /// nothing changed, and a start whose profile builds own work is not
    /// read (31, 171; core.16, core.17). Run by a settle alone, so status,
    /// the list and a wait write nothing of it (core.52).
    pub(crate) fn read_waiting(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let Some((_, start)) = read.state.live_start() else {
            return;
        };
        let Some(agent) = self.agent(&start.program) else {
            return;
        };
        if agent.parts().builds(Part::OwnWork) {
            return;
        }
        let folder = self.store.folder(id);
        let waiting = self.state_of(Some(agent.as_ref()), &folder, start) == AgentState::Waiting;
        let now = self.host.now();
        let identity = start.identity.clone();
        self.update(id, |held| {
            let Some(index) = held.index_of(&identity) else {
                return Step::Keep(());
            };
            let Some(start) = held.stages[index].start.as_mut().filter(|start| start.live) else {
                return Step::Keep(());
            };
            let read = waiting.then(|| WaitingRead {
                since: start.waiting.map_or(now, |kept| kept.since),
                at: now,
            });
            if read == start.waiting {
                return Step::Keep(());
            }
            start.waiting = read;
            Step::Write(())
        });
    }

    /// The agent's last words: its profile's answer from its own record
    /// where it builds last words; otherwise what the terminal shows of
    /// the agent, trimmed as a dialog's words are, none where it shows
    /// nothing, the profile asked nothing (43, 136, 171; core.39). Every
    /// reading of an agent's last words is this one, so an end and status
    /// read them alike.
    pub(crate) fn read_last_words(
        &self,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
    ) -> Option<String> {
        if agent.parts().builds(Part::LastWords) {
            return agent.last_words(folder, &start.identity);
        }
        self.agent_on(start)
            .screen()
            .map(|screen| shown(&screen))
            .filter(|words| !words.is_empty())
    }

    /// What a stuck, lost or never-begun end carries: the agent's last
    /// words as its own record holds them, the commits on the place's
    /// branch since the start, for a prompt never begun what the
    /// terminal shows of the agent, and a repetition's task group (43,
    /// 142).
    pub(crate) fn read_out(
        &self,
        held: &HeldState,
        start: &StartHeld,
        never_begun: bool,
    ) -> StageEnd {
        let place = &held.place;
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| start.start_head.clone());
        let folder = self.store.folder(&place.id);
        StageEnd {
            commits: self
                .history
                .commits_since(&place.repository, &head, &start.start_head),
            head: Some(head),
            last_words: self
                .agent(&start.program)
                .and_then(|agent| self.read_last_words(agent.as_ref(), &folder, start)),
            screen: never_begun.then(|| self.agent_on(start).screen()).flatten(),
            made: self.stage_made(held, start),
            group: start.group.clone().map(Box::new),
            ..StageEnd::default()
        }
    }
}

/// Whether an agent the terminal reads in `state` asks a person, for a
/// profile that does not build asking (90, 171).
fn asking_of(state: AgentState) -> Reading {
    match state {
        AgentState::Asking => Reading::Yes,
        AgentState::Working | AgentState::Waiting | AgentState::Stopped => Reading::No,
        AgentState::Unknown => Reading::Unknown,
    }
}

/// What a terminal shows, each line's trailing blanks and the blank lines
/// around it taken off, so a dialog's words and an agent's last words read
/// as it draws them.
pub(crate) fn shown(screen: &str) -> String {
    screen
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_matches('\n')
        .to_string()
}

/// How many lines of `made.jsonl` a stage's end passes over: the count
/// a document written before kept for a repeating stage between its
/// repetitions, else where the stage's count began, else where the
/// start's began, so the end carries every line since the stage started
/// (129, 140).
pub(crate) fn lines_from(stage: Option<&StageHeld>, start: &StartHeld) -> usize {
    stage
        .and_then(|stage| {
            stage
                .made_from
                .or_else(|| stage.counts.as_ref().map(|counts| counts.made_from))
        })
        .unwrap_or_else(|| start.counted().made_from)
}

/// Whether the agent under the start's identity keeps the record its
/// start began with: an agent the terminal resumed is the stage's only
/// when both its identity and its own record are its start's, and a
/// record the agent seam cannot read is unknown, never taken as the
/// start's (77, 93). A start whose prompt no reading has named received
/// keeps no record yet, and has none to differ from.
fn own_record(agent: Option<&dyn Agent>, folder: &str, start: &StartHeld) -> Reading {
    let Some(kept) = &start.record else {
        return Reading::Yes;
    };
    let Some(agent) = agent else {
        return Reading::Unknown;
    };
    match agent.received(folder, &start.identity, &start.prompt.text) {
        Received::Received { record } | Received::NotReceived { record } if &record == kept => {
            Reading::Yes
        }
        Received::Received { .. } | Received::NotReceived { .. } => Reading::No,
        Received::Unknown => Reading::Unknown,
    }
}
