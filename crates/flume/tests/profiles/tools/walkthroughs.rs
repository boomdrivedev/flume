//! The walkthroughs (design 18): flume's built binary under the printed
//! command, on a scratch repository bound to `git-worktree`, `openspec`
//! and `sluice`, landing on a local bare remote, each agent the stand-in
//! for Claude Code run as a person runs the printed command, and the test
//! settling as a caller would between its commands. Each proves that the
//! seams and the binary carry a whole run with the pinned tools; the
//! conformance scenarios prove each behaviour of the core over the fakes.
//! A fix lands through flume's own submission (S1); a failed submission
//! goes back to code with sluice's words and lands on the second (112); a
//! bare `change` runs flume's newest through construct, code and verify
//! (S2, S35, S119, 162); a standing agent under herdr takes a fix's end
//! (43, 132, C.5); a question for east's ops is answered once by its
//! woken run (S93); a fix on a stream under Rex lands (190, S91); and a
//! person reaches a fix's agent under herdr through `flume relay` (107,
//! 221, 222, C.5).

use super::walk::*;
use flume::profiles::store::state_dir::StateDir;
use flume_seams::{PlaceId, Store};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// The acts of a code stage that writes `file` and commits it.
fn commits(file: &str, text: &str) -> Value {
    json!([
        { "write": { "path": file, "text": text } },
        { "bash": format!("git add -A && git commit --quiet -m 'write {file}'") },
    ])
}

/// Settles the place until it is tidied, its work landed.
fn until_landed(walk: &Walk, place: &str) {
    walk.until(place, "landed and tidied", Value::is_null);
}

/// The stand-in writing the change `name` as OpenSpec's spec-driven
/// schema has it, two tasks unticked, and committing it.
fn construct(name: &str) -> Value {
    construct_tasks(
        name,
        "## 1. Parser\n\n- [ ] 1.1 Write the parser\n- [ ] 1.2 Test the parser\n",
    )
}

/// The stand-in writing the change `name` with `tasks` as its tasks
/// file, and committing it.
fn construct_tasks(name: &str, tasks: &str) -> Value {
    let at = format!("openspec/changes/{name}");
    json!([
        { "bash": format!("openspec new change {name}") },
        { "write": { "path": format!("{at}/proposal.md"), "text": "## Why\n\nThe tool cannot read its input.\n\n## What Changes\n\n- Add a parser.\n" } },
        { "write": { "path": format!("{at}/specs/parser/spec.md"), "text": "## ADDED Requirements\n\n### Requirement: Parsing\nThe tool SHALL parse its input.\n\n#### Scenario: Plain input\n- **WHEN** the input is a word\n- **THEN** the parse yields the word\n" } },
        { "write": { "path": format!("{at}/design.md"), "text": "## Context\n\nA parser of words.\n" } },
        { "write": { "path": format!("{at}/tasks.md"), "text": tasks } },
        { "bash": format!("git add -A && git commit --quiet -m 'the change {name}'") },
    ])
}

/// The stand-in's verify: the report written, the change archived with
/// it, and committed.
fn verify_holds(name: &str) -> Value {
    verify_archives(name, "")
}

/// The stand-in's verify, archiving with `archive`'s further words.
fn verify_archives(name: &str, archive: &str) -> Value {
    json!([
        { "write": { "path": format!("openspec/changes/{name}/verify.md"), "text": "# Verify\n\nEvery requirement met, every task ticked.\n" } },
        { "bash": format!("openspec archive {name} --yes{archive}") },
        { "bash": format!("git add -A && git commit --quiet -m 'archive {name}'") },
    ])
}

#[test]
fn a_fix_lands_through_flumes_own_submission() {
    // S1: the fix's agent commits, tries to push and to submit, both
    // refused; flume submits the place, sluice lands it on the remote, and
    // flume tidies the place and its branch.
    let walk = Walk::new("walk-fix");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let out = walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
            { "bash": "git push origin HEAD" },
            { "bash": "sluice submit main --no-wait" },
        ]),
    );
    assert_eq!(out.matches("Bash refused").count(), 2, "{out}");
    until_landed(&walk, &place);

    // One submission, flume's: the agent's own was refused.
    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    let entry = &submissions[0];
    assert_eq!(entry["state"], "landed", "{entry}");
    assert_eq!(entry["from"]["path"], place.as_str(), "{entry}");
    assert!(entry["from"].get("session").is_none(), "{entry}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(
        walk.remote_log()[0].contains("fix/one"),
        "{:?}",
        walk.remote_log()
    );
    assert!(!std::path::Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("fix/one"), "the place's branch is left");
}

