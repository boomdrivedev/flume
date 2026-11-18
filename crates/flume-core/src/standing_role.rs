//! A standing role (193, 212; design 2, 3, 5): a role whose kind has one
//! stage, a conversation its kind hibernates, its entry giving
//! `repositories`. Its wake reads, for each repository, its default
//! branch, its binding, and its base's revision where its merge queue's
//! profile says the base lives (9, 126, C.1), a repository whose default
//! branch, binding or base cannot be read refusing the wake naming it;
//! its kind is resolved in its first repository at that revision, as a
//! run's kind is. Its wake is refused, naming the address and why and
//! before anything is made, where its kind is no standing role's kind,
//! lands, its place committing nothing, or asks in its stage's prompt for
//! a part nothing fills for a place with no branch; and a wake whose
//! entry gives `repository` with a standing role's kind is refused naming
//! `repositories`, which a standing role's entry gives in its place (4,
//! 27, C.7; core.8).
//!
//! Its place is the folder the place seam gives a place of its first
//! repository named `<kind>--<name>`, under flume's own data directory,
//! holding a checkout of each repository at `<location>/<folder>`,
//! detached at its base's tip on no branch; its record's repository,
//! address, binding, base and base's revision are its first checkout's,
//! and its branch is empty. Its held state is written naming every
//! checkout before the place seam makes any, and a refused make removes
//! it, so the place is made whole or not at all; no readiness runs in a
//! checkout (2, 3, 83, C.1; core.17, core.40, core.47). `standing.rs`
//! keeps its own meaning, a standing agent flume did not open (132).

use crate::prompt::asks;
use crate::{Bound, Engine, Step};
use flume_seams::*;

/// Whether `kind` is a standing role's kind: one stage, a conversation
/// its kind hibernates (193, 212).
pub(crate) fn standing_role(kind: &Kind) -> bool {
    matches!(kind.stages.as_slice(), [stage] if stage.conversation && stage.hibernates)
}

/// The parts of a stage's prompt nothing fills for a place with no
/// branch: its branch, its base, its change and a report on its branch
/// (27, 193).
const NO_BRANCH: [PromptPart; 4] = [
    PromptPart::Branch,
    PromptPart::Base,
    PromptPart::Change,
    PromptPart::Report,
];

/// Why the role at `address`, whose entry gives `repositories` where
/// `standing`, wakes nothing as the kind `identity`: the kind no
/// standing role's where the entry gives `repositories`, a standing
/// role's where it gives `repository`, a standing role's kind that lands,
/// or one whose stage asks for a part nothing fills; none where it may
/// wake (4, 27, 193, C.7).
pub(crate) fn refused(
    address: &str,
    kind: &Kind,
    identity: &KindIdentity,
    standing: bool,
) -> Option<String> {
    let is = standing_role(kind);
    if standing && !is {
        return Some(format!(
            "{address}'s entry gives repositories, and the kind {identity} is no standing role's, \
             whose one stage is a conversation its kind hibernates: only a standing role's entry \
             gives repositories (193)"
        ));
    }
    if !standing && is {
        return Some(format!(
            "the kind {identity} is a standing role's, whose one stage is a conversation its kind \
             hibernates, and {address}'s entry gives repository: a standing role's entry gives \
             repositories in its place (193, C.7)"
        ));
    }
    if !standing {
        return None;
    }
    if kind.lands {
        return Some(format!(
            "the kind {identity} lands, and a standing role's kind lands nothing, since its place \
             commits nothing (193)"
        ));
    }
    kind.stages.iter().find_map(|stage| {
        NO_BRANCH
            .into_iter()
            .find(|part| asks(&stage.prompt, *part))
            .map(|part| {
                format!(
                    "the stage {} of the kind {identity} asks for {{{{{}}}}}, which nothing fills \
                     for a standing role's place, which has no branch (27, 193)",
                    stage.name,
                    part.name()
                )
            })
    })
}

