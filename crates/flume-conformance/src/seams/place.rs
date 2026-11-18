//! The place seam's scenarios (B.1, 78 to 83).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::path::Path;

/// What the place scenarios need beyond the seam: a repository to make
/// places of, and the world arranged around a place.
pub trait PlaceHarness {
    type Seam: Place + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A repository, its base branch and the base's head.
    fn repository(&self) -> (String, String, Revision);

    /// A branch made at a commit of its own, as by a hand.
    fn branch(&self, repository: &str, branch: &str) -> Revision;

    /// A commit on a place's branch, made in it.
    fn commit(&self, place: &PlaceRef) -> Revision;

    fn head(&self, repository: &str, branch: &str) -> Option<Revision>;

    /// Something at `location` that no make put there.
    fn occupy(&self, location: &str);

    /// The repository's readiness: each step, passing or failing.
    fn readiness(&self, repository: &str, steps: &[(&str, bool)]);

    /// The repository pins the versions of tools a step of its readiness
    /// runs, and nothing on the host has installed them, as in a fresh
    /// clone: what its readiness then runs; none where the profile reads
    /// no pins.
    fn pinned(&self, repository: &str) -> Option<Pinned>;

    /// A file in the place neither committed nor ignored.
    fn untracked(&self, place: &PlaceRef, file: &str);

    /// The place's teardown fails at `step`.
    fn fail_teardown(&self, place: &PlaceRef, step: &str);

    /// The base takes `commit`'s change by a commit of its own.
    fn apply_to_base(&self, repository: &str, base: &str, commit: &Revision);

    /// A second repository, beside [`PlaceHarness::repository`], with its
    /// base branch and the base's head, for a standing role's checkouts
    /// (193).
    fn second_repository(&self) -> (String, String, Revision);

    /// The checkout `checkout` holds an uncommitted change to a file its
    /// repository's base then changes by a commit of its own: the base's
    /// new head, which a move there would overwrite the change by (193).
    fn in_the_way(&self, checkout: &PlaceRef) -> Revision;

    /// Whether anything stands at `location`.
    fn stands(&self, location: &str) -> bool;
}

/// A readiness over tools a repository pins that nothing installed.
pub struct Pinned {
    /// Each step it runs, in order.
    pub steps: Vec<String>,
    /// The step among them that installs the pins.
    pub install: String,
    /// What the install says it installed.
    pub said: String,
}

/// The location the seam gives the place `fix--<name>` of `repository`.
fn location<H: PlaceHarness>(harness: &H, repository: &str, name: &str) -> Result<String, String> {
    harness
        .seam()
        .location(repository, &format!("fix--{name}"))
        .map_err(|reason| format!("no location for fix--{name}: {reason}"))
}

fn made<H: PlaceHarness>(harness: &H, name: &str) -> Result<(PlaceRef, Revision), String> {
    let (repository, base, revision) = harness.repository();
    let place = PlaceRef {
        location: location(harness, &repository, name)?,
        branch: format!("fix/{name}"),
        repository: repository.clone(),
        base,
    };
    let outcome = harness
        .seam()
        .make(&repository, &revision, &place.branch, &place.location);
    ensure_eq!(outcome, MakeOutcome::Made, "make");
    Ok((place, revision))
}

