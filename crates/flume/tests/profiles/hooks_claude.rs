//! The plugin's hooks run by the built binary (C.4; design 14, 17;
//! record 0016): a place launched by flume's own binary under the printed
//! command, its agent the command prints for Claude Code, and each hook
//! run as Claude Code 2.1.322 runs it, `flume hook <event>` with the
//! hook's input on standard input and the agent's environment, over a
//! fixture transcript on a scratch state directory. No Claude Code runs:
//! the test's own process stands in for the program the hook runs under,
//! a scratch `claude` on the hook's `PATH` naming it (161). The same
//! binary's status gives a host's Claude Code accounts' configuration
//! homes, each its `CLAUDE_CONFIG_DIR` read against the scratch `HOME`
//! (56, C.4).

use super::agent_claude::fixture;
use crate::common::*;
use flume::profiles::place::git_worktree;
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// A host's scratch and a repository bound to `git-worktree` alone.
struct Host {
    /// Ends first, before its state is taken away.
    resident: std::cell::RefCell<Option<resident::Resident>>,
    repository: Repository,
    user: PathBuf,
}

/// One agent the printed command started, as its environment names it.
struct Agent {
    place: String,
    identity: String,
    secret: String,
    stage: String,
    plugin: PathBuf,
    transcript: PathBuf,
    /// The prompt the command carries, its last word.
    prompt: String,
    /// The `PATH` Claude Code hands its hooks: a scratch folder first whose
    /// `claude` is the program the hook runs under, this test's own.
    path: String,
}

/// What a command printed and its exit status.
struct Ran {
    out: String,
    code: i32,
}

impl Ran {
    fn json(&self) -> Value {
        serde_json::from_str(self.out.trim())
            .unwrap_or_else(|error| panic!("JSON ({error}): {}", self.out))
    }
}

/// A kind of two code stages that does not land, and a conversation.
const KINDS: &str = r#"[kinds.fix]
version = 1
branch = "fix"
starts_unready = true
lands = false

[[kinds.fix.stages]]
name = "code"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]

[[kinds.fix.stages]]
name = "review"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]

[kinds.talk]
version = 1
branch = "talk"
starts_unready = true
lands = false

[[kinds.talk.stages]]
name = "chat"
prompt = "prompts/work.md"
conversation = true
agents = [{}]
"#;

impl Host {
    fn new(label: &str) -> Host {
        let repository = Repository::new(label);
        repository.write(
            ".config/flume.toml",
            "[profiles]\nplace = \"git-worktree\"\n",
        );
        repository.commit("the binding");
        let host = Host {
            resident: std::cell::RefCell::new(None),
            repository,
            user: resident::user_scratch(),
        };
        for dir in [
            "state",
            "config/flume",
            "data",
            "home",
            "kinds/prompts",
            "bin",
        ] {
            std::fs::create_dir_all(host.path(dir)).expect("the host's directory");
        }
        // The hook's parent is this test's process, so the program the
        // start's record names, `claude`, is found as this test's own.
        std::os::unix::fs::symlink(
            std::env::current_exe().expect("this test's program"),
            host.path("bin/claude"),
        )
        .expect("the stand-in program");
        std::fs::write(
            host.path("kinds/prompts/work.md"),
            "Do the work of {{name}}. To stop short: {{needs}}\n",
        )
        .expect("the prompt");
        std::fs::write(host.path("kinds/kinds.toml"), KINDS).expect("the kinds");
        host
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.repository.scratch.path(relative)
    }

