//! A caller's hold and keyed answers through the command line over the
//! fakes (command-line spec, "needs --to, hold --waiting, retry --answers
//! and message --answers"): each command's words parsed as flume's binary
//! parses them and asked of one core built over the conformance world,
//! whose change `plan` in `east` waits on construct's stop-short naming
//! `ops`. The fakes keep construct's agent open for its answer, which the
//! printed command cannot (41, 42, 49, 53, 61, 130, 198).

use flume::commands;
use flume_conformance::engine::caller_acts::asks_ops;
use flume_conformance::engine::role_waking::{configure_at, planner_run};
use flume_conformance::engine::world::{launch, launched, World};
use flume_core::{Answer, PlaceStatus};
use flume_seams::*;
use serde_json::Value;

/// The key of construct's first stop-short, as the inbox and status give
/// it (42, 43).
const KEY: &str = "stage/construct/1/ended-short";

/// What flume printed for `words`, asked of a core built over `world`
/// in an environment that sets no variable, and its exit status.
fn run(world: &World, words: &[&str]) -> (String, u8) {
    run_in(world, &[], words)
}

/// What flume printed for `words` in the environment `env`, asked of a
/// core built over `world`, and its exit status.
fn run_in(world: &World, env: &[(&str, &str)], words: &[&str]) -> (String, u8) {
    let outcome = commands::run(
        std::iter::once("flume").chain(words.iter().copied()),
        &|name| {
            env.iter()
                .find(|(given, _)| *given == name)
                .map(|(_, value)| value.to_string())
        },
        &|| Ok(world.engine()),
    );
    (outcome.text, outcome.code)
}

/// The answer `words` printed as its JSON, exiting 0.
fn answered(world: &World, words: &[&str]) -> Answer {
    let (text, code) = run(world, words);
    assert_eq!(code, 0, "{words:?}: {text}");
    serde_json::from_str(text.trim()).unwrap_or_else(|error| panic!("{error}: {text}"))
}

/// The refusal's reason `words` printed in the JSON form, exiting 1.
fn refused(world: &World, words: &[&str]) -> String {
    let (text, code) = run(world, words);
    assert_eq!(code, 1, "{words:?}: {text}");
    let answer: Value = serde_json::from_str(text.trim()).expect("the refusal's JSON");
    answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("no reason: {text}"))
        .to_string()
}

/// The place's status an act answered with, the act checked.
fn acted(answer: Answer, act: Act) -> PlaceStatus {
    match answer {
        Answer::Acted {
            act: done,
            status: Some(status),
            ..
        } if done == act => status,
        other => panic!("{act:?} answered {other:?}"),
    }
}

/// The answer the end keyed construct's first stop-short holds, as status
/// gives it.
fn answer_on(status: &PlaceStatus) -> Option<AnswerHeld> {
    status
        .ends
        .iter()
        .find(|end| end.key.to_string() == KEY)
        .unwrap_or_else(|| panic!("no end keyed {KEY}: {:?}", status.ends))
        .end
        .answer
        .clone()
}

/// The fake terminal's closes of the change `plan`'s agents.
fn closes(world: &World) -> usize {
    world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| call.starts_with("close") && call.contains("change--plan"))
        .count()
}

#[test]
fn a_hold_on_the_waiting_move_then_a_retry_naming_its_key() {
    // The spec's "The forms" (41, 42, 49, 130): the hold answers with its
    // words and bound, and the retry with the place's status, its answer
    // recorded on the stop-short with its key and the hold ended.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let place = &id.to_string();
    let held = acted(
        answered(
            &world,
            &[
                "hold",
                place,
                "on the user's card",
                "--waiting",
                "--for",
                "1h",
                "--json",
            ],
        ),
        Act::Hold,
    );
    let hold = held
        .holds
        .iter()
        .find(|hold| hold.on == HoldOn::Waiting)
        .unwrap_or_else(|| panic!("no hold on the waiting move: {:?}", held.holds));
    assert_eq!(
        (hold.words.as_deref(), hold.bound, hold.lapsed),
        (Some("on the user's card"), Some(60 * 60), None)
    );
    assert!(!held.stalled, "stalled under the hold");

    let retried = acted(
        answered(
            &world,
            &[
                "retry",
                place,
                "--stage",
                "construct",
                "--answers",
                KEY,
                "--words",
                "yes",
                "--json",
            ],
        ),
        Act::Retry,
    );
    let answer = answer_on(&retried).expect("no answer recorded on the stop-short");
    assert_eq!(
        (
            answer.words.as_str(),
            answer.act,
            answer.key.map(|key| key.to_string())
        ),
        ("yes", Act::Retry, Some(KEY.to_string()))
    );
    assert!(
        !retried.holds.iter().any(|hold| hold.on == HoldOn::Waiting),
        "the hold outlived the move: {:?}",
        retried.holds
    );
}

