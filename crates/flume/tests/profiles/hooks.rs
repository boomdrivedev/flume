//! The plugin's hooks over the fakes (C.4; design 14, 17; record 0016):
//! each hook asked in-process as `flume hook <event>` asks it, with the
//! hook's input and the agent's environment as values, of a core whose
//! agents the Claude Code profile runs under the printed command, its
//! held state on a scratch state directory, and its place, change, merge
//! queue and history the fakes'. No process, git or network is started:
//! the program the hook runs under is a scratch `claude` on the hook's
//! `PATH`, given as the hook's parent (161). The hooks through the built
//! binary are `hooks_claude.rs`'s.

use super::agent_claude::fixture;
use crate::common::*;
use flume::commands::launch as launch_words;
use flume::profiles::agent::claude::hook::{self, Start, EVENTS};
use flume::profiles::agent::claude::Claude;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::printed::Printed;
use flume_conformance::engine::account_pools::{account, pool};
use flume_conformance::engine::world::{kind, launch, launched, stage, World, REPOSITORY};
use flume_core::{Defaults, Engine};
use flume_seams::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The prompt of every stage of the host's kinds.
const PROMPT: &str = "Do the work of {{name}}. To stop short: {{needs}}";

/// A host over the fakes whose agents the Claude Code profile runs under
/// the printed command, its state on a scratch directory.
pub struct Host {
    pub world: World,
    pub scratch: Scratch,
    store: Arc<StateDir>,
    printed: Arc<dyn Terminal>,
}

/// One agent the printed command started, as its environment names it.
pub struct Agent {
    pub place: String,
    pub identity: String,
    pub secret: String,
    pub stage: String,
    pub plugin: PathBuf,
    pub transcript: PathBuf,
    /// The prompt the command carries, its last word.
    pub prompt: String,
    /// The `PATH` Claude Code hands its hooks: a scratch folder first whose
    /// `claude` is the program the hook runs under.
    pub path: String,
}

impl Host {
    /// The repository holds `fix`, of two code stages, `code` and
    /// `review`, that does not land, and `talk`, a conversation.
    pub fn new(label: &str) -> Host {
        let scratch = Scratch::new(label);
        let world = World::new();
        world.kinds.set_kinds(
            REPOSITORY,
            vec![
                kind(
                    "fix",
                    false,
                    vec![
                        stage("code", Some(Deliverable::Commits), PROMPT),
                        stage("review", Some(Deliverable::Commits), PROMPT),
                    ],
                ),
                kind("talk", false, vec![stage("chat", None, PROMPT)]),
            ],
        );
        world.resolver.bind_agent(
            "claude",
            Arc::new(Claude::new(
                &scratch.path("data"),
                &scratch.path("bin/flume"),
            )),
        );
        let printed: Arc<dyn Terminal> = Arc::new(Printed);
        world.resolver.with_terminal("printed", printed.clone());
        world.host.set_host_display(Some("printed"), None, None);
        let bin = scratch.path("bin");
        std::fs::create_dir_all(&bin).expect("the scratch bin");
        // The program the start's record names, `claude`, which the hook's
        // parent is given as running.
        std::fs::write(bin.join("claude"), "the program the hook runs under\n")
            .expect("the stand-in program");
        let store = Arc::new(StateDir::at(scratch.path("state/flume")));
        Host {
            world,
            scratch,
            store,
            printed,
        }
    }

    pub fn path(&self, relative: &str) -> PathBuf {
        self.scratch.path(relative)
    }

    /// The core over this host.
    pub fn engine(&self) -> Engine {
        Engine::new(
            self.world.resolver.clone(),
            self.printed.clone(),
            self.store.clone(),
            self.world.history.clone(),
            self.world.host.clone(),
            Defaults {
                settings: Settings {
                    program: Some("claude".into()),
                    ..self.world.defaults.settings.clone()
                },
                ..self.world.defaults.clone()
            },
        )
    }

    /// The place's own folder in the state directory.
    pub fn folder(&self, place: &str) -> String {
        self.store.folder(&PlaceId::new(place))
    }