    /// flume's binary in the repository, `env` over nothing of the
    /// caller's flume, terminal or Claude Code session.
    fn command(&self, env: &[(&str, &str)]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_flume"));
        command.current_dir(&self.repository.dir);
        for (name, _) in std::env::vars() {
            if ["FLUME_", "HERDR_", "REX_", "CLAUDE"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
            {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.path("state"))
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("HOME", self.path("home"))
            .env(resident::USER_SCRATCH, &self.user)
            .envs(env.iter().copied());
        command
    }

    /// The host's resident, started where none runs yet.
    fn resident(&self) {
        if self.resident.borrow().is_none() {
            let mut command = self.command(&[]);
            command.args(["host", "--every", resident::EVERY]);
            *self.resident.borrow_mut() = Some(resident::Resident::start(command, &self.user));
        }
    }

    /// flume's binary in the repository, `env` over nothing of the
    /// caller's flume, terminal or Claude Code session, `input` on its
    /// standard input.
    fn flume(&self, words: &[&str], env: &[(&str, &str)], input: &str) -> Ran {
        if resident::asks_status(words) {
            self.resident();
        }
        let mut command = self.command(env);
        command
            .args(words)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().expect("flume runs");
        child
            .stdin
            .take()
            .expect("its input")
            .write_all(input.as_bytes())
            .expect("the input written");
        let output = child.wait_with_output().expect("flume ends");
        Ran {
            out: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// Launches `<kind>--one` and gives its agent, as the printed command
    /// names it.
    fn launch(&self, kind: &str) -> Agent {
        self.launch_with(kind, &[])
    }

    /// Launches `<kind>--one` with `more` words of the launch's own, and
    /// gives its agent.
    fn launch_with(&self, kind: &str, more: &[&str]) -> Agent {
        let kinds = self.path("kinds/kinds.toml");
        let kinds = kinds.to_string_lossy();
        let mut words = vec![
            "launch",
            "--kind",
            kind,
            "--name",
            "one",
            "--brief",
            "fix the parser",
            "--kinds",
            &kinds,
            "--recipient",
            "caller",
            "--json",
        ];
        words.extend(more);
        let ran = self.flume(&words, &[], "");
        assert_eq!(ran.code, 0, "the launch: {}", ran.out);
        let place = git_worktree::location(
            &self.repository.name(),
            &format!("{kind}--one"),
            Some(&self.path("data/flume")),
        )
        .expect("the place's location");
        self.agent(&place)
    }

    /// The place's current agent, from the command status prints.
    fn agent(&self, place: &str) -> Agent {
        let status = self.status(place);
        let agent = &status["places"][0]["agent"];
        let printed = agent["printed"].as_str().expect("a printed command");
        // The command's environment is in the file the line names, never
        // in the line (32, 56).
        let env = printed
            .split(' ')
            .map(|word| word.trim_matches('\''))
            .find(|word| word.ends_with(".env"))
            .map(|file| std::fs::read_to_string(file).expect("the command's environment"))
            .unwrap_or_else(|| panic!("no environment file in {printed}"));
        let word = |name: &str| {
            env.lines()
                .find_map(|line| line.strip_prefix(&format!("{name}=")))
                .map(|value| value.trim_matches('\'').to_string())
                .unwrap_or_else(|| panic!("{name} in {env}"))
        };
        let plugin = printed
            .split(' ')
            .skip_while(|word| *word != "--plugin-dir")
            .nth(1)
            .expect("the plugin's directory")
            .trim_matches('\'')
            .to_string();
        let prompt = printed
            .rsplit_once(" '")
            .map(|(_, quoted)| quoted.trim_end_matches('\'').replace("'\\''", "'"))
            .expect("the prompt, quoted");
        let identity = word("FLUME_AGENT");
        let stage = word("FLUME_STAGE");
        let transcript = self.path(&format!("sessions/{stage}.jsonl"));
        std::fs::create_dir_all(transcript.parent().expect("a folder")).expect("sessions");
        std::fs::write(&transcript, fixture("session.jsonl")).expect("the transcript");
        Agent {
            place: word("FLUME_PLACE"),
            identity,
            secret: word("FLUME_SECRET"),
            stage,
            plugin: PathBuf::from(plugin),
            transcript,
            prompt,
            path: format!(
                "{}:{}",
                self.path("bin").display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        }
    }

    fn status(&self, place: &str) -> Value {
        let ran = self.flume(&["status", place, "--json"], &[], "");
        assert_eq!(ran.code, 0, "status: {}", ran.out);
        ran.json()["status"].clone()
    }

    /// The place's agent as status gives it.
    fn agent_status(&self, place: &str) -> Value {
        self.status(place)["places"][0]["agent"].clone()
    }

    /// The hook for `event` run as Claude Code runs it in `agent`'s
    /// session, with `input` over the fields every hook's input carries.
    fn hook(&self, agent: &Agent, event: &str, input: Value) -> Ran {
        self.hook_in(agent, event, input, &agent.env())
    }

    fn hook_in(
        &self,
        agent: &Agent,
        event: &str,
        mut input: Value,
        env: &[(String, String)],
    ) -> Ran {
        let common = json!({
            "session_id": "4e3c05c8-51c4-4449-8577-4f7038e394bf",
            "transcript_path": agent.transcript,
            "cwd": agent.place,
            "hook_event_name": event_name(event),
        });
        for (key, value) in common.as_object().expect("an object") {
            input
                .as_object_mut()
                .expect("an object")
                .entry(key)
                .or_insert(value.clone());
        }
        let env: Vec<(&str, &str)> = env
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect();
        let ran = self.flume(&["hook", event], &env, &input.to_string());
        assert_eq!(ran.code, 0, "the hook {event} exits 0: {}", ran.out);
        ran
    }

    /// The hook for `event` run as [`Host::hook`] runs it, with one more
    /// descriptor, 3, the write end of a pipe that each process the hook
    /// starts inherits; and whether a process still held that end when the
    /// hook ended, which is a process that the hook left running
    /// (nothing-detached). It reads the pipe once, after the hook ended,
    /// and waits for nothing.
    fn hook_leaving(&self, agent: &Agent, event: &str, input: Value) -> (Ran, bool) {
        use command_fds::{CommandFdExt as _, FdMapping};
        let (reader, writer) = std::io::pipe().expect("the pipe");
        let mut command = self.command(&[]);
        command
            .envs(agent.env())
            .args(["hook", event])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .fd_mappings(vec![FdMapping {
                parent_fd: writer.into(),
                child_fd: 3,
            }])
            .expect("descriptor 3 given to the hook");
        let mut input = input;
        let common = json!({
            "session_id": "4e3c05c8-51c4-4449-8577-4f7038e394bf",
            "transcript_path": agent.transcript,
            "cwd": agent.place,
            "hook_event_name": event_name(event),
        });
        for (key, value) in common.as_object().expect("an object") {
            input
                .as_object_mut()
                .expect("an object")
                .entry(key)
                .or_insert(value.clone());
        }
        let mut child = command.spawn().expect("flume runs");
        // The write end is the child's alone once it started.
        drop(command);
        child
            .stdin
            .take()
            .expect("its input")
            .write_all(input.to_string().as_bytes())
            .expect("the input written");
        let output = child.wait_with_output().expect("flume ends");
        let mut reader = reader;
        nix::fcntl::fcntl(
            &reader,
            nix::fcntl::FcntlArg::F_SETFL(nix::fcntl::OFlag::O_NONBLOCK),
        )
        .expect("the pipe's read end made non-blocking");
        let left = match std::io::Read::read(&mut reader, &mut [0u8; 1]) {
            Ok(0) => false,
            Ok(_) => true,
            Err(error) => error.kind() == std::io::ErrorKind::WouldBlock,
        };
        let ran = Ran {
            out: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        };
        assert_eq!(ran.code, 0, "the hook {event} exits 0: {}", ran.out);
        (ran, left)
    }

    /// Every file under flume's state directory and its text.
    fn state(&self) -> Vec<(PathBuf, Vec<u8>)> {
        let mut found = vec![];
        let mut todo = vec![self.path("state")];
        while let Some(dir) = todo.pop() {
            for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    todo.push(path);
                } else {
                    let text = std::fs::read(&path).unwrap_or_default();
                    found.push((path, text));
                }
            }
        }
        found.sort();
        found
    }
}

impl Agent {
    /// The environment Claude Code gives the agent's hooks: the four
    /// FLUME_* words, the plugin's root and its `PATH`.
    fn env(&self) -> Vec<(String, String)> {
        vec![
            ("FLUME_AGENT".into(), self.identity.clone()),
            ("FLUME_SECRET".into(), self.secret.clone()),
            ("FLUME_PLACE".into(), self.place.clone()),
            ("FLUME_STAGE".into(), self.stage.clone()),
            (
                "CLAUDE_PLUGIN_ROOT".into(),
                self.plugin.to_string_lossy().into_owned(),
            ),
            ("PATH".into(), self.path.clone()),
        ]
    }
}

/// The event Claude Code names for the word `flume hook` takes.
fn event_name(word: &str) -> &'static str {
    flume::profiles::agent::claude::hook::EVENTS
        .iter()
        .find(|(given, _)| *given == word)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}

/// The hook's answer as Claude Code reads it.
fn decision(ran: &Ran) -> Option<Value> {
    (!ran.out.trim().is_empty()).then(|| ran.json())
}

/// Commits in the place, as its stage's agent would.
fn commit_in(place: &str, file: &str) {
    std::fs::write(Path::new(place).join(file), file).expect("the work");
    git(Path::new(place), &["add", file]);
    git(
        Path::new(place),
        &[
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "-m",
            file,
        ],
    );
}

/// The place's status once `holds` holds, read again each quarter of a
/// second, with no bound on the time it waits.
fn until_it_holds(host: &Host, place: &str, holds: impl Fn(&Value) -> bool) -> Value {
    loop {
        let status = host.status(place);
        if holds(&status) {
            return status;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

#[test]
fn the_started_and_received_acks() {
    // SessionStart's ack makes the start started; UserPromptSubmit's acks
    // the submitted prompt by its digest; both clear the asking mark (35,
    // 89, 90, 135).
    let host = Host::new("acks");
    let agent = host.launch("fix");
    assert_eq!(host.agent_status(&agent.place)["started"], "no");

    let ran = host.hook(&agent, "session-start", json!({ "source": "startup" }));
    assert_eq!(decision(&ran), None);
    let status = host.agent_status(&agent.place);
    assert_eq!(status["started"], "yes", "{status}");
    assert_eq!(status["notes"], json!([]), "{status}");

    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "AskUserQuestion", "tool_input": {}, "tool_use_id": "toolu_q" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "go on  with\nthe parser" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
    let record = flume::profiles::agent::claude::record::Record::named(
        &host
            .path("state/flume/places")
            .join(agent.place.trim_start_matches('/').replace('/', "%"))
            .to_string_lossy(),
        &agent.identity,
    );
    let wanted = flume::profiles::agent::claude::digest("go on with the parser");
    assert_eq!(record.received(&wanted), Some(true), "the prompt's digest");

    // An ack without the start's secret is refused, and writes nothing.
    let mut wrong = agent.env();
    wrong[1].1 = "not-the-secret".into();
    let before = host.state();
    host.hook_in(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "x" }),
        &wrong,
    );
    assert_eq!(host.state(), before, "an ack without the secret wrote");
    let refused = host.flume(&["ack", "received", "--digest", "abc"], &[], "");
    assert_eq!(refused.code, 1, "{}", refused.out);
}

/// The start's first prompt submitted as Claude Code 2.1.322 runs
/// UserPromptSubmit, its input naming the session's mode where `mode` is
/// given.
fn first_prompt(host: &Host, agent: &Agent, mode: Option<&str>) -> Ran {
    let mut input = json!({ "prompt": agent.prompt });
    if let Some(mode) = mode {
        input["permission_mode"] = json!(mode);
    }
    host.hook(agent, "user-prompt-submit", input)
}

#[test]
fn the_stop_asks_flume_host_and_leaves_no_process() {
    // The Stop hook lets an agent with its deliverable made stop. It asks
    // flume host to settle the place and starts no process: when the hook
    // ends, no process that it started runs (nothing-detached;
    // a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place).
    // flume host settles the place, and the next stage starts with no
    // settle of the test's (121).
    let host = Host::new("stop-host");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": agent.prompt }),
    );
    commit_in(&agent.place, "parser.rs");
    let (ran, left) = host.hook_leaving(&agent, "stop", json!({ "stop_hook_active": false }));
    assert_eq!(decision(&ran), None, "a stop with the deliverable made");
    assert!(!left, "the Stop hook left a process running");
    host.hook(
        &agent,
        "session-end",
        json!({ "reason": "prompt_input_exit" }),
    );
    let status = until_it_holds(&host, &agent.place, |status| {
        status["places"][0]["current"] == "review"
    });
    assert_eq!(
        status["places"][0]["stages"][0]["state"], "delivered",
        "{status}"
    );
}

#[test]
fn with_no_flume_host_the_stop_leaves_no_process() {
    // Where nothing answers on flume host's socket, the Stop hook lets the
    // stop through and starts no process (nothing-detached).
    let host = Host::new("stop-no-host");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": agent.prompt }),
    );
    commit_in(&agent.place, "parser.rs");
    host.resident
        .borrow()
        .as_ref()
        .expect("the host's resident")
        .kill();
    let (ran, left) = host.hook_leaving(&agent, "stop", json!({ "stop_hook_active": false }));
    assert_eq!(decision(&ran), None, "the stop let through");
    assert!(!left, "the Stop hook left a process running");
}

/// The pane herdr's environment names in a session flume did not start,
/// on a server whose socket nothing listens on: herdr is asked nothing to
/// name the session (7, C.5).
fn pane_env(host: &Host, pane: &str) -> Vec<(String, String)> {
    vec![
        ("HERDR_PANE_ID".into(), pane.into()),
        (
            "HERDR_SOCKET_PATH".into(),
            host.path("no-herdr.sock").to_string_lossy().into_owned(),
        ),
    ]
}

/// The session herdr's environment names for `pane`, as flume names it.
fn pane_session(host: &Host, pane: &str) -> String {
    format!("herdr:{pane}@{}", host.path("no-herdr.sock").display())
}

/// The fix `fix--one` launched for the recipient `recipient` beside the
/// test's caller, its code agent started and its prompt received, stopped
/// short asking which file holds the log, and the place settled, so it
/// waits on a retry of code; then the host's configuration names herdr as
/// its terminal, whose environment names the calling session, the fix's
/// start keeping the terminal it recorded.
fn asks_for(host: &Host, recipient: &str) -> Agent {
    let agent = host.launch_with("fix", &["--recipient", recipient]);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    first_prompt(host, &agent, None);
    let env = agent.env();
    let env: Vec<(&str, &str)> = env
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();
    let asked = host.flume(&["needs", "Which file holds the log?", "--json"], &env, "");
    assert_eq!(asked.code, 0, "the stop-short: {}", asked.out);
    let settled = host.flume(&["settle", &agent.place, "--json"], &[], "");
    assert_eq!(settled.code, 0, "the settle: {}", settled.out);
    let status = host.status(&agent.place);
    assert_eq!(
        status["places"][0]["waiting_on"]["why"], "answer",
        "{status}"
    );
    std::fs::write(
        host.path("config/flume/config.toml"),
        "[host]\nterminal = \"herdr\"\n",
    )
    .expect("the configuration");
    agent
}

/// The added context SessionStart gave, where it gave any.
fn context(ran: &Ran) -> Option<String> {
    if ran.out.trim().is_empty() {
        return None;
    }
    let output = ran.json();
    assert_eq!(
        output["hookSpecificOutput"]["hookEventName"], "SessionStart",
        "{output}"
    );
    Some(
        output["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("the added context")
            .to_string(),
    )
}

/// SessionStart run in a session flume did not start, in herdr's pane
/// `pane`, `more` over its environment, with the fixture's input.
fn session_start_in(host: &Host, agent: &Agent, pane: &str, more: &[(&str, &str)]) -> Ran {
    let mut env = pane_env(host, pane);
    env.extend(
        more.iter()
            .map(|(name, value)| (name.to_string(), value.to_string())),
    );
    let input: Value = serde_json::from_str(
        &fixture_text("session-start-another-session.json")
            .replace("__TRANSCRIPT__", &agent.transcript.to_string_lossy())
            .replace("__CWD__", &host.repository.dir.to_string_lossy()),
    )
    .expect("the fixture's JSON");
    host.hook_in(agent, "session-start", input, &env)
}

/// The plugin's own fixture `name`, as Claude Code 2.1.322 hands a hook
/// its input.
fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../plugins/claude/tests/fixtures")
            .join(name),
    )
    .expect("the plugin's fixture")
}