/// A keyed answer by `verb`, `message` or its alias `tell`: construct's
/// agent kept open takes it as its answer, recorded by a message with the
/// key; a second naming the key is refused, naming the answer taken.
fn answers_naming_its_key(verb: &str) {
    let world = World::new();
    let (id, _) = asks_ops(&world, Some("1h")).expect("construct asks ops");
    let place = &id.to_string();
    let told = acted(
        answered(&world, &[verb, place, "--answers", KEY, "yes", "--json"]),
        Act::Message,
    );
    let answer = answer_on(&told).expect("no answer recorded on the stop-short");
    assert_eq!(
        (
            answer.words.as_str(),
            answer.act,
            answer.key.map(|key| key.to_string()),
            answer.session.is_some()
        ),
        ("yes", Act::Message, Some(KEY.to_string()), true)
    );
    let again = refused(&world, &[verb, place, "--answers", KEY, "no", "--json"]);
    assert!(
        again.contains(KEY) && again.contains("yes") && again.contains("taken once"),
        "{again}"
    );
}

#[test]
fn a_tell_naming_its_key() {
    // The spec's "A tell's key" (42, 53, 198): `tell`, message's alias,
    // answers as `flume message` does, its act `message`.
    answers_naming_its_key("tell");
}

#[test]
fn a_message_naming_its_key() {
    // The spec's "A message's key" (42, 53, 198): its act `message`, the
    // message delivered as the answer to that stop-short.
    answers_naming_its_key("message");
}

#[test]
fn tell_runs_as_message() {
    // The spec's "A caller not yet moved" (53; record 0069): `flume tell`
    // runs as `flume message`, the agent receives the words, the answer
    // is the one `flume message` gives, its act `message`, and the help
    // lists `message` alone.
    let world = World::new();
    let id = world
        .begun("build", Some(Deliverable::Commits), "one")
        .expect("build begun");
    let work = World::identity("build", "one", "work", 1);
    let place = &id.to_string();
    let told = answered(&world, &["tell", place, "go on", "--json"]);
    assert_eq!(
        world.terminal.sent_to(&work).last().map(String::as_str),
        Some("go on")
    );
    let messaged = answered(&world, &["message", place, "go on", "--json"]);
    assert_eq!(
        serde_json::to_value(&told).expect("tell's answer"),
        serde_json::to_value(&messaged).expect("message's answer")
    );
    acted(told, Act::Message);
    let (help, code) = run(&world, &["--help"]);
    assert_eq!(code, 0, "{help}");
    let commands: Vec<&str> = help
        .lines()
        .filter_map(|line| line.strip_prefix("  ")?.split_whitespace().next())
        .collect();
    assert!(commands.contains(&"message"), "{help}");
    assert!(!commands.contains(&"tell"), "{help}");
}

#[test]
fn a_bound_and_a_key_in_a_wrong_form() {
    // The spec's "A hold's bound in a wrong form" and "A key in a wrong
    // form" (42, 130): each refused naming the forms, nothing held and no
    // agent ended.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let place = &id.to_string();
    let reason = refused(
        &world,
        &[
            "hold",
            place,
            "words",
            "--waiting",
            "--for",
            "soon",
            "--json",
        ],
    );
    assert_eq!(reason, "soon is no time: it is <n>, <n>s, <n>m or <n>h");
    let held = world.held(&id).expect("the place");
    assert!(
        !held.holds.iter().any(|hold| hold.on == HoldOn::Waiting),
        "a hold was put: {:?}",
        held.holds
    );

    let closed = closes(&world);
    let reason = refused(
        &world,
        &["retry", place, "--answers", "construct", "--json"],
    );
    assert_eq!(
        reason,
        format!("`construct` is no end's key: a key is {END_KEY_FORMS}")
    );
    assert_eq!(closes(&world), closed, "an agent was ended");
    assert_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::EndedShort)
    );
}

