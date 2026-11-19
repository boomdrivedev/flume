//! Delivering a stage's prompt (35, 37, 38): a start is starting until
//! its agent says it started, read begun without its settings where its
//! agent's word shows so, at its start or, for a mode its program names
//! only with a prompt, at its start's first prompt (161), and never begun
//! at once when its agent stops first, or at the stage's limit while it
//! stays starting or so begun; the
//! prompt is sent only once the agent said it started and waits and is
//! not asking, read within 60 seconds of each sending whether it was
//! received, sent once more where the terminal can send, and reported
//! never begun when no sending is received. A start that carried its
//! prompt counts its one sending from the agent's word that it started.
//! A sending is written to held state before it is made, so two settles
//! at once send it once (75). While the command opening a start's agent
//! lives, the terminal may still be bringing its program up, so nothing
//! else is sent to it or answered in it; a start whose opener is gone
//! is read as opened (34, 35). A dialog the agent's program shows a
//! person before it takes its prompt holds the start, its words kept
//! with it: the place waits on a person, and the start is never read
//! never begun while the dialog shows (34, 35). A stage read never begun
//! whose agent is still live and then says it started, or received its
//! prompt, goes on as begun: its start is the stage's after all (35,
//! 38). A start that carries its stage's conversation and would be read
//! never begun, at its agent's stop, at the stage's limit or on a prompt
//! not received, is read as a carried start that did not begin, its agent
//! sent nothing more, and its stage starts afresh (146, 179). Where the
//! agent's profile does not build received, the prompt is sent by the
//! terminal's send that watches the turn it began, and read received on
//! the terminal's word that it saw it, or where a reading within 60 seconds
//! of the sending reads the agent working or asking; such a start names no
//! record of its agent's (38, 77, 89, 171).

use crate::reading::shown;
use crate::start::Starts;
use crate::uncarried::{carries, uncarried, ENDED_FIRST, NOT_BY_THE_LIMIT, NOT_RECEIVED};
use crate::{Engine, Step, POLL, QUIET_LIMIT, RECEIVED_WITHIN};
use flume_seams::*;
use std::time::Duration;

impl Engine {
    /// Moves the current start's prompt on as far as its agent allows,
    /// reading each sending back within 60 seconds; true when the stage's
    /// state changed.
    pub(crate) fn deliver(&self, id: &PlaceId) -> bool {
        self.deliver_reading(id, true)
    }

    /// Sends the current start's prompt where its agent allows it now,
    /// and leaves its receipt to the settles that follow; true when the
    /// stage's state changed (6, 38).
    pub(crate) fn send_prompt(&self, id: &PlaceId) -> bool {
        self.deliver_reading(id, false)
    }