#[test]
fn a_restarted_pane() {
    // S94, 43, C.4, design 9; weir 103: the claude-code-plugin spec's "A
    // restarted pane": a session flume did not start, begun again in the
    // pane a stop-short was handed to before, is given at its start the
    // place, the stop-short's words, its key and the three ways on; the
    // inbox it ran is the pane's and confirms on receipt, so the
    // stop-short reads handed to the pane by session_start and confirmed
    // by it, and the place waits as it did.
    let host = Host::new("restarted-pane");
    let session = pane_session(&host, "w1-p2");
    let agent = asks_for(&host, &session);
    let collected = host.flume(&["inbox", "--recipient", &session, "--json"], &[], "");
    assert_eq!(collected.code, 0, "the inbox: {}", collected.out);
    assert_eq!(
        collected.json()["inbox"]["ends"].as_array().map(Vec::len),
        Some(1),
        "{}",
        collected.out
    );
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    let words = context(&ran).unwrap_or_else(|| panic!("no context given: {}", ran.out));
    for said in [
        &session,
        "fix--one (fix the parser), in the stream default, waits on a retry of code",
        "Its stop-short stage/code/1/ended-short needs an answer:",
        "  Which file holds the log?",
        "- answer it, naming its key: flume retry fix--one --stage code --answers stage/code/1/ended-short --words \"<answer>\"",
        "- hold the move with your words, saying who has it: flume hold fix--one \"<words>\" --waiting --for <time>",
        "- leave it: once it stalls, the answerer behind its stream has it.",
    ] {
        assert!(words.contains(said), "{said:?} not in:\n{words}");
    }
    assert!(
        words.contains("Ends delivered to this session now")
            && words.contains("[stage/code/1/ended-short]"),
        "the stop-short handed before not given:\n{words}"
    );
    assert_eq!(ran.code, 0, "{}", ran.out);
    let status = host.status(&agent.place);
    let end = status["places"][0]["ends"]
        .as_array()
        .expect("ends")
        .iter()
        .find(|end| end["key"] == "stage/code/1/ended-short")
        .unwrap_or_else(|| panic!("no stop-short: {status}"))
        .clone();
    let handed: Vec<&str> = end["deliveries"]
        .as_array()
        .expect("deliveries")
        .iter()
        .filter(|delivery| delivery["recipient"] == session.as_str())
        .filter_map(|delivery| delivery["by"].as_str())
        .collect();
    assert_eq!(handed, vec!["inbox", "session_start"], "{end}");
    assert!(
        end["confirmed"]
            .as_array()
            .expect("confirmed")
            .iter()
            .any(|confirmed| confirmed["recipient"] == session.as_str()),
        "{end}"
    );
    assert!(
        !end["pending_for"]
            .as_array()
            .expect("pending_for")
            .iter()
            .any(|pending| pending == session.as_str()),
        "{end}"
    );
}

