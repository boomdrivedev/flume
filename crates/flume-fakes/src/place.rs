use crate::{lock, FakeGit, Log};
use flume_seams::{
    BranchRemoved, Checkout, Holds, MakeOutcome, Moved, Place, PlaceRead, PlaceRef, Readiness,
    ReadyStep, Removed, Revision,
};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex};

/// flume's own data directory on the fake host, where every place goes
/// (record 0027).
const DATA: &str = "/host/data/flume";

/// Where the fake place makes the place `place`, `<kind>--<name>`, of
/// `repository`: under the fake host's data directory by an id of the
/// repository's directory, so no place is in its repository's directory
/// (3, 78; record 0027).
pub fn place_location(repository: &str, place: &str) -> String {
    under(DATA, repository, place)
}

/// The place `place` of `repository` under flume's data directory `data`,
/// by twelve hex digits of a digest of the repository's directory, as
/// the place profile names a repository by its id (C.4).
fn under(data: &str, repository: &str, place: &str) -> String {
    let mut digest = DefaultHasher::new();
    repository.hash(&mut digest);
    let id = &format!("{:016x}", digest.finish())[..12];
    format!("{data}/places/{id}/{place}")
}

/// The place seam over the fake git: worktrees by location, each
/// repository's readiness, what each place holds and how its teardown
/// goes; and a standing role's checkouts by location, each detached at a
/// revision, what each holds that a move would overwrite, and the
/// repositories whose checkout is refused (193).
pub struct FakePlace {
    git: Arc<FakeGit>,
    state: Mutex<PlaceState>,
    pub log: Log,
}

#[derive(Default)]
struct PlaceState {
    worktrees: BTreeMap<String, (String, String)>,
    occupied: BTreeSet<String>,
    readiness: BTreeMap<String, Vec<ReadyStep>>,
    holds: BTreeMap<String, Holds>,
    teardown: BTreeMap<String, (String, String)>,
    unreadable: BTreeMap<String, String>,
    refusal: Option<String>,
    unlocated: Option<String>,
    inside: bool,
    data: Option<String>,
    /// Each checkout by its location: its repository and the revision it
    /// stands detached at.
    checkouts: BTreeMap<String, (String, Revision)>,
    /// Each checkout holding a change a move would overwrite, with the
    /// words its move is kept with.
    in_the_way: BTreeMap<String, String>,
    /// Each repository whose checkout is refused, with why.
    refused_checkouts: BTreeMap<String, String>,
}

/// The fake's word for a checkout's branch, which is none: git's own.
const DETACHED: &str = "(detached)";

