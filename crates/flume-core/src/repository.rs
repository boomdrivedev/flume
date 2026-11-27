//! What flume reads of a repository for a caller about to launch there,
//! with no launch: its binding as it holds its default branch, whether
//! the merge queue it binds lands that branch, and each kind flume would
//! run there with whether it lands and what of the binding refuses it
//! (20, 105; core.3, core.15). Status gives it for each repository its
//! caller names, read where it lives at each ask, fetching, writing and
//! keeping nothing.

use crate::reading::reads_change;
use crate::{side_by_side, Answer, Engine};
use flume_seams::{
    BindingHeld, Kind, KindIdentity, KindsRead, LandsOn, Launch, LockRead, PlaceId, Revision,
    LAUNCH_SOURCE, OWN_SOURCE, REPO_SOURCE,
};
use serde::{Deserialize, Serialize};

/// One repository a caller named, as status reads it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryStatus {
    /// The repository's folder, as the caller named it.
    pub repository: String,
    /// Its binding as it holds its default branch, or why it could not be
    /// read.
    pub binding: BindingHeld,
    /// The merge queue it binds and whether that queue lands its default
    /// branch.
    pub merge_queue: MergeQueueBound,
    /// Each kind flume would run there: flume's own, the repository's at
    /// its default branch, and each its lock records from a host's
    /// source.
    pub kinds: Vec<KindLands>,
}

/// The merge queue a repository binds: its profile's name, none where it
/// names none; whether flume carries that profile and it lands the
/// default branch; and, where not, the words saying what is missing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueBound {
    pub profile: Option<String>,
    pub ready: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,
}

/// A kind flume would run in a repository: its identity, whether it lands
/// (none where flume could not read the kind), and the words a launch of
/// it there is refused with by the repository's binding, none where the
/// binding refuses it nothing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindLands {
    pub kind: String,
    pub lands: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refused: Option<String>,
}

impl Engine {
    /// Status of one place or every place, and of the host, with each of
    /// `repositories` read beside them (56, 58).
    pub fn status_of(&self, place: Option<&PlaceId>, repositories: &[String]) -> Answer {
        match self.status(place) {
            Answer::Status {
                places,
                host,
                repositories: _,
            } => Answer::Status {
                places,
                host,
                repositories: self.repositories(repositories),
            },
            refused => refused,
        }
    }

    /// Each of `repositories` read side by side, in their order.
    pub fn repositories(&self, repositories: &[String]) -> Vec<RepositoryStatus> {
        side_by_side(repositories, |repository| self.repository(repository))
    }

    /// `repository` as it holds its default branch.
    pub fn repository(&self, repository: &str) -> RepositoryStatus {
        let kinds = self.resolver.kinds();
        let binding = kinds.read_binding_held(repository);
        let merge_queue = self.merge_queue_bound(repository, &binding);
        let change = self.change_words(repository, &binding);
        let mut read = vec![];
        for own in kinds.flumes_own() {
            read.push((own.identity.spelled, Ok(own.kind)));
        }
        if let BindingHeld::Read { revision, .. } = &binding {
            read.extend(self.repositorys_kinds(repository, revision));
            read.extend(self.locked_kinds(repository, revision));
        }
        let kinds = read
            .into_iter()
            .map(|(kind, read)| match read {
                Ok(declared) => KindLands {
                    refused: refused(&kind, &declared, &merge_queue, change.as_deref()),
                    kind,
                    lands: Some(declared.lands),
                },
                Err(words) => KindLands {
                    kind,
                    lands: None,
                    refused: Some(words),
                },
            })
            .collect();
        RepositoryStatus {
            repository: repository.to_string(),
            binding,
            merge_queue,
            kinds,
        }
    }