#[test]
fn a_bound_on_a_stage_hold() {
    // 130: a bound is the waiting move's alone, refused on a stage's hold
    // by the core, nothing held.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let reason = refused(
        &world,
        &[
            "hold",
            &id.to_string(),
            "after review",
            "--stage",
            "verify",
            "--for",
            "1h",
            "--json",
        ],
    );
    assert!(reason.contains("a bound is given only"), "{reason}");
    assert!(world.held(&id).expect("the place").holds.is_empty());
}

#[test]
fn settle_on_quiet_starts_a_conversations_quiet_again() {
    // 121, 212: a research place's talk, read quiet at t, takes a turn no
    // settle reads and is quiet again at t plus 50 minutes, when its own
    // quiet runs `flume settle <place> --on-quiet`; a settle at t plus 70
    // minutes does not hibernate it, its quiet running from the on-quiet
    // settle.
    let world = World::new();
    let id = world
        .researching(launch("research", "one", Some("weigh it")))
        .expect("talk begun");
    let talk = World::identity("research", "one", "talk", 1);
    world.settle(&id);
    let t = world.host.now();
    world.terminal.set_state(&talk, AgentState::Working);
    world.host.advance(50 * 60);
    world.terminal.set_state(&talk, AgentState::Waiting);
    let place = &id.to_string();
    match answered(&world, &["settle", place, "--on-quiet", "--json"]) {
        Answer::Settled { .. } => {}
        other => panic!("the settle answered {other:?}"),
    }
    let first_quiet = |world: &World| {
        world.held(&id).expect("held").stages[0]
            .start
            .as_ref()
            .and_then(|start| start.first_quiet)
    };
    assert_eq!(first_quiet(&world), Some(t + 50 * 60));
    world.host.advance(20 * 60);
    world.settle(&id);
    assert_eq!(world.stage_state(&id, "talk"), Some(StageState::Begun));
    assert_eq!(
        world
            .terminal
            .log
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("close"))
            .count(),
        0,
        "talk's agent closed"
    );
    world.host.advance(40 * 60);
    world.settle(&id);
    assert_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Hibernating)
    );
}

/// The planner's address, which acme declares with a run.
const PLANNER: &str = "east/planner";

/// The deliveries the host's deliveries hold keyed as a message's.
fn message_keys(world: &World) -> Vec<String> {
    let (_, records) = flume_core::deliveries::read(&*world.store).expect("the host's deliveries");
    flume_core::deliveries::owing(&records, world.host.now())
        .deliveries
        .into_keys()
        .filter(|key| key.starts_with("message/"))
        .collect()
}

#[test]
fn a_message_to_an_address_answers_with_its_key() {
    // The command-line spec's "Through the binary" (53, 61, 214): the
    // address's run live and working, the JSON gives the key
    // `message/east/planner/<token>`, the address and its run live, and
    // the exit is 0; the person's form gives the key under the address.
    let world = World::new();
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    assert!(world.engine.wake(PLANNER).refusal().is_none());
    let planner = World::id("turn", "east-planner");
    world.settle(&planner);
    let agent = World::identity("turn", "east-planner", "plan", 1);
    world.terminal.set_state(&agent, AgentState::Working);
    let (text, code) = run(
        &world,
        &["message", PLANNER, "look at the nightly job", "--json"],
    );
    assert_eq!(code, 0, "{text}");
    let answer: Value = serde_json::from_str(text.trim()).expect("the answer's JSON");
    let messaged = &answer["messaged"];
    let key = messaged["key"].as_str().expect("a key");
    let token = key
        .strip_prefix("message/east/planner/")
        .unwrap_or_else(|| panic!("the key {key}"));
    assert!(
        token.len() == 8 && token.chars().all(|c| c.is_ascii_hexdigit()),
        "{key}"
    );
    assert_eq!(
        (&messaged["address"], &messaged["found"], &messaged["run"]),
        (
            &Value::from(PLANNER),
            &Value::from("live"),
            &Value::from(planner.0.as_str())
        ),
        "{answer}"
    );
    let (text, code) = run(&world, &["message", PLANNER, "and the weekly one"]);
    assert_eq!(code, 0, "{text}");
    assert!(
        text.starts_with("messaged east/planner: message/east/planner/")
            && text.contains("is live, and takes it at its next wait"),
        "{text}"
    );
    assert_eq!(message_keys(&world).len(), 2);
}

