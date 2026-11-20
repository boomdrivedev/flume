//! A scenario of the suite and how scenarios are run.

use flume_seams::*;
use std::collections::BTreeMap;

/// What a scenario comes to: passed, or the failure it names.
pub type Outcome = Result<(), String>;

/// A scenario: its name, the clauses, invariants and scenarios of the
/// statement it cites, and its run over a harness `H`.
pub struct Scenario<H: ?Sized> {
    pub name: &'static str,
    pub cites: &'static [&'static str],
    pub run: fn(&H) -> Outcome,
}

/// A scenario's name and citations, apart from what it runs over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cited {
    pub name: String,
    pub cites: Vec<String>,
}

impl<H: ?Sized> Scenario<H> {
    pub fn cited(&self) -> Cited {
        Cited {
            name: self.name.into(),
            cites: self.cites.iter().map(|cite| cite.to_string()).collect(),
        }
    }
}

/// Runs each scenario over a fresh harness, and gives each failure with
/// the scenario's name.
pub fn failures<H>(scenarios: &[Scenario<H>], fresh: impl Fn() -> H) -> Vec<(String, String)> {
    scenarios
        .iter()
        .filter_map(|scenario| {
            (scenario.run)(&fresh())
                .err()
                .map(|failure| (scenario.name.to_string(), failure))
        })
        .collect()
}

/// Runs each scenario over a fresh harness and panics naming every one
/// that failed.
pub fn pass_all<H>(of: &str, scenarios: &[Scenario<H>], fresh: impl Fn() -> H) {
    passed(of, scenarios.len(), failures(scenarios, fresh));
}

/// Panics naming every one of `failed`, out of `total` scenarios of
/// `of`, unless none failed.
fn passed(of: &str, total: usize, failed: Vec<(String, String)>) {
    assert!(
        failed.is_empty(),
        "{} of {total} {of} scenarios failed:\n{}",
        failed.len(),
        failed
            .iter()
            .map(|(name, failure)| format!("  {name}: {failure}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Runs each scenario over a fresh harness of its own, `at_once` of them
/// side by side, and panics naming every one that failed, in the
/// scenarios' order: for a profile whose harnesses share nothing, each
/// on a server and in a scratch of its own, so its scenarios wait on no
/// other's.
pub fn pass_all_side_by_side<H>(
    of: &str,
    scenarios: &[Scenario<H>],
    at_once: usize,
    fresh: impl Fn() -> H + Sync,
) {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let outcomes: Vec<(usize, Outcome)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..at_once.clamp(1, scenarios.len().max(1)))
            .map(|_| {
                scope.spawn(|| {
                    let mut ran = vec![];
                    loop {
                        let index = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(scenario) = scenarios.get(index) else {
                            break ran;
                        };
                        ran.push((index, (scenario.run)(&fresh())));
                    }
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| {
                worker
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });
    let mut failed: Vec<(usize, String)> = outcomes
        .into_iter()
        .filter_map(|(index, outcome)| outcome.err().map(|failure| (index, failure)))
        .collect();
    failed.sort_by_key(|(index, _)| *index);
    passed(
        of,
        scenarios.len(),
        failed
            .into_iter()
            .map(|(index, failure)| (scenarios[index].name.to_string(), failure))
            .collect(),
    );
}

/// Fails the scenario with the words given unless the condition holds.
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $($words:tt)+) => {
        if !$condition {
            return Err(format!($($words)+));
        }
    };
}

/// Fails the scenario naming what differs unless the two are equal.
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $what:expr) => {{
        let (left, right) = (&$left, &$right);
        if left != right {
            return Err(format!("{}: {:?} where {:?} was due", $what, left, right));
        }
    }};
}

/// A place's held state for the store's scenarios: a fix at its launch,
/// its code stage not reached.
pub fn sample_held(id: &PlaceId, repository: &str) -> HeldState {
    let kind = Kind {
        name: "fix".into(),
        version: 1,
        branch_prefix: "fix".into(),
        starts_unready: true,
        lands: true,
        agents: KindSettings::default(),
        label: None,
        stages: vec![Stage {
            name: "code".into(),
            prompt: "{{brief}}\n\n{{needs}}".into(),
            conversation: false,
            deliverables: vec![Deliverable::Commits],
            agents: vec![Settings::default()],
            waits: false,
            quiet_limit: None,
            send_back_to: Some("code".into()),
            send_back_limit: Some(2),
            makes: vec![],
            repeats: false,
            hibernates: false,
        }],
        fix: None,
        fix_limit: None,
    };
    HeldState {
        place: PlaceRecord {
            id: id.clone(),
            repository: repository.into(),
            address: None,
            kind,
            declared: Declared::Flume,
            kind_from: None,
            name: "one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
            base_revision: Revision::new("c1"),
            base_from: None,
            location: id.0.clone(),
            binding: Binding::default(),
            brief: Some("mend it".into()),
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
                name: "one".into(),
                digest: "d1".into(),
            },
            fix_of: None,
            fixes_as: None,
            token: None,
            checkouts: Vec::new(),
        },
        ends: vec![],
        role: None,
        unwoken: Default::default(),
        moves: vec![],
        unsettled: vec![],
        uncarried: vec![],
        terminals: vec![],
        holds: vec![],
        waiting: None,
        stall_after: STALL_AFTER,
        phase: Phase::Live,
        readiness: Readiness::default(),
        stages: vec![StageHeld {
            name: "code".into(),
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
        }],
        current: None,
        submission: SubmissionHeld::default(),
        failures: vec![],
        priority: 0,
        shown: None,
        fixes: vec![],
        trace: None,
        waits: vec![],
        forecast: None,
    }
}