    /// The merge queue `binding` names, and whether it lands the default
    /// branch the binding was read at.
    fn merge_queue_bound(&self, repository: &str, binding: &BindingHeld) -> MergeQueueBound {
        let unready = |profile: Option<String>, words: String| MergeQueueBound {
            profile,
            ready: false,
            words: Some(words),
        };
        let (branch, revision, declaration, named) = match binding {
            BindingHeld::Read {
                branch,
                revision,
                declaration,
                binding,
                ..
            } => (branch, revision, declaration, &binding.merge_queue),
            BindingHeld::CouldNotRead { reason } => {
                return unready(
                    None,
                    format!("the binding of {repository} could not be read: {reason}"),
                )
            }
        };
        let Some(name) = named else {
            let words = match (declaration, self.resolver.kinds().declaration_path()) {
                (Some(declaration), _) => {
                    format!("{declaration} at {branch} ({revision}) names no merge_queue in its [profiles]")
                }
                (None, Some(path)) => {
                    format!("{branch} ({revision}) holds no {path}, so nothing binds a merge queue")
                }
                (None, None) => format!(
                    "its kinds profile reads no declaration, so {repository} binds no merge queue"
                ),
            };
            return unready(None, words);
        };
        let Some(merge_queue) = self.resolver.merge_queue(name) else {
            return unready(
                Some(name.clone()),
                format!("no merge queue profile is named {name}"),
            );
        };
        match merge_queue.lands_on(repository, branch) {
            LandsOn::Lands => MergeQueueBound {
                profile: Some(name.clone()),
                ready: true,
                words: None,
            },
            LandsOn::Not { reason } => unready(
                Some(name.clone()),
                format!("{name} does not land {branch}: {reason}"),
            ),
            LandsOn::CouldNotRead { reason } => unready(
                Some(name.clone()),
                format!("{name} could not say whether it lands {branch}: {reason}"),
            ),
        }
    }

    /// The words a kind that reads a change is refused with by `binding`,
    /// none where it binds a change profile flume carries.
    fn change_words(&self, repository: &str, binding: &BindingHeld) -> Option<String> {
        match binding {
            BindingHeld::Read { binding, .. } => match &binding.change {
                Some(name) if self.resolver.change(name).is_some() => None,
                Some(name) => Some(format!("no change profile is named {name}")),
                None => Some(format!("{repository} binds no change profile")),
            },
            BindingHeld::CouldNotRead { reason } => Some(format!(
                "the binding of {repository} could not be read: {reason}"
            )),
        }
    }

    /// Each kind the repository declares at `revision`, by its identity.
    fn repositorys_kinds(
        &self,
        repository: &str,
        revision: &Revision,
    ) -> Vec<(String, Result<Kind, String>)> {
        let kinds = self.resolver.kinds();
        match kinds.read_kinds(repository, revision) {
            KindsRead::Read {
                kinds: declared, ..
            } => declared
                .into_iter()
                .map(|kind| {
                    let spelled = kinds
                        .identity_of(REPO_SOURCE, &kind.name, kind.version)
                        .map_or_else(
                            |_| format!("{REPO_SOURCE}/{}@{}", kind.name, kind.version),
                            |identity: KindIdentity| identity.spelled,
                        );
                    (spelled, Ok(kind))
                })
                .collect(),
            KindsRead::CouldNotRead { reason } => vec![(
                format!("{REPO_SOURCE}/*"),
                Err(format!(
                    "the kinds {repository} declares at {revision} could not be read: {reason}"
                )),
            )],
        }
    }

    /// Each kind the repository's lock at `revision` records from a
    /// source of the host's, resolved as a launch of it would be.
    fn locked_kinds(
        &self,
        repository: &str,
        revision: &Revision,
    ) -> Vec<(String, Result<Kind, String>)> {
        let entries = match self.resolver.kinds().read_lock_at(repository, revision) {
            LockRead::Read { entries } => entries,
            LockRead::None | LockRead::CouldNotRead { .. } => return vec![],
        };
        entries
            .into_iter()
            .filter(|entry| {
                ![OWN_SOURCE, REPO_SOURCE, LAUNCH_SOURCE].contains(&entry.kind.source.as_str())
            })
            .map(|entry| {
                let launch = Launch {
                    repository: repository.to_string(),
                    kind: entry.kind.spelled.clone(),
                    ..Launch::default()
                };
                let kind = self
                    .resolve_kind(&launch, revision)
                    .map(|resolved| resolved.kind);
                (entry.kind.spelled, kind)
            })
            .collect()
    }
}