// ---- the place seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: PlaceHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "place: each place's location is its own, and the same at every ask",
            cites: &["3", "78"],
            run: a_location_is_the_places_own::<H>,
        },
        Scenario {
            name: "place: a place's location lies outside its repository's directory",
            cites: &["3", "78"],
            run: a_location_outside_the_repository::<H>,
        },
        Scenario {
            name: "place: make cuts a new branch at the revision",
            cites: &["78", "9"],
            run: make_cuts_a_branch::<H>,
        },
        Scenario {
            name: "place: make refuses an existing branch and touches it not",
            cites: &["78", "83", "3"],
            run: make_refuses_an_existing_branch::<H>,
        },
        Scenario {
            name: "place: make refuses an existing location and makes no branch",
            cites: &["78", "83", "3"],
            run: make_refuses_an_existing_location::<H>,
        },
        Scenario {
            name: "place: ready runs each step in order, stopping at the first that fails",
            cites: &["79", "11", "12"],
            run: ready_stops_at_the_first_failure::<H>,
        },
        Scenario {
            name: "place: ready installs what the repository pins, a step of its own with its words, before the step that runs it",
            cites: &["79", "11", "12"],
            run: ready_installs_the_pins::<H>,
        },
        Scenario {
            name: "place: read gives the branch, the head and what the place holds",
            cites: &["80", "65"],
            run: read_names_what_it_holds::<H>,
        },
        Scenario {
            name: "place: remove runs the teardown, or keeps the place naming the step",
            cites: &["81", "66", "83"],
            run: remove_runs_the_teardown::<H>,
        },
        Scenario {
            name: "place: remove-branch keeps a branch holding a change its base lacks",
            cites: &["82", "67", "I4"],
            run: remove_branch_judges_by_change::<H>,
        },
        Scenario {
            name: "place: a standing role's checkouts are made under the location, each detached at its revision on no branch",
            cites: &["78", "80", "193"],
            run: checkouts_made_under_the_location::<H>,
        },
        Scenario {
            name: "place: make of checkouts refuses where something stands at the location, and makes none",
            cites: &["78", "83", "193"],
            run: checkouts_refused_where_something_stands::<H>,
        },
        Scenario {
            name: "place: a checkout's move over a change is kept with the profile's words, its head and its change as they were",
            cites: &["80", "83", "193"],
            run: a_move_kept_over_a_change::<H>,
        },
        Scenario {
            name: "place: a standing role's checkouts are removed with what stands at the location, no teardown run and no branch removed",
            cites: &["81", "83", "193"],
            run: checkouts_removed_with_no_teardown::<H>,
        },
    ]
}

/// The location of the standing role's place `desk--<name>` of
/// `repository`, and the checkouts of the harness's two repositories at
/// their bases' heads under it, named `one` and `two`.
fn checkouts<H: PlaceHarness>(harness: &H, name: &str) -> Result<(String, Vec<Checkout>), String> {
    let (one, _, at_one) = harness.repository();
    let (two, _, at_two) = harness.second_repository();
    let location = harness
        .seam()
        .location(&one, &format!("desk--{name}"))
        .map_err(|reason| format!("no location for desk--{name}: {reason}"))?;
    Ok((
        location,
        vec![
            Checkout {
                repository: one,
                folder: "one".into(),
                revision: at_one,
            },
            Checkout {
                repository: two,
                folder: "two".into(),
                revision: at_two,
            },
        ],
    ))
}

/// The checkout `checkout` under `location` as the seam reads it.
fn checkout_at(location: &str, checkout: &Checkout) -> PlaceRef {
    PlaceRef {
        repository: checkout.repository.clone(),
        location: format!("{location}/{}", checkout.folder),
        branch: String::new(),
        base: "main".into(),
    }
}

/// The checkouts made under their location.
fn checkouts_made<H: PlaceHarness>(
    harness: &H,
    name: &str,
) -> Result<(String, Vec<Checkout>), String> {
    let (location, checkouts) = checkouts(harness, name)?;
    ensure_eq!(
        harness.seam().make_checkouts(&location, &checkouts),
        MakeOutcome::Made,
        "make the checkouts"
    );
    Ok((location, checkouts))
}

fn checkouts_made_under_the_location<H: PlaceHarness>(harness: &H) -> Outcome {
    let (location, checkouts) = checkouts_made(harness, "made")?;
    for checkout in &checkouts {
        let place = checkout_at(&location, checkout);
        match harness.seam().read(&place) {
            PlaceRead::Live {
                branch,
                head,
                holds,
            } => {
                ensure_eq!(
                    head,
                    checkout.revision,
                    format!("{}'s head", checkout.folder)
                );
                ensure!(
                    harness.head(&checkout.repository, &branch).is_none(),
                    "{} is on the branch {branch}",
                    checkout.folder
                );
                ensure!(!holds.any(), "a made checkout holds {holds:?}");
            }
            other => return Err(format!("read {} as {other:?}", checkout.folder)),
        }
    }
    let held = checkout_at(&location, &checkouts[1]);
    harness.untracked(&held, "notes.txt");
    match harness.seam().read(&held) {
        PlaceRead::Live { holds, .. } => ensure!(
            holds
                .untracked
                .iter()
                .any(|file| file.ends_with("notes.txt")),
            "read names no untracked file: {holds:?}"
        ),
        other => return Err(format!("read a held checkout as {other:?}")),
    }
    Ok(())
}