impl FakePlace {
    pub fn new(git: Arc<FakeGit>) -> FakePlace {
        FakePlace {
            git,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    /// The readiness steps `repository` declares, each passing or failing
    /// with its words.
    pub fn set_readiness(&self, repository: &str, steps: &[(&str, bool, &str)]) {
        lock(&self.state).readiness.insert(
            repository.into(),
            steps
                .iter()
                .map(|(step, passed, words)| ReadyStep {
                    step: step.to_string(),
                    passed: *passed,
                    words: words.to_string(),
                })
                .collect(),
        );
    }

    /// Something stands at `location` that no worktree of flume's made.
    pub fn occupy(&self, location: &str) {
        lock(&self.state).occupied.insert(location.into());
    }

    pub fn set_holds(&self, location: &str, holds: Holds) {
        lock(&self.state).holds.insert(location.into(), holds);
    }

    /// The place's teardown fails at `step` with `words`.
    pub fn fail_teardown(&self, location: &str, step: &str, words: &str) {
        lock(&self.state)
            .teardown
            .insert(location.into(), (step.into(), words.into()));
    }

    pub fn mend_teardown(&self, location: &str) {
        lock(&self.state).teardown.remove(location);
    }

    pub fn unreadable(&self, location: &str, reason: &str) {
        lock(&self.state)
            .unreadable
            .insert(location.into(), reason.into());
    }

    /// Every make refused with `reason`, as a full disk would.
    pub fn refuse_makes(&self, reason: &str) {
        lock(&self.state).refusal = Some(reason.into());
    }

    /// No place given a location, for `reason`, as a host with no data
    /// directory would.
    pub fn refuse_locations(&self, reason: &str) {
        lock(&self.state).unlocated = Some(reason.into());
    }

    /// Each place given a location in its repository's directory,
    /// `<repository>/places/<place>`, as flume gave one before record
    /// 0027, while `inside` holds.
    pub fn locate_inside(&self, inside: bool) {
        lock(&self.state).inside = inside;
    }

    /// Each place given a location under `data` for flume's data
    /// directory, as a test whose terminal opens the place's real
    /// directory needs.
    pub fn locate_under(&self, data: &str) {
        lock(&self.state).data = Some(data.into());
    }

    /// The worktree at `location` removed through its teardown by another
    /// than flume, as a merge queue that removes a landed place does.
    pub fn vanish(&self, location: &str) {
        let mut state = lock(&self.state);
        state.worktrees.remove(location);
        state.holds.remove(location);
    }

    pub fn exists(&self, location: &str) -> bool {
        let state = lock(&self.state);
        state.worktrees.contains_key(location) || state.checkouts.contains_key(location)
    }

    /// Whether anything stands at `location` or under it: a place, a
    /// checkout or what no make put there.
    pub fn stands(&self, location: &str) -> bool {
        let state = lock(&self.state);
        let under = format!("{location}/");
        let at = |held: &str| held == location || held.starts_with(&under);
        state.worktrees.keys().any(|held| at(held))
            || state.checkouts.keys().any(|held| at(held))
            || state.occupied.iter().any(|held| at(held))
    }

    /// The revision the checkout at `location` stands at, none where no
    /// checkout stands there.
    pub fn checkout(&self, location: &str) -> Option<Revision> {
        lock(&self.state)
            .checkouts
            .get(location)
            .map(|(_, revision)| revision.clone())
    }

    /// The checkout at `location` holds a change a move would overwrite,
    /// its move kept with `words`, until [`FakePlace::out_of_the_way`].
    pub fn in_the_way(&self, location: &str, words: &str) {
        let mut state = lock(&self.state);
        state.in_the_way.insert(location.into(), words.into());
        state.holds.insert(
            location.into(),
            Holds {
                uncommitted: vec!["in-the-way.md".into()],
                ..Holds::default()
            },
        );
    }

    /// The change in the checkout at `location` is gone.
    pub fn out_of_the_way(&self, location: &str) {
        let mut state = lock(&self.state);
        state.in_the_way.remove(location);
        state.holds.remove(location);
    }

    /// Every checkout of `repository` refused with `reason`.
    pub fn refuse_checkouts(&self, repository: &str, reason: &str) {
        lock(&self.state)
            .refused_checkouts
            .insert(repository.into(), reason.into());
    }
}

impl Place for FakePlace {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        let state = lock(&self.state);
        match &state.unlocated {
            Some(reason) => Err(reason.clone()),
            None if state.inside => Ok(format!("{repository}/places/{place}")),
            None => Ok(under(
                state.data.as_deref().unwrap_or(DATA),
                repository,
                place,
            )),
        }
    }

    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.log
            .push(format!("make {repository} {revision} {branch} {location}"));
        let mut state = lock(&self.state);
        if let Some(reason) = &state.refusal {
            return MakeOutcome::Refused {
                reason: reason.clone(),
            };
        }
        if self.git.branch_exists(repository, branch) {
            return MakeOutcome::BranchExists;
        }
        if state.worktrees.contains_key(location) || state.occupied.contains(location) {
            return MakeOutcome::LocationExists;
        }
        self.git.set_branch(repository, branch, revision);
        state
            .worktrees
            .insert(location.into(), (repository.into(), branch.into()));
        MakeOutcome::Made
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.log
            .push(format!("ready {} {revision}", place.location));
        let state = lock(&self.state);
        let mut steps = vec![];
        for step in state
            .readiness
            .get(&place.repository)
            .cloned()
            .unwrap_or_default()
        {
            let passed = step.passed;
            steps.push(step);
            if !passed {
                break;
            }
        }
        Readiness { steps }
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.log.push(format!("read {}", place.location));
        let state = lock(&self.state);
        if let Some(reason) = state.unreadable.get(&place.location) {
            return PlaceRead::CouldNotRead {
                reason: reason.clone(),
            };
        }
        if let Some((_, revision)) = state.checkouts.get(&place.location) {
            return PlaceRead::Live {
                branch: DETACHED.into(),
                head: revision.clone(),
                holds: state
                    .holds
                    .get(&place.location)
                    .cloned()
                    .unwrap_or_default(),
            };
        }
        let Some((repository, branch)) = state.worktrees.get(&place.location) else {
            return PlaceRead::Gone;
        };
        let Some(head) = self.git.branch_head(repository, branch) else {
            return PlaceRead::Gone;
        };
        PlaceRead::Live {
            branch: branch.clone(),
            head,
            holds: state
                .holds
                .get(&place.location)
                .cloned()
                .unwrap_or_default(),
        }
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        self.log.push(format!("remove {}", place.location));
        let mut state = lock(&self.state);
        if let Some((step, words)) = state.teardown.get(&place.location) {
            return Removed::Refused {
                step: step.clone(),
                words: words.clone(),
            };
        }
        state.worktrees.remove(&place.location);
        state.holds.remove(&place.location);
        Removed::Removed
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.log
            .push(format!("remove_branch {repository} {branch} {base}"));
        if !self.git.branch_exists(repository, branch) {
            return BranchRemoved::Gone;
        }
        let commits = self.git.lacking(repository, branch, base);
        if !commits.is_empty() {
            return BranchRemoved::Kept { commits };
        }
        self.git.delete_branch(repository, branch);
        BranchRemoved::Removed
    }