#[test]
fn a_target_names_an_address_or_a_place() {
    // The caller-acts spec's "An address and a place" (53, 132): an
    // address is a delivery at it; a place, named here by the location
    // the fakes' world holds it at, is messaged its agent; and
    // `./east/planner` is a place's location, never the address.
    let world = World::new();
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    answered(&world, &["message", PLANNER, "words", "--json"]);
    assert_eq!(message_keys(&world).len(), 1, "the delivery at the address");
    let id = world
        .begun("fix", Some(Deliverable::Commits), "one")
        .expect("fix begun");
    let work = World::identity("fix", "one", "work", 1);
    let place = &id.to_string();
    acted(
        answered(&world, &["message", place, "words", "--json"]),
        Act::Message,
    );
    assert_eq!(
        world.terminal.sent_to(&work).last().map(String::as_str),
        Some("words")
    );
    let refusal = refused(&world, &["message", "./east/planner", "words", "--json"]);
    assert!(
        refusal.contains("no place") && refusal.contains("/east/planner"),
        "{refusal}"
    );
    assert_eq!(
        message_keys(&world).len(),
        1,
        "a delivery at ./east/planner"
    );
}

/// What `flume relay` wrote, line by line as JSON, serving the place
/// `place` over `world` with `input`, and its exit.
fn relay(world: &World, place: &str, input: &str) -> (Vec<Value>, u8) {
    let mut output = vec![];
    let code = commands::relay::serve(
        place,
        std::io::Cursor::new(input.as_bytes().to_vec()),
        &mut output,
        &|| Ok(world.engine()),
        None,
    );
    let text = String::from_utf8(output).expect("the relay writes text");
    let lines = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("{error}: {line}")))
        .collect();
    (lines, code)
}

/// A fix named one launched over `world`, its code agent live and
/// showing `ready`; its place's location.
fn a_live_fix(world: &World) -> String {
    let answer = world
        .engine
        .launch(&flume_conformance::engine::world::launch(
            "fix",
            "one",
            Some("x"),
        ));
    assert!(matches!(answer, Answer::Launched { .. }), "{answer:?}");
    world
        .terminal
        .set_screen(&World::identity("fix", "one", "code", 1), Some("ready"));
    World::id("fix", "one").to_string()
}

#[test]
fn relay_writes_screens_and_types_each_line_of_input() {
    // The spec's "Lines out and in" (221, 222): the agent's screen first,
    // the text and the keys typed through the fake's relay, and the
    // relayed answer last, ended by the caller.
    let world = World::new();
    let place = a_live_fix(&world);
    world.terminal.log.clear();
    let (lines, code) = relay(
        &world,
        &place,
        "{\"text\":\"yes\"}\n{\"keys\":[\"ctrl+c\",\"enter\"]}\n",
    );
    assert_eq!(code, 0, "{lines:?}");
    assert_eq!(lines.first(), Some(&serde_json::json!({"screen": "ready"})));
    let agent = World::identity("fix", "one", "code", 1).to_string();
    assert_eq!(
        lines.last(),
        Some(&serde_json::json!({"relayed": {
            "place": place,
            "agent": agent,
            "ended": "caller",
        }})),
        "{lines:?}"
    );
    let typed: Vec<String> = world
        .terminal
        .log
        .of("relay")
        .into_iter()
        .filter(|call| call.starts_with("relay text") || call.starts_with("relay keys"))
        .collect();
    assert_eq!(typed, vec!["relay text", "relay keys ctrl+c enter"]);
    assert_eq!(
        world
            .terminal
            .sent_to(&World::identity("fix", "one", "code", 1))
            .last()
            .map(String::as_str),
        Some("yes"),
        "enter submitted what was typed"
    );
}

