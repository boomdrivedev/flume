//! A standing role (193, 212 to 214; record 0069): a role whose kind's
//! one stage is a conversation its kind hibernates, kept as one place of
//! checkouts of the repositories its entry names, detached at each base's
//! tip and moved there at each start, committing nothing; its prompt
//! naming the kinds it launches its projects as, which report back to
//! its address, whose deliveries resume it.

use super::role_waking::{addresses, asks, fix_for, runs};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::FakeAgent;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "standing-roles: A landing kind: an entry naming a kind whose one stage is a hibernating conversation and that lands is not woken, status naming the address and that a standing role's kind lands nothing",
            cites: &["4", "193"],
            run: a_landing_kind,
        },
        Scenario {
            name: "standing-roles: A branch asked for: a standing role's talk asking for {{branch}} is not woken, status naming the part nothing fills for a place with no branch",
            cites: &["27", "193"],
            run: a_branch_asked_for,
        },
        Scenario {
            name: "standing-roles: Repositories for a run's kind: an entry giving repositories and the kind turn, no conversation, is not woken, status naming that only a standing role's entry gives repositories",
            cites: &["4", "193"],
            run: repositories_for_a_runs_kind,
        },
        Scenario {
            name: "standing-roles: One repository for a standing role's kind: an entry giving repository and the kind desk is not woken, status naming repositories",
            cites: &["4", "193"],
            run: one_repository_for_a_standing_roles_kind,
        },
        Scenario {
            name: "standing-roles: A standing role woken: an end for east/ops has the settle launch desk--east-ops, its role east/ops, its talk starting under the session name east/ops",
            cites: &["132", "193"],
            run: a_standing_role_woken,
        },
        Scenario {
            name: "standing-roles: Two checkouts: the fake place holds repository and blueprints under the place's location, each detached at its main's head, the place's branch empty, no readiness run, and the held state naming both with their revisions",
            cites: &["9", "193"],
            run: two_checkouts,
        },
        Scenario {
            name: "standing-roles: The base read where it lives: the world's repository's merge queue lands main ahead of the host's, and its checkout is detached at the merge queue's main, status giving where its base was read",
            cites: &["9", "126", "193"],
            run: the_base_read_where_it_lives,
        },
        Scenario {
            name: "standing-roles: A checkout refused: the fake place refusing /blueprints' checkout refuses the wake naming it and why, no checkout and no held state of the place standing, and the end that woke east/ops still pending",
            cites: &["2", "83", "193"],
            run: a_checkout_refused,
        },
        Scenario {
            name: "standing-roles: A repository with no default branch: git naming no default branch of /blueprints refuses the wake naming /blueprints, and nothing is made",
            cites: &["2", "9", "193"],
            run: a_repository_with_no_default_branch,
        },
        Scenario {
            name: "standing-roles: Resumed onto a newer tip: east/ops' talk hibernated, /blueprints' main gains a commit, and a caller's resume moves the blueprints checkout to the new head before the resume's agent opens, the other where it stood, each with its revision and when it was moved",
            cites: &["9", "193", "213"],
            run: resumed_onto_a_newer_tip,
        },
        Scenario {
            name: "standing-roles: A retry: east/ops' talk stopped, main moves, and the retry's fresh start opens its agent with each checkout at its base's tip",
            cites: &["49", "193"],
            run: a_retry,
        },
    ]
}

/// The standing role's address.
pub const OPS: &str = "east/ops";

/// `east/ops`'s entry as acme declares it: the kind `desk`, reading the
/// world's repository and `/blueprints`, the first its repository, and
/// naming no kinds (193).
pub fn ops_run() -> RoleRun {
    RoleRun {
        source: ACME.into(),
        repository: REPOSITORY.into(),
        repositories: vec![REPOSITORY.into(), BLUEPRINTS.into()],
        kind: "desk".into(),
        priority: 0,
        base: None,
        brief: None,
        name: None,
        intent: None,
        agents: RoleAgents::default(),
        answers: None,
        answerer: false,
        kinds: BTreeMap::new(),
    }
}

/// The standing role's place, `desk--east-ops`, at the location the
/// fake place gives the world's repository's.
pub fn ops_id() -> PlaceId {
    World::id("desk", "east-ops")
}

