//! Hibernating a stage (212; record 0069; design 6): an act of its own
//! beside the stage's state machine, taken only for a stage whose kind
//! hibernates it. Its agent is ended once it is quiet, or, at its
//! caller's forced word, working (31), through the terminal its start
//! opened it in, and its session is kept: the start reads not live, so
//! its room and its account are free (70, 145), and records since when
//! it hibernated, why and the account it ran under. A running stage then
//! reads hibernating, its end none, neither ended nor moved on; a stage
//! ended short whose agent was kept open for its answer stays ended
//! short, waiting on its answer, its close recorded hibernated (42, 198).
//! Nothing counts a start, a retry or a going back (48).
//!
//! One step, used by a settle and by the caller's act: the hibernation
//! is claimed on the start in one compare-and-set write before the agent
//! is closed, so two settles at once close it once and record it once;
//! the agent is then closed, a close the terminal refuses a failed end
//! and the claim let go; and the start is then read not live in one more
//! write. A claim a settle finds standing on a live start, its close or
//! its last write not made, is carried on by that settle once the agent
//! is quiet, so every step is safe to repeat (31, 74, 75; core.17).
//!
//! A resume (213, 214): its caller's `flume resume`, a message, the
//! answer the hibernated agent waits on, or a batch pending at the
//! address of a role whose run's stage hibernates, reached by the
//! address's wake or by a settle handing the batch, puts the stage
//! waiting for room to carry
//! the hibernated start's conversation, as a lost conversation's retry does,
//! with the resume's words, and starts it: one fresh agent under an
//! identity of its own, carrying the session where its profile can, on
//! the account the hibernated agent ran under while it is usable, and
//! afresh otherwise (30, 49, 70, 145, 179, 207; core.10, core.32).

use crate::acts::{asked, waited_key, Answering};
use crate::open_sessions::waits_on_answer;
use crate::start::{Start, StartReason};
use crate::{Answer, Engine, Step, MESSAGE_BOUND};
use flume_seams::*;

/// What a hibernate of a place's current stage finds: the stage's index
/// and its live start, or why it is refused.
struct Due {
    index: usize,
    start: StartHeld,
}

