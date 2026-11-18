//! A statechart kind's step (74, 175, 176; core.42): each settle of a
//! place under a statechart kind steps its sessions as a simple kind's
//! current stage is stepped, then reads the place's branch's head once,
//! serves every evidence the statechart and the stage template read at
//! it, ticks the statechart once on the family's engine, and in one write
//! by compare-and-set holds the next state with each effect the tick
//! planned performed: `start_sessions` records its stage's entry and the
//! entry's sessions, each held as a stage of the place, and each is
//! started as any stage is. flume moves no region of its own: the engine
//! decides every transition, and an effect whose proof the evidence
//! holds is planned no more, so a settle repeated performs nothing twice.

use crate::{atoms, Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;
use toml::Value;

impl Engine {
    /// The settle's step for a place of a statechart kind: a session's
    /// step where one moves, else the statechart's one tick of the settle,
    /// which `ticked` says was taken; true when something changed (176).
    pub(crate) fn chart_step(&self, id: &PlaceId, held: &HeldState, ticked: &mut bool) -> bool {
        if (0..held.stages.len()).any(|index| self.step_session(id, held, index)) {
            return true;
        }
        if *ticked {
            return false;
        }
        *ticked = true;
        self.tick_chart(id)
    }

    /// Steps the session at `index` as a simple kind's current stage is
    /// stepped, by its index: its prompt delivered, its beginning read, its
    /// end read, and its stop-short's agent let go once quiet (30, 31, 175,
    /// 198); true when it changed something.
    fn step_session(&self, id: &PlaceId, held: &HeldState, index: usize) -> bool {
        let at = Some(index);
        match held.stages[index].state {
            StageState::Starting | StageState::Started => self.deliver(id, at),
            StageState::NeverBegun => self.begun_after(id, at),
            StageState::Begun | StageState::Stuck | StageState::Lost => self.read_end(id, at),
            StageState::EndedShort => self.found_work(id, at) || self.close_session(id, at),
            _ => false,
        }
    }

    /// The statechart's one tick: its held text and the stage template's
    /// read back through the kinds seam, the head read once, the evidence
    /// served at it, the engine's tick run over the held state, and, where
    /// anything fired, the next state written by compare-and-set with
    /// `start_sessions` performed for each stage it was planned at, then
    /// each session it recorded started (74, 75, 175, 176, 178). True when
    /// the tick fired anything.
    fn tick_chart(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return false;
        }
        let (Some(chart), Some(statechart)) = (&held.chart, &held.place.kind.statechart) else {
            return false;
        };
        let charts = match self
            .resolver
            .kinds()
            .read_statechart(statechart, &chart.template)
        {
            Ok(charts) => charts,
            Err(reason) => {
                self.chart_failed(
                    id,
                    format!(
                        "the statechart of {} could not be read back: {reason}",
                        held.place.id
                    ),
                );
                return false;
            }
        };
        let place = &held.place;
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        let state = chart.state.state();
        let now = self.host.now();
        let snapshot = snapshot_of(&held, chart, &state, now);
        let proofs = BTreeMap::from([(
            atoms::START_SESSIONS.to_string(),
            atoms::STAGE_STARTED.to_string(),
        )]);
        let tick =
            match weir_engine::tick(&charts.chart, &charts.templates, &proofs, &state, &snapshot) {
                Ok(tick) => tick,
                Err(refusal) => {
                    self.chart_failed(
                        id,
                        format!(
                            "the statechart of {} did not tick at {}: {} (176)",
                            place.id, refusal.path, refusal.reason
                        ),
                    );
                    return false;
                }
            };
        if tick.fired.is_empty() {
            return false;
        }
        // Each stage `start_sessions` was planned at, by its path, with the
        // entry it records and the sessions of that entry (175, 178).
        let planned: Vec<(String, u32, Vec<ChartSession>)> = tick
            .fired
            .iter()
            .flat_map(|fired| &fired.effects)
            .filter(|effect| effect.atom == atoms::START_SESSIONS)
            .filter_map(|effect| stage_of(&effect.region))
            .filter(|path| state.is_live(&template_region(path)))
            .filter_map(|path| {
                let stage = statechart.stages.get(&path)?;
                let entry = entry_of(&state, &path);
                let sessions = self.sessions_of(&held, &path, entry, stage, &head);
                Some((path, entry, sessions))
            })
            .collect();
        let from = chart.state.clone();
        let next = ObjectHeld::of(&tick.next);
        let started = self.update(id, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(vec![]);
            }
            let Some(chart) = held.chart.as_mut() else {
                return Step::Keep(vec![]);
            };
            // A settle beside this one ticked first: its tick stands, and
            // this one's is not taken (75).
            if chart.state != from {
                return Step::Keep(vec![]);
            }
            chart.state = next.clone();
            let mut started = vec![];
            for (path, entry, sessions) in &planned {
                let recorded = chart.stages.entry(path.clone()).or_default();
                if recorded.entry == *entry {
                    continue;
                }
                recorded.entry = *entry;
                let words = (!sessions.is_empty()).then(|| chart.words.take()).flatten();
                for session in sessions {
                    started.push((held.stages.len(), words.clone()));
                    held.stages.push(StageHeld {
                        name: session.name(),
                        state: StageState::NotReached,
                        starts: 0,
                        refused_part: None,
                        refused_setting: None,
                        end: None,
                        start: None,
                        since: None,
                        room: None,
                        made_from: None,
                        counts: None,
                        session: Some(session.clone()),
                    });
                }
            }
            Step::Write(started)
        });
        for (index, words) in started.unwrap_or_default() {
            self.start_stage(id, index, words);
        }
        true
    }

    /// The sessions of the stage at `path` for its entry `entry`: one for
    /// each agent it lists, with that agent's settings as its own, or, for
    /// `every-file`, one for each file of the place's branch at `head` its
    /// pattern matches, in path order, with the file held and no settings
    /// of its own; each numbered from 1 in that order (175, 178, C.2).
    fn sessions_of(
        &self,
        held: &HeldState,
        path: &str,
        entry: u32,
        stage: &ChartStage,
        head: &Revision,
    ) -> Vec<ChartSession> {
        let given: Vec<(Settings, Option<String>)> = match &stage.agents {
            ChartAgents::Entries(entries) => entries
                .iter()
                .map(|settings| (settings.clone(), None))
                .collect(),
            ChartAgents::Rule {
                rule,
                pattern: Some(pattern),
            } if rule == atoms::EVERY_FILE => {
                let mut files: Vec<String> = self
                    .history
                    .files(&held.place.repository, head)
                    .into_iter()
                    .filter(|file| crate::glob::matches(pattern, file))
                    .collect();
                files.sort();
                files.dedup();
                files
                    .into_iter()
                    .map(|file| (Settings::default(), Some(file)))
                    .collect()
            }
            ChartAgents::Rule { .. } => vec![],
        };
        given
            .into_iter()
            .zip(1..)
            .map(|((settings, file), n)| session(path, entry, n, file, settings, stage))
            .collect()
    }

    /// A tick that could not be made, recorded once among the place's
    /// failures (56).
    fn chart_failed(&self, id: &PlaceId, failure: String) {
        self.log(&failure);
        self.update(id, |held| {
            if held.failures.last() != Some(&failure) {
                held.failures.push(failure.clone());
            }
            Step::Write(())
        });
    }
}