    /// The agent's record in its place's folder.
    pub fn record(&self, agent: &Agent) -> flume::profiles::agent::claude::record::Record {
        flume::profiles::agent::claude::record::Record::named(
            &self.folder(&agent.place),
            &agent.identity,
        )
    }

    /// Launches `<kind>--one` and gives its agent.
    pub fn launch(&self, kind: &str) -> Agent {
        self.launch_with(kind, &[])
    }

    /// Launches `<kind>--one`, `agent` the words each `--agent` gives, and
    /// gives its agent.
    pub fn launch_with(&self, kind: &str, agent: &[&str]) -> Agent {
        let mut given = launch(kind, "one", Some("fix the parser"));
        for words in agent {
            launch_words::agent(
                words,
                Path::new("/"),
                &mut given.every_stage,
                &mut given.settings,
            )
            .expect("the launch's settings");
        }
        launched(&self.engine().launch(&given)).expect("the launch");
        self.agent(&World::id(kind, "one").0)
    }

    /// The place's current agent, from the command status prints.
    pub fn agent(&self, place: &str) -> Agent {
        let status = self.status(place);
        let agent = &status["places"][0]["agent"];
        let printed = agent["printed"].as_str().expect("a printed command");
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
        let stage = word("FLUME_STAGE");
        let transcript = self.path(&format!("sessions/{stage}.jsonl"));
        std::fs::create_dir_all(transcript.parent().expect("a folder")).expect("sessions");
        std::fs::write(&transcript, fixture("session.jsonl")).expect("the transcript");
        Agent {
            place: word("FLUME_PLACE"),
            identity: word("FLUME_AGENT"),
            secret: word("FLUME_SECRET"),
            stage,
            plugin: PathBuf::from(plugin),
            transcript,
            prompt,
            path: self.path("bin").to_string_lossy().into_owned(),
        }
    }

    /// The place's status, as `flume status <place> --json` gives it.
    pub fn status(&self, place: &str) -> Value {
        let answer = self.engine().status(Some(&PlaceId::new(place)));
        serde_json::from_str::<Value>(&answer.to_json()).expect("the status's JSON")["status"]
            .clone()
    }

    /// The place's agent as status gives it.
    pub fn agent_status(&self, place: &str) -> Value {
        self.status(place)["places"][0]["agent"].clone()
    }

    /// Settles the place, as `flume settle <place>` does.
    pub fn settle(&self, place: &str) {
        self.engine().settle(Some(&PlaceId::new(place)));
    }

    /// The hook for `event` asked as Claude Code runs it in `agent`'s
    /// session, with `input` over the fields every hook's input carries.
    pub fn hook(&self, agent: &Agent, event: &str, input: Value) -> String {
        self.hook_in(agent, event, input, &agent.env())
    }

    /// The hook for `event` asked in the environment `env`, under the
    /// scratch `claude`.
    pub fn hook_in(
        &self,
        agent: &Agent,
        event: &str,
        mut input: Value,
        env: &[(String, String)],
    ) -> String {
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
        self.hook_given(event, &input.to_string(), env)
    }

    /// The hook for `event` asked with `input` as written, in `env`.
    pub fn hook_given(&self, event: &str, input: &str, env: &[(String, String)]) -> String {
        let env: BTreeMap<String, String> = env.iter().cloned().collect();
        hook::hook(
            event,
            input,
            &Start::of(env, Some(self.path("bin/claude"))),
            &|| Ok(self.engine()),
        )
    }

