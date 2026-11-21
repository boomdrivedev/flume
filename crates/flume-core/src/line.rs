//! A place's one line (56, 139, 142): its kind and name, its kind's
//! version, its current stage with a repeating stage's group and how
//! many times that stage started, its change's tasks ticked, and its
//! state in words. It is read from the place's held state, the change at
//! its head and its live agent's asking mark alone, never the terminal,
//! the merge queue or the host, so it may be asked at every render of an
//! agent's pane; status gives the same line from the facts it read (57,
//! 58, 60; S14; core.14, core.15). Nothing here writes, sends or starts
//! anything (57; core.16).

use crate::base::submission_open;
use crate::clues::word;
use crate::held::waiting_now;
use crate::reading::reads_change;
use crate::start::Starts;
use crate::status::{group_status, session_wait};
use crate::{label, side_by_side, Answer, Engine, PlaceLine};
use flume_seams::*;

/// What a full line ends with, asked for a session that is not the
/// current stage's live agent, so a session that outlives its start never
/// shows a later start's stage as its own (35, 140).
pub const NOT_ITS_AGENT: &str = "this session is not its agent";

/// What joins a line's parts, the list's among them (200).
pub(crate) const SEPARATOR: &str = " · ";

impl Engine {
    /// The line of each place, of the place named or of every place on
    /// the host, a place tidied whose document stays for its pending ends
    /// among them, each asked alone with no session's identity, the
    /// places read side by side; refused for a place no document of the
    /// host holds (43, 58, 62). It settles nothing, since it acts on no
    /// place (119).
    pub fn lines(&self, place: Option<&PlaceId>) -> Answer {
        let held = match place {
            Some(id) => {
                let held = self.held_for_line(id);
                if held.is_empty() {
                    return Answer::refused(format!("no place {id} stands on this host"));
                }
                held
            }
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state)
                .collect(),
        };
        Answer::Lines {
            lines: side_by_side(&held, |held| self.line_of(held, None)),
        }
    }

    /// The place's line asked alone, for the session under `identity`
    /// where one asks, or none where no document of the host holds the
    /// place; of a place tidied twice, its newest document's.
    pub fn line(&self, place: &PlaceId, identity: Option<&Identity>) -> Option<PlaceLine> {
        let held = self.held_for_line(place).pop()?;
        Some(self.line_of(&held, identity))
    }

    /// The place's line from what it reads itself: the held state given,
    /// the change at the place's head once where its kind reads a change
    /// and the place is live, the live agent's asking mark from its
    /// profile, and, for a base place, the held state of the places on
    /// its branch; no terminal operation, no merge-queue operation and no
    /// host reading (57, 58, 90, 203).
    pub(crate) fn line_of(&self, held: &HeldState, identity: Option<&Identity>) -> PlaceLine {
        let change = match held.phase {
            Phase::Live => self.change_at_head(held),
            _ => None,
        };
        Engine::place_line(
            held,
            change.as_ref(),
            self.asking(held),
            self.on_branch_count(held),
            identity,
        )
    }

    /// Whether the live agent's profile reads it asking a person, from
    /// its own record and the transcript it names alone (90).
    pub(crate) fn asking(&self, held: &HeldState) -> bool {
        held.live_start().is_some_and(|(_, start)| {
            self.agent(&start.program).is_some_and(|agent| {
                agent.asking(&self.store.folder(&held.place.id), &start.identity) == Reading::Yes
            })
        })
    }

    /// The place's documents as status reads them: its own, else each
    /// tidied one kept for its pending ends.
    fn held_for_line(&self, place: &PlaceId) -> Vec<HeldState> {
        match self.store.read(place) {
            Some(read) => vec![read.state],
            None => self
                .tidied_of(place)
                .iter()
                .filter_map(|id| self.store.read(id))
                .map(|read| read.state)
                .collect(),
        }
    }

    /// The place's change at its branch's head, as status reads it; none
    /// where its kind reads no change, no change profile is bound, or the
    /// change cannot be read (58).
    fn change_at_head(&self, held: &HeldState) -> Option<ChangeFacts> {
        let place = &held.place;
        if !reads_change(&place.kind) {
            return None;
        }
        let change = self.bound(place).ok()?.change?;
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        match change.read_change(&place.repository, &head, &place.name) {
            ChangeRead::Read { facts } => Some(facts),
            ChangeRead::CouldNotRead { .. } => None,
        }
    }

    /// The place's line from facts already read: its held state, the
    /// change at its head where it was read, whether the live agent's
    /// profile reads it asking a person, for a base place how many places
    /// are live on its branch, and the identity of the session asking
    /// where one asks. It reads nothing itself, so status and the line
    /// asked alone give one line from one place's facts (60, 203; S14).
    pub fn place_line(
        held: &HeldState,
        change: Option<&ChangeFacts>,
        asking: bool,
        on_branch: Option<usize>,
        identity: Option<&Identity>,
    ) -> PlaceLine {
        let place = &held.place;
        let live = held.phase == Phase::Live;
        let change = change.filter(|_| live);
        let at = live.then(|| stage_at(held)).flatten();
        // A statechart place names each stage whose sessions are live by
        // its path, and no run of one session's (175, 200).
        let chart = held.chart.is_some();
        let stage = if chart {
            live.then(|| live_stages(held)).flatten()
        } else {
            at.map(|index| {
                let group = change.and_then(|facts| group_status(held, index, facts));
                label(&held.stages[index].name, group.as_ref())
            })
        };
        let run = at
            .filter(|_| !chart)
            .map(|index| held.stages[index].starts)
            .filter(|starts| *starts > 0);
        let tasks = change.filter(|facts| !facts.tasks.is_empty()).map(|facts| {
            let ticked = facts.tasks.iter().filter(|task| task.ticked).count();
            format!("{ticked}/{}", facts.tasks.len())
        });
        let state = state_words(held, at, asking, on_branch);
        let short = [
            stage.clone(),
            run.map(|run| format!("run {run}")),
            tasks.as_ref().map(|tasks| format!("tasks {tasks}")),
            Some(state.clone()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(SEPARATOR);
        let name = place.short();
        let kind = format!("{}@{}", place.kind.name, place.kind.version);
        let mut full = [name.as_str(), kind.as_str(), short.as_str()].join(SEPARATOR);
        if identity.is_some_and(|identity| !its_agent(held, identity)) {
            full.push_str(SEPARATOR);
            full.push_str(NOT_ITS_AGENT);
        }
        PlaceLine {
            name,
            kind,
            stage,
            run,
            tasks,
            state,
            full,
            short,
        }
    }
}

/// The stage a live place's line names: the stage it most recently
/// started, else the first it reached without a start, held, waiting on
/// its readiness or refused (47, 12, 27); the list's too (200).
pub(crate) fn stage_at(held: &HeldState) -> Option<usize> {
    held.current.or_else(|| {
        held.stages
            .iter()
            .position(|stage| stage.state != StageState::NotReached)
    })
}

/// The stages of a statechart place whose sessions are live, by their
/// paths in the order their sessions were recorded, joined by `, `; none
/// where no session is live (175, 200).
pub(crate) fn live_stages(held: &HeldState) -> Option<String> {
    let mut paths: Vec<&str> = vec![];
    for (index, _) in held.live_starts() {
        if let Some(session) = &held.stages[index].session {
            if !paths.contains(&session.stage.as_str()) {
                paths.push(&session.stage);
            }
        }
    }
    (!paths.is_empty()).then(|| paths.join(", "))
}

/// Whether `identity` is the current stage's live start, or a live
/// session's of a statechart place (175).
fn its_agent(held: &HeldState, identity: &Identity) -> bool {
    if held.chart.is_some() {
        return held
            .live_starts()
            .any(|(_, start)| &start.identity == identity);
    }
    held.live_start()
        .is_some_and(|(index, start)| Some(index) == held.current && &start.identity == identity)
}

/// The place's state in words, the first that holds, each read from its
/// held state or its live agent's asking mark, never the terminal or the
/// merge queue: `submitted` is flume's own count of its submits and
/// `landed` its own record of the work over, which the merge queue's
/// record moves at the next settle (12, 27, 34, 35, 38, 42, 45, 47, 48,
/// 51, 63, 70, 90, 112, 114, 118, 130); a stage waiting for room while a
/// process runs the conversation its start would carry reads `waiting on
/// its session`, as the last settle left its wait (49, 207); the list
/// gives a place the same words (200; core.14). A place whose current
/// stage hibernates reads `hibernating`, after `stalled` and before `held`
/// (57, 212). A base place's, with
/// `on_branch` the count of the live places on its branch, is `<n> on its
/// branch · open` while any is live and its submission is not open, and
/// `waiting on its caller` while it waits for its submission's retry
/// under no hold or failure (130, 139, 200, 203).
pub(crate) fn state_words(
    held: &HeldState,
    at: Option<usize>,
    asking: bool,
    on_branch: Option<usize>,
) -> String {
    match &held.phase {
        Phase::Making { .. } => return "being made".into(),
        Phase::Over { over, .. } | Phase::Gone { over, .. } | Phase::Tidied { over, .. } => {
            return match over {
                Over::Landed { .. } => "landed",
                Over::Dropped => "dropped",
                Over::Done => "done",
            }
            .into();
        }
        Phase::Live => {}
    }
    if let Some(count) = on_branch.filter(|count| *count > 0) {
        if !submission_open(held) {
            return format!("{count} on its branch{SEPARATOR}open");
        }
    }
    let waiting = waiting_now(held, on_branch);
    if waiting.as_ref().is_some_and(|waiting| waiting.stalled) {
        return "stalled".into();
    }
    // A hibernating stage, after a stall and before a hold: its agent is
    // ended and its session kept for a resume (212).
    if held
        .current
        .is_some_and(|index| held.stages[index].state == StageState::Hibernating)
    {
        return "hibernating".into();
    }
    if let Some(waiting) = &waiting {
        return match waiting.on.why {
            Waits::OnHold => "held",
            Waits::Answer => "waiting on an answer",
            Waits::Dialog => "asking",
            Waits::Lost => "lost",
            Waits::NeverBegun => "never begun",
            Waits::Unready => "not ready",
            Waits::Refused => "refused",
            Waits::Stopped => "stopped",
            Waits::PastBound => "past its bound",
            Waits::Submission => "submission refused",
            Waits::ForgeRefused => "landing refused",
            Waits::Delivered => "waiting on done",
            Waits::Unsubmitted => "waiting on its caller",
            Waits::Decision => "waiting on a decision",
        }
        .into();
    }
    if session_wait(held).is_some() {
        return "waiting on its session".into();
    }
    if held
        .stages
        .iter()
        .any(|stage| stage.state == StageState::WaitingForRoom)
    {
        return "waiting for room".into();
    }
    if let Some((index, _)) = held.live_start() {
        let state = held.stages[index].state;
        return if state == StageState::Starting {
            "starting"
        } else if asking {
            "asking"
        } else if state == StageState::Stuck {
            "stuck"
        } else if held.declared(index).conversation {
            "in conversation"
        } else {
            "working"
        }
        .into();
    }
    let submission = &held.submission;
    if submission.submits > 0
        && !submission.due
        && !held.stages.iter().any(|stage| stage.state.running())
    {
        return "submitted".into();
    }
    match at {
        Some(index) => word(&held.stages[index].state),
        None => word(&StageState::NotReached),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};
    use std::collections::BTreeMap;

    /// A base place `train--spring` on `main`, live, its kind landing
    /// with no stages (202).
    fn spring() -> HeldState {
        let place = PlaceRecord {
            id: PlaceId::new("/d/places/1/train--spring"),
            repository: "/r".into(),
            address: None,
            kind: Kind {
                name: "train".into(),
                version: 1,
                branch_prefix: "train".into(),
                starts_unready: false,
                lands: true,
                agents: KindSettings::default(),
                stages: vec![],
                label: None,
                fix: None,
                fix_limit: None,
                statechart: None,
            },
            declared: Declared::Repository,
            kind_from: None,
            name: "spring".into(),
            branch: "train/spring".into(),
            base: "main".into(),
            base_revision: Revision::new("c1"),
            base_from: None,
            location: "/d/places/1/train--spring".into(),
            binding: Binding::default(),
            brief: None,
            mark: None,
            settings: BTreeMap::new(),
            every_stage: Settings::default(),
            repository_settings: KindSettings::default(),
            repository_agents: Settings::default(),
            hold: vec![],
            hold_submission: false,
            hold_words: BTreeMap::new(),
            recipients: vec![],
            recipients_from: None,
            stream: StreamHeld {
                name: DEFAULT_STREAM.into(),
                from: StreamFrom::Default,
            },
            intent: IntentKey::Launch {
                name: "spring".into(),
                digest: "d1".into(),
            },
            fix_of: None,
            fixes_as: None,
            token: None,
            checkouts: Vec::new(),
        };
        serde_json::from_value(json!({
            "place": to_value(place).unwrap(),
            "phase": to_value(Phase::Live).unwrap(),
            "readiness": to_value(Readiness::default()).unwrap(),
            "stages": [],
            "current": null,
            "submission": to_value(SubmissionHeld::default()).unwrap(),
            "failures": [],
            "moves": [],
            "unsettled": [],
        }))
        .unwrap()
    }

    /// The base place's line with `on_branch` places live on its branch.
    fn line(held: &HeldState, on_branch: usize) -> PlaceLine {
        Engine::place_line(held, None, false, Some(on_branch), None)
    }

    #[test]
    fn a_base_places_line_counts_the_places_on_its_branch() {
        // 139, 200, 203: `<n> on its branch · open` while places on its
        // branch are live, whatever the last settle wrote of its wait.
        let mut held = spring();
        held.waiting = Some(Waiting {
            on: WaitingOn {
                act: Act::Retry,
                stage: None,
                why: Waits::Unsubmitted,
                since: Some(10),
                words: None,
                open_until: None,
                closed: None,
            },
            stalled: true,
        });
        let two = line(&held, 2);
        assert_eq!(two.short, "2 on its branch · open");
        assert_eq!(two.full, "train--spring · train@1 · 2 on its branch · open");
        assert_eq!((two.stage, two.run, two.tasks), (None, None, None));
        assert_eq!(line(&held, 1).short, "1 on its branch · open");
    }

    #[test]
    fn a_base_place_with_none_on_its_branch_waits_on_its_caller() {
        // 130, 203: with no place on its branch live and no submission
        // made it waits on its caller; stalled once a settle read it so.
        let mut held = spring();
        assert_eq!(line(&held, 0).short, "waiting on its caller");
        held.waiting = Some(Waiting {
            on: WaitingOn {
                act: Act::Retry,
                stage: None,
                why: Waits::Unsubmitted,
                since: Some(10),
                words: None,
                open_until: None,
                closed: None,
            },
            stalled: true,
        });
        assert_eq!(line(&held, 0).short, "stalled");
    }

    #[test]
    fn a_base_places_submission_reads_as_any_places() {
        // 111, 112, 203: submitted while its submission is open, its
        // failure's words as a refused submission's, and landed once
        // over.
        let mut held = spring();
        held.submission.submits = 1;
        assert_eq!(line(&held, 0).short, "submitted");
        held.submission.verdict = Some(Verdict::Failed {
            words: "test failed".into(),
        });
        held.submission.waiting = Some("the submission failed: test failed".into());
        assert_eq!(line(&held, 0).short, "submission refused");
        assert_eq!(line(&held, 1).short, "1 on its branch · open");
        // 112, 130: a landing the forge refused, held open by the merge
        // queue, reads as its own state (core.58).
        let mut refused = spring();
        refused.submission.submits = 1;
        refused.submission.waiting =
            Some("the forge refused the submission's landing: push declined".into());
        refused.submission.landing_refused = true;
        assert_eq!(line(&refused, 0).short, "landing refused");
        held.phase = Phase::Over {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            kept: None,
        };
        assert_eq!(line(&held, 0).short, "landed");
    }
}