impl Engine {
    /// The checkouts of the standing role `address`'s `repositories`, in
    /// their order, each at its repository's default branch's tip read
    /// where its merge queue's profile says the base lives, with its
    /// binding and its address as the wake read them; or why one cannot
    /// be read, naming its repository (9, 126, 193, C.1; core.55).
    pub(crate) fn checkouts_of(
        &self,
        address: &str,
        repositories: &[String],
    ) -> Result<Vec<CheckoutHeld>, String> {
        let kinds = self.resolver.kinds();
        repositories
            .iter()
            .map(|repository| {
                let unread = |why: String| {
                    format!("the standing role {address}'s repository {repository} {why}")
                };
                let base = self
                    .history
                    .default_branch(repository)
                    .ok_or_else(|| unread("has no default branch git names".into()))?;
                let binding = match kinds.read_binding(repository) {
                    BindingRead::Read { binding } => binding,
                    BindingRead::CouldNotRead { reason } => {
                        return Err(unread(format!(
                            "has a binding that could not be read: {reason}"
                        )))
                    }
                };
                let Bound { merge_queue, .. } = self
                    .bind(repository, &binding)
                    .map_err(|reason| unread(format!("could not be bound: {reason}")))?;
                let (revision, base_from) = self
                    .base_now(repository, &base, merge_queue.as_deref())
                    .map_err(|reason| unread(format!("could not be read: {reason}")))?;
                Ok(CheckoutHeld {
                    repository: repository.clone(),
                    address: self.repository_name(repository).address,
                    folder: checkout_folder(repository),
                    base,
                    binding,
                    revision,
                    base_from: Some(base_from),
                    moved_at: None,
                    unmoved: None,
                })
            })
            .collect()
    }

    /// The standing role's place `held`, its held state just written
    /// naming every checkout, made by the place seam `seam` whole or not
    /// at all, a refusal removing the held state and refusing the wake;
    /// then, with no readiness run, its first stage started unless a
    /// hold or its stage waits (2, 76, 83, 193; core.8, core.17,
    /// core.47).
    pub(crate) fn open_standing_role(
        &self,
        held: &HeldState,
        seam: &dyn Place,
    ) -> Result<HeldState, String> {
        let id = &held.place.id;
        let location = &held.place.location;
        let made = seam.make_checkouts(location, &checkouts(&held.place));
        let refused = match made {
            MakeOutcome::Made => None,
            MakeOutcome::LocationExists => Some(format!("something already stands at {location}")),
            MakeOutcome::BranchExists => Some(format!(
                "the checkouts at {location} could not be made: the place seam named a branch, and \
                 a standing role's checkouts are on none"
            )),
            MakeOutcome::Refused { reason } => Some(format!(
                "the standing role's checkouts could not be made: {reason}"
            )),
        };
        if let Some(reason) = refused {
            self.remove_held(id);
            return Err(reason);
        }
        let first = &held.place.kind.stages[0];
        if held.hold_on(&first.name).is_some() || first.waits {
            self.update(id, |held| {
                held.stages[0].state = StageState::OnHold;
                Step::Write(())
            });
        } else {
            self.start_stage(id, 0, None);
        }
        self.update(id, |held| {
            held.phase = Phase::Live;
            Step::Write(())
        });
        self.store
            .read(id)
            .map(|read| read.state)
            .ok_or_else(|| format!("the place {id} was removed while it was woken"))
    }
}

/// The checkouts of a standing role's place as the place seam is handed
/// them, each at the revision its held state names (193, C.1).
pub(crate) fn checkouts(place: &PlaceRecord) -> Vec<Checkout> {
    place
        .checkouts
        .iter()
        .map(|checkout| Checkout {
            repository: checkout.repository.clone(),
            folder: checkout.folder.clone(),
            revision: checkout.revision.clone(),
        })
        .collect()
}