    /// Every file under the state directory and its text.
    pub fn state(&self) -> Vec<(PathBuf, Vec<u8>)> {
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
    pub fn env(&self) -> Vec<(String, String)> {
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
    EVENTS
        .iter()
        .find(|(given, _)| *given == word)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}

/// The hook's answer as Claude Code reads it.
pub fn decision(out: &str) -> Option<Value> {
    (!out.trim().is_empty())
        .then(|| serde_json::from_str(out.trim()).unwrap_or_else(|error| panic!("{error}: {out}")))
}

/// The host's pool `work` of one account, `work-1`, whose environment
/// gives `CLAUDE_CONFIG_DIR` a scratch folder, and `more` besides; the
/// folder.
fn with_an_account(host: &Host, more: &[(&str, &str)]) -> PathBuf {
    let folder = host.path("account-1");
    std::fs::create_dir_all(&folder).expect("the account's folder");
    let mut work = Account {
        program: "claude".into(),
        ..account("work-1", None)
    };
    work.env.insert(
        "CLAUDE_CONFIG_DIR".into(),
        folder.to_string_lossy().into_owned(),
    );
    for (name, value) in more {
        work.env.insert((*name).into(), (*value).into());
    }
    pool(&host.world, "work", Strategy::FillFirst, vec![work]);
    host.world.host.set_host_pool(Some("work"));
    folder
}

/// The start's mode as its record names it, from the place's held state.
fn recorded_mode(host: &Host, agent: &Agent) -> Option<String> {
    host.status(&agent.place)["places"][0]["agent"]["mode"]["passed"]
        .as_str()
        .map(str::to_string)
}

/// The start's first prompt submitted as Claude Code 2.1.322 runs
/// UserPromptSubmit, its input naming the session's mode where `mode` is
/// given.
fn first_prompt(host: &Host, agent: &Agent, mode: Option<&str>) -> String {
    let mut input = json!({ "prompt": agent.prompt });
    if let Some(mode) = mode {
        input["permission_mode"] = json!(mode);
    }
    host.hook(agent, "user-prompt-submit", input)
}

/// The place's word that its start began otherwise, as a settle reads it.
fn unsettled_read(host: &Host, place: &str) -> Value {
    host.settle(place);
    let status = host.status(place);
    let read = status["places"][0]["unsettled"][0].clone();
    assert!(read.is_object(), "nothing unsettled: {status}");
    read
}

/// The host's clock, in seconds.
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("after 1970")
        .as_secs()
}

#[test]
fn an_agent_tries_to_push() {
    // S25: the guard refuses the push, status reports it, and the stage
    // goes on (116, 108).
    let host = Host::new("push");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git add -A && git push origin HEAD" },
            "tool_use_id": "toolu_push",
        }),
    );
    let answer = decision(&ran).expect("a refusal");
    assert_eq!(answer["hookSpecificOutput"]["hookEventName"], "PreToolUse");
    assert_eq!(answer["hookSpecificOutput"]["permissionDecision"], "deny");
    let reason = answer["hookSpecificOutput"]["permissionDecisionReason"]
        .as_str()
        .unwrap_or_default();
    assert!(reason.contains("flume submits this place"), "{reason}");
    let status = host.status(&agent.place);
    let place = &status["places"][0];
    let acts = place["refused_acts"].to_string();
    assert!(acts.contains("git push"), "{status}");
    assert_eq!(place["current"], "code", "{status}");
}

#[test]
fn a_subagents_push_is_refused() {
    // The guard refuses a subagent's command as its agent's (C.4).
    let host = Host::new("subagent-push");
    let agent = host.launch("fix");
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "cd sub && sluice submit main --no-wait" },
            "tool_use_id": "toolu_sub",
            "agent_id": "a1b2",
            "agent_type": "general-purpose",
        }),
    );
    let answer = decision(&ran).expect("a refusal");
    assert_eq!(answer["hookSpecificOutput"]["permissionDecision"], "deny");
    let status = host.status(&agent.place);
    assert!(
        status["places"][0]["refused_acts"]
            .to_string()
            .contains("sluice submit"),
        "{status}"
    );
}

#[test]
fn an_agent_merges_the_base_for_a_conflict() {
    // A conflict's fix merges the base into the place's branch: never
    // refused, nothing recorded (112).
    let host = Host::new("merge");
    let agent = host.launch("fix");
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git fetch origin && git merge 4f2a9c1" },
            "tool_use_id": "toolu_merge",
        }),
    );
    assert_eq!(decision(&ran), None);
    let status = host.status(&agent.place);
    assert_eq!(status["places"][0]["refused_acts"], json!([]), "{status}");
}