#[test]
fn a_failed_submission_goes_back_to_code_and_lands_on_the_second() {
    // 112: the gate fails the first submission; the code stage starts
    // again with sluice's notice in its prompt, its agent fixes it, and
    // the second submission lands.
    let walk = Walk::new("walk-failed");
    let place = walk.launch("fix", "two", &["--brief", "fix the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "write": { "path": "broken", "text": "not yet\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
        ]),
    );
    let status = walk.until(&place, "its code stage started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    // The failure flume acted on is confirmed, so the next settle is
    // handed nothing and sends nothing back again (133).
    let first = walk.submissions();
    assert_eq!(first.len(), 1, "{first:?}");
    let delivery = walk.delivery(&first[0]["id"]);
    assert_eq!(
        delivery["state"], "done",
        "the failure flume acted on, confirmed: {delivery}"
    );
    walk.settle(&place);
    let status = walk.status(&place);
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    assert_eq!(stage(&status, "code")["starts"], 2, "{status}");
    let prompt = walk.prompt(&place);
    // sluice's notice, as sluice words it, names the failure and where
    // to read why.
    assert!(prompt.contains("[sluice] #1 failed on main"), "{prompt}");
    walk.agent(
        &place,
        json!([{ "bash": "git rm --quiet broken && git commit --quiet -m 'the parser test passes'" }]),
    );
    until_landed(&walk, &place);
    let states: Vec<Value> = walk
        .submissions()
        .iter()
        .map(|entry| entry["state"].clone())
        .collect();
    assert_eq!(states.len(), 2, "{states:?}");
    assert!(states.contains(&json!("landed")), "{states:?}");
    assert!(walk.on_remote("parser.rs").is_some());
    assert_eq!(walk.on_remote("broken"), None);
    assert!(!walk.has_branch("fix/two"));
}

/// Asserts the agent `status` gives runs in `full`, given by the kind, its
/// start passing Claude Code's `bypassPermissions` (162, 169; record
/// 0064).
fn in_full_from_the_kind(status: &Value) {
    let agent = &status["agent"];
    assert_eq!(
        agent["settings"]["from"]["mode"],
        json!({ "layer": "kind", "value": "full" }),
        "{status}"
    );
    assert_eq!(agent["mode"]["passed"], "bypassPermissions", "{status}");
}