#[test]
fn relay_gives_back_what_it_cannot_type_and_a_refusal_alone() {
    // The spec's "Input flume cannot read" (222): an untyped line for each
    // line flume cannot read, the key's naming it and flume's names,
    // nothing for an empty line, and the text after typed.
    let world = World::new();
    let place = a_live_fix(&world);
    let (lines, code) = relay(
        &world,
        &place,
        "hello\n{\"keys\":[\"hyper+x\"]}\n\n{\"text\":\"ok\"}\n",
    );
    assert_eq!(code, 0, "{lines:?}");
    let untyped: Vec<&str> = lines
        .iter()
        .filter_map(|line| line["untyped"].as_str())
        .collect();
    assert_eq!(untyped.len(), 2, "{lines:?}");
    assert!(!untyped[0].contains("hello"), "{}", untyped[0]);
    assert!(
        untyped[1].contains("hyper+x") && untyped[1].contains("pageup, pagedown"),
        "{}",
        untyped[1]
    );
    assert_eq!(
        world
            .terminal
            .typed(&World::identity("fix", "one", "code", 1))
            .map(|(typed, _)| typed),
        Some("ok".to_string())
    );
    // The spec's "A refusal is the only line" (4, 221): a place whose
    // first stage waits for room.
    let world = World::new();
    world.host.set_bound(Some(1));
    a_live_fix(&world);
    let answer = world
        .engine
        .launch(&flume_conformance::engine::world::launch(
            "fix",
            "two",
            Some("x"),
        ));
    assert!(matches!(answer, Answer::Launched { .. }), "{answer:?}");
    let two = World::id("fix", "two").to_string();
    let (lines, code) = relay(&world, &two, "{\"text\":\"never\"}\n");
    assert_eq!(code, 1, "{lines:?}");
    assert_eq!(lines.len(), 1, "{lines:?}");
    let reason = lines[0]["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("no refusal: {lines:?}"));
    assert!(
        reason.contains(&two) && reason.contains("no live agent"),
        "{reason}"
    );
}

#[test]
fn relays_help_names_its_lines_and_no_caller() {
    // The spec's "The help" (221; core.35).
    let world = World::new();
    let (text, code) = run(&world, &["relay", "--help"]);
    assert_eq!(code, 0, "{text}");
    let words = text.split_whitespace().collect::<Vec<_>>().join(" ");
    for named in [
        "a person's screen and keys",
        "JSON line",
        "{\"text\":",
        "{\"keys\":",
        "ends when its input closes",
        "the agent's terminal goes",
    ] {
        assert!(words.contains(named), "the help names no {named}: {text}");
    }
    // A terminal's name in the help's source is core.1's own check's to
    // find (conformance:test), so this test names no terminal.
    for caller in ["Boomdrive", "boomdrive", "runner", "crew"] {
        assert!(!words.contains(caller), "the help names {caller}: {text}");
    }
}

/// What `<command> --help` prints, its whitespace made single spaces.
fn help(world: &World, command: &str) -> String {
    let (text, code) = run(world, &[command, "--help"]);
    assert_eq!(code, 0, "{command} --help: {text}");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn each_commands_help_says_its_words() {
    // The command-line spec's "The helps" and "The helps of confirm and
    // wait": needs --to's says a role alone is the place's own stream's,
    // hold's names the waiting move and its bound, retry's and message's
    // the key an answer names, launch --recipient's a role's address of
    // any stream, a name no pane holds collecting its ends and the
    // answerer behind the place's stream, inbox's the place it may be
    // given, confirm's the keys and `--recipient`, wait's `--confirm`,
    // list's that status gives a place's every fact, and wake's that an
    // address is a role a source declares for a stream the host declares;
    // none names a word of ours, launch's names no `--caller`, and needs'
    // never says "cannot deliver past" (7, 41, 42, 43, 49, 53, 61, 130,
    // 132, 143, 193, 200; core.35).
    let world = World::new();
    for (command, said) in [
        ("needs", "--to <ROLE OR ADDRESS>"),
        ("needs", "a role alone, `<role>`, is the place's own stream's, and an address, `<stream>/<role>`, any stream's"),
        ("needs", "in place of the place's recipients"),
        ("needs", "--found The stage found work a stage before its own must do again"),
        ("hold", "--waiting"),
        ("hold", "--for <TIME>"),
        ("hold", "hold the move the place waits on its caller for"),
        ("hold", "once it passes, the hold lapses and the place stalls at once"),
        ("retry", "--answers <KEY>"),
        ("retry", "taken only while the place waits on that stop-short and only once"),
        ("message", "--answers <KEY>"),
        ("message", "its agent kept open for its answer"),
        ("message", "a role's address, `<stream>/<role>`"),
        ("launch", "a role's address, `<stream>/<role>`, of any stream"),
        ("launch", "the answerer behind the place's stream"),
        ("launch", "A name no pane holds is sent no notice"),
        ("launch", "until it collects them by `flume inbox` or `flume wait` as that name"),
        ("inbox", "[PLACE]"),
        ("inbox", "--confirm"),
        ("inbox", "an inbox naming a place collects that place's ends alone"),
        ("inbox", "an end the inbox gives stays pending"),
        ("inbox", "a tidied document's `<place>@<at>` naming that one alone"),
        ("confirm", "<KEY>..."),
        ("confirm", "--recipient <RECIPIENT>"),
        ("confirm", "a hand-over delivers nothing until its recipient confirms it"),
        ("wait", "--confirm"),
        ("wait", "A hand-over delivers nothing until its recipient confirms it"),
        ("list", "A line for each place on the host and each stream it declares"),
        ("list", "reading only what flume keeps"),
        ("list", "a place that landed or was dropped left out unless `--all` is given"),
        ("list", "`flume status <place>` gives a place's every fact"),
        ("list", "--all Every place, those that landed or were dropped among them"),
        ("wake", "Wake a role's address, `<stream>/<role>`: a role a source of the host declares in its roles file, for a stream the host declares"),
        ("wake", "launched in the address's stream where none stands"),
        ("wake", "An address carried to another host, declared by no source or by two, or whose stream the host does not declare is refused, saying why"),
        ("wake", "<ADDRESS> The role's address, `<stream>/<role>`"),
    ] {
        let words = help(&world, command);
        assert!(
            words.contains(said),
            "{command}'s help says not {said:?}: {words}"
        );
    }
    for (command, unsaid) in [
        ("needs", "partition"),
        ("needs", "cannot deliver past"),
        ("hold", "partition"),
        ("retry", "partition"),
        ("message", "partition"),
        ("launch", "partition"),
        ("launch", "--caller"),
        ("inbox", "partition"),
        ("wake", "partition"),
        ("wake", "[roles]"),
    ] {
        let words = help(&world, command);
        assert!(
            !words.contains(unsaid),
            "{command}'s help says {unsaid:?}: {words}"
        );
    }
}

#[test]
fn a_key_no_end_is_under_is_refused_naming_it() {
    // The command-line spec's "A key refused": a key no end is under is
    // refused naming it, in the JSON form, exiting 1 (43; weir 102).
    let world = World::new();
    let reason = refused(
        &world,
        &[
            "confirm",
            "no/such/key",
            "--recipient",
            "boomdrive",
            "--json",
        ],
    );
    assert!(reason.contains("no/such/key"), "{reason}");
    assert!(reason.contains("`key`"), "{reason}");
}

#[test]
fn a_refused_act_exits_1_in_either_form() {
    // S26, 61, 212, 213: done of a fix, a hibernate of a fix, forced or
    // not, and a resume of a fix that hibernates nothing are each refused
    // naming the place, exiting 1, the person's form beginning
    // `refused: `; the core's refusals are the scenarios' own.
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x")))).expect("the fix");
    world.settle(&World::id("fix", "one"));
    let place = World::id("fix", "one").to_string();
    for words in [
        &["done", &place][..],
        &["hibernate", &place],
        &["hibernate", &place, "--force"],
        &["resume", &place],
    ] {
        let json: Vec<&str> = words.iter().copied().chain(["--json"]).collect();
        let reason = refused(&world, &json);
        assert!(reason.contains(&place), "{words:?}: {reason}");
        let (text, code) = run(&world, words);
        assert_eq!(code, 1, "{words:?}: {text}");
        assert!(text.starts_with("refused: "), "{words:?}: {text}");
    }
    let reason = refused(&world, &["hibernate", &place, "--json"]);
    assert!(reason.contains("does not hibernate"), "{reason}");
}

#[test]
fn a_watch_with_a_place_named_or_no_bound_is_refused_settling_nothing() {
    // The command-line spec's "A watch with a place named": refused naming
    // `--every`, the places it settles and the place, nothing settled and
    // no watch recorded; a bound that is no positive duration refused the
    // same (208, 225).
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x")))).expect("the fix");
    let id = World::id("fix", "one");
    let version = world.store.read(&id).expect("the fix's document").version;
    assert_eq!(
        refused(&world, &["settle", &id.0, "--every", "2m", "--json"]),
        format!("--every settles the places where something is pending as a caller's watch over the host, and takes no place: {} was named", id.0)
    );
    for bound in ["0", "0m", "soon"] {
        let reason = refused(&world, &["settle", "--every", bound, "--json"]);
        assert!(reason.contains(bound), "--every {bound}: {reason}");
    }
    assert_eq!(
        world.store.read(&id).map(|read| read.version),
        Some(version),
        "the fix's document after the refusals"
    );
    assert!(
        world.store.read_host(&HostKey::Watch).is_none(),
        "a watch recorded by a refused settle"
    );
}

#[test]
fn the_recipient_an_inbox_defaults_to() {
    // 7, 132, design 7: an inbox naming no recipient where none is read
    // is refused naming each way to name one; $FLUME_PLACE naming a
    // role's run collects as its address; and the recipient the host
    // reads from the command's environment, $FLUME_RECIPIENT, before it.
    // Which recipient the core reads is the role-waking scenarios' own.
    let world = World::new();
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    assert!(world.engine.wake(PLANNER).refusal().is_none());
    let planner = World::id("turn", "east-planner");
    world.settle(&planner);
    world.host.set_recipient(None);

    let reason = refused(&world, &["inbox", "--json"]);
    for way in [
        "--recipient",
        "$FLUME_RECIPIENT",
        "a terminal's session",
        "a role's run",
        "a standing agent's pane",
    ] {
        assert!(reason.contains(way), "the refusal names no {way}: {reason}");
    }
    let recipient = |env: &[(&str, &str)]| {
        let (text, code) = run_in(&world, env, &["inbox", "--json"]);
        assert_eq!(code, 0, "{text}");
        let answer: Value = serde_json::from_str(text.trim()).expect("the answer's JSON");
        answer["inbox"]["recipient"].clone()
    };
    assert_eq!(recipient(&[("FLUME_PLACE", &planner.0)]), PLANNER);
    world.host.set_recipient(Some("ops"));
    assert_eq!(recipient(&[("FLUME_PLACE", &planner.0)]), "ops");
    let (text, code) = run(&world, &["inbox", "--recipient", "ops"]);
    assert_eq!(code, 0, "{text}");
    assert_eq!(text.trim(), "nothing pending for ops");
}

/// The replies of flume's MCP server to `requests`, each given an id in
/// turn, served over a core built over `world` in the environment `env`.
fn mcp(world: &World, env: &[(&str, &str)], requests: &[Value]) -> Vec<Value> {
    let input: String = requests
        .iter()
        .enumerate()
        .map(|(id, request)| {
            let mut request = request.clone();
            request["jsonrpc"] = "2.0".into();
            request["id"] = id.into();
            format!("{request}\n")
        })
        .collect();
    let mut output = Vec::new();
    commands::mcp::serve(
        input.as_bytes(),
        &mut output,
        &|name| {
            env.iter()
                .find(|(given, _)| *given == name)
                .map(|(_, value)| value.to_string())
        },
        &|| Ok(world.engine()),
        None,
    );
    String::from_utf8(output)
        .expect("the replies")
        .lines()
        .map(|line| serde_json::from_str(line).expect("a reply's JSON"))
        .collect()
}

/// A tool's result as its JSON answer, and whether it is marked an error.
fn over_mcp(world: &World, tool: &str, arguments: Value) -> (Value, bool) {
    let replies = mcp(
        world,
        &[],
        &[serde_json::json!({
            "method": "tools/call",
            "params": { "name": tool, "arguments": arguments },
        })],
    );
    let result = &replies[0]["result"];
    let text = result["content"][0]["text"]
        .as_str()
        .expect("the result's text");
    (
        serde_json::from_str(text).expect("the result's JSON"),
        result["isError"] == true,
    )
}

#[test]
fn the_mcp_tools_listed() {
    // The agent program's plugin spec's "The tools listed" (section 9;
    // design 12): status, message, launch, wake, hibernate and resume,
    // each with its input schema, status alone read-only (212, 213).
    let replies = mcp(
        &World::new(),
        &[],
        &[serde_json::json!({"method": "tools/list"})],
    );
    let tools = replies[0]["result"]["tools"].as_array().expect("the tools");
    let names: Vec<&str> = tools
        .iter()
        .filter_map(|tool| tool["name"].as_str())
        .collect();
    assert_eq!(
        names,
        ["status", "message", "launch", "wake", "hibernate", "resume"]
    );
    for tool in tools {
        assert_eq!(tool["inputSchema"]["type"], "object", "{tool}");
        assert_eq!(
            tool["annotations"]["readOnlyHint"],
            tool["name"] == "status",
            "{tool}"
        );
    }
    let required = |name: &str| {
        tools
            .iter()
            .find(|tool| tool["name"] == name)
            .map(|tool| tool["inputSchema"]["required"].clone())
            .unwrap_or_default()
    };
    assert_eq!(required("message"), serde_json::json!(["target", "words"]));
    assert_eq!(required("launch"), serde_json::json!(["kind", "name"]));
    assert_eq!(required("wake"), serde_json::json!(["address"]));
    assert_eq!(required("hibernate"), serde_json::json!(["place"]));
    assert_eq!(required("resume"), serde_json::json!(["place"]));
}

#[test]
fn an_act_over_mcp_answers_as_the_command_line_answers() {
    // The agent program's plugin spec's "A hibernate refused as the
    // command line refuses it", "A resume of nothing hibernating" and
    // "message over MCP" (53, 60, 212, 213, 214, C.2): each tool's result
    // is the JSON the command line gives for the same words, a refusal
    // marked an error, and a refused resume starts nothing.
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x")))).expect("the fix");
    let id = World::id("fix", "one");
    world.settle(&id);
    let starts = |world: &World| world.held(&id).expect("the fix").stages[0].starts;
    let before = starts(&world);
    for (tool, arguments, words) in [
        (
            "hibernate",
            serde_json::json!({"place": id.0, "force": true}),
            vec!["hibernate", "--force", "--json", "--", &id.0],
        ),
        (
            "resume",
            serde_json::json!({"place": id.0}),
            vec!["resume", "--json", "--", &id.0],
        ),
    ] {
        let (answer, error) = over_mcp(&world, tool, arguments);
        assert!(error, "{tool}: {answer}");
        let (text, code) = run(&world, &words);
        assert_eq!(code, 1, "{text}");
        let by_hand: Value = serde_json::from_str(text.trim()).expect("the JSON");
        assert_eq!(answer, by_hand, "{tool}");
        let reason = answer["refused"]["refusal"]["reason"]
            .as_str()
            .unwrap_or_default();
        assert!(reason.contains(&id.0), "{tool}: {reason}");
    }
    assert_eq!(starts(&world), before, "a refused act started a stage");

    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let (answer, error) = over_mcp(
        &world,
        "message",
        serde_json::json!({"target": PLANNER, "words": "--look at the nightly job"}),
    );
    assert!(!error, "{answer}");
    let (text, code) = run(
        &world,
        &["message", PLANNER, "and the weekly one", "--json"],
    );
    assert_eq!(code, 0, "{text}");
    let by_hand: Value = serde_json::from_str(text.trim()).expect("the JSON");
    for answer in [&answer, &by_hand] {
        let messaged = &answer["messaged"];
        assert_eq!(messaged["address"], PLANNER, "{answer}");
        assert!(
            messaged["key"]
                .as_str()
                .is_some_and(|key| key.starts_with("message/east/planner/")),
            "{answer}"
        );
    }
    assert_eq!(
        answer
            .as_object()
            .map(|answer| answer.keys().collect::<Vec<_>>()),
        by_hand
            .as_object()
            .map(|answer| answer.keys().collect::<Vec<_>>()),
    );
}