/// acme holds `kinds` and declares `run` for `east/ops`, the host
/// declaring `east` (132, 193).
pub fn declares(world: &World, kinds: Vec<Kind>, run: RoleRun) {
    world.acme_holds(kinds);
    world.host.set_stream("east", Some(Stream::default()));
    world.host.set_role(OPS, Some(run));
}

/// An end comes for `east/ops`: a fix launched for it asks, and the
/// settle that records its stop-short wakes the address. The fix.
pub fn an_end_for_ops(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for(world, name, &[OPS])?;
    asks(world, &id)?;
    Ok(id)
}

/// `east/ops` not woken for an end that came for it: no run made, no
/// checkout made, its wake refused naming the address and `named`, and
/// status giving the address with why (4, 132, 193).
fn not_woken(world: &World, named: &str) -> Outcome {
    world.blueprints();
    an_end_for_ops(world, "one")?;
    ensure_eq!(runs(world), Vec::<PlaceId>::new(), "runs woken");
    ensure_eq!(world.place.log.count("make_checkouts"), 0, "checkouts made");
    ensure!(world.held(&ops_id()).is_none(), "the place's held state");
    let ops = addresses(world)?
        .into_iter()
        .find(|status| status.address == OPS)
        .ok_or("status gives no east/ops")?;
    ensure!(
        ops.unwoken
            .as_deref()
            .is_some_and(|why| why.contains(OPS) && why.contains(named)),
        "why east/ops was not woken: {:?}",
        ops.unwoken
    );
    let reason = refused(&world.engine().wake(OPS))?;
    ensure!(
        reason.contains(named),
        "the carrier's wake refused as {reason}"
    );
    Ok(())
}

fn a_landing_kind(_: &()) -> Outcome {
    let world = World::new();
    let mut landing = desk();
    landing.lands = true;
    declares(&world, vec![landing], ops_run());
    not_woken(&world, "a standing role's kind lands nothing")
}

fn a_branch_asked_for(_: &()) -> Outcome {
    let world = World::new();
    let mut branched = desk();
    branched.stages[0].prompt = "Keep {{branch}}.\n\n{{needs}}".into();
    declares(&world, vec![branched], ops_run());
    not_woken(&world, "{{branch}}")
}

fn repositories_for_a_runs_kind(_: &()) -> Outcome {
    let world = World::new();
    let turn = kind(
        "turn",
        false,
        vec![stage(
            "plan",
            Some(Deliverable::Report {
                path: "{{state}}/turn.md".into(),
            }),
            "Take your turn; write {{report}}.",
        )],
    );
    declares(
        &world,
        vec![turn],
        RoleRun {
            kind: "turn".into(),
            ..ops_run()
        },
    );
    not_woken(&world, "only a standing role's entry gives repositories")
}

fn one_repository_for_a_standing_roles_kind(_: &()) -> Outcome {
    let world = World::new();
    declares(
        &world,
        vec![desk()],
        RoleRun {
            repositories: vec![],
            ..ops_run()
        },
    );
    not_woken(&world, "gives repositories in its place")
}

/// acme declares `east/ops` a standing role of the kind `desk` over the
/// world's repository and `/blueprints`, each on `main`, and an end
/// comes for it: the place its settle woke (132, 193).
pub fn ops_woken(world: &World) -> Result<PlaceId, String> {
    world.blueprints();
    declares(world, vec![desk()], ops_run());
    an_end_for_ops(world, "one")?;
    let id = ops_id();
    ensure_eq!(runs(world), vec![id.clone()], "the runs woken");
    Ok(id)
}

/// The checkout `folder` of the standing role's place `id` as the fake
/// place holds it: its location and the revision it stands at.
pub fn checkout_at(world: &World, id: &PlaceId, folder: &str) -> (String, Option<Revision>) {
    let location = format!("{}/{folder}", id.0);
    let revision = world.place.checkout(&location);
    (location, revision)
}