#[test]
fn a_change_runs_construct_code_and_verify() {
    // S2, S35, S119, 162: a bare `change` runs flume's newest, change@5,
    // each stage's agent started in `full` from the kind (records 0064,
    // 0077); construct writes the change, code's one agent ticks its tasks,
    // status showing them ticked along the way, and verify commits its
    // report and archives the change, the code stage started once; the place
    // lands with the report in the archive.
    let walk = Walk::new("walk-change");
    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    let status = walk.until(&place, "its construct's agent", |status| {
        status["current"] == "construct" && !status["agent"].is_null()
    });
    assert_eq!(status["identity"], "flume/change@5", "{status}");
    assert_eq!(status["version"], 5, "{status}");
    in_full_from_the_kind(&status);
    walk.agent(&place, construct("add-parser"));
    let status = walk.until(&place, "its code stage", |status| {
        status["current"] == "code" && !status["agent"].is_null()
    });
    in_full_from_the_kind(&status);
    assert_eq!(stage(&status, "construct")["state"], "delivered");
    assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
    assert!(stage(&status, "code")["group"].is_null(), "{status}");
    let ticked = |status: &Value| -> Vec<bool> {
        status["tasks"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|task| task["ticked"] == true)
            .collect()
    };
    assert_eq!(ticked(&status), vec![false, false], "{status}");

    let midway = walk.signal("midway");
    let tasks = "openspec/changes/add-parser/tasks.md";
    let agent = walk.start_agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "tick": { "path": tasks, "count": 1 } },
            { "bash": "git add -A && git commit --quiet -m 'write the parser'" },
            { "wait_for": midway },
            { "tick": { "path": tasks } },
            { "bash": "git add -A && git commit --quiet -m 'test the parser'" },
        ]),
    );
    let began = std::time::Instant::now();
    let status = loop {
        let status = walk.status(&place);
        if ticked(&status) == vec![true, false]
            || began.elapsed() > std::time::Duration::from_secs(60)
        {
            break status;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    };
    assert_eq!(ticked(&status), vec![true, false], "midway: {status}");
    assert_eq!(status["current"], "code", "{status}");
    std::fs::write(&midway, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);

    let status = walk.until(&place, "its verify stage", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    assert_eq!(ticked(&status), vec![true, true], "{status}");
    assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
    in_full_from_the_kind(&status);
    walk.agent(&place, verify_holds("add-parser"));
    until_landed(&walk, &place);

    let files = walk.remote_files();
    let report = files
        .iter()
        .find(|file| {
            file.starts_with("openspec/changes/archive/") && file.ends_with("-add-parser/verify.md")
        })
        .unwrap_or_else(|| panic!("no report in the archive: {files:?}"));
    assert!(walk.on_remote(report).is_some());
    assert!(
        !files
            .iter()
            .any(|file| file.starts_with("openspec/changes/add-parser/")),
        "the change is left unarchived: {files:?}"
    );
    assert!(
        files.contains(&"openspec/specs/parser/spec.md".to_string()),
        "{files:?}"
    );
    assert!(!walk.has_branch("change/add-parser"));
}

/// A standing agent under herdr: a script run as `claude` that tells
/// herdr it is ready, with the pane's own `HERDR_*` left out, then keeps
/// each line it takes in `$STANDING_LOG`, what a ctrl+c before it cleared
/// left out, as an agent a person keeps in its session takes a notice.
const STANDING: &str = r#"#!/bin/sh
pane="$HERDR_PANE_ID"
env -u HERDR_PANE_ID -u HERDR_TAB_ID -u HERDR_WORKSPACE_ID -u HERDR_ENV \
  "$HERDR_BIN_PATH" pane report-agent "$pane" --source flume-test --agent claude --state idle >/dev/null 2>&1
ctrl_c=$(printf '\003')
stty -isig 2>/dev/null
while IFS= read -r line; do
  printf '%s\n' "${line##*"$ctrl_c"}" >> "$STANDING_LOG"
done
"#;

/// The standing agent `name` started in the session `session` of the
/// host's herdr, as a person keeps one: the session's server started
/// headless, a workspace of its own whose shell reads no startup file and
/// finds [`STANDING`] as `claude`, and `herdr agent start <name>` there;
/// its pane, `herdr:<pane>@<socket>`, once herdr lists it, and the file it
/// keeps what it takes in (132, C.5).
fn standing_under_herdr(host: &UnderHerdr, session: &str, name: &str) -> (String, PathBuf) {
    standing_under_herdr_running(host, session, name, STANDING, &[])
}

/// As [`standing_under_herdr`], the agent the script `script`, its
/// workspace given each of `env` beside.
fn standing_under_herdr_running(
    host: &UnderHerdr,
    session: &str,
    name: &str,
    script: &str,
    env: &[(&str, String)],
) -> (String, PathBuf) {
    let scratch = host.walk.repository.scratch.path("standing");
    std::fs::create_dir_all(scratch.join("bin")).expect("the standing agent's folder");
    let write = |path: PathBuf, text: &str| {
        std::fs::write(&path, text).expect("a script written");
        std::fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o755))
            .expect("the script made executable");
        path
    };
    write(scratch.join("bin/claude"), script);
    let shell = write(scratch.join("shell"), "#!/bin/sh\nexec /bin/sh -i\n");
    let log = scratch.join("took");
    let socket = scratch_session(host, session, &scratch, &shell);
    let mut variables = vec![
        format!("PATH={}:/usr/bin:/bin", scratch.join("bin").display()),
        format!("SHELL={}", shell.display()),
        format!("STANDING_LOG={}", log.display()),
    ];
    variables.extend(env.iter().map(|(name, value)| format!("{name}={value}")));
    let scratch_folder = scratch.to_string_lossy();
    let mut create = vec!["workspace", "create", "--cwd", &scratch_folder];
    for variable in &variables {
        create.extend(["--env", variable.as_str()]);
    }
    create.extend(["--label", name, "--no-focus"]);
    let made = host.herdr(Some(&socket), &create);
    let pane = made["root_pane"]["pane_id"]
        .as_str()
        .unwrap_or_else(|| panic!("herdr gave no pane in {made}"))
        .to_string();
    host.herdr(
        Some(&socket),
        &[
            "pane",
            "wait-output",
            &pane,
            "--source",
            "visible",
            "--regex",
            r"\S",
            "--timeout",
            "20000",
        ],
    );
    let started = host.herdr(
        Some(&socket),
        &[
            "agent",
            "start",
            name,
            "--kind",
            "claude",
            "--pane",
            &pane,
            "--timeout",
            "20000",
        ],
    );
    assert_eq!(
        started["agent"]["name"].as_str(),
        Some(name),
        "herdr agent start {name}: {started}"
    );
    (format!("herdr:{pane}@{socket}"), log)
}