#[test]
fn an_end_pending_for_the_pane_collected_at_its_start() {
    // 43, C.4, design 9: an end still pending for the pane is collected by
    // SessionStart's inbox, given with its key, and delivered.
    let host = Host::new("pending-for-pane");
    let session = pane_session(&host, "w1-p2");
    let agent = asks_for(&host, &session);
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    let words = context(&ran).unwrap_or_else(|| panic!("no context given: {}", ran.out));
    assert!(
        words.contains(
            "- fix--one code needs an answer [stage/code/1/ended-short]; read: flume status fix--one"
        ),
        "{words}"
    );
    let again = host.flume(&["inbox", "--recipient", &session, "--json"], &[], "");
    assert_eq!(
        again.json()["inbox"]["ends"].as_array().map(Vec::len),
        Some(0),
        "collected again: {}",
        again.out
    );
}

#[test]
fn nothing_waits_on_the_session() {
    // 43, C.4: SessionStart in a session nothing is pending for and
    // nothing waits on gives nothing, and exits 0.
    let host = Host::new("nothing-waits");
    let agent = asks_for(&host, &pane_session(&host, "w1-p2"));
    let ran = session_start_in(&host, &agent, "w9-p9", &[]);
    assert_eq!(context(&ran), None, "{}", ran.out);
}

