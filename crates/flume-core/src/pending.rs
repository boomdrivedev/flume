//! What is pending on a place (225; core.62): something is pending while
//! a change that calls flume nothing may have moved the place. flume alone
//! reads it, from the place's held state, the host's deliveries as one
//! read of them gives them and the host's clock, asking no seam: no merge
//! queue, terminal, agent program, place or change. A watch's settle, and
//! the settle an acting command or a wait runs first where none keeps
//! watch, settle only the places it reads pending (119, 208).
//!
//! Each reason mirrors the step of the settle that acts on it, and tests
//! what that step tests before it acts: `step` for the phases, a live
//! start and the submission, `forecast_base` for a base place, the stall's
//! reading for the times, and the delivering of ends, the recording of a
//! notice's claim and the waking of roles for the hand-overs. A live place
//! whose move, read again, waits on no caller, with no stage waiting for
//! room, is pending too, so a state no reason foresaw is settled as before
//! rather than left (design 1).

use crate::base::{base_place, submission_open};
use crate::held::read_stall;
use crate::start::Starts;
use crate::status::waiting_on;
use flume_seams::{EndHeld, EndOf, HeldState, Owing, Phase, Seconds, StageState};

/// Why something is pending on a place, one of a closed set (225; design
/// 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pending {
    /// It is being made.
    Making,
    /// Its work is over, and it is not yet tidied away.
    Over,
    /// Its document, kept once tidied for its ends, is to be written again
    /// or let go: the recipients its ends stand pending for differ from
    /// those it holds, or none are (62).
    Kept,
    /// A stage of it starts or runs: a start held live, whose agent may
    /// have ended with no word, never received its prompt, worked past its
    /// limit or spent its account; or an agent kept open for its answer
    /// whose letting go is not yet recorded whole (38, 45, 118, 146, 198).
    Runs,
    /// Its submission is in flight: made with no outcome held, or due with
    /// no hold on it and no live start (111, 124).
    Submission,
    /// It is a base place, forecast at each settle (202, 203).
    Base,
    /// A time it waits on has come: the move it waits on its caller for
    /// waited its stall's bound, a hold on that move passed its bound, or
    /// a stall an earlier flume recorded has its answerer unread (130).
    Time,
    /// An end of it, or a wake for it, waits to be handed on: an end with
    /// recipients whose delivery the host's deliveries do not hold yet, a
    /// notice answered sent whose hand-over they lack, or an end a notice
    /// carries pending for a recipient nothing handed it to, a role's
    /// address among them (43, 132).
    Handing,
    /// It waits on nothing: a live place whose move, read again, waits on
    /// no caller and no room, so a step is the settle's to take.
    Nothing,
}

/// Why something is pending on `held` at `now`, as `owing` reads the
/// host's deliveries; none where it waits only on an answer, its caller's
/// word or a drop, or on room the host's settle starts host-wide (42, 47,
/// 49, 53, 70, 225; core.62).
pub(crate) fn pending(held: &HeldState, owing: &Owing, now: Seconds) -> Option<Pending> {
    match &held.phase {
        Phase::Making { .. } => return Some(Pending::Making),
        Phase::Over { .. } | Phase::Gone { .. } => return Some(Pending::Over),
        Phase::Tidied { pending_for, .. } if held.pending_for(owing) != *pending_for => {
            return Some(Pending::Kept);
        }
        Phase::Tidied { .. } | Phase::Live => {}
    }
    if held.ends.iter().any(|end| handing(held, end, owing)) {
        return Some(Pending::Handing);
    }
    if held.phase != Phase::Live {
        return None;
    }
    if held.live_start().is_some() || kept_open_unclosed(held) {
        return Some(Pending::Runs);
    }
    let submission = &held.submission;
    if submission_open(held)
        || (submission.due && held.submission_hold().is_none() && held.live_start().is_none())
    {
        return Some(Pending::Submission);
    }
    if base_place(&held.place.kind) {
        return Some(Pending::Base);
    }
    if time_come(held, now) {
        return Some(Pending::Time);
    }
    let room = held
        .stages
        .iter()
        .any(|stage| stage.state == StageState::WaitingForRoom);
    if waiting_on(held, None).is_none() && !room {
        return Some(Pending::Nothing);
    }
    None
}

/// Whether `end` of `held` waits to be handed on, as `owing` reads the
/// host's deliveries: its delivery not yet recorded though it has
/// recipients, a stall an earlier flume recorded aside, which is recorded
/// once its answerer is read; a notice of it answered sent whose hand-over
/// is not recorded, to a recipient it is still owed to; or, for an end a
/// notice carries, a recipient it stands pending for that nothing handed
/// it to (43, 75, 130, 132; weir 103).
fn handing(held: &HeldState, end: &EndHeld, owing: &Owing) -> bool {
    let place = &held.place;
    let earlier = matches!(
        end.of,
        EndOf::Stall {
            answerer_read: false,
            ..
        }
    );
    let Some(delivery) = end.delivery(place, owing) else {
        return !earlier && !end.recipients(&place.recipients).is_empty();
    };
    let unrecorded = end.noticing.iter().any(|claim| {
        !claim.sending
            && !delivery
                .handovers
                .iter()
                .any(|handover| handover.recipient == claim.recipient)
            && delivery
                .owed
                .iter()
                .any(|owed| owed.recipient == claim.recipient)
    });
    unrecorded
        || (end.noticed()
            && end
                .pending(place, owing)
                .iter()
                .any(|recipient| !end.handed_to(place, owing, recipient)))
}

/// Whether the agent of the place's current stage, ended short needing an
/// answer and kept open for it, has its letting go or its end still to
/// record: none recorded, or recorded with no time it ended, as the
/// settle's close of a kept session tests (42, 198).
fn kept_open_unclosed(held: &HeldState) -> bool {
    let Some(index) = held.current else {
        return false;
    };
    let stage = &held.stages[index];
    if stage.state != StageState::EndedShort || stage.start.is_none() {
        return false;
    }
    stage
        .end
        .as_ref()
        .filter(|end| end.open_until.is_some())
        .is_some_and(|end| {
            end.closed
                .as_ref()
                .is_none_or(|closed| closed.ended_at.is_none())
        })
}

/// Whether a time the place waits on has come at `now`, as the settle's
/// reading of the stall reads it: the move it waits on its caller for,
/// read again at the clock, recorded stalled, a hold on that move lapsed,
/// or a stall an earlier flume recorded read for its answerer. Read on a
/// copy, so nothing is written and no answerer is asked (130, 193).
fn time_come(held: &HeldState, now: Seconds) -> bool {
    let mut next = held.clone();
    read_stall(&mut next, now, None, &|_| None);
    let stalled = |held: &HeldState| held.waiting.as_ref().is_some_and(|waiting| waiting.stalled);
    next.ends != held.ends || next.holds != held.holds || stalled(&next) != stalled(held)
}