fn checkouts_refused_where_something_stands<H: PlaceHarness>(harness: &H) -> Outcome {
    let (location, checkouts) = checkouts(harness, "occupied")?;
    harness.occupy(&location);
    ensure_eq!(
        harness.seam().make_checkouts(&location, &checkouts),
        MakeOutcome::LocationExists,
        "make over a location"
    );
    for checkout in &checkouts {
        ensure_eq!(
            harness.seam().read(&checkout_at(&location, checkout)),
            PlaceRead::Gone,
            format!("{} after a refused make", checkout.folder)
        );
    }
    Ok(())
}

fn a_move_kept_over_a_change<H: PlaceHarness>(harness: &H) -> Outcome {
    let (location, checkouts) = checkouts_made(harness, "kept")?;
    let place = checkout_at(&location, &checkouts[0]);
    let tip = harness.in_the_way(&place);
    match harness.seam().move_checkout(&place, &tip) {
        Moved::Kept { reason } => ensure!(!reason.is_empty(), "kept with no words"),
        other => return Err(format!("a move over a change was {other:?}")),
    }
    match harness.seam().read(&place) {
        PlaceRead::Live { head, holds, .. } => {
            ensure_eq!(head, checkouts[0].revision, "the kept checkout's head");
            ensure!(!holds.uncommitted.is_empty(), "the change went: {holds:?}");
        }
        other => return Err(format!("read a kept checkout as {other:?}")),
    }
    let elsewhere = PlaceRef {
        location: format!("{location}/elsewhere"),
        ..place
    };
    ensure_eq!(
        harness.seam().move_checkout(&elsewhere, &tip),
        Moved::Gone,
        "a move where no checkout stands"
    );
    Ok(())
}

fn checkouts_removed_with_no_teardown<H: PlaceHarness>(harness: &H) -> Outcome {
    let (location, checkouts) = checkouts_made(harness, "removed")?;
    let (repository, base, _) = harness.repository();
    let kept = harness.branch(&repository, "fix/kept");
    let head = harness.head(&repository, &base);
    for checkout in &checkouts {
        harness.fail_teardown(&checkout_at(&location, checkout), "pre-remove");
    }
    ensure_eq!(
        harness.seam().remove_checkouts(&location, &checkouts),
        Removed::Removed,
        "remove the checkouts"
    );
    for checkout in &checkouts {
        ensure_eq!(
            harness.seam().read(&checkout_at(&location, checkout)),
            PlaceRead::Gone,
            format!("{} once removed", checkout.folder)
        );
    }
    ensure!(!harness.stands(&location), "something stands at {location}");
    ensure_eq!(
        harness.head(&repository, "fix/kept"),
        Some(kept),
        "a branch by hand"
    );
    ensure_eq!(harness.head(&repository, &base), head, "the base");
    Ok(())
}

fn a_location_is_the_places_own<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.repository();
    let one = location(harness, &repository, "one")?;
    let two = location(harness, &repository, "two")?;
    ensure!(one != two, "two places given one location: {one}");
    ensure_eq!(
        location(harness, &repository, "one")?,
        one,
        "the location asked again"
    );
    Ok(())
}

fn a_location_outside_the_repository<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.repository();
    let location = location(harness, &repository, "outside")?;
    ensure!(
        !Path::new(&location).starts_with(&repository),
        "the place is in its repository's directory: {location}"
    );
    Ok(())
}

fn make_cuts_a_branch<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, revision) = made(harness, "cut")?;
    ensure_eq!(
        harness.head(&place.repository, &place.branch),
        Some(revision.clone()),
        "the new branch's head"
    );
    match harness.seam().read(&place) {
        PlaceRead::Live { branch, head, .. } => {
            ensure_eq!(branch, place.branch, "the place's branch");
            ensure_eq!(head, revision, "the place's head");
            Ok(())
        }
        other => Err(format!("read a made place as {other:?}")),
    }
}

fn make_refuses_an_existing_branch<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, revision) = harness.repository();
    let held = harness.branch(&repository, "fix/taken");
    let location = location(harness, &repository, "taken")?;
    let outcome = harness
        .seam()
        .make(&repository, &revision, "fix/taken", &location);
    ensure_eq!(outcome, MakeOutcome::BranchExists, "make over a branch");
    ensure_eq!(
        harness.head(&repository, "fix/taken"),
        Some(held),
        "the existing branch's head"
    );
    let place = PlaceRef {
        repository,
        location,
        branch: "fix/taken".into(),
        base: "main".into(),
    };
    ensure_eq!(harness.seam().read(&place), PlaceRead::Gone, "the location");
    Ok(())
}