impl Engine {
    /// The caller's hibernate of the place's current stage (212): taken
    /// for a stage whose kind hibernates it with a live agent, running or
    /// kept open for its answer, its agent ended once quiet, one not quiet
    /// only when `force`; refused, naming the place, for a stage whose
    /// kind does not hibernate it and for a place with no live agent, and
    /// naming the agent where it is not quiet, or the message where one
    /// is being handed to it, and the caller did not force it (54). The
    /// place is then settled, so a stage waiting for the room it freed
    /// starts (70).
    pub fn hibernate(&self, place: &PlaceId, force: bool) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        let index = held.current.unwrap_or(0);
        let declared = held.place.kind.stages.get(index);
        if !declared.is_some_and(|stage| stage.hibernates) {
            let stage = declared.map_or("", |stage| stage.name.as_str());
            return Answer::refused(format!(
                "the place {place}'s kind {} does not hibernate its stage {stage} (212)",
                held.place.kind.name
            ));
        }
        let due = match due(&held, place) {
            Ok(due) => due,
            Err(reason) => return Answer::refused(reason),
        };
        match self.hibernate_start(place, &due, HibernatedBy::Caller, force) {
            Ok(_) => {
                self.settle_place(place);
                self.acted(Act::Hibernate, place)
            }
            Err(reason) => Answer::refused(reason),
        }
    }

    /// The caller's resume of the place's hibernating stage, with flume's
    /// words that its session resumes and what ran in it then has stopped
    /// as its prompt's words; refused, naming the place, where nothing of
    /// it hibernates, or where its agent hibernated waiting on its answer,
    /// which a message or a retry with words gives (42, 213).
    pub fn resume(&self, place: &PlaceId) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let index = held.current.unwrap_or(0);
        let stage = &held.stages[index];
        if held.phase == Phase::Live && stage.state == StageState::Hibernating {
            return match self.resume_stage(
                place,
                index,
                ResumedBy::Caller,
                Some(StartReason::Woken.words()),
                &|_| true,
            ) {
                Ok(()) => self.acted(Act::Resume, place),
                Err(reason) => Answer::refused(reason),
            };
        }
        let answer_resumes = stage.state == StageState::EndedShort
            && stage
                .end
                .as_ref()
                .and_then(|end| end.closed)
                .is_some_and(|closed| closed.by == ClosedBy::Hibernated);
        if held.phase == Phase::Live && answer_resumes {
            return Answer::refused(format!(
                "the stage {} of the place {place} hibernated waiting on its answer, which resumes it: flume message {place} <answer>, or flume retry {place} --words <answer> (213)",
                stage.name
            ));
        }
        Answer::refused(format!(
            "nothing of the place {place} hibernates, so there is nothing to resume (213)"
        ))
    }

    /// The answer `words` to the stop-short the place's stage at `index`
    /// waits on, whose agent hibernated and was ended: claimed on its
    /// stop-short, its key checked as `answering` names it, then, in the
    /// one write that puts the stage waiting to resume its session with
    /// the question then the answer as its words, recorded on the
    /// stop-short with `act`, who gave it, its key and when, the start's
    /// count of answers raised and each hold on the stage ended (42, 49,
    /// 53 to 55, 213).
    pub(crate) fn answer_hibernated(
        &self,
        place: &PlaceId,
        index: usize,
        words: &str,
        act: Act,
        answering: &Answering,
    ) -> Answer {
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let Some(key) = waited_key(&held) else {
            return Answer::refused(format!("the place {place} waits on no answer"));
        };
        let stage = &held.stages[index];
        let question = stage
            .end
            .as_ref()
            .and_then(|end| end.words.clone())
            .unwrap_or_default();
        let name = stage.name.clone();
        let _claimed = match self.claim_answer(place, &key, answering, MESSAGE_BOUND) {
            Ok(claimed) => claimed,
            Err(reason) => return Answer::refused(reason),
        };
        let answer = AnswerHeld {
            words: words.to_string(),
            act,
            at: self.host.now(),
            session: None,
            by: answering.by.clone(),
            key: answering.key.clone(),
        };
        let record = |held: &mut HeldState| -> bool {
            let Some(end) = held
                .ends
                .iter_mut()
                .find(|end| end.answer.is_none() && end.of.end_key() == key)
            else {
                return false;
            };
            end.answer = Some(answer.clone());
            end.claim = None;
            if let Some(start) = held.stages[index].start.as_mut() {
                start.answered += 1;
            }
            let on = HoldOn::Stage {
                stage: name.clone(),
            };
            held.holds.retain(|hold| hold.on != on);
            true
        };
        match self.resume_stage(
            place,
            index,
            ResumedBy::Answer,
            Some(asked(&question, words)),
            &record,
        ) {
            Ok(()) => self.acted(act, place),
            Err(reason) => Answer::refused(reason),
        }
    }

    /// A message's words to the place's hibernating stage, its resume's
    /// words: taken at once and answered with the place's status (53, 213).
    pub(crate) fn message_hibernating(&self, place: &PlaceId, index: usize, words: &str) -> Answer {
        match self.resume_stage(
            place,
            index,
            ResumedBy::Message,
            Some(words.to_string()),
            &|_| true,
        ) {
            Ok(()) => self.acted(Act::Message, place),
            Err(reason) => Answer::refused(reason),
        }
    }

    /// Puts the hibernated stage at `index` waiting for room to carry its
    /// start's conversation, resumed by `by`, with `words` as the resume's
    /// prompt's words, none for a resume by a delivery, whose prompt is
    /// its address's batch as its start claims it, and starts it; why not
    /// where the terminal did not open it or its prompt or settings
    /// refused it (49, 70, 213, 214). What `record` writes, an answer on
    /// the stop-short it answers, is written in the same write, which is
    /// not made where it records nothing (42, 75).
    pub(crate) fn resume_stage(
        &self,
        place: &PlaceId,
        index: usize,
        by: ResumedBy,
        words: Option<String>,
        record: &dyn Fn(&mut HeldState) -> bool,
    ) -> Result<(), String> {
        let room = self.host.room();
        let order = self.queue().next_order();
        let woken = self
            .update(place, |held| {
                if held.phase != Phase::Live
                    || held.current != Some(index)
                    || !matches!(
                        held.stages[index].state,
                        StageState::Hibernating | StageState::EndedShort
                    )
                {
                    return Step::Keep(false);
                }
                if !record(held) {
                    return Step::Keep(false);
                }
                let stage = &mut held.stages[index];
                let Some(start) = stage.start.as_ref().filter(|start| !start.live) else {
                    return Step::Keep(false);
                };
                let carry = CarryFrom {
                    start: start.identity.clone(),
                    cause: CarryCause::Hibernated,
                    carry: true,
                    why: None,
                    resumed_by: Some(by),
                };
                stage.state = StageState::WaitingForRoom;
                stage.room = Some(RoomWait {
                    order,
                    words: words.clone(),
                    carry: Some(carry),
                    ..RoomWait::default()
                });
                held.current = Some(index);
                Step::Write(true)
            })
            .unwrap_or(false);
        drop(room);
        if !woken {
            return Err(format!(
                "the place {place} moved before its hibernating stage was resumed"
            ));
        }
        match self.start_stage(place, index, words) {
            Start::Opened(_) | Start::Held(_) | Start::NeverBegun(_) | Start::WaitingForRoom => {
                Ok(())
            }
            Start::Refused(part) => Err(format!(
                "the resumed stage's prompt asks for {}, which has nothing to fill it",
                part.name()
            )),
            Start::AgentLive => Err(format!("an agent of the place {place} is live")),
            Start::NotOpened(reason) => {
                Err(format!("the terminal did not open the agent: {reason}"))
            }
            Start::Unresolved(reason) => Err(reason),
            Start::NoPlace => Err(format!("no place {place} stands on this host")),
        }
    }

    /// The resume by a delivery of the run of the role's address
    /// `address`, where its run stands with its current stage
    /// hibernating and a batch is pending at the address: the stage put
    /// waiting for room with no words of its own, carrying its
    /// conversation as any resume does, and started, its start claiming
    /// the address's batch as its prompt; the place resumed, none where
    /// nothing was. A stage ended short whose agent hibernated while it
    /// was kept open waits on its answer and takes no delivery, its batch
    /// pending until the answer resumes it and its agent next waits (42,
    /// 132, 213, 214; core.10, core.32).
    pub(crate) fn resume_by_delivery(&self, address: &str) -> Option<PlaceId> {
        let (id, index) = self.store.read_every().into_iter().find_map(|read| {
            let held = read.state;
            let index = held.current?;
            (held.role.as_deref() == Some(address)
                && held.phase == Phase::Live
                && held.stages[index].state == StageState::Hibernating)
                .then(|| (held.place.id.clone(), index))
        })?;
        if !self.batch_pending(address) {
            return None;
        }
        match self.resume_stage(&id, index, ResumedBy::Delivery, None, &|held| {
            held.stages[index].state == StageState::Hibernating
        }) {
            Ok(()) => Some(id),
            Err(why) => {
                self.host.log(&format!(
                    "the run of {address} was not resumed by its delivery: {why}"
                ));
                None
            }
        }
    }

    /// Hibernates the place's current stage, as a settle does, for `why`:
    /// its agent ended once quiet, or working where `force`; true where
    /// this call recorded the hibernation, false where another had or the
    /// place moved on; refused where the agent is not quiet and not forced,
    /// a message is being handed to it, or the terminal did not end it
    /// (31, 212).
    pub(crate) fn hibernate_stage(
        &self,
        id: &PlaceId,
        why: HibernatedBy,
        force: bool,
    ) -> Result<bool, String> {
        let Some(read) = self.store.read(id) else {
            return Ok(false);
        };
        match due(&read.state, id) {
            Ok(due) => self.hibernate_start(id, &due, why, force),
            Err(_) => Ok(false),
        }
    }

    /// Settles the place as its agent's own quiet starts a settle: a
    /// running conversation whose kind hibernates it has its quiet started
    /// again first, since a turn ended then that no settle need have read
    /// (44, 121, 212).
    pub fn settle_on_quiet(&self, place: &PlaceId) -> Answer {
        self.update(place, |held| {
            let Some(index) = held.current else {
                return Step::Keep(());
            };
            let declared = &held.place.kind.stages[index];
            if !(declared.conversation && declared.hibernates) {
                return Step::Keep(());
            }
            match held.stages[index].start.as_mut() {
                Some(start) if start.live && start.first_quiet.is_some() => {
                    start.first_quiet = None;
                    Step::Write(())
                }
                _ => Step::Keep(()),
            }
        });
        self.settle(Some(place))
    }

    /// The step itself for the start `due` holds: claimed, closed, then
    /// recorded not live (design 6).
    fn hibernate_start(
        &self,
        id: &PlaceId,
        due: &Due,
        why: HibernatedBy,
        force: bool,
    ) -> Result<bool, String> {
        let identity = due.start.identity.clone();
        let now = self.host.now();
        if let Some(until) = due.start.handing.filter(|until| now < *until && !force) {
            return Err(format!(
                "a message is being handed to the agent {identity}, until {until} at the latest; ask again once it ends, or forced to hibernate it"
            ));
        }
        let reading = self.read_agent(id, &due.start);
        let stopped = reading.state == AgentState::Stopped;
        if !stopped && !reading.quiet() && !force {
            return Err(format!(
                "the agent {identity} is not quiet; ask again forced to hibernate it"
            ));
        }
        let claimed = self
            .update(id, |held| {
                let Some(start) = current_start(held, due.index, &identity) else {
                    return Step::Keep(false);
                };
                if !start.live || start.hibernated.is_some() {
                    return Step::Keep(false);
                }
                start.hibernated = Some(HibernationHeld {
                    since: now,
                    why,
                    account: start.account.as_ref().map(|account| account.name.clone()),
                });
                let stage = &mut held.stages[due.index];
                if stage.state == StageState::EndedShort {
                    if let Some(end) = stage.end.as_mut().filter(|end| end.closed.is_none()) {
                        end.closed = Some(SessionClosed {
                            at: now,
                            by: ClosedBy::Hibernated,
                            ended_at: None,
                            left_in_pane: false,
                        });
                    }
                }
                Step::Write(true)
            })
            .unwrap_or(false);
        if !claimed {
            return Ok(false);
        }
        self.end_hibernated(id, due, !stopped)
    }

    /// Ends the agent of the start `due` holds, whose hibernation is
    /// claimed, where `close` says it still runs, and records the start
    /// not live; a close the terminal refuses is a failed end, and lets
    /// go of the claim (31, 212).
    fn end_hibernated(&self, id: &PlaceId, due: &Due, close: bool) -> Result<bool, String> {
        let identity = &due.start.identity;
        if close {
            if let Closed::Refused { reason } = self.agent_on(&due.start).close() {
                self.failed_end(id, identity, &reason);
                self.update(id, |held| match current_start(held, due.index, identity) {
                    Some(start) if start.live => {
                        start.hibernated = None;
                        if let Some(end) = held.stages[due.index].end.as_mut() {
                            if end
                                .closed
                                .is_some_and(|closed| closed.by == ClosedBy::Hibernated)
                            {
                                end.closed = None;
                            }
                        }
                        Step::Write(())
                    }
                    _ => Step::Keep(()),
                });
                return Err(format!("the agent {identity} did not end: {reason}"));
            }
        }
        let now = self.host.now();
        Ok(self
            .update(id, |held| {
                let Some(start) = current_start(held, due.index, identity) else {
                    return Step::Keep(false);
                };
                if !start.live || start.hibernated.is_none() {
                    return Step::Keep(false);
                }
                start.live = false;
                start.first_quiet = None;
                start.past_limit = false;
                start.handing = None;
                let stage = &mut held.stages[due.index];
                if stage.state == StageState::EndedShort {
                    if let Some(closed) = stage.end.as_mut().and_then(|end| end.closed.as_mut()) {
                        closed.ended_at.get_or_insert(now);
                    }
                } else {
                    stage.state = StageState::Hibernating;
                    stage.end = None;
                }
                Step::Write(true)
            })
            .unwrap_or(false))
    }

    /// A settle's step for a start whose hibernation was claimed and whose
    /// agent was not yet recorded ended: carried on once the agent is
    /// quiet or stopped, never forced; true when it changed something
    /// (31, 74, 212).
    pub(crate) fn carry_hibernation_on(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let Some(index) = read.state.current else {
            return false;
        };
        let Some(start) = read.state.stages[index]
            .start
            .clone()
            .filter(|start| start.live && start.hibernated.is_some())
        else {
            return false;
        };
        let reading = self.read_agent(id, &start);
        let stopped = reading.state == AgentState::Stopped;
        if !stopped && !reading.quiet() {
            return false;
        }
        self.end_hibernated(id, &Due { index, start }, !stopped)
            .unwrap_or(false)
    }
}