#[test]
fn a_dismissed_question_clears_its_mark() {
    // A question dismissed with no PostToolUse: the next prompt's hook
    // clears the mark, and asking reads no (90).
    let host = Host::new("dismissed");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "AskUserQuestion", "tool_input": {"questions": []}, "tool_use_id": "toolu_ask" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "never mind" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");

    // A plan put to a person is asking too, until the call's own
    // PostToolUse; another call's does not clear it.
    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "ExitPlanMode", "tool_input": {"plan": "p"}, "tool_use_id": "toolu_plan" }),
    );
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "Read", "tool_input": {}, "tool_response": {}, "tool_use_id": "toolu_other" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "ExitPlanMode", "tool_input": {"plan": "p"}, "tool_response": {}, "tool_use_id": "toolu_plan" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
}

#[test]
fn a_permission_prompt_is_asking() {
    // Claude Code asks a person's permission: PermissionRequest leaves the
    // mark, the permitted call's PostToolUse clears it (90).
    let host = Host::new("permission");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let call = json!({ "command": "rm -rf build" });
    let ran = host.hook(
        &agent,
        "permission-request",
        json!({ "tool_name": "Bash", "tool_input": call }),
    );
    assert_eq!(decision(&ran), None, "the hook decides no permission");
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "Bash", "tool_input": call, "tool_response": {}, "tool_use_id": "toolu_rm" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
}

#[test]
fn a_stop_failure_as_last_words() {
    // A usage limit ends the turn: StopFailure's reason is the agent's
    // last words, which status gives with the agent, and the agent's
    // record holds the failure's error, its words and when the hook ran,
    // from which the profile reads the usage limit and its reset (43,
    // 151, C.4).
    let host = Host::new("stop-failure");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let before = now();
    host.hook(
        &agent,
        "stop-failure",
        json!({
            "error": "rate_limit",
            "last_assistant_message": "You've hit your session limit · resets 5pm (UTC)",
        }),
    );
    let after = now();
    let status = host.agent_status(&agent.place);
    assert_eq!(
        status["last_words"], "You've hit your session limit · resets 5pm (UTC) (rate_limit)",
        "{status}"
    );
    let failure = host.record(&agent).failure().expect("the failure recorded");
    assert_eq!(failure.error, "rate_limit");
    assert_eq!(
        failure.words,
        "You've hit your session limit · resets 5pm (UTC)"
    );
    assert!(
        (before..=after).contains(&failure.at),
        "the hook ran at {}, between {before} and {after}",
        failure.at
    );
}

#[test]
fn a_conversations_agent_stops() {
    // The stop check never blocks a conversation's agent (109).
    let host = Host::new("conversation");
    let agent = host.launch("talk");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = host.hook(&agent, "stop", json!({ "stop_hook_active": false }));
    assert_eq!(decision(&ran), None);
}

#[test]
fn a_session_the_operator_started() {
    // Outside an agent flume started, every hook allows what it was asked
    // and writes nothing (C.4).
    let host = Host::new("operator");
    let agent = host.launch("fix");
    let before = host.state();
    for (word, _) in EVENTS {
        let input = json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git push" },
            "tool_use_id": "toolu_x",
            "prompt": "hello",
            "source": "startup",
            "error": "rate_limit",
            "reason": "other",
        });
        let ran = host.hook_in(&agent, word, input, &[]);
        assert_eq!(decision(&ran), None, "{word}");
    }
    assert_eq!(host.state(), before, "a hook wrote outside flume's agent");
}

#[test]
fn a_prompt_submitted_as_a_paste_is_acked_as_sent() {
    // A terminal sends the prompt as a paste, and Claude Code 2.1.322,
    // where the account's features turn paste tags on, submits it inside
    // `<pasted_content id="....">` tags, newlines around them: the ack is
    // the digest of the prompt as sent, so the stage begins (38, 89;
    // record 0016).
    let host = Host::new("paste-acks");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "\n\n<pasted_content id=\"fad0\">\nfix the  parser\nand commit it\n</pasted_content id=\"fad0\">\n" }),
    );
    let wanted = flume::profiles::agent::claude::digest("fix the parser and commit it");
    assert_eq!(
        host.record(&agent).received(&wanted),
        Some(true),
        "the prompt's digest"
    );
}