fn a_standing_role_woken(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_woken(&world)?;
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(held.role.as_deref(), Some(OPS), "the place's role");
    ensure_eq!(held.place.short(), "desk--east-ops", "the place");
    ensure!(
        matches!(
            world.stage_state(&id, "talk"),
            Some(StageState::Started | StageState::Begun)
        ),
        "the talk is {:?}",
        world.stage_state(&id, "talk")
    );
    let start = held.stages[0].start.as_ref().ok_or("no start")?;
    ensure_eq!(
        start.identity,
        World::identity("desk", "east-ops", "talk", 1),
        "the talk's identity"
    );
    ensure!(
        world.terminal.session_state(OPS).is_some(),
        "no session is named {OPS}"
    );
    Ok(())
}

fn two_checkouts(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_woken(&world)?;
    let held = world.held(&id).ok_or("no place")?;
    let main = world.git.branch_head(REPOSITORY, "main");
    let blueprints = world.git.branch_head(BLUEPRINTS, "main");
    ensure_eq!(
        checkout_at(&world, &id, "repository").1,
        main.clone(),
        "the world's repository's checkout"
    );
    ensure_eq!(
        checkout_at(&world, &id, "blueprints").1,
        blueprints.clone(),
        "blueprints' checkout"
    );
    ensure_eq!(held.place.branch, "", "the place's branch");
    ensure!(
        !world
            .place
            .log
            .calls()
            .iter()
            .any(|call| call.starts_with(&format!("ready {}", id.0))),
        "a readiness ran in the standing role's place"
    );
    ensure_eq!(
        world.place.log.count("make"),
        1,
        "worktrees made: the fix's alone"
    );
    ensure!(held.readiness.steps.is_empty(), "the place's readiness");
    let checkouts: Vec<(&str, &str, Option<Revision>)> = held
        .place
        .checkouts
        .iter()
        .map(|checkout| {
            (
                checkout.repository.as_str(),
                checkout.folder.as_str(),
                Some(checkout.revision.clone()),
            )
        })
        .collect();
    ensure_eq!(
        checkouts,
        vec![
            (REPOSITORY, "repository", main.clone()),
            (BLUEPRINTS, "blueprints", blueprints)
        ],
        "the held state's checkouts"
    );
    ensure_eq!(
        (
            held.place.repository.as_str(),
            Some(held.place.base_revision.clone())
        ),
        (REPOSITORY, main),
        "the place's repository and base's revision, its first checkout's"
    );
    Ok(())
}

fn the_base_read_where_it_lives(_: &()) -> Outcome {
    let world = World::new();
    let remote = world.git.commit(REPOSITORY, "remote-main", &["remote.rs"]);
    world.queue.set_base(
        REPOSITORY,
        "main",
        BaseRevision::At {
            revision: remote.clone(),
        },
    );
    let id = ops_woken(&world)?;
    ensure_eq!(
        checkout_at(&world, &id, "repository").1,
        Some(remote),
        "the world's repository's checkout"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        held.place.checkouts[0].base_from,
        Some(BaseFrom::MergeQueue),
        "where the first checkout's base was read"
    );
    ensure_eq!(
        held.place.checkouts[1].base_from,
        Some(BaseFrom::NoOrigin),
        "where blueprints' base was read"
    );
    ensure_eq!(
        world.status(&id)?.base_from,
        Some(BaseFrom::MergeQueue),
        "where status gives the place's base was read"
    );
    Ok(())
}

fn a_checkout_refused(_: &()) -> Outcome {
    let world = World::new();
    world.blueprints();
    declares(&world, vec![desk()], ops_run());
    world.place.refuse_checkouts(BLUEPRINTS, "the disk is full");
    an_end_for_ops(&world, "one")?;
    let id = ops_id();
    ensure!(world.held(&id).is_none(), "the place's held state stands");
    ensure!(!world.place.stands(&id.0), "something stands at the place");
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs woken");
    let ops = addresses(&world)?
        .into_iter()
        .find(|status| status.address == OPS)
        .ok_or("status gives no east/ops")?;
    ensure_eq!(ops.pending.len(), 1, "the ends pending for east/ops");
    ensure!(
        ops.unwoken
            .as_deref()
            .is_some_and(|why| why.contains(BLUEPRINTS) && why.contains("the disk is full")),
        "why east/ops was not woken: {:?}",
        ops.unwoken
    );
    Ok(())
}