    /// Moves the current start's prompt on, waiting on a sending's
    /// receipt only where `read_back`.
    fn deliver_reading(&self, id: &PlaceId, read_back: bool) -> bool {
        let mut sent = false;
        let mut read_again = false;
        loop {
            let Some(read) = self.store.read(id) else {
                return sent;
            };
            let held = read.state;
            let Some(index) = held.current else {
                return sent;
            };
            let stage = &held.stages[index];
            let Some(start) = stage.start.clone().filter(|start| start.live) else {
                return sent;
            };
            if !matches!(stage.state, StageState::Starting | StageState::Started) {
                return sent;
            }
            if start.opening.as_ref().is_some_and(|by| self.host.alive(by)) {
                return sent;
            }
            let Some(agent) = self.agent(&start.program) else {
                return sent;
            };
            let identity = &start.identity;
            let prompt = &start.prompt;
            let folder = self.store.folder(id);
            // A carried start read as not begun is sent nothing more: it
            // is ended once quiet and its stage started afresh (146).
            if let Some(words) = uncarried(&held, &start) {
                return self.read_uncarried(id, index, &start, words) || sent;
            }
            // An agent begun without its settings is sent no prompt, or
            // none again where its word came at its start's first prompt,
            // which it then did not take: it is ended once quiet and its
            // stage started afresh (161).
            let unsettled = self.began_otherwise_held(&held, agent.as_ref(), &folder, &start);
            if let Some(unsettled) = &unsettled {
                if self.read_unsettled(id, index, &start, unsettled.clone()) {
                    return true;
                }
            }
            if stage.state == StageState::Starting || unsettled.is_some() {
                if unsettled.is_none()
                    && self.started(agent.as_ref(), &folder, &start) == Reading::Yes
                {
                    self.with_start(id, identity, |start, state| {
                        start.dialog = None;
                        *state = StageState::Started;
                    });
                    sent = true;
                    continue;
                }
                // The program may ask whether it may work in the place
                // after the open returned: the terminal answers it where
                // it shows it now (34, 35).
                let answered =
                    stage.state == StageState::Starting && self.agent_on(&start).answer_trust();
                let stopped =
                    self.state_of(Some(agent.as_ref()), &folder, &start) == AgentState::Stopped;
                // Any other question it shows a person holds the start,
                // which waits on them however long (34, 35).
                let dialog = match (answered, stopped) {
                    (_, true) => false,
                    (true, false) => start.dialog.is_some(),
                    (false, false) => self.read_dialog(id, &start),
                };
                let limit = held.place.kind.stages[index]
                    .quiet_limit
                    .unwrap_or(QUIET_LIMIT);
                let waited = self.host.now().saturating_sub(start.started_at);
                if stopped || (!dialog && waited >= limit) {
                    // A start read begun without its settings is read so
                    // once more before it is read never begun, from the
                    // held state as it stands now: its agent, read not
                    // quiet there, may have stopped since, so it is ended
                    // as that reading says, and the held state read
                    // before it may not hold how it began, which its end
                    // names (161).
                    if unsettled.is_some() && !read_again {
                        read_again = true;
                        continue;
                    }
                    if carries(&start) {
                        let words = if stopped {
                            ENDED_FIRST
                        } else {
                            NOT_BY_THE_LIMIT
                        };
                        return self.read_uncarried(id, index, &start, words) || sent;
                    }
                    self.never_begun(id, &held, &start, stopped);
                    return true;
                }
                return sent;
            }
            if let Some(record) = self.received_now(agent.as_ref(), &folder, &start) {
                self.begin(id, identity, record);
                return true;
            }
            let state = self.state_of(Some(agent.as_ref()), &folder, &start);
            if state == AgentState::Stopped {
                let end = self.read_out(&held, &start, false);
                self.end_start(
                    id,
                    identity,
                    |start, state| {
                        start.live = false;
                        *state = StageState::Lost;
                    },
                    Some(end),
                );
                return true;
            }
            // A dialog shown before the agent takes its prompt holds the
            // prompt: nothing is sent into it, and the prompt is never
            // read never begun while it shows (34, 35, 37).
            let dialog = self.read_dialog(id, &start);
            let now = self.host.now();
            if let Some(last) = prompt.sendings.last() {
                let waited = now.saturating_sub(*last);
                if waited < RECEIVED_WITHIN {
                    if !read_back {
                        return sent;
                    }
                    self.host.sleep(POLL.min(RECEIVED_WITHIN - waited));
                    continue;
                }
                if prompt.carried || prompt.sendings.len() >= 2 {
                    if dialog {
                        return sent;
                    }
                    if carries(&start) {
                        return self.read_uncarried(id, index, &start, NOT_RECEIVED) || sent;
                    }
                    self.never_begun(id, &held, &start, false);
                    return true;
                }
            }
            if prompt.carried {
                self.claim_sending(id, identity, prompt.sendings.len(), now);
                continue;
            }
            if dialog
                || state != AgentState::Waiting
                || self.read_asking(agent.as_ref(), &folder, &start) != Reading::No
            {
                return sent;
            }
            if !self.claim_sending(id, identity, prompt.sendings.len(), now) {
                continue;
            }
            let answer = if agent.parts().builds(Part::Received) {
                self.agent_on(&start).send(&prompt.text)
            } else {
                match self
                    .agent_on(&start)
                    .send_watching(&prompt.text, Duration::from_secs(RECEIVED_WITHIN))
                {
                    SentWatching::Began => {
                        self.begin(id, identity, None);
                        return true;
                    }
                    SentWatching::NotSeen { .. } => Sent::Sent,
                    SentWatching::Refused { reason } => Sent::Refused { reason },
                }
            };
            match answer {
                Sent::Sent => sent = true,
                Sent::Refused { reason } => {
                    let failure =
                        format!("the terminal did not send {identity} its prompt: {reason}");
                    self.never_begun(id, &held, &start, false);
                    self.update(id, |held| {
                        held.failures.push(failure.clone());
                        Step::Write(())
                    });
                    return true;
                }
            }
        }
    }

    /// The start's prompt reported never begun, with what the terminal
    /// shows of the agent, its last words and the commits since the start,
    /// and how its agent began without its settings where the place
    /// recorded so; its agent read ended where it `stopped` (35, 38, 43,
    /// 161).
    pub(crate) fn never_begun(
        &self,
        id: &PlaceId,
        held: &HeldState,
        start: &StartHeld,
        stopped: bool,
    ) {
        let mut end = self.read_out(held, start, true);
        end.unsettled = held
            .unsettled
            .iter()
            .find(|read| read.stage == start.identity.stage && read.start == start.n)
            .map(|read| Box::new(read.unsettled.clone()));
        self.end_start(
            id,
            &start.identity,
            |start, state| {
                if stopped {
                    start.live = false;
                }
                start.dialog = None;
                *state = StageState::NeverBegun;
            },
            Some(end),
        );
    }

    /// Reads whether the agent's program shows a person a dialog before
    /// it takes its prompt: where the terminal reads it asking, its words
    /// as the terminal shows them now are kept with the start, since when
    /// it was first read so; where the terminal reads it otherwise, the
    /// dialog is gone; where it cannot read it, the start keeps what it
    /// held. Whether a dialog holds the start (34, 35).
    fn read_dialog(&self, id: &PlaceId, start: &StartHeld) -> bool {
        let on = self.agent_on(start);
        let dialog = match on.state() {
            AgentState::Asking => Some(Dialog {
                words: on
                    .screen()
                    .map(|screen| shown(&screen))
                    .filter(|words| !words.is_empty())
                    .or_else(|| start.dialog.as_ref().map(|dialog| dialog.words.clone()))
                    .unwrap_or_default(),
                since: start
                    .dialog
                    .as_ref()
                    .map_or_else(|| self.host.now(), |dialog| dialog.since),
            }),
            AgentState::Unknown => start.dialog.clone(),
            _ => None,
        };
        if dialog != start.dialog {
            self.with_start(id, &start.identity, |start, _| {
                start.dialog.clone_from(&dialog);
            });
        }
        dialog.is_some()
    }