#[test]
fn a_subagents_input_acks_nothing() {
    let host = Host::new("subagent-acks");
    let agent = host.launch("fix");
    host.hook(
        &agent,
        "session-start",
        json!({ "source": "startup", "agent_id": "a1b2", "agent_type": "Explore" }),
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
}

#[test]
fn a_bare_resume_acks_nothing() {
    // herdr's bare `claude --resume` carries no FLUME_* and loads no
    // plugin directory: its SessionStart acks nothing, and the start is
    // not started (77, S41).
    let host = Host::new("bare-resume");
    let agent = host.launch("fix");
    let before = host.state();
    let ran = host.hook_in(&agent, "session-start", json!({ "source": "resume" }), &[]);
    assert_eq!(decision(&ran), None);
    assert_eq!(host.state(), before, "a session flume did not start wrote");
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
}

#[test]
fn a_plugin_of_another_version() {
    // The started ack names the plugin's version; one other than the
    // binary's is reported with the agent (135).
    let host = Host::new("plugin-version");
    let agent = host.launch("fix");
    let other = host.path("other-plugin");
    std::fs::create_dir_all(other.join(".claude-plugin")).expect("a plugin");
    std::fs::write(
        other.join(".claude-plugin/plugin.json"),
        r#"{"name": "flume", "version": "9.9.9"}"#,
    )
    .expect("its manifest");
    let mut env = agent.env();
    env[4].1 = other.to_string_lossy().into_owned();
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    let status = host.agent_status(&agent.place);
    assert_eq!(status["started"], "yes");
    let notes = status["notes"].to_string();
    assert!(notes.contains("9.9.9"), "{status}");
}

#[test]
fn a_variable_with_another_value() {
    // The start's record names CLAUDE_CONFIG_DIR; the hook's environment
    // holds another value: no started ack, the agent's record names the
    // variable as different and holds no value, and a settle of the place
    // reads the start begun without its settings (161).
    let host = Host::new("another-value");
    let account = with_an_account(&host, &[]);
    let agent = host.launch("fix");
    let mut env = agent.env();
    let other = host.path("account-elsewhere");
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        other.to_string_lossy().into_owned(),
    ));
    let ran = host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(decision(&ran), None);
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
    let record = host.record(&agent);
    assert!(record.started().is_none(), "a started ack was written");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(unsettled.unsettled.different, vec!["CLAUDE_CONFIG_DIR"]);
    assert!(unsettled.unsettled.missing.is_empty() && !unsettled.unsettled.program);
    let written =
        std::fs::read_to_string(record.dir().join("unsettled.json")).expect("the word as written");
    for value in [account.to_string_lossy(), other.to_string_lossy()] {
        assert!(!written.contains(value.as_ref()), "a value in {written}");
    }
    let read = unsettled_read(&host, &agent.place);
    assert_eq!(read["different"], json!(["CLAUDE_CONFIG_DIR"]), "{read}");
    assert_eq!(read["start"], 1, "{read}");

    // The same variable as the start gave it: the ack is taken.
    let host = Host::new("same-value");
    let account = with_an_account(&host, &[]);
    let agent = host.launch("fix");
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        account.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
}