/// The session `session` of the host's herdr started headless in `dir`,
/// its panes' shell `shell`, as a person keeps one: its socket, once
/// herdr lists it running. It stops through its socket when the host is
/// dropped (C.5; core.29).
fn scratch_session(host: &UnderHerdr, session: &str, dir: &Path, shell: &Path) -> String {
    let mut server = host.walk.command("herdr", dir);
    server
        .args(["--session", session, "server"])
        .env("SHELL", shell)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    drop(server.spawn().expect("herdr's session server starts"));
    let began = std::time::Instant::now();
    loop {
        if let Some((_, socket)) = host.running().into_iter().find(|(s, _)| s == session) {
            return socket;
        }
        assert!(
            began.elapsed() < std::time::Duration::from_secs(20),
            "herdr's session {session} did not run"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
fn a_standing_agent_under_herdr_takes_a_fixs_end_through_the_binary() {
    // The end-delivery spec's "A standing agent under herdr through the
    // binary" (43, 132, 193, C.5): the stream `w`'s terminal is the host's,
    // herdr, where the source acme's roles file gives `w/planner` the
    // agent `w-planner` in the session `w`; the fix runs in `t` under the
    // printed command, its submission held so one end is pending for the
    // planner.
    let host = UnderHerdr::new(Walk::new("walk-standing"), "w");
    host.walk.configure_whole(&format!(
        "[host]\nmultiplexer = \"herdr\"\n\n[streams.w]\n\n[streams.t]\nmultiplexer = \"printed\"\npool = \"stand-in\"\n\n\
         [pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [{{ name = \"stand-in-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?}, CLAUDE_CODE_OAUTH_TOKEN = \"{STAND_IN_TOKEN}\" }} }}]\n",
        host.walk.account().display().to_string()
    ));
    let config = host.walk.config_dir();
    let roles = config.join("acme-roles.toml");
    std::fs::write(
        &roles,
        "[roles.\"w/planner\"]\nagent = \"w-planner\"\nsession = \"w\"\n",
    )
    .expect("acme's roles file");
    std::fs::create_dir_all(config.join("sources")).expect("the sources folder");
    std::fs::write(
        config.join("sources/acme.toml"),
        format!(
            "kinds_dir = {:?}\nroles = {:?}\n",
            config.join("kinds/acme").display().to_string(),
            roles.display().to_string()
        ),
    )
    .expect("acme's source file");
    let (at, took) = standing_under_herdr(&host, "w", "w-planner");
    let took = || -> Vec<String> {
        std::fs::read_to_string(&took)
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    };
    let place = host.walk.launch(
        "fix",
        "one",
        &[
            "--brief",
            "fix the parser",
            "--stream",
            "t",
            "--recipient",
            "w/planner",
            "--hold-submission",
        ],
    );
    host.walk
        .agent(&place, commits("parser.rs", "fn parse() {}\n"));
    host.walk
        .until(&place, "its end taken by the standing agent", |_| {
            !took().is_empty()
        });
    // The submission is held, so the place waits on its caller's move and
    // the notice points at the inbox, and names the recipient and the key
    // that confirm the end's delivery (43; design 4, 9).
    let delivery = host.walk.everything()["places"][0]["ends"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|end| end["key"] == "stage/code/1/delivered")
        .and_then(|end| end["delivery"].as_str().map(str::to_string))
        .unwrap_or_else(|| panic!("code's end gives no delivery key"));
    let notice = format!(
        "flume: fix--one (fix the parser) code delivered [stage/code/1/delivered]; read: flume inbox; confirm: flume confirm --recipient w/planner {delivery}"
    );
    assert_eq!(took(), vec![notice.clone()], "what the standing agent took");
    host.walk.settle(&place);
    assert_eq!(took(), vec![notice], "taken again at the next settle");

    let everything = host.walk.everything();
    let places: Vec<&Value> = everything["places"]
        .as_array()
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(places.len(), 1, "a planner's place made: {everything}");
    assert_eq!(places[0]["place"], place.as_str(), "{everything}");
    let planner = everything["host"]["addresses"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|address| address["address"] == "w/planner")
        .cloned()
        .unwrap_or_else(|| panic!("status gives no w/planner: {everything}"));
    assert_eq!(planner["run"], Value::Null, "{planner}");
    // Handed over by its notice, the end stays pending for the address
    // until the agent confirms it (43; weir 103).
    assert_eq!(
        planner["pending"].as_array().map(|pending| pending
            .iter()
            .map(|end| end["key"].clone())
            .collect::<Vec<_>>()),
        Some(vec![json!("stage/code/1/delivered")]),
        "{planner}"
    );
    assert_eq!(
        planner["standing"],
        json!({
            "agent": "w-planner",
            "stream": "w",
            "session": {"name": "w", "from": "role"},
            "terminal": "herdr",
            "found": {"one": {"at": at, "state": "waiting"}},
        }),
        "{planner}"
    );

    let woken = host.walk.flume(&["wake", "w/planner"]);
    assert_eq!(woken["held"]["address"], "w/planner", "{woken}");
    assert_eq!(
        woken["held"]["standing"]["found"]["one"]["at"],
        at.as_str(),
        "{woken}"
    );
    assert_eq!(
        host.walk.everything()["places"].as_array().map(Vec::len),
        Some(1),
        "a planner's place made by the wake"
    );
}

/// The kind `name@1` of the host's source acme, whose one stage `stage`
/// delivers commits and which lands nothing: a role's run of the
/// walkthrough's own (114, 132, 157).
fn role_package(kinds_dir: &Path, name: &str, stage: &str) {
    let dir = kinds_dir.join(format!("{name}@1"));
    std::fs::create_dir_all(dir.join("prompts")).expect("the package's folder");
    std::fs::write(
        dir.join("kind.toml"),
        format!(
            "[kinds.{name}]\nversion = 1\nbranch = \"{name}\"\nstarts_unready = true\nlands = false\n\n\
             [[kinds.{name}.stages]]\nname = \"{stage}\"\nprompt = \"prompts/{stage}.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{}}]\n"
        ),
    )
    .expect("the package's declaration");
    std::fs::write(
        dir.join(format!("prompts/{stage}.md")),
        "Read what is pending for you with flume inbox, and answer it.\n",
    )
    .expect("the package's prompt");
}

/// The host under herdr declaring the stream `east` on the stand-in's
/// pool, and the source acme, its kinds `door@1` and `ops@1` locked on the
/// host, whose roles file declares runs of the walkthrough's repository
/// for `east/operator`, the answerer behind `east`, and `east/ops`, each
/// with the words of what it answers (132, 143, 191, 193; design 2, 5).
fn east_with_its_roles(host: &UnderHerdr) {
    host.configure("", "[streams.east]\npool = \"stand-in\"\n");
    let config = host.walk.config_dir();
    let kinds = config.join("kinds/acme");
    role_package(&kinds, "door", "door");
    role_package(&kinds, "ops", "answer");
    let roles = config.join("acme-roles.toml");
    let repository = host.walk.repository.dir.display().to_string();
    std::fs::write(
        &roles,
        format!(
            "[roles.\"east/operator\"]\nrepository = {repository:?}\nkind = \"door\"\nanswerer = true\n\
             answers = \"whatever no other role of east answers\"\n\n\
             [roles.\"east/ops\"]\nrepository = {repository:?}\nkind = \"ops\"\n\
             answers = \"east's live systems: its queues, its deploys and their settings\"\n"
        ),
    )
    .expect("acme's roles file");
    std::fs::create_dir_all(config.join("sources")).expect("the sources folder");
    std::fs::write(
        config.join("sources/acme.toml"),
        format!(
            "kinds_dir = {:?}\nroles = {:?}\n",
            kinds.display().to_string(),
            roles.display().to_string()
        ),
    )
    .expect("acme's source file");
    host.walk
        .flume(&["kinds", "lock", "--host", "acme/door@1", "acme/ops@1"]);
}

/// Settles every place, as a caller does between its commands, until
/// `holds` holds, for up to three minutes; `what` names it where it never
/// does, with every place's line and what each start's stand-in did.
fn settled_until(host: &UnderHerdr, what: &str, holds: impl Fn() -> bool) {
    let began = std::time::Instant::now();
    loop {
        host.walk.flume(&["settle"]);
        if holds() {
            return;
        }
        if began.elapsed() > std::time::Duration::from_secs(180) {
            let lines: Vec<Value> = host.everything()["places"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|place| json!([place["place"], place["line"]["full"], place["waiting_on"]]))
                .collect();
            panic!("never came to {what}: {lines:?}\n{}", host.logs());
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

/// The JSON a file holds, null where it holds none yet.
fn json_in(path: &Path) -> Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or(Value::Null)
}

/// Where the place `<kind>--<name>` of the walkthrough's repository is
/// made (C.1).
fn location(host: &UnderHerdr, place: &str) -> String {
    flume::profiles::place::git_worktree::location(
        &host.walk.repository.name(),
        place,
        Some(&host.home().join("data/flume")),
    )
    .expect("the place's location")
}

/// Whether `deliveries`, an end's hand-overs, reach `recipient` as a
/// settle's notice and the recipient's own inbox may meet: at least once,
/// by a notice at most once, and by no notice later than a hand-over by
/// another way, which may follow a notice since an inbox gives an end
/// again until its recipient confirms it (43; core.25).
fn handed_with_one_notice(deliveries: &[&Value], recipient: &str) -> bool {
    let to: Vec<&Value> = deliveries
        .iter()
        .copied()
        .filter(|delivery| delivery["recipient"] == recipient)
        .collect();
    let at = |by_notice: bool| -> Vec<u64> {
        to.iter()
            .filter(|delivery| (delivery["by"] == "notice") == by_notice)
            .filter_map(|delivery| delivery["at"].as_u64())
            .collect()
    };
    let (notices, others) = (at(true), at(false));
    !to.is_empty()
        && notices.len() <= 1
        && notices
            .iter()
            .all(|notice| others.iter().all(|other| notice <= other))
}

/// The question construct asks east's ops in S93.
const ASKED_OPS: &str = "Which queue's settings are live in east?";

/// The answer east's ops gives it.
const OPS_READ: &str = "east runs the queue at 2.4, as deployed.";

/// The key of construct's first stop-short, as callers name it (42, 43).
const CONSTRUCT_KEY: &str = "stage/construct/1/ended-short";

#[test]
fn s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run() {
    // S93; 41, 42, 43, 130, 132, 191, 193, 198: a change launched from no
    // session in `east` takes the answerer behind east, `east/operator`,
    // as its recipient. Its construct stops short naming `ops`, so the end
    // goes to `east/ops` alone, whose run is woken in east. The run
    // collects the question with its key, holds the waiting move and
    // retries naming the key, which reaches construct's stand-in, kept
    // open for its answer, in its session. A second retry naming the key
    // is refused. No run of `east/operator` is launched, and the run of
    // `east/ops` delivers and is tidied with no caller's word.
    let host = Walk::under_herdr("walk-s93");
    east_with_its_roles(&host);
    let out = host.walk.repository.scratch.path("ops");
    std::fs::create_dir_all(&out).expect("the run's folder");
    let (inbox, hold, retry) = (
        out.join("inbox.json"),
        out.join("hold.json"),
        out.join("retry.json"),
    );
    host.script(
        "construct",
        1,
        json!([
            [{ "bash": format!("flume needs --to ops \"{ASKED_OPS}\"") }],
            [{ "wait_for": host.signal("never") }],
        ]),
    );
    host.script(
        "answer",
        1,
        json!([
            { "bash": format!("flume inbox --json > '{}'", inbox.display()) },
            { "bash": format!(
                "key=$(grep -o '\"key\":\"stage/construct/[^\"]*\"' '{}' | head -n 1 | cut -d '\"' -f 4) \
                 && flume hold change--s93 'east/ops has it' --waiting --for 1h --json > '{}' \
                 && flume retry change--s93 --stage construct --answers \"$key\" --words '{OPS_READ}' --json > '{}'",
                inbox.display(),
                hold.display(),
                retry.display()
            ) },
            { "write": { "path": "answered.md", "text": "construct's question answered\n" } },
            { "bash": "git add -A && git commit --quiet -m 'answer construct'" },
        ]),
    );
    let (launched, refused) = host.walk.flume_ran_from(
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "s93",
            "--stream",
            "east",
            "--brief",
            "Choose the queue's settings.",
        ],
        &[],
    );
    assert!(!refused, "the launch from no session: {launched}");
    let place = launched["launched"]["place"]["place"]
        .as_str()
        .unwrap_or_else(|| panic!("no place launched: {launched}"))
        .to_string();
    assert_eq!(
        launched["launched"]["place"]["recipients"],
        json!(["east/operator"]),
        "{launched}"
    );
    assert_eq!(
        launched["launched"]["place"]["recipients_from"]["from"], "answerer",
        "{launched}"
    );

    let run = location(&host, "ops--east-ops");
    settled_until(&host, "the run of east/ops answered and tidied", || {
        host.prompts("construct", 1).len() == 2 && host.status(&run).is_null()
    });

    // The run's inbox gives the place waiting on it with the question by
    // its key; the end itself reached the run once, delivered as a notice
    // into its session by the settle its inbox runs first, or by the inbox
    // where no settle delivered it before (43, 132; design 9).
    let inbox = json_in(&inbox);
    assert_eq!(inbox["inbox"]["recipient"], "east/ops", "{inbox}");
    let waiting = inbox["inbox"]["waiting"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|waiting| waiting["place"] == place.as_str())
        .unwrap_or_else(|| panic!("the inbox gives no waiting place: {inbox}"));
    assert_eq!(
        (
            &waiting["stop_short"]["key"],
            &waiting["stop_short"]["addressee"],
            &waiting["stop_short"]["words"]
        ),
        (&json!(CONSTRUCT_KEY), &json!("east/ops"), &json!(ASKED_OPS)),
        "{inbox}"
    );

    let hold = json_in(&hold);
    let held = hold["acted"]["status"]["holds"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|held| held["on"] == "waiting")
        .unwrap_or_else(|| panic!("the run held no waiting move: {hold}"));
    assert_eq!(
        (&held["words"], &held["bound"]),
        (&json!("east/ops has it"), &json!(3600)),
        "{hold}"
    );

    let retry = json_in(&retry);
    assert_eq!(retry["acted"]["act"], "retry", "{retry}");
    let answered = retry["acted"]["status"]["ends"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|end| end["key"] == CONSTRUCT_KEY)
        .map(|end| end["answer"].clone())
        .unwrap_or_default();
    assert_eq!(
        (&answered["words"], &answered["by"], &answered["key"]),
        (&json!(OPS_READ), &json!("east/ops"), &json!(CONSTRUCT_KEY)),
        "{retry}"
    );
    assert!(
        answered["session"].is_object(),
        "the answer reached no session: {retry}"
    );
    let deliveries: Vec<&Value> = retry["acted"]["status"]["ends"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|end| end["key"] == CONSTRUCT_KEY)
        .flat_map(|end| end["deliveries"].as_array().into_iter().flatten())
        .collect();
    // Handed to east/ops alone: by its run's inbox, by a notice into its
    // run, or by a notice its inbox then gives again (43, 132).
    assert!(
        deliveries
            .iter()
            .all(|delivery| delivery["recipient"] == "east/ops")
            && handed_with_one_notice(&deliveries, "east/ops"),
        "the stop-short's deliveries: {retry}"
    );
    let prompts = host.prompts("construct", 1);
    assert_eq!(
        prompts[1],
        format!("The question: {ASKED_OPS}\n\nThe answer: {OPS_READ}"),
        "the answer construct's session took"
    );
    assert_eq!(host.log("construct", 2), "", "a second start of construct");

    let (again, refused) = host.walk.flume_ran(&[
        "retry",
        "change--s93",
        "--stage",
        "construct",
        "--answers",
        CONSTRUCT_KEY,
        "--words",
        "2.3",
    ]);
    let words = again.to_string();
    assert!(
        refused
            && words.contains("east/ops")
            && words.contains(OPS_READ)
            && words.contains("taken once"),
        "a second retry naming the key: {again}"
    );
    assert_eq!(host.prompts("construct", 1).len(), 2, "{prompts:?}");

    assert_eq!(host.log("door", 1), "", "a run of east/operator started");
    let everything = host.everything();
    assert!(
        !everything["places"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|listed| listed["kind"] == "door"),
        "a run of east/operator launched: {everything}"
    );
    assert!(
        host.log("answer", 1).contains("session ended"),
        "{}",
        host.log("answer", 1)
    );
}

#[test]
fn a_fix_on_a_stream_under_rex_lands_through_the_binary() {
    // 190, S91, C.5: the stream-terminals scenario "A fix on a stream under
    // Rex": the host's `[streams.trial]` names `multiplexer = "rex"`, and a
    // fix launched on `trial` through the binary, with the stand-in for
    // Claude Code: its agent opens in a block of the Rex server at
    // <state>/terminal/rex/trial.sock, in its base's Rex session and the
    // window made for `fix--one`; while it works, a settle labels the
    // window `fix--one · ` and the place's short line, and the Rex session
    // `<repository> main · 1 running · 0 asking · 0 waiting on caller · 0
    // stalled` (139); it takes its prompt, delivers, and the fix lands;
    // status gives the agent's terminal `rex` from `stream`, and once the
    // place is tidied its window and its Rex session are gone and the
    // server still runs.
    let host = Walk::under_rex("walk-rex");
    let signal = host.signal("commit");
    host.script(
        "code",
        1,
        json!([[
            { "wait_for": signal.to_string_lossy() },
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
        ]]),
    );
    let place = host.walk.launch(
        "fix",
        "one",
        &["--brief", "fix the parser", "--stream", "trial"],
    );
    let status = host
        .walk
        .until(&place, "its agent working under Rex", |status| {
            status["agent"]["terminal"]["profile"] == "rex" && !host.prompts("code", 1).is_empty()
        });
    assert_eq!(
        status["agent"]["terminal"],
        json!({"profile": "rex", "from": "stream"}),
        "{status}"
    );
    assert!(
        host.socket("trial").exists(),
        "no Rex server at trial's socket"
    );
    let repository = host
        .walk
        .repository
        .dir
        .file_name()
        .expect("the repository's folder")
        .to_string_lossy()
        .into_owned();
    let counted =
        format!("{repository} main · 1 running · 0 asking · 0 waiting on caller · 0 stalled");
    let sessions = || -> Vec<(String, String)> {
        host.rex("trial", &["api", "call", "session.list"])["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|session| {
                Some((
                    session["session_id"].as_str()?.to_string(),
                    session["label"].as_str()?.to_string(),
                ))
            })
            .collect()
    };
    let window_labels = |session: &str| -> Vec<String> {
        host.rex("trial", &["api", "call", "-s", session, "session.view"])["windows"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|window| window["label"].as_str().map(str::to_string))
            .collect()
    };
    let labelled = host
        .walk
        .until(&place, "its window and Rex session labelled", |status| {
            let short = status["line"]["short"].as_str().unwrap_or_default();
            let listed = sessions();
            !short.is_empty()
                && listed.len() == 1
                && listed[0].1 == counted
                && window_labels(&listed[0].0) == vec![format!("fix--one · {short}")]
        });
    let short = labelled["line"]["short"]
        .as_str()
        .expect("the place's short line");
    let listed = sessions();
    let [(session, label)] = &listed[..] else {
        panic!("the trial server's Rex sessions: {listed:?}");
    };
    assert_eq!(label, &counted);
    let view = host.rex("trial", &["api", "call", "-s", session, "session.view"]);
    let windows: Vec<(&str, Vec<&str>)> = view["windows"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|window| {
            (
                window["label"].as_str().unwrap_or_default(),
                window["layers"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .flat_map(|layer| layer["blocks"].as_array().into_iter().flatten())
                    .filter_map(|block| block["label"].as_str())
                    .collect(),
            )
        })
        .collect();
    assert_eq!(windows.len(), 1, "{view}");
    assert_eq!(windows[0].0, format!("fix--one · {short}"), "{view}");
    assert!(
        windows[0].1.len() == 1 && windows[0].1[0].starts_with("f-fix--one-code-1-"),
        "{view}"
    );
    std::fs::write(&signal, "").expect("the signal");
    until_landed(&host.walk, &place);
    assert_eq!(
        host.walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    let status = host.rex("trial", &["api", "call", "server.status"]);
    assert!(status["pid"].is_i64(), "the trial server no longer runs");
    assert_eq!(
        host.rex("trial", &["api", "call", "session.list"])["sessions"],
        json!([]),
        "the place's Rex session was left"
    );
}

/// Every file under `folder` holding `text`, by its path.
fn files_holding(folder: &Path, text: &str) -> Vec<PathBuf> {
    let mut found = vec![];
    let mut todo = vec![folder.to_path_buf()];
    while let Some(directory) = todo.pop() {
        for entry in std::fs::read_dir(&directory)
            .into_iter()
            .flatten()
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() && !path.is_symlink() {
                todo.push(path);
            } else if std::fs::read(&path)
                .is_ok_and(|read| read.windows(text.len()).any(|at| at == text.as_bytes()))
            {
                found.push(path);
            }
        }
    }
    found
}

#[test]
fn a_person_reaches_a_fixs_agent_through_a_relay() {
    // The place-relay scenario "Through the binary under herdr" (107, 221,
    // 222, C.5): a fix's code agent, the stand-in in a pane of flume's own
    // herdr session, its terminal echoing what is typed as Claude Code's
    // input shows it, relayed through the binary as
    // `relayed_through_the_binary` says.
    let host = Walk::under_herdr("walk-relay");
    let echoing = host.signal("echoing");
    host.script(
        "code",
        1,
        json!([
            [{ "bash": format!("stty echo < /dev/tty && touch '{}'", echoing.display()) }],
            [],
        ]),
    );
    let place = host.launch("fix", "one", &["--brief", "wait for a person"]);
    host.until(
        &place,
        "code's agent said it started and echoes",
        |status| stage(status, "code")["state"] == "begun" && echoing.exists(),
    );
    relayed_through_the_binary(&host, &place);
}

/// The place's code agent, said started and echoing what is typed,
/// relayed through the binary: `flume relay fix--one` run as a child
/// gives a screen first, types `relay-marker` written to its input, gives
/// a screen holding it, and, its input closed, ends by its caller with
/// exit 0, its last line the relayed answer naming code's first start;
/// the place's held state stands at its version, and no file of flume's
/// state holds what was typed (107, 221, 222).
fn relayed_through_the_binary(host: &Walk, place: &str) {
    use std::io::{BufRead, BufReader, Write};
    use std::process::Stdio;
    use std::sync::mpsc::channel;
    use std::time::{Duration, Instant};
    host.resident();
    let store = StateDir::at(host.home().join("state/flume"));
    let id = PlaceId::new(place);
    let before = store.read(&id).expect("the place's held state").version;

    let mut relay = host
        .command(env!("CARGO_BIN_EXE_flume"), &host.repository.dir)
        .args(["relay", "fix--one"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("flume relay runs");
    let (lines, read) = channel();
    let stdout = relay.stdout.take().expect("the relay's output");
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if lines.send(line).is_err() {
                break;
            }
        }
    });
    let next = |what: &str| -> Value {
        let line = read
            .recv_timeout(Duration::from_secs(20))
            .unwrap_or_else(|_| panic!("no line came: {what}\n{}", host.logs()));
        serde_json::from_str(&line).unwrap_or_else(|error| panic!("{error}: {line}"))
    };
    let first = next("the first screen");
    assert!(first["screen"].is_string(), "the first line: {first}");
    let mut input = relay.stdin.take().expect("the relay's input");
    writeln!(input, r#"{{"text":"relay-marker"}}"#).expect("the input written");
    input.flush().expect("the input flushed");
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        assert!(Instant::now() < deadline, "no screen held relay-marker");
        let line = next("a screen holding relay-marker");
        assert!(line["untyped"].is_null(), "{line}");
        if line["screen"]
            .as_str()
            .is_some_and(|screen| screen.contains("relay-marker"))
        {
            break;
        }
    }
    drop(input);
    let mut last = Value::Null;
    while let Ok(line) = read.recv_timeout(Duration::from_secs(10)) {
        last = serde_json::from_str(&line).unwrap_or_else(|error| panic!("{error}: {line}"));
    }
    let exit = relay.wait().expect("the relay ends");
    assert_eq!(
        last,
        json!({"relayed": {
            "place": place,
            "agent": format!("flume:{}:fix--one:code:1", repository_id(host)),
            "ended": "caller",
        }}),
        "the last line"
    );
    assert!(exit.success(), "{exit:?}");
    assert_eq!(
        store.read(&id).expect("the place's held state").version,
        before,
        "the relay wrote the held state"
    );
    let state = host.home().join("state/flume");
    assert_eq!(
        files_holding(&state, "relay-marker"),
        Vec::<PathBuf>::new(),
        "files of flume's state holding what was typed"
    );
}

/// The repository's id as the walkthrough's agents' identities carry it.
fn repository_id(host: &Walk) -> String {
    let common = std::process::Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(&host.repository.dir)
        .output()
        .expect("git runs");
    flume_core::repository_id(String::from_utf8_lossy(&common.stdout).trim())
}