    /// A stage read never begun whose agent is still live and has since
    /// said it started, before any sending, or received its prompt, goes
    /// on as its start's: started, its prompt sent as 37 allows, or begun;
    /// its never-begun end is read no more, and what was delivered of it
    /// stays delivered. True when the stage changed (35, 38, 43).
    pub(crate) fn begun_after(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let Some(index) = held.current else {
            return false;
        };
        let stage = &held.stages[index];
        if stage.state != StageState::NeverBegun {
            return false;
        }
        let Some(start) = stage.start.clone().filter(|start| start.live) else {
            return false;
        };
        if start.opening.as_ref().is_some_and(|by| self.host.alive(by)) {
            return false;
        }
        let Some(agent) = self.agent(&start.program) else {
            return false;
        };
        let folder = self.store.folder(id);
        let identity = &start.identity;
        if self
            .began_otherwise_held(&held, agent.as_ref(), &folder, &start)
            .is_some()
        {
            return false;
        }
        let received = self.received_now(agent.as_ref(), &folder, &start);
        if received.is_none()
            && !(start.prompt.sendings.is_empty()
                && self.started(agent.as_ref(), &folder, &start) == Reading::Yes)
        {
            return false;
        }
        let now = self.host.now();
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let stage = &mut held.stages[index];
            if stage.state != StageState::NeverBegun {
                return Step::Keep(false);
            }
            let Some(start) = stage.start.as_mut().filter(|start| start.live) else {
                return Step::Keep(false);
            };
            start.dialog = None;
            stage.end = None;
            stage.state = match &received {
                Some(record) => {
                    start.prompt.received_at = Some(now);
                    if let Some(record) = record {
                        start.record = Some(record.clone());
                    }
                    start.begun_at = Some(now);
                    StageState::Begun
                }
                None => StageState::Started,
            };
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// The start under `identity` reported never begun at once, the
    /// terminal having read its program not come up, with what the
    /// terminal showed then; no prompt is sent (35, 43).
    pub(crate) fn never_begun_with(&self, id: &PlaceId, identity: &Identity, screen: String) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let Some(start) = held
            .index_of(identity)
            .and_then(|index| held.stages[index].start.clone())
        else {
            return;
        };
        let mut end = self.read_out(&held, &start, false);
        end.screen = Some(screen);
        self.end_start(
            id,
            identity,
            |start, state| {
                start.dialog = None;
                *state = StageState::NeverBegun;
            },
            Some(end),
        );
    }

    /// Whether the start's prompt was received, with the record its agent
    /// keeps where its profile names one: from its profile where it builds
    /// received; otherwise where a reading of the terminal within 60
    /// seconds of the last sending reads the agent working or asking, since
    /// it was sent only while read waiting and not asking, naming no record
    /// (38, 77, 89, 171). None where it was not.
    fn received_now(
        &self,
        agent: &dyn Agent,
        folder: &str,
        start: &StartHeld,
    ) -> Option<Option<String>> {
        if agent.parts().builds(Part::Received) {
            return match agent.received(folder, &start.identity, &start.prompt.text) {
                Received::Received { record } => Some(Some(record)),
                _ => None,
            };
        }
        let last = start.prompt.sendings.last()?;
        if self.host.now().saturating_sub(*last) > RECEIVED_WITHIN {
            return None;
        }
        matches!(
            self.state_of(Some(agent), folder, start),
            AgentState::Working | AgentState::Asking
        )
        .then_some(None)
    }

    /// The start under `identity` begun now, its prompt received, with
    /// the record its agent keeps where one was named (38, 77).
    fn begin(&self, id: &PlaceId, identity: &Identity, record: Option<String>) {
        let now = self.host.now();
        self.with_start(id, identity, |start, state| {
            start.prompt.received_at = Some(now);
            if let Some(record) = &record {
                start.record = Some(record.clone());
            }
            start.begun_at = Some(now);
            start.dialog = None;
            *state = StageState::Begun;
        });
    }

    /// Records a sending at `now`, if the start has made `made` so far,
    /// clearing when a settle read its agent waiting, since it is sent to;
    /// false when another act made it first (31, 171).
    fn claim_sending(&self, id: &PlaceId, identity: &Identity, made: usize, now: Seconds) -> bool {
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let Some(start) = held.stages[index].start.as_mut() else {
                return Step::Keep(false);
            };
            if start.prompt.sendings.len() != made {
                return Step::Keep(false);
            }
            start.prompt.sendings.push(now);
            start.waiting = None;
            Step::Write(true)
        })
        .unwrap_or(false)
    }
}