/// The index of the place's current stage where it hibernates (212).
pub(crate) fn hibernating(held: &HeldState) -> Option<usize> {
    let index = held.current?;
    (held.phase == Phase::Live && held.stages[index].state == StageState::Hibernating)
        .then_some(index)
}

/// The index of the place's current stage where it waits on its answer and
/// its agent kept open for it hibernated and was ended, so its answer
/// resumes its session (42, 198, 213).
pub(crate) fn hibernated_waiting(held: &HeldState) -> Option<usize> {
    waits_on_answer(held)?;
    let index = held.current?;
    let stage = &held.stages[index];
    let closed = stage.end.as_ref()?.closed?;
    let ended = stage.start.as_ref().is_some_and(|start| !start.live);
    (closed.by == ClosedBy::Hibernated && closed.ended_at.is_some() && ended).then_some(index)
}

/// Whether the place's current start's hibernation is claimed and its
/// agent not yet recorded ended, which a settle carries on (212).
pub(crate) fn hibernation_claimed(held: &HeldState) -> bool {
    held.current
        .and_then(|index| held.stages[index].start.as_ref())
        .is_some_and(|start| start.live && start.hibernated.is_some())
}

/// The current stage at `index` and its live start under `identity`, as a
/// write decides it: none where the place moved on (75).
fn current_start<'a>(
    held: &'a mut HeldState,
    index: usize,
    identity: &Identity,
) -> Option<&'a mut StartHeld> {
    if held.phase != Phase::Live || held.current != Some(index) {
        return None;
    }
    held.stages[index]
        .start
        .as_mut()
        .filter(|start| &start.identity == identity)
}

/// The place's current stage and its live start where a hibernate takes
/// it: a stage whose agent runs, or one ended short whose agent is kept
/// open for its answer and not let go; otherwise why not, naming the
/// place (212).
fn due(held: &HeldState, place: &PlaceId) -> Result<Due, String> {
    let index = held.current.unwrap_or(0);
    let stage = &held.stages[index];
    let Some(start) = stage.start.clone().filter(|start| start.live) else {
        return Err(format!("the place {place} has no live agent"));
    };
    let kept_open = stage.state == StageState::EndedShort
        && stage
            .end
            .as_ref()
            .is_some_and(|end| end.open_until.is_some() && end.closed.is_none());
    if !stage.state.running() && !kept_open {
        return Err(format!(
            "the place {place}'s stage {} is {}, and only a stage whose agent runs or is kept open for its answer hibernates (212)",
            stage.name,
            stage.state.word()
        ));
    }
    if start.hibernated.is_some() {
        return Err(format!(
            "the place {place}'s agent {} is hibernating already",
            start.identity
        ));
    }
    Ok(Due { index, start })
}