#[test]
fn a_recipient_in_the_environment() {
    // 43, C.4, design 9: SessionStart collects as the session the
    // terminal names, never as the environment's recipient, so the end
    // pending for that recipient stays pending for it.
    let host = Host::new("recipient-in-env");
    let agent = asks_for(&host, "planner");
    let ran = session_start_in(&host, &agent, "w9-p9", &[("FLUME_RECIPIENT", "planner")]);
    assert_eq!(context(&ran), None, "{}", ran.out);
    let status = host.status(&agent.place);
    let ends = status["places"][0]["ends"].as_array().expect("ends");
    assert!(
        ends.iter().any(|end| end["pending_for"]
            .as_array()
            .is_some_and(|named| named.contains(&json!("planner")))),
        "the end is not pending for planner: {status}"
    );
}

#[test]
fn flume_unreachable_at_a_sessions_start() {
    // 43, C.4: where the host's configuration cannot be read, SessionStart
    // in a session flume did not start gives nothing and exits 0.
    let host = Host::new("unreachable");
    let agent = asks_for(&host, &pane_session(&host, "w1-p2"));
    std::fs::write(host.path("config/flume/config.toml"), "[host\n").expect("the configuration");
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    assert_eq!(context(&ran), None, "{}", ran.out);
}

/// The fix `one` of a one-stage kind begun over the fakes, its agent's
/// profile a stand-in offering deliver, a message waiting on its start:
/// the world, the place, the hook's start and the message's words (215).
fn delivering() -> (
    flume_conformance::engine::world::World,
    flume_seams::PlaceId,
    flume::profiles::agent::claude::hook::Start,
) {
    use flume_conformance::engine::world::World;
    let world = World::new();
    world.agent.offer_deliver();
    let id = world
        .begun("build", Some(flume_seams::Deliverable::Commits), "one")
        .expect("build begun");
    let mut held = world.held(&id).expect("the place");
    let start = held.stages[0].start.as_mut().expect("a start");
    start.messages.push(flume_seams::MessageHeld {
        words: "use the second".into(),
        at: 10,
        by: Some("ops".into()),
        taken_at: None,
    });
    let hook = flume::profiles::agent::claude::hook::Start {
        agent: Some(start.identity.to_string()),
        secret: Some(start.secret.0.clone()),
        place: Some(id.0.clone()),
        stage: Some("work".into()),
        ..Default::default()
    };
    world.store.put(&id, held);
    (world, id, hook)
}

