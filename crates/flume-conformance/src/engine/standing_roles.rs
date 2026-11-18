//! A standing role (193, 212 to 214; record 0069): a role whose kind's
//! one stage is a conversation its kind hibernates, kept as one place of
//! checkouts of the repositories its entry names, detached at each base's
//! tip and moved there at each start, committing nothing; its prompt
//! naming the kinds it launches its projects as, which report back to
//! its address, whose deliveries resume it.

use super::role_waking::{addresses, asks, fix_for, runs};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
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
/// declaring `east`, and `/blueprints` stands on `main` (132, 193).
pub fn declares(world: &World, kinds: Vec<Kind>, run: RoleRun) {
    world.blueprints();
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