/// The session `n` of the stage at `path` for its entry `entry`: its
/// declaration the stage's prompt, its deliverables with `{{session}}` and
/// `{{file}}` filled, and `settings` as its one agent (30, 33, 175, C.2).
fn session(
    path: &str,
    entry: u32,
    n: u32,
    file: Option<String>,
    settings: Settings,
    stage: &ChartStage,
) -> ChartSession {
    let fill = |text: &str| {
        let text = text.replace(
            &format!("{{{{{}}}}}", PromptPart::Session.name()),
            &n.to_string(),
        );
        match &file {
            Some(file) => text.replace(
                &format!("{{{{{}}}}}", PromptPart::File.name()),
                file.as_str(),
            ),
            None => text,
        }
    };
    let deliverables = stage
        .deliverables
        .iter()
        .map(|deliverable| match deliverable {
            Deliverable::Report { path } => Deliverable::Report { path: fill(path) },
            Deliverable::CommittedReport { path } => {
                Deliverable::CommittedReport { path: fill(path) }
            }
            other => other.clone(),
        })
        .collect();
    let mut session = ChartSession {
        stage: path.to_string(),
        entry,
        n,
        file,
        declared: Stage {
            name: String::new(),
            prompt: stage.prompt.clone().unwrap_or_default(),
            conversation: false,
            deliverables,
            agents: vec![settings],
            waits: false,
            quiet_limit: None,
            send_back_to: None,
            send_back_limit: None,
            makes: vec![],
            repeats: false,
        },
    };
    session.declared.name = session.name();
    session
}

