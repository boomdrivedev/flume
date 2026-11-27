//! The plugin's hooks run by the built binary (C.4; design 14, 17;
//! record 0016), the end-to-end tests of the hooks' seam: a place launched
//! by flume's own binary under the printed command, and each hook run as
//! Claude Code 2.1.322 runs it, `flume hook <event>` with the hook's input
//! on standard input and the agent's environment. SessionStart's and
//! UserPromptSubmit's acks are taken through the binary, and the Stop
//! hook asks flume host to settle the place and leaves no process, or
//! lets the stop through where no flume host answers. Each hook's logic
//! is tested in-process over the fakes in `hooks.rs`.

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