/// The words a launch of `kind` is refused with by the repository's
/// binding, none where it refuses nothing.
fn refused(
    identity: &str,
    kind: &Kind,
    merge_queue: &MergeQueueBound,
    change: Option<&str>,
) -> Option<String> {
    if kind.lands && !merge_queue.ready {
        let words = merge_queue.words.as_deref().unwrap_or_default();
        return Some(format!(
            "the kind {identity} lands, and no merge queue lands it: {words}"
        ));
    }
    match change {
        Some(words) if reads_change(kind) => {
            Some(format!("the kind {identity} reads a change, and {words}"))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Defaults;
    use flume_fakes::*;
    use flume_seams::{Binding, Deliverable, HeldFrom, Settings, Stage};
    use std::sync::Arc;

    /// An engine over fresh fakes, with its kinds, resolver and the merge
    /// queue `queue` it carries.
    struct World {
        engine: Engine,
        kinds: Arc<FakeKinds>,
        queue: Arc<FakeMergeQueue>,
    }

    fn world() -> World {
        let git = FakeGit::new();
        let host = Arc::new(FakeHost::default());
        let kinds = Arc::new(FakeKinds::default());
        let resolver = Arc::new(FakeResolver::new(kinds.clone()));
        let queue = Arc::new(FakeMergeQueue::new(git.clone()));
        resolver.bind_merge_queue("queue", queue.clone());
        resolver.bind_change("change", Arc::new(FakeChange::new(git.clone())));
        let engine = Engine::new(
            resolver,
            Arc::new(FakeTerminal::new(host.clone())),
            Arc::new(FakeStore::default()),
            Arc::new(FakeHistory::new(git)),
            host,
            Defaults {
                settings: Settings::default(),
                send_backs: 2,
            },
        );
        kinds.set_own(vec![kind("fix", true, false)]);
        World {
            engine,
            kinds,
            queue,
        }
    }

    /// The kind `name`@1, landing where `lands`, its one stage writing a
    /// change where `change`.
    fn kind(name: &str, lands: bool, change: bool) -> Kind {
        let mut kind = crate::tests::record(Binding::default()).kind;
        kind.name = name.into();
        kind.lands = lands;
        kind.stages = vec![Stage {
            name: "work".into(),
            prompt: String::new(),
            conversation: false,
            deliverables: if change {
                vec![Deliverable::ChangeWritten]
            } else {
                vec![Deliverable::Commits]
            },
            agents: vec![],
            waits: false,
            quiet_limit: None,
            send_back_to: None,
            send_back_limit: None,
            makes: vec![],
            repeats: false,
            hibernates: false,
        }];
        kind
    }

    fn bound(merge_queue: Option<&str>, change: Option<&str>) -> Binding {
        Binding {
            place: None,
            change: change.map(str::to_string),
            merge_queue: merge_queue.map(str::to_string),
        }
    }

    fn held(declaration: bool, binding: Binding) -> BindingHeld {
        BindingHeld::Read {
            branch: "main".into(),
            revision: Revision::new("c1"),
            from: HeldFrom::Origin,
            declaration: declaration.then(|| ".config/flume.toml".to_string()),
            binding,
        }
    }

    #[test]
    fn a_repository_with_no_declaration_binds_no_merge_queue_and_names_the_file() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(false, Binding::default()));
        let status = world.engine.repository("/r");
        assert_eq!(status.merge_queue.profile, None);
        assert!(!status.merge_queue.ready);
        let words = status.merge_queue.words.unwrap();
        assert_eq!(
            words,
            "main (c1) holds no .config/flume.toml, so nothing binds a merge queue"
        );
        assert_eq!(
            status.kinds,
            vec![KindLands {
                kind: "flume/fix@1".into(),
                lands: Some(true),
                refused: Some(format!(
                    "the kind flume/fix@1 lands, and no merge queue lands it: {words}"
                )),
            }]
        );
    }

    #[test]
    fn a_declaration_naming_no_merge_queue_is_named() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(true, bound(None, Some("change"))));
        let status = world.engine.repository("/r");
        assert_eq!(
            status.merge_queue.words.as_deref(),
            Some(".config/flume.toml at main (c1) names no merge_queue in its [profiles]")
        );
    }

    #[test]
    fn a_bound_queue_that_lands_the_default_branch_is_ready_and_refuses_no_kind() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(true, bound(Some("queue"), Some("change"))));
        world
            .kinds
            .set_kinds_at("/r", &Revision::new("c1"), vec![kind("spike", false, true)]);
        let status = world.engine.repository("/r");
        assert_eq!(
            status.merge_queue,
            MergeQueueBound {
                profile: Some("queue".into()),
                ready: true,
                words: None,
            }
        );
        assert_eq!(
            status.kinds,
            vec![
                KindLands {
                    kind: "flume/fix@1".into(),
                    lands: Some(true),
                    refused: None,
                },
                KindLands {
                    kind: "repo/spike@1".into(),
                    lands: Some(false),
                    refused: None,
                },
            ]
        );
        assert!(
            world
                .queue
                .log
                .calls()
                .contains(&"lands_on /r main".to_string()),
            "the queue is asked of the default branch"
        );
    }

    #[test]
    fn a_queue_that_does_not_land_the_branch_gives_its_words() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(true, bound(Some("queue"), None)));
        world.queue.set_lands_on(
            "/r",
            "main",
            LandsOn::Not {
                reason: "no policy at main".into(),
            },
        );
        world
            .kinds
            .set_kinds_at("/r", &Revision::new("c1"), vec![kind("spec", false, true)]);
        let status = world.engine.repository("/r");
        assert_eq!(
            status.merge_queue,
            MergeQueueBound {
                profile: Some("queue".into()),
                ready: false,
                words: Some("queue does not land main: no policy at main".into()),
            }
        );
        assert_eq!(
            status.kinds[1].refused.as_deref(),
            Some("the kind repo/spec@1 reads a change, and /r binds no change profile"),
            "a kind that lands not is refused only for the change it reads"
        );
    }

    #[test]
    fn a_profile_flume_does_not_carry_is_no_merge_queue() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(true, bound(Some("other"), None)));
        let status = world.engine.repository("/r");
        assert_eq!(
            status.merge_queue,
            MergeQueueBound {
                profile: Some("other".into()),
                ready: false,
                words: Some("no merge queue profile is named other".into()),
            }
        );
    }

    #[test]
    fn a_binding_that_cannot_be_read_lists_flumes_own_kinds_refused() {
        let world = world();
        world.kinds.set_binding_held(
            "/r",
            BindingHeld::CouldNotRead {
                reason: "no default branch".into(),
            },
        );
        let status = world.engine.repository("/r");
        assert_eq!(
            status.merge_queue.words.as_deref(),
            Some("the binding of /r could not be read: no default branch")
        );
        assert_eq!(status.kinds.len(), 1);
        assert!(status.kinds[0].refused.is_some());
    }

    #[test]
    fn status_reads_each_repository_named_and_none_unnamed() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(false, Binding::default()));
        world
            .kinds
            .set_binding_held("/s", held(true, bound(Some("queue"), None)));
        let Answer::Status { repositories, .. } = world
            .engine
            .status_of(None, &["/r".to_string(), "/s".to_string()])
        else {
            panic!("a status");
        };
        assert_eq!(
            repositories
                .iter()
                .map(|status| (status.repository.as_str(), status.merge_queue.ready))
                .collect::<Vec<_>>(),
            vec![("/r", false), ("/s", true)]
        );
        let Answer::Status { repositories, .. } = world.engine.status(None) else {
            panic!("a status");
        };
        assert!(repositories.is_empty());
        let json = world.engine.status(None).to_json();
        assert!(!json.contains("repositories"), "{json}");
    }

    #[test]
    fn a_repository_read_fetches_and_launches_nothing() {
        let world = world();
        world
            .kinds
            .set_binding_held("/r", held(true, bound(Some("queue"), None)));
        world.engine.repository("/r");
        let kinds = world.kinds.log.calls();
        assert!(
            !kinds.iter().any(|entry| entry.starts_with("read_binding ")),
            "the launch's fetching read is not made: {kinds:?}"
        );
        assert_eq!(
            world.queue.log.calls(),
            vec!["lands_on /r main".to_string()],
            "the queue is asked nothing else"
        );
    }
}