fn make_refuses_an_existing_location<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, revision) = harness.repository();
    let location = location(harness, &repository, "occupied")?;
    harness.occupy(&location);
    let outcome = harness
        .seam()
        .make(&repository, &revision, "fix/occupied", &location);
    ensure_eq!(outcome, MakeOutcome::LocationExists, "make over a location");
    ensure_eq!(
        harness.head(&repository, "fix/occupied"),
        None,
        "the branch a refused make named"
    );
    Ok(())
}

fn ready_stops_at_the_first_failure<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, revision) = made(harness, "ready")?;
    let none = harness.seam().ready(&place, &revision);
    ensure!(
        none.steps.is_empty(),
        "a repository declaring no readiness ran {:?}",
        none.steps
    );
    harness.readiness(
        &place.repository,
        &[("toolchain", true), ("setup", false), ("seed", true)],
    );
    let readiness = harness.seam().ready(&place, &revision);
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    ensure_eq!(
        steps,
        vec![("toolchain", true), ("setup", false)],
        "the steps run"
    );
    ensure!(
        readiness.failed().is_some_and(|step| step.step == "setup"),
        "the failed step is not named"
    );
    Ok(())
}

fn ready_installs_the_pins<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.repository();
    let Some(pinned) = harness.pinned(&repository) else {
        return Ok(());
    };
    let (place, revision) = made(harness, "pinned")?;
    let readiness = harness.seam().ready(&place, &revision);
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    let due: Vec<(&str, bool)> = pinned
        .steps
        .iter()
        .map(|step| (step.as_str(), true))
        .collect();
    ensure_eq!(steps, due, "the steps run");
    let install = readiness
        .steps
        .iter()
        .position(|step| step.step == pinned.install);
    ensure!(
        install.is_some_and(|at| at + 1 < readiness.steps.len()),
        "no step after the install {:?} ran: {steps:?}",
        pinned.install
    );
    let words = install.map(|at| readiness.steps[at].words.as_str());
    ensure!(
        words.is_some_and(|words| words.contains(&pinned.said)),
        "the install's words {words:?} do not say {:?}",
        pinned.said
    );
    Ok(())
}

fn read_names_what_it_holds<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, _) = made(harness, "holds")?;
    harness.untracked(&place, "notes.txt");
    match harness.seam().read(&place) {
        PlaceRead::Live { holds, .. } => {
            ensure!(
                holds
                    .untracked
                    .iter()
                    .any(|file| file.ends_with("notes.txt")),
                "read names no untracked file: {holds:?}"
            );
            Ok(())
        }
        other => Err(format!("read a made place as {other:?}")),
    }
}

fn remove_runs_the_teardown<H: PlaceHarness>(harness: &H) -> Outcome {
    let (kept, _) = made(harness, "kept")?;
    harness.fail_teardown(&kept, "pre-remove");
    match harness.seam().remove(&kept) {
        Removed::Refused { step, .. } => ensure_eq!(step, "pre-remove", "the failed step"),
        other => return Err(format!("a failing teardown removed as {other:?}")),
    }
    ensure!(
        matches!(harness.seam().read(&kept), PlaceRead::Live { .. }),
        "a refused remove left the place half removed"
    );
    let (place, _) = made(harness, "gone")?;
    ensure_eq!(harness.seam().remove(&place), Removed::Removed, "remove");
    ensure_eq!(
        harness.seam().read(&place),
        PlaceRead::Gone,
        "a removed place"
    );
    Ok(())
}

fn remove_branch_judges_by_change<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, _) = made(harness, "branch")?;
    let commit = harness.commit(&place);
    ensure_eq!(harness.seam().remove(&place), Removed::Removed, "remove");
    match harness
        .seam()
        .remove_branch(&place.repository, &place.branch, &place.base)
    {
        BranchRemoved::Kept { commits } => {
            ensure!(
                commits.contains(&commit),
                "kept naming {commits:?}, not {commit}"
            )
        }
        other => {
            return Err(format!(
                "a branch holding a change its base lacks: {other:?}"
            ))
        }
    }
    ensure!(
        harness.head(&place.repository, &place.branch).is_some(),
        "a kept branch is gone"
    );
    harness.apply_to_base(&place.repository, &place.base, &commit);
    ensure_eq!(
        harness
            .seam()
            .remove_branch(&place.repository, &place.branch, &place.base),
        BranchRemoved::Removed,
        "a branch whose change its base holds"
    );
    ensure_eq!(
        harness.head(&place.repository, &place.branch),
        None,
        "a removed branch"
    );
    Ok(())
}