/// The stage template's region under the stage at `path` (175).
pub(crate) fn template_region(path: &str) -> String {
    format!("{path}.{}", atoms::TEMPLATE_REGION)
}

/// The stage whose template region is `region`, by its path; none for any
/// other region (175).
pub(crate) fn stage_of(region: &str) -> Option<String> {
    region
        .strip_suffix(&format!(".{}", atoms::TEMPLATE_REGION))
        .map(str::to_string)
}

/// The entry of the stage at `path` its template's region now runs: one
/// more than the instances the engine set aside under `<path>#`, so a
/// settle repeated reads the same entry (74, 175; weir 97).
pub(crate) fn entry_of(state: &weir_engine::ObjectState, path: &str) -> u32 {
    let prefix = format!("{path}#");
    let aside = state
        .set_aside
        .keys()
        .filter(|key| key.starts_with(&prefix))
        .count();
    u32::try_from(aside).unwrap_or(u32::MAX).saturating_add(1)
}

/// The tick's one snapshot of the place at `now` (176, 178): at each live
/// region, `region.entered`, when it entered the state it holds; at each
/// stage template's region, `stage.started`, whether the entry it runs is
/// recorded, read true where the region is not live, so nothing is
/// started for a stage its region has left; and for the object,
/// `place.submission`, its newest submission's outcome as flume confirmed
/// it, `open` while one is in flight and none before the first, and
/// `place.submissions`, how many times flume submitted it.
fn snapshot_of(
    held: &HeldState,
    chart: &ChartHeld,
    state: &weir_engine::ObjectState,
    now: Seconds,
) -> weir_engine::Snapshot {
    let mut regions: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
    for (path, at) in &chart.state.entered_at {
        if !state.is_live(path) {
            continue;
        }
        let at = i64::try_from(*at)
            .ok()
            .and_then(|at| chrono::DateTime::from_timestamp(at, 0));
        if let Some(at) = at {
            regions.entry(path.clone()).or_default().insert(
                atoms::REGION_ENTERED.into(),
                Value::String(at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            );
        }
    }
    for region in state.regions.keys() {
        let Some(path) = stage_of(region) else {
            continue;
        };
        let started = !state.is_live(region)
            || chart
                .stages
                .get(&path)
                .is_some_and(|recorded| recorded.entry == entry_of(state, &path));
        regions
            .entry(region.clone())
            .or_default()
            .insert(atoms::STAGE_STARTED.into(), Value::Boolean(started));
    }
    let submission = &held.submission;
    let mut evidence = BTreeMap::from([(
        atoms::PLACE_SUBMISSIONS.to_string(),
        Value::Integer(i64::from(submission.submits)),
    )]);
    let outcome = match &submission.verdict {
        Some(verdict) => Some(verdict_word(verdict)),
        None if submission.submits > 0 => Some("open"),
        None => None,
    };
    if let Some(outcome) = outcome {
        evidence.insert(
            atoms::PLACE_SUBMISSION.into(),
            Value::String(outcome.into()),
        );
    }
    weir_engine::Snapshot {
        at: time_at(now),
        evidence,
        regions,
        responses: vec![],
        owner: None,
        children: vec![],
    }
}

/// A verdict's word, as `place.submission` reads it (133, 178).
pub(crate) fn verdict_word(verdict: &Verdict) -> &'static str {
    match verdict {
        Verdict::Landed { .. } => "landed",
        Verdict::Failed { .. } => "failed",
        Verdict::Conflict { .. } => "conflict",
        Verdict::Withdrawn => "withdrawn",
        Verdict::Refused { .. } => "refused",
    }
}