/// What `flume hook deliver` prints for `start` over `world`.
fn deliver_hook(
    world: &flume_conformance::engine::world::World,
    start: &flume::profiles::agent::claude::hook::Start,
) -> String {
    flume::profiles::agent::claude::hook::hook(
        flume::profiles::agent::claude::hook::DELIVER,
        "",
        start,
        &|| Ok(world.engine()),
    )
}

/// Whether the message waiting on the place's first start was taken.
fn taken(world: &flume_conformance::engine::world::World, id: &flume_seams::PlaceId) -> bool {
    world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .is_some_and(|start| start.messages.iter().any(|held| held.taken_at.is_some()))
}

#[test]
fn hook_deliver_prints_the_message_waiting() {
    // The agent-profile spec's "A message waiting for the take" (54, 215,
    // C.4): the message's words alone, and the message marked taken.
    let (world, id, start) = delivering();
    assert_eq!(deliver_hook(&world, &start), "use the second");
    assert!(taken(&world, &id), "the message was not marked taken");
    assert_eq!(deliver_hook(&world, &start), "", "a second take");
}

#[test]
fn hook_deliver_prints_nothing_for_another_secret() {
    // The agent-profile spec's "Another start's secret" (41, 215):
    // nothing printed, and nothing taken or recorded handed.
    let (world, id, mut start) = delivering();
    start.secret = Some("another".into());
    assert_eq!(deliver_hook(&world, &start), "");
    assert!(!taken(&world, &id), "a message taken under another secret");
}