    fn make_checkouts(&self, location: &str, checkouts: &[Checkout]) -> MakeOutcome {
        let each: Vec<String> = checkouts
            .iter()
            .map(|checkout| format!("{}@{}", checkout.folder, checkout.revision))
            .collect();
        self.log
            .push(format!("make_checkouts {location} {}", each.join(" ")));
        if let Some(reason) = &lock(&self.state).refusal {
            return MakeOutcome::Refused {
                reason: reason.clone(),
            };
        }
        if self.stands(location) {
            return MakeOutcome::LocationExists;
        }
        let mut state = lock(&self.state);
        if let Some((checkout, reason)) = checkouts.iter().find_map(|checkout| {
            state
                .refused_checkouts
                .get(&checkout.repository)
                .map(|reason| (checkout, reason))
        }) {
            // All or nothing (83): no checkout is made.
            return MakeOutcome::Refused {
                reason: format!(
                    "the checkout of {} could not be made: {reason}",
                    checkout.repository
                ),
            };
        }
        for checkout in checkouts {
            state.checkouts.insert(
                format!("{location}/{}", checkout.folder),
                (checkout.repository.clone(), checkout.revision.clone()),
            );
        }
        MakeOutcome::Made
    }

    fn move_checkout(&self, checkout: &PlaceRef, revision: &Revision) -> Moved {
        self.log
            .push(format!("move_checkout {} {revision}", checkout.location));
        let mut state = lock(&self.state);
        if let Some(words) = state.in_the_way.get(&checkout.location) {
            return Moved::Kept {
                reason: words.clone(),
            };
        }
        match state.checkouts.get_mut(&checkout.location) {
            Some((_, at)) => {
                *at = revision.clone();
                Moved::Moved
            }
            None => Moved::Gone,
        }
    }

    fn remove_checkouts(&self, location: &str, checkouts: &[Checkout]) -> Removed {
        self.log.push(format!("remove_checkouts {location}"));
        let mut state = lock(&self.state);
        for checkout in checkouts {
            let at = format!("{location}/{}", checkout.folder);
            if let Some(reason) = state.unreadable.get(&at) {
                return Removed::Refused {
                    step: "remove".into(),
                    words: reason.clone(),
                };
            }
            if state.holds.get(&at).is_some_and(Holds::any) {
                return Removed::Refused {
                    step: "remove".into(),
                    words: format!("{at} holds work"),
                };
            }
        }
        let under = format!("{location}/");
        state.checkouts.retain(|held, _| !held.starts_with(&under));
        state.holds.retain(|held, _| !held.starts_with(&under));
        state.occupied.remove(location);
        Removed::Removed
    }
}