fn a_repository_with_no_default_branch(_: &()) -> Outcome {
    let world = World::new();
    declares(&world, vec![desk()], ops_run());
    let reason = refused(&world.engine.wake(OPS))?;
    ensure!(
        reason.contains(BLUEPRINTS) && reason.contains("default branch"),
        "the wake refused as {reason}"
    );
    ensure!(world.held(&ops_id()).is_none(), "the place's held state");
    ensure_eq!(world.place.log.count("make_checkouts"), 0, "checkouts made");
    Ok(())
}

/// The standing role's talk's start `start`.
pub fn talk_of(start: u32) -> Identity {
    World::identity("desk", "east-ops", "talk", start)
}

/// east/ops woken and its talk begun, its first start's word naming the
/// session `s-1`, which the login environment's configuration home
/// reaches, then hibernated at its caller's word (193, 212).
pub fn ops_hibernated(world: &World) -> Result<PlaceId, String> {
    world.agent.set_session(&talk_of(1), "s-1");
    world.agent.carry_to(
        FakeAgent::LOGIN,
        Carry::Yes {
            session: "s-1".into(),
        },
    );
    let id = ops_woken(world)?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Begun),
        "the talk once woken"
    );
    taken(&world.engine().hibernate(&id, false))?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Hibernating),
        "the talk once hibernated"
    );
    Ok(id)
}

/// The act taken, or why it was refused.
pub fn taken(answer: &flume_core::Answer) -> Outcome {
    match answer.refusal() {
        Some(reason) => Err(format!("the act was refused: {reason}")),
        None => Ok(()),
    }
}

/// The revision and when it was moved of each checkout the place's held
/// state names, in order.
fn moves(world: &World, id: &PlaceId) -> Result<Vec<(Revision, Option<Seconds>)>, String> {
    Ok(world
        .held(id)
        .ok_or("no place")?
        .place
        .checkouts
        .into_iter()
        .map(|checkout| (checkout.revision, checkout.moved_at))
        .collect())
}

fn resumed_onto_a_newer_tip(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_hibernated(&world)?;
    let before = moves(&world, &id)?;
    let woken_at = before[0].1;
    world.host.advance(3600);
    let tip = world.git.commit(BLUEPRINTS, "main", &["design.md"]);
    let opens = world.terminal.log.count("open");
    taken(&world.engine().resume(&id))?;
    ensure_eq!(
        world.terminal.log.count("open"),
        opens + 1,
        "agents opened by the resume"
    );
    let now = world.host.now();
    ensure_eq!(
        checkout_at(&world, &id, "blueprints").1,
        Some(tip.clone()),
        "blueprints' checkout"
    );
    ensure_eq!(
        checkout_at(&world, &id, "repository").1,
        Some(before[0].0.clone()),
        "the world's repository's checkout"
    );
    ensure_eq!(
        moves(&world, &id)?,
        vec![(before[0].0.clone(), Some(now)), (tip, Some(now))],
        "each checkout's revision and when it was moved"
    );
    ensure!(
        woken_at.is_some_and(|at| at < now),
        "moved at the wake's start"
    );
    let start = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    ensure_eq!(start.identity.resume, 1, "the talk's start resumed");
    ensure_eq!(start.started_at, now, "when the resume started");
    Ok(())
}

fn a_retry(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_woken(&world)?;
    world.settle(&id);
    taken(&world.engine().stop(&id, true))?;
    world.host.advance(600);
    let main = world.git.commit(REPOSITORY, "main", &["ops.rs"]);
    let blueprints = world.git.commit(BLUEPRINTS, "main", &["design.md"]);
    taken(&world.engine().retry(&id, Retry::Current, None, false))?;
    let held = world.held(&id).ok_or("no place")?;
    let start = held.stages[0].start.as_ref().ok_or("no start")?;
    ensure_eq!(start.identity, talk_of(2), "the retry's fresh start");
    ensure_eq!(
        (
            checkout_at(&world, &id, "repository").1,
            checkout_at(&world, &id, "blueprints").1
        ),
        (Some(main.clone()), Some(blueprints.clone())),
        "the checkouts as the fresh start opened"
    );
    ensure_eq!(
        moves(&world, &id)?,
        vec![
            (main, Some(world.host.now())),
            (blueprints, Some(world.host.now()))
        ],
        "each checkout's revision and when it was moved"
    );
    Ok(())
}