#[test]
fn hook_deliver_prints_nothing_while_the_agent_asks() {
    // The agent-profile spec's "An agent asking" (215; core.11): nothing
    // printed, and the message still waits untaken.
    let (world, id, start) = delivering();
    let work = flume_conformance::engine::world::World::identity("build", "one", "work", 1);
    world.agent.set_asking(&work, flume_seams::Reading::Yes);
    assert_eq!(deliver_hook(&world, &start), "");
    assert!(!taken(&world, &id), "a message taken while the agent asks");
}

#[test]
fn each_claude_code_accounts_home_in_status() {
    // The agent-profile spec's scenario "Claude Code accounts' homes in
    // the built binary's status" (56, C.4; core.30): a host's pool of two
    // Claude Code accounts, work-1 whose variables give `CLAUDE_CONFIG_DIR`
    // and a token and work-2 whose give neither; status gives work-1's
    // home read against the host user's `HOME`, work-2's `null`, the
    // person's form names work-1's home under its line, and neither gives
    // the token.
    let host = Host::new("claude-homes");
    let token = "a-token-of-the-claude-homes-test";
    std::fs::write(
        host.path("config/flume/config.toml"),
        format!(
            "[pools.work]\nstrategy = \"fill-first\"\naccounts = [\n  {{ name = \"work-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = \"~/.claude-work-1\", CLAUDE_CODE_OAUTH_TOKEN = \"{token}\" }} }},\n  {{ name = \"work-2\", program = \"claude\" }},\n]\n\n[streams.work]\npool = \"work\"\n"
        ),
    )
    .expect("the host's configuration");
    let home = host.path("home").join(".claude-work-1");
    let home = home.to_string_lossy();

    let ran = host.flume(&["status", "--json"], &[], "");
    assert_eq!(ran.code, 0, "{}", ran.out);
    assert!(!ran.out.contains(token), "{}", ran.out);
    let json = ran.json();
    let accounts = &json["status"]["host"]["pools"][0]["accounts"];
    assert_eq!(accounts[0]["name"], json!("work-1"), "{json}");
    assert_eq!(accounts[0]["home"], json!(home), "{json}");
    assert_eq!(accounts[1]["name"], json!("work-2"), "{json}");
    assert_eq!(accounts[1].get("home"), Some(&Value::Null), "{json}");

    let ran = host.flume(&["status"], &[], "");
    assert_eq!(ran.code, 0, "{}", ran.out);
    assert!(!ran.out.contains(token), "{}", ran.out);
    let lines: Vec<&str> = ran.out.lines().collect();
    let work_1 = lines
        .iter()
        .position(|line| line.trim_start().starts_with("work-1 (claude)"))
        .unwrap_or_else(|| panic!("no line of work-1: {}", ran.out));
    assert_eq!(
        lines.get(work_1 + 1).map(|line| line.trim()),
        Some(format!("configuration home {home}").as_str()),
        "{}",
        ran.out
    );
}