#[test]
fn a_credential_the_program_keeps_from_its_hooks() {
    // The account gives CLAUDE_CONFIG_DIR and CLAUDE_CODE_OAUTH_TOKEN;
    // Claude Code keeps the token from its hooks, so the hook's
    // environment holds CLAUDE_CONFIG_DIR alone. The start's record names
    // no token and the ack is taken (161).
    let host = Host::new("withheld");
    let account = with_an_account(&host, &[("CLAUDE_CODE_OAUTH_TOKEN", "not-a-token")]);
    let agent = host.launch("fix");
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        account.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
    fn records(value: &Value, found: &mut Vec<Value>) {
        match value {
            Value::Object(fields) => {
                for (key, field) in fields {
                    if key == "start_record" {
                        found.push(field.clone());
                    } else {
                        records(field, found);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|item| records(item, found)),
            _ => {}
        }
    }
    let mut found = vec![];
    for (_, text) in host.state() {
        if let Ok(document) = serde_json::from_slice::<Value>(&text) {
            records(&document, &mut found);
        }
    }
    assert_eq!(found.len(), 1, "the start's record");
    let names: Vec<&String> = found[0]["env"]
        .as_object()
        .expect("the record's variables")
        .keys()
        .collect();
    assert_eq!(
        names,
        vec![
            "CLAUDE_CONFIG_DIR",
            "FLUME_AGENT",
            "FLUME_PLACE",
            "FLUME_STAGE"
        ]
    );
}

#[test]
fn another_program() {
    // The hook's parent runs another program than the `claude` its PATH
    // finds: no started ack, and the agent's record says the program
    // differed (161).
    let host = Host::new("another-program");
    let agent = host.launch("fix");
    let elsewhere = host.path("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("a folder");
    std::fs::write(elsewhere.join("claude"), "another program\n").expect("another program");
    let mut env = agent.env();
    env[5].1 = format!("{}:{}", elsewhere.display(), agent.path);
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
    let unsettled = host
        .record(&agent)
        .unsettled()
        .expect("the word that it began otherwise");
    assert!(unsettled.unsettled.program, "{unsettled:?}");
    assert!(unsettled.unsettled.missing.is_empty() && unsettled.unsettled.different.is_empty());
}

#[test]
fn another_mode() {
    // The start's record names bypassPermissions. SessionStart's input
    // names no mode, as 2.1.322's never does, and the started ack is taken
    // on the program and the variables. The start's first prompt's input
    // reads `default`, as Claude Code begins where the account's
    // configuration disables the mode asked: the prompt is blocked with
    // words naming both modes, so the agent takes no turn, no received ack
    // is written, the agent's record names the mode asked and the mode
    // begun with the started ack's transcript, and a settle of the place
    // reads the start begun without its settings (161, 162, S67).
    let host = Host::new("another-mode");
    let agent = host.launch_with("fix", &["mode=full"]);
    assert_eq!(
        recorded_mode(&host, &agent).as_deref(),
        Some("bypassPermissions")
    );
    let ran = host.hook(&agent, "session-start", json!({ "source": "startup" }));
    assert_eq!(decision(&ran), None);
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
    let ran = first_prompt(&host, &agent, Some("default"));
    let blocked = decision(&ran).expect("the prompt blocked");
    assert_eq!(blocked["decision"], "block", "{blocked}");
    assert!(
        blocked["reason"].as_str().is_some_and(|reason| reason
            .contains("in the mode `manual`, where its start asked `bypassPermissions`")),
        "{blocked}"
    );
    let record = host.record(&agent);
    assert!(!record.received_any(), "the prompt held back was acked");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(
        unsettled.unsettled.mode,
        Some(ModeBegan {
            asked: "bypassPermissions".into(),
            began: Some("manual".into()),
        })
    );
    assert!(
        unsettled.unsettled.missing.is_empty()
            && unsettled.unsettled.different.is_empty()
            && !unsettled.unsettled.program,
        "{unsettled:?}"
    );
    assert_eq!(
        Path::new(&unsettled.transcript_path),
        agent.transcript.as_path(),
        "the started ack's transcript"
    );
    let read = unsettled_read(&host, &agent.place);
    assert_eq!(
        read["mode"],
        json!({ "asked": "bypassPermissions", "began": "manual" }),
        "{read}"
    );
}

#[test]
fn manual_read_back_as_default() {
    // Claude Code 2.1.322 reads `--permission-mode manual` as its own
    // `default` and reports it so: the first prompt is taken (161, C.4).
    let host = Host::new("manual-mode");
    let agent = host.launch_with("fix", &["mode=ask"]);
    assert_eq!(recorded_mode(&host, &agent).as_deref(), Some("manual"));
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("default"));
    assert_eq!(decision(&ran), None);
    let record = host.record(&agent);
    assert!(record.received_any(), "the first prompt was not acked");
    assert!(record.unsettled().is_none());
}

#[test]
fn no_mode_in_the_record() {
    // A start whose settings give no mode passes none and compares none,
    // whatever mode the account's configuration began the session in
    // (161, 162).
    let host = Host::new("no-mode");
    let agent = host.launch("fix");
    assert_eq!(recorded_mode(&host, &agent), None);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("acceptEdits"));
    assert_eq!(decision(&ran), None);
    let record = host.record(&agent);
    assert!(record.received_any(), "the first prompt was not acked");
    assert!(record.unsettled().is_none());
}

#[test]
fn no_mode_in_the_input() {
    // The start's record names acceptEdits and its first prompt's input
    // names no mode: the prompt is blocked, no received ack, the agent's
    // record naming the mode asked and no mode begun (161).
    let host = Host::new("no-mode-input");
    let agent = host.launch_with("fix", &["mode=edits"]);
    assert_eq!(recorded_mode(&host, &agent).as_deref(), Some("acceptEdits"));
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, None);
    assert_eq!(
        decision(&ran).map(|blocked| blocked["decision"].clone()),
        Some(json!("block"))
    );
    let record = host.record(&agent);
    assert!(!record.received_any(), "the prompt held back was acked");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(
        unsettled.unsettled.mode,
        Some(ModeBegan {
            asked: "acceptEdits".into(),
            began: None,
        })
    );
}

