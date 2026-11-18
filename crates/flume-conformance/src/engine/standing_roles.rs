//! A standing role (193, 212 to 214; record 0069): a role whose kind's
//! one stage is a conversation its kind hibernates, kept as one place of
//! checkouts of the repositories its entry names, detached at each base's
//! tip and moved there at each start, committing nothing; its prompt
//! naming the kinds it launches its projects as, which report back to
//! its address, whose deliveries resume it.

use super::role_waking::{addresses, fix_for, runs};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::{FakeAgent, Plan};
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
        Scenario {
            name: "standing-roles: A file changed in a checkout: blueprints holding a change the move to main's new head would overwrite, a resume's agent opens with blueprints where it stood, its change kept, and the move not made held with when, the tip and the fake's words",
            cites: &["193", "213"],
            run: a_file_changed_in_a_checkout,
        },
        Scenario {
            name: "standing-roles: Moved at the next start: the change gone, the talk resumed again moves the checkout, and no move not made is held",
            cites: &["193", "213"],
            run: moved_at_the_next_start,
        },
        Scenario {
            name: "standing-roles: The kinds in the first prompt: east/ops' entry giving the kinds fix and change, its talk's first prompt gives, after the roles a stop-short may name, flume's words on launching by flume launch naming no recipient, then change and fix each with its words",
            cites: &["28", "193"],
            run: the_kinds_in_the_first_prompt,
        },
        Scenario {
            name: "standing-roles: A kind the entry does not name: the talk's agent launching a chore from its place, naming no recipient, is taken, its recipient east/ops",
            cites: &["7", "193"],
            run: a_kind_the_entry_does_not_name,
        },
        Scenario {
            name: "standing-roles: Two places launched and the role asleep: east/ops' talk launches a fix and a chore naming no recipient and hibernates; the chore's question resumes the talk as its prompt; once the talk hibernates again, the two ends resume it as one prompt; and while they ran, status gave both under east/ops (S128)",
            cites: &["7", "56", "132", "193", "213"],
            run: two_places_launched_and_the_role_asleep,
        },
        Scenario {
            name: "standing-roles: Finished, then woken again: a caller finishes east/ops' talk while it hibernates, the place is done and tidied, its checkouts removed, and an end later wakes desk--east-ops again, its checkouts at each base's tip then",
            cites: &["109", "114", "132", "193"],
            run: finished_then_woken_again,
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

/// An end comes for `east/ops`: a fix launched for it delivers its code,
/// its submission held by the merge queue so it waits on no caller, and
/// the settle that records its delivery wakes the address. The fix.
pub fn an_end_for_ops(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for(world, name, &[OPS])?;
    deliver_fix(world, &id);
    world.queue.plan_submits(vec![Plan::Hold]);
    world.settle(&id);
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
/// reaches (179, 193).
pub fn ops_talking(world: &World) -> Result<PlaceId, String> {
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
    Ok(id)
}

/// east/ops talking, then hibernated at its caller's word (193, 212).
pub fn ops_hibernated(world: &World) -> Result<PlaceId, String> {
    let id = ops_talking(world)?;
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

/// The fake's words for a move over a change in the checkout.
pub const IN_THE_WAY: &str = "your local changes to design.md would be overwritten by checkout";

/// east/ops hibernated, `/blueprints`' `main` moved on, and the fake
/// place told the blueprints checkout holds a change that move would
/// overwrite, then the talk resumed: the place and the tip (193, 213).
pub fn resumed_over_a_change(world: &World) -> Result<(PlaceId, Revision), String> {
    let id = ops_hibernated(world)?;
    world.host.advance(3600);
    let tip = world.git.commit(BLUEPRINTS, "main", &["design.md"]);
    let (blueprints, _) = checkout_at(world, &id, "blueprints");
    world.place.in_the_way(&blueprints, IN_THE_WAY);
    taken(&world.engine().resume(&id))?;
    Ok((id, tip))
}

fn a_file_changed_in_a_checkout(_: &()) -> Outcome {
    let world = World::new();
    let before = world.blueprints();
    let (id, tip) = resumed_over_a_change(&world)?;
    ensure_eq!(
        world
            .held(&id)
            .and_then(|held| held.stages[0].start.clone())
            .map(|start| start.identity),
        Some(Identity {
            resume: 1,
            ..talk_of(1)
        }),
        "the resume's start"
    );
    ensure_eq!(
        checkout_at(&world, &id, "blueprints").1,
        Some(before.clone()),
        "blueprints' checkout"
    );
    let held = world.held(&id).ok_or("no place")?;
    let blueprints = &held.place.checkouts[1];
    ensure_eq!(blueprints.revision, before, "blueprints' revision held");
    ensure_eq!(
        blueprints.unmoved,
        Some(Unmoved {
            at: world.host.now(),
            tip: Some(tip),
            reason: IN_THE_WAY.into(),
        }),
        "blueprints' move not made"
    );
    ensure_eq!(
        held.place.checkouts[0].moved_at,
        Some(world.host.now()),
        "the world's repository's checkout moved"
    );
    let (location, _) = checkout_at(&world, &id, "blueprints");
    ensure!(
        matches!(
            world.place.read(&held.place.checkouts[1].place_ref(&id.0)),
            PlaceRead::Live { holds, .. } if holds.any()
        ),
        "the change in {location} went"
    );
    Ok(())
}

fn moved_at_the_next_start(_: &()) -> Outcome {
    let world = World::new();
    let (id, tip) = resumed_over_a_change(&world)?;
    world.settle(&id);
    taken(&world.engine().hibernate(&id, false))?;
    let (blueprints, _) = checkout_at(&world, &id, "blueprints");
    world.place.out_of_the_way(&blueprints);
    world.host.advance(600);
    taken(&world.engine().resume(&id))?;
    ensure_eq!(
        checkout_at(&world, &id, "blueprints").1,
        Some(tip.clone()),
        "blueprints' checkout"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        (
            &held.place.checkouts[1].revision,
            held.place.checkouts[1].moved_at,
            &held.place.checkouts[1].unmoved
        ),
        (&tip, Some(world.host.now()), &None),
        "blueprints' revision, when it was moved and its move not made"
    );
    Ok(())
}

/// A message to `east/ops` `words`, as `flume message east/ops` sends
/// it: its delivery's key (53, 214).
pub fn message_ops(world: &World, words: &str) -> Result<String, String> {
    match world
        .engine()
        .message_address(OPS, words, None, &flume_core::Answering::default())
    {
        flume_core::Answer::Messaged { key, .. } => Ok(key),
        other => Err(format!("the message answered {other:?}")),
    }
}

/// A message to `east/ops` `words` recorded at the address with no act's
/// wake after it, as by an act whose wake never ran: its key (43, 214).
pub fn recorded_for_ops(world: &World, words: &str) -> Result<String, String> {
    let key = flume_core::message::message_key(OPS, "0a1b2c3d");
    let at = flume_core::deliveries::time(world.host.now());
    let message = flume_core::message::message_words(OPS, None, words);
    flume_core::deliveries::update(&*world.store, |records| {
        match weir_delivery::record(records, &key, &message, &[OPS], &at, 604_800) {
            Ok(weir_delivery::Recorded::Add(added)) => added,
            _ => vec![],
        }
    })?;
    Ok(key)
}

/// The hand-overs to `east/ops` of the delivery `key` the host's
/// deliveries hold now (43).
pub fn handovers_to_ops(world: &World, key: &str) -> usize {
    world.owing().deliveries.get(key).map_or(0, |delivery| {
        delivery
            .handovers
            .iter()
            .filter(|handover| handover.recipient == OPS)
            .count()
    })
}

/// `east/ops`' entry naming the kinds `fix` and `change` with their
/// words (193).
pub fn ops_with_kinds() -> RoleRun {
    RoleRun {
        kinds: BTreeMap::from([
            ("fix".to_string(), "a failing landing or check".to_string()),
            (
                "change".to_string(),
                "a change a design asks for".to_string(),
            ),
        ]),
        ..ops_run()
    }
}

fn the_kinds_in_the_first_prompt(_: &()) -> Outcome {
    let world = World::new();
    world.blueprints();
    declares(&world, vec![desk()], ops_with_kinds());
    // A role a stop-short may name, which the prompt lists before the
    // kinds (28, 41).
    world.host.set_stream("north", Some(Stream::default()));
    world.host.set_role(
        "north/design",
        Some(RoleRun {
            answers: Some("A decision the design has not made.".into()),
            repositories: vec![],
            ..ops_run()
        }),
    );
    an_end_for_ops(&world, "one")?;
    let id = ops_id();
    let prompt = world.prompt(&id, &talk_of(1)).ok_or("no prompt")?;
    let roles = prompt
        .find("- `north/design`: A decision the design has not made.")
        .ok_or_else(|| format!("the roles are not listed: {prompt}"))?;
    let words = prompt
        .find(flume_core::KINDS_WORDS)
        .ok_or_else(|| format!("flume's words on launching are not given: {prompt}"))?;
    ensure!(roles < words, "the kinds before the roles: {prompt}");
    ensure!(
        prompt.ends_with(&format!(
            "{}\n- `change`: a change a design asks for\n- `fix`: a failing landing or check",
            flume_core::KINDS_WORDS
        )),
        "the kinds as the prompt gives them: {prompt}"
    );
    ensure!(
        flume_core::KINDS_WORDS.contains("flume launch")
            && flume_core::KINDS_WORDS.contains("naming no recipient"),
        "flume's words: {}",
        flume_core::KINDS_WORDS
    );
    ensure!(
        crate::no_tool::hits(flume_core::KINDS_WORDS).is_empty(),
        "flume's words name a tool (core.1): {}",
        flume_core::KINDS_WORDS
    );
    Ok(())
}

fn a_kind_the_entry_does_not_name(_: &()) -> Outcome {
    let world = World::new();
    world.blueprints();
    declares(&world, vec![desk()], ops_with_kinds());
    an_end_for_ops(&world, "one")?;
    // The talk's command, whose environment names no recipient and names
    // the talk's place.
    world.host.set_recipient(None);
    let mut chore = launch("chore", "tidy", Some("tidy the notes"));
    chore.from_place = Some(ops_id());
    let place = launched(&world.engine.launch(&chore))?;
    let held = world.held(&place.place).ok_or("no chore")?;
    ensure_eq!(
        (
            held.place.recipients.clone(),
            held.place.recipients_from.clone()
        ),
        (vec![OPS.to_string()], Some(RecipientsFrom::RoleRun)),
        "the chore's recipients"
    );
    Ok(())
}

/// A place of `kind` named `name` launched from the talk's place, its
/// command naming no recipient and naming the talk's place, and begun
/// (7, 132).
fn launched_from_the_talk(world: &World, kind: &str, name: &str) -> Result<PlaceId, String> {
    world.host.set_recipient(None);
    let mut given = launch(kind, name, Some("the work"));
    given.from_place = Some(ops_id());
    let place = launched(&world.engine.launch(&given))?;
    world.settle(&place.place);
    ensure_eq!(
        world.stage_state(&place.place, "code"),
        Some(StageState::Begun),
        "{kind}--{name}'s code"
    );
    Ok(place.place)
}

/// The talk hibernated at its caller's word, its last resume's batch
/// first read received (212).
fn talk_hibernates(world: &World, id: &PlaceId) -> Outcome {
    world.settle(id);
    taken(&world.engine().hibernate(id, false))?;
    ensure_eq!(
        world.stage_state(id, "talk"),
        Some(StageState::Hibernating),
        "the talk once hibernated"
    );
    Ok(())
}

fn two_places_launched_and_the_role_asleep(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_talking(&world)?;
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    let fix = launched_from_the_talk(&world, "fix", "f")?;
    let chore = launched_from_the_talk(&world, "chore", "c")?;
    for place in [&fix, &chore] {
        ensure_eq!(
            world.held(place).map(|held| held.place.recipients),
            Some(vec![OPS.to_string()]),
            "{place}'s recipients"
        );
    }
    talk_hibernates(&world, &id)?;
    let ops = addresses(&world)?
        .into_iter()
        .find(|status| status.address == OPS)
        .ok_or("status gives no east/ops")?;
    ensure!(
        ops.reporting.contains(&fix) && ops.reporting.contains(&chore),
        "the places status gives under east/ops: {:?}",
        ops.reporting
    );
    // The chore's question resumes the talk, its prompt (41, 213, 214).
    let asked = "Which notes does the chore tidy?";
    super::open_sessions::stops_short(
        &world,
        &chore,
        0,
        &World::identity("chore", "c", "code", 1),
        asked,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&chore);
    let start = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    ensure_eq!(
        (
            start.identity.resume,
            start.resumes.last().map(|resume| resume.by)
        ),
        (1, Some(ResumedBy::Delivery)),
        "the talk's resume by the chore's question"
    );
    ensure!(
        start.prompt.text.contains("chore--c")
            && start.prompt.text.contains("needs an answer")
            && start.prompt.text.matches("flume confirm").count() == 1,
        "the resume's prompt: {}",
        start.prompt.text
    );
    taken(&world.engine().message(&chore, "The release notes.", None))?;
    talk_hibernates(&world, &id)?;
    // The host's room is held by the chore's agent while the fix
    // delivers, so the talk's resume waits for room until the chore
    // delivers too, and takes both ends as one prompt (70, 214).
    world.host.set_bound(Some(1));
    world.commit("fix/f", &["fix.rs"]);
    world.settle(&fix);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::WaitingForRoom),
        "the talk once the fix delivered"
    );
    world.commit("chore/c", &["notes.md"]);
    world.settle(&chore);
    world.settle(&id);
    let start = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    ensure_eq!(
        (
            start.identity.resume,
            start.resumes.last().map(|resume| resume.by)
        ),
        (2, Some(ResumedBy::Delivery)),
        "the talk's resume by the two ends"
    );
    let prompt = &start.prompt.text;
    ensure!(
        prompt.contains("fix--f") && prompt.contains("chore--c"),
        "the resume's prompt: {prompt}"
    );
    ensure_eq!(
        prompt.matches("flume confirm").count(),
        2,
        "the confirms the prompt gives"
    );
    Ok(())
}

fn finished_then_woken_again(_: &()) -> Outcome {
    let world = World::new();
    let id = ops_hibernated(&world)?;
    taken(&world.engine().finish(&id, false))?;
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the finished place stands");
    ensure_eq!(
        (
            checkout_at(&world, &id, "repository").1,
            checkout_at(&world, &id, "blueprints").1
        ),
        (None, None),
        "the checkouts once finished"
    );
    world.host.advance(3600);
    let main = world.git.commit(REPOSITORY, "main", &["ops.rs"]);
    let blueprints = world.git.commit(BLUEPRINTS, "main", &["design.md"]);
    an_end_for_ops(&world, "two")?;
    ensure_eq!(runs(&world), vec![id.clone()], "the runs woken again");
    ensure_eq!(
        (
            checkout_at(&world, &id, "repository").1,
            checkout_at(&world, &id, "blueprints").1
        ),
        (Some(main), Some(blueprints)),
        "the checkouts once woken again"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        held.stages[0]
            .start
            .as_ref()
            .map(|start| start.identity.clone()),
        Some(talk_of(1)),
        "the new place's talk"
    );
    Ok(())
}