#[test]
fn a_later_prompt_and_a_compaction_after_the_mode_moved() {
    // A start's first prompt taken in acceptEdits; once started, a person
    // moves its session's mode. A later prompt, a message, compares no mode
    // and is acked received; the session compacts, SessionStart compares
    // no mode either, so the started ack is written again with the
    // session's transcript, and nothing is unsettled (161, 162).
    let host = Host::new("compaction-mode");
    let agent = host.launch_with("fix", &["mode=edits"]);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("acceptEdits"));
    assert_eq!(decision(&ran), None);
    let ran = host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "a message", "permission_mode": "default" }),
    );
    assert_eq!(decision(&ran), None, "a later prompt blocked");
    let compacted = host.path("sessions/compacted.jsonl");
    std::fs::write(&compacted, fixture("session.jsonl")).expect("the transcript");
    host.hook(
        &agent,
        "session-start",
        json!({ "source": "compact", "transcript_path": compacted }),
    );
    let record = host.record(&agent);
    let started = record.started().expect("the started ack");
    assert_eq!(started.source, "compact");
    assert_eq!(
        Path::new(&started.transcript_path),
        compacted.as_path(),
        "{started:?}"
    );
    assert!(
        record.unsettled().is_none(),
        "unsettled after the mode moved"
    );
    assert_eq!(
        std::fs::read_to_string(record.received_path())
            .unwrap_or_default()
            .lines()
            .count(),
        2,
        "both prompts acked received"
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
}

#[test]
fn an_ack_by_hand_with_no_mode() {
    // `flume ack received` at the first prompt of a start whose record
    // names acceptEdits: with `--mode acceptEdits` it is taken; with no
    // `--mode` it acks nothing, naming the mode asked and no mode begun
    // (135, 161).
    let by_hand = |label: &str, mode: Option<&str>| {
        let host = Host::new(label);
        let agent = host.launch_with("fix", &["mode=edits"]);
        let start = Start::of(
            agent.env().into_iter().collect(),
            Some(host.path("bin/claude")),
        );
        let acked = hook::ack_received(&start, "a1b2c3", mode, "", "", &|| Ok(host.engine()))
            .map_err(hook::Unacked::words);
        (acked, host.record(&agent), host)
    };

    let (acked, record, _host) = by_hand("ack-with-mode", Some("acceptEdits"));
    assert!(acked.is_ok(), "{acked:?}");
    assert_eq!(
        record.received("a1b2c3"),
        Some(true),
        "the ack was not taken"
    );
    assert!(record.unsettled().is_none());

    let (acked, record, _host) = by_hand("ack-without-mode", None);
    let words = acked.expect_err("an ack with no mode taken");
    assert!(!record.received_any(), "a received ack was written");
    assert_eq!(
        record.unsettled().map(|unsettled| unsettled.unsettled.mode),
        Some(Some(ModeBegan {
            asked: "acceptEdits".into(),
            began: None,
        }))
    );
    assert!(
        words.contains("in a mode its word did not name, where its start asked `acceptEdits`"),
        "{words}"
    );
}
