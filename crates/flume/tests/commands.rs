//! The commands against the built binary on scratch repositories bound to
//! `git-worktree` and no merge queue (section 9; design 16; command-line
//! spec). Each command runs in a process of its own, its state and the
//! host's configuration under the test's scratch directories, nothing of
//! the caller's session in its environment.
//!
//! The launches run through this test binary re-run as flume: the same
//! commands over the profiles built in, beside a stand-in agent program
//! bound by its name, which reads its start and its end from marks the
//! test writes in the place's folder, and stand-in merge queues for the
//! scenarios that need a kind that lands, one of them holding each
//! submission and recording each step for a caller's watch. Every answer read back is
//! read through flume's own binary where the scenario allows.

mod common;

use common::{git, Container, Repository, Scratch};
use flume::commands;
use flume::host::config::{self, HostConfig};
use flume::profiles::kinds::weir::Weir;
use flume::profiles::place::git_worktree;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::{core_over, core_with, BuiltIn};
use flume_core::Answer;
use flume_seams::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

/// The stand-in agent program's name.
const STAND_IN: &str = "stand-in";

/// The recipient a launch's environment names, `$FLUME_RECIPIENT`, where
/// the test gives none of its own: a launch from no session that names
/// none and finds no role behind its stream is refused (7, 193).
const CALLER: &str = "caller";

/// `env` for the command `words`: a launch's names `$FLUME_RECIPIENT` as
/// `CALLER` where `env` names none, and every other command's is `env`.
fn launch_env<'a>(words: &[&str], env: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
    let mut env = env.to_vec();
    if words.contains(&"launch") && !env.iter().any(|(name, _)| *name == "FLUME_RECIPIENT") {
        env.push(("FLUME_RECIPIENT", CALLER));
    }
    env
}

/// The stand-in merge queue's name.
const STAND_IN_QUEUE: &str = "stand-in-queue";

/// The holding stand-in merge queue's name.
const HOLDING_QUEUE: &str = "stand-in-holding-queue";

/// The variable naming the file the holding stand-in merge queue keeps
/// its record in, one line for each submit and each step, across the
/// commands' processes.
const QUEUE_LOG: &str = "FLUME_TEST_QUEUE_LOG";

/// The variable that makes this test binary run as flume, the command's
/// words in it as a JSON list.
const AS_FLUME: &str = "FLUME_TEST_AS_FLUME";

/// The stand-in agent's name that never says it started, as a program
/// the printed command gives a person to run reads until it runs (35).
const UNSTARTED: &str = "stand-in-unstarted";

/// The stand-in agent: it says it started, where it is `stand-in`, and
/// received its prompt at once, never asks or keeps own work, and reads
/// its session ended once the test writes the mark
/// `stand-in/<stage>-<start>` in the place's folder. It maps each mode to
/// its own word and reads none back, takes its program's own settings,
/// passing them nowhere (195), and takes every effort, passing it as
/// `--effort <word>` (197).
struct StandInAgent {
    says_started: bool,
}

fn ended_mark(folder: &str, agent: &Identity) -> PathBuf {
    Path::new(folder)
        .join(STAND_IN)
        .join(format!("{}-{}", agent.stage, agent.start))
}

impl Agent for StandInAgent {
    fn command(
        &self,
        _folder: &str,
        settings: &Settings,
        _mode: Option<&str>,
        _own: &BTreeMap<String, serde_json::Value>,
        _resume: Option<&str>,
        env: &BTreeMap<String, String>,
        _place: &PlaceRef,
        _identity: &Identity,
        _secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        let mut args = vec![];
        if let Some(model) = &settings.model {
            args.extend(["--model".to_string(), model.clone()]);
        }
        if let Some(effort) = &settings.effort {
            args.extend(["--effort".to_string(), effort.clone()]);
        }
        if let Some(instructions) = &settings.instructions {
            args.extend(["--instructions".to_string(), instructions.clone()]);
        }
        AgentCommand {
            program: settings.program.clone().unwrap_or_default(),
            args,
            env: env.clone(),
            prompt_args: vec![prompt.to_string()],
            folder_trust: None,
            kind: None,
            files: BTreeMap::new(),
        }
    }
    fn started(&self, _folder: &str, _agent: &Identity) -> Reading {
        if self.says_started {
            Reading::Yes
        } else {
            Reading::No
        }
    }
    fn received(&self, _folder: &str, _agent: &Identity, _text: &str) -> Received {
        Received::Received {
            record: STAND_IN.into(),
        }
    }
    fn asking(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }
    fn own_work(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }
    fn ended(&self, folder: &str, agent: &Identity) -> Reading {
        if ended_mark(folder, agent).exists() {
            Reading::Yes
        } else {
            Reading::No
        }
    }
    fn last_words(&self, _folder: &str, _agent: &Identity) -> Option<String> {
        None
    }
    fn stop_short(&self, _agent: &Identity) -> String {
        "flume needs \"<words>\"".into()
    }
    fn stop_short_to(&self) -> String {
        "flume needs --to <role>".into()
    }
    /// Each of flume's four as its word, and an own value as written.
    fn mode(&self, entry: &ModeEntry) -> Result<String, String> {
        Ok(entry.to_string())
    }
    fn own(&self, _given: &BTreeMap<String, serde_json::Value>) -> Result<(), String> {
        Ok(())
    }
    /// Every word, passed as written.
    fn effort(&self, _word: &str) -> Result<(), String> {
        Ok(())
    }
    fn send_back(&self) -> Offered {
        Offered::NotOffered
    }
    fn guard(&self, _folder: &str, _agent: &Identity) -> Guarded {
        Guarded::NotOffered
    }
    fn on_quiet(&self) -> Offered {
        Offered::NotOffered
    }
}

/// The stand-in merge queue: the base where the repository holds it,
/// nothing submitted and nothing to collect.
struct StandInQueue;

impl MergeQueue for StandInQueue {
    fn submit(&self, _place: &PlaceRef, _head: &SubmitHead) -> Submitted {
        Submitted::Refused {
            reason: "the stand-in queue takes nothing".into(),
        }
    }
    fn inbox(&self, _place: &PlaceRef) -> Inbox {
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![],
        }
    }
    fn confirm(&self, _place: &PlaceRef, _key: &str) -> Confirmed {
        Confirmed::Confirmed
    }
    fn read_submissions(&self, _repository: &str, branches: &[&str]) -> Vec<ReadAcross> {
        branches
            .iter()
            .map(|_| SubmissionRead::None.into())
            .collect()
    }
    fn step(&self, _repository: &str, _base: &str) -> Stepped {
        Stepped::MovesOnItsOwn
    }
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        match flume::git::Git::at(repository).commit(base) {
            Ok(revision) => BaseRevision::At {
                revision: Revision::new(revision),
            },
            Err(reason) => BaseRevision::Refused { reason },
        }
    }
    fn open(&self, _repository: &str, _branch: &str, _base: &str) -> BranchOpened {
        BranchOpened::Refused {
            reason: "the stand-in queue opens nothing".into(),
        }
    }
    fn close(&self, _repository: &str, _branch: &str, _into: &str) -> BranchClosed {
        BranchClosed::Gone
    }
    fn forecast(&self, _repository: &str, _branch: &str, _base: &str) -> Forecast {
        Forecast::CouldNotRead {
            reason: "the stand-in queue forecasts nothing".into(),
        }
    }
}

/// The holding stand-in merge queue: each submission held open through
/// every round, each submit and each step a line of the file
/// `$FLUME_TEST_QUEUE_LOG`, `submit <branch>` and `step <base>`; a step
/// runs its round, deciding nothing.
struct HoldingQueue;

impl HoldingQueue {
    fn log() -> PathBuf {
        PathBuf::from(std::env::var(QUEUE_LOG).expect("the holding queue's record"))
    }

    fn record(line: &str) {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(Self::log())
            .expect("the holding queue's record");
        writeln!(file, "{line}").expect("the holding queue's record written");
    }

    fn submission(branch: &str) -> Submission {
        Submission {
            id: format!("held-{branch}"),
            branch: branch.into(),
            target: "main".into(),
            state: SubmissionState::Waiting,
            request: None,
        }
    }
}

impl MergeQueue for HoldingQueue {
    fn submit(&self, place: &PlaceRef, _head: &SubmitHead) -> Submitted {
        Self::record(&format!("submit {}", place.branch));
        Submitted::Submitted {
            submission: Self::submission(&place.branch),
        }
    }
    fn inbox(&self, _place: &PlaceRef) -> Inbox {
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![],
        }
    }
    fn confirm(&self, _place: &PlaceRef, _key: &str) -> Confirmed {
        Confirmed::Confirmed
    }
    fn read_submissions(&self, _repository: &str, branches: &[&str]) -> Vec<ReadAcross> {
        let record = std::fs::read_to_string(Self::log()).unwrap_or_default();
        branches
            .iter()
            .map(|branch| {
                if record
                    .lines()
                    .any(|line| line == format!("submit {branch}"))
                {
                    SubmissionRead::Found {
                        submission: Self::submission(branch),
                    }
                    .into()
                } else {
                    SubmissionRead::None.into()
                }
            })
            .collect()
    }
    fn step(&self, _repository: &str, base: &str) -> Stepped {
        Self::record(&format!("step {base}"));
        Stepped::Stepped
    }
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        StandInQueue.base_revision(repository, base)
    }
    fn open(&self, repository: &str, branch: &str, base: &str) -> BranchOpened {
        StandInQueue.open(repository, branch, base)
    }
    fn close(&self, repository: &str, branch: &str, into: &str) -> BranchClosed {
        StandInQueue.close(repository, branch, into)
    }
    fn forecast(&self, _repository: &str, _branch: &str, _base: &str) -> Forecast {
        Forecast::CouldNotRead {
            reason: "the holding stand-in queue forecasts nothing".into(),
        }
    }
}

/// The variable that has the stand-in flume make its places at
/// `<repository>/places/<kind>--<name>`, inside a clone's main worktree
/// or beside a bare repository's worktrees, as flume made them before
/// record 0027.
const PLACES_INSIDE: &str = "FLUME_TEST_PLACES_INSIDE";

/// The place profile built in, its places made in the repository's
/// directory.
struct Inside(Arc<dyn Place>);

impl Place for Inside {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        Ok(format!("{repository}/places/{place}"))
    }
    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.0.make(repository, revision, branch, location)
    }
    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.0.ready(place, revision)
    }
    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.0.read(place)
    }
    fn remove(&self, place: &PlaceRef) -> Removed {
        self.0.remove(place)
    }
    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.0.remove_branch(repository, branch, base)
    }
    fn make_checkouts(&self, location: &str, checkouts: &[Checkout]) -> MakeOutcome {
        self.0.make_checkouts(location, checkouts)
    }
    fn move_checkout(&self, checkout: &PlaceRef, revision: &Revision) -> Moved {
        self.0.move_checkout(checkout, revision)
    }
    fn remove_checkouts(&self, location: &str, checkouts: &[Checkout]) -> Removed {
        self.0.remove_checkouts(location, checkouts)
    }
}

/// The profiles built in, with the stand-ins beside them.
struct StandIn(BuiltIn);

impl Resolver for StandIn {
    fn kinds(&self) -> Arc<dyn Kinds> {
        self.0.kinds()
    }
    fn place_profiles(&self) -> Vec<String> {
        self.0.place_profiles()
    }
    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>> {
        let place = self.0.place(repository, binding)?;
        if std::env::var_os(PLACES_INSIDE).is_some() {
            return Some(Arc::new(Inside(place)));
        }
        Some(place)
    }
    fn change(&self, profile: &str) -> Option<Arc<dyn Change>> {
        self.0.change(profile)
    }
    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>> {
        match profile {
            STAND_IN_QUEUE => Some(Arc::new(StandInQueue)),
            HOLDING_QUEUE => Some(Arc::new(HoldingQueue)),
            _ => self.0.merge_queue(profile),
        }
    }
    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        let says_started = match program {
            STAND_IN => true,
            UNSTARTED => false,
            _ => return None,
        };
        Some(Arc::new(StandInAgent { says_started }))
    }
    fn terminal(&self, profile: &str) -> Option<Arc<dyn Terminal>> {
        self.0.terminal(profile)
    }
    fn terminal_profiles(&self) -> Vec<String> {
        self.0.terminal_profiles()
    }
    fn window(&self, profile: &str) -> Option<Arc<dyn Window>> {
        self.0.window(profile)
    }
    fn window_profiles(&self) -> Vec<String> {
        self.0.window_profiles()
    }
}

/// This test binary as flume, where the test that re-runs it asks: the
/// commands over the profiles built in and the stand-ins, the core built
/// as flume's binary builds it, the login environment its `HOME` alone.
#[test]
fn stand_in_flume() {
    let Ok(words) = std::env::var(AS_FLUME) else {
        return;
    };
    let words: Vec<String> = serde_json::from_str(&words).expect("the command's words");
    let build: commands::Build = std::sync::Arc::new(|| {
        let home = std::env::var("HOME").unwrap_or_default();
        let state = config::flume_state()?;
        core_with(
            &state,
            HostConfig::of_host()?,
            flume::host::recipient(),
            Arc::new(StandIn(
                BuiltIn::with_login(&[("HOME", &home)]).over(&state),
            )),
            Some(vec![("HOME".into(), home)]),
        )
    });
    let outcome = commands::run_built(std::iter::once("flume".to_string()).chain(words), &build);
    let mut out = std::io::stdout().lock();
    writeln!(out, "{}", outcome.text).expect("the answer written");
    out.flush().expect("the answer written");
    std::process::exit(i32::from(outcome.code));
}

/// One host's scratch: flume's state, its configuration, a home, and
/// its resident once a command asks status, run as the stand-in flume
/// (flume-host-serves-status-and-its-subscription-on-its-socket).
struct Host {
    /// Ends first, before its state is taken away.
    resident: std::cell::RefCell<Option<common::resident::Resident>>,
    scratch: Scratch,
    user: PathBuf,
}

/// What a command printed and its exit status.
struct Ran {
    text: String,
    code: i32,
}

impl Ran {
    fn answer(&self) -> Answer {
        serde_json::from_str(self.text.trim())
            .unwrap_or_else(|error| panic!("an answer's JSON ({error}): {}", self.text))
    }
    fn json(&self) -> Value {
        serde_json::from_str(self.text.trim()).expect("the answer's JSON")
    }
}

impl Host {
    fn new(label: &str) -> Host {
        let scratch = Scratch::new(label);
        for dir in ["state", "config/flume", "home"] {
            std::fs::create_dir_all(scratch.path(dir)).expect("the host's directory");
        }
        Host {
            scratch,
            user: common::resident::user_scratch(),
            resident: std::cell::RefCell::new(None),
        }
    }

    /// The host's resident, started where none runs yet, with `env` over
    /// the host's own.
    fn resident(&self, env: &[(&str, &str)]) {
        if self.resident.borrow().is_some() {
            return;
        }
        let me = std::env::current_exe().expect("the test binary");
        let words = ["host", "--every", common::resident::EVERY];
        let mut command = self.command(&me, &self.scratch.root, env);
        command
            .env(AS_FLUME, serde_json::to_string(&words).expect("words"))
            .args([
                "--exact",
                "stand_in_flume",
                "--nocapture",
                "--test-threads=1",
                "-q",
            ]);
        *self.resident.borrow_mut() = Some(common::resident::Resident::start(command, &self.user));
    }

    fn state(&self) -> PathBuf {
        self.scratch.path("state/flume")
    }

    fn home(&self) -> PathBuf {
        self.scratch.path("home")
    }

    /// flume's own data on the host, under its home, where the stand-in
    /// flume's login environment, its `HOME` alone, puts it too.
    fn data(&self) -> PathBuf {
        self.home().join(".local/share/flume")
    }

    /// Where flume makes the place `<kind>--<name>` of `repository` on
    /// this host (C.1; record 0027).
    fn place_of(&self, repository: &Repository, place: &str) -> String {
        self.place_in(&repository.name(), place)
    }

    /// Where flume makes the place `<kind>--<name>` of the repository
    /// named by the directory `repository` on this host.
    fn place_in(&self, repository: &str, place: &str) -> String {
        git_worktree::location(repository, place, Some(&self.data())).expect("the place's location")
    }

    /// Writes the host's configuration.
    fn configure(&self, text: &str) {
        std::fs::write(self.scratch.path("config/flume/config.toml"), text)
            .expect("the configuration");
    }

    /// Writes the host's local layer, `config.local.toml` (C.7).
    fn configure_local(&self, text: &str) {
        std::fs::write(self.scratch.path("config/flume/config.local.toml"), text)
            .expect("the local layer");
    }

    /// The command's process, run in `dir` with `env` set over nothing of
    /// the caller's flume or terminal session.
    fn command(&self, program: &Path, dir: &Path, env: &[(&str, &str)]) -> Command {
        let mut command = Command::new(program);
        command.current_dir(dir);
        for (name, _) in std::env::vars() {
            if name.starts_with("FLUME_") || name.starts_with("HERDR_") || name.starts_with("REX_")
            {
                command.env_remove(name);
            }
        }
        command
            .env(common::resident::USER_SCRATCH, &self.user)
            .env("XDG_STATE_HOME", self.scratch.path("state"))
            .env("XDG_CONFIG_HOME", self.scratch.path("config"))
            .env("XDG_DATA_HOME", self.home().join(".local/share"))
            .env("HOME", self.home());
        for (name, value) in env {
            command.env(name, value);
        }
        command
    }

    /// flume's own binary.
    fn flume(&self, dir: &Path, words: &[&str], env: &[(&str, &str)]) -> Ran {
        if common::resident::asks_status(words) {
            self.resident(env);
        }
        let env = launch_env(words, env);
        let output = self
            .command(Path::new(env!("CARGO_BIN_EXE_flume")), dir, &env)
            .args(words)
            .output()
            .expect("flume runs");
        Ran {
            text: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// This test binary as flume, with the stand-ins.
    fn stand_in(&self, dir: &Path, words: &[&str], env: &[(&str, &str)]) -> Ran {
        if common::resident::asks_status(words) {
            self.resident(env);
        }
        let me = std::env::current_exe().expect("the test binary");
        let env = launch_env(words, env);
        let output = self
            .command(&me, dir, &env)
            .env(AS_FLUME, serde_json::to_string(words).expect("words"))
            .args([
                "--exact",
                "stand_in_flume",
                "--nocapture",
                "--test-threads=1",
                "-q",
            ])
            .output()
            .expect("the test binary runs");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let text = stdout
            .split_once("running 1 test\n")
            .map_or(stdout.as_str(), |(_, after)| after)
            .to_string();
        Ran {
            text,
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// The place's own folder in the store.
    fn folder(&self, place: &str) -> String {
        StateDir::at(self.state()).folder(&PlaceId::new(place))
    }

    /// Writes the stand-in agent's mark that its session ended.
    fn end_agent(&self, place: &str, stage: &str, start: u32) {
        let identity = Identity {
            repository: String::new(),
            place: String::new(),
            stage: stage.into(),
            start,
            resume: 0,
        };
        let mark = ended_mark(&self.folder(place), &identity);
        std::fs::create_dir_all(mark.parent().expect("a folder")).expect("the mark's folder");
        std::fs::write(mark, "").expect("the mark");
    }
}

/// A scratch repository bound to `git-worktree` and the merge queue
/// `queue`, none where it is none.
fn repository(label: &str, queue: Option<&str>) -> Repository {
    let repository = Repository::new(label);
    let mut binding = "[profiles]\nplace = \"git-worktree\"\n".to_string();
    if let Some(queue) = queue {
        binding.push_str(&format!("merge_queue = \"{queue}\"\n"));
    }
    repository.write(".config/flume.toml", &binding);
    repository.commit("the binding");
    repository
}

/// A declaration the launch carries, beside its prompt: `change` with
/// construct, code and verify, and `fix` with code, each stage's agent
/// the stand-in and its deliverable commits; `fix` lands where `lands`.
fn kinds_file(host: &Host, lands: bool) -> String {
    let dir = host.scratch.path("kinds");
    std::fs::create_dir_all(dir.join("prompts")).expect("the kinds' folder");
    std::fs::write(
        dir.join("prompts/work.md"),
        "Do the work of the {{kind}} {{name}} in {{place}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let stage = |kind: &str, name: &str| {
        format!(
            "[[kinds.{kind}.stages]]\nname = \"{name}\"\nprompt = \"prompts/work.md\"\ndeliverables = [\"commits\"]\nagents = [{{ program = \"{STAND_IN}\" }}]\n\n"
        )
    };
    let text = format!(
        "[kinds.change]\nversion = 1\nbranch = \"change\"\nstarts_unready = false\nlands = false\n\n{}{}{}[kinds.fix]\nversion = 1\nbranch = \"fix\"\nstarts_unready = true\nlands = {lands}\n\n{}",
        stage("change", "construct"),
        stage("change", "code"),
        stage("change", "verify"),
        stage("fix", "code"),
    );
    let file = dir.join("kinds.toml");
    std::fs::write(&file, text).expect("the declaration");
    file.to_string_lossy().into_owned()
}

/// The one place a status or a launch answers.
fn the_place(answer: &Answer) -> &flume_core::PlaceStatus {
    match answer {
        Answer::Launched { place } => place,
        Answer::Status { places, .. } if places.len() == 1 => &places[0],
        other => panic!("one place's answer: {other:?}"),
    }
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

#[test]
fn the_same_answer_through_the_command_line_and_the_core() {
    // S14, 60: status through flume's binary is the core's own answer.
    let host = Host::new("same-answer");
    let repository = repository("same-answer", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    let ran = host.flume(&repository.dir, &["status", &place, "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    // The host's resident answers status, over the profiles the stand-in
    // flume builds its core with.
    let home = host.home().to_string_lossy().into_owned();
    let core = core_with(
        &host.state(),
        HostConfig::default(),
        flume::host::recipient(),
        Arc::new(StandIn(
            BuiltIn::with_login(&[("HOME", &home)]).over(&host.state()),
        )),
        Some(vec![("HOME".into(), home.clone())]),
    )
    .expect("the core over the same state");
    assert_eq!(ran.answer(), core.status(Some(&PlaceId::new(&place))));
    let person = host.flume(&repository.dir, &["status", &place], &[]);
    assert_eq!(person.code, 0);
    assert!(
        person.text.starts_with(&format!("{place}: fix@1")),
        "{}",
        person.text
    );
}

#[test]
fn status_with_binding_gives_the_repositorys_binding_through_flume_host() {
    // 20, 105: `flume status --binding` asks flume host of the repository
    // the command runs in, and the answer names the merge queue it lacks.
    let host = Host::new("binding");
    let repository = repository("binding", None);
    let ran = host.flume(&repository.dir, &["status", "--binding", "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    let json = ran.json();
    let read = &json["status"]["repositories"][0];
    assert_eq!(
        read["repository"],
        repository
            .dir
            .canonicalize()
            .expect("the repository")
            .to_string_lossy()
            .as_ref()
    );
    assert_eq!(read["binding"]["read"]["binding"]["place"], "git-worktree");
    assert_eq!(read["merge_queue"]["ready"], false);
    assert!(
        read["merge_queue"]["words"]
            .as_str()
            .is_some_and(|words| words.contains("names no merge_queue")),
        "{read}"
    );
    assert!(read["kinds"].as_array().is_some_and(|kinds| kinds
        .iter()
        .any(|kind| kind["kind"] == "flume/fix@1"
            && kind["lands"] == true
            && kind["refused"].is_string())));
    let person = host.flume(&repository.dir, &["status", "--binding"], &[]);
    assert_eq!(person.code, 0, "{}", person.text);
    assert!(
        person.text.contains("merge queue: not ready"),
        "{}",
        person.text
    );
    let plain = host.flume(&repository.dir, &["status", "--json"], &[]);
    assert!(plain.json()["status"].get("repositories").is_none());
    let both = host.flume(&repository.dir, &["status", "--binding", "--line"], &[]);
    assert_ne!(both.code, 0, "a line names no repository: {}", both.text);
}

#[test]
fn a_launch_naming_no_base_is_cut_from_the_remotes_default_branch() {
    // 2, 20: in a repository made by `git init --bare`, whose HEAD names
    // master, a branch never made, and that has no origin/HEAD, every
    // launch naming no base is cut from the remote's main.
    let host = Host::new("default-base");
    let origin = repository("default-base", None);
    let remote = origin.bare_remote();
    let container = Container::of(&remote, Scratch::new("default-base"));
    let kinds = kinds_file(&host, false);
    for name in ["one", "two"] {
        let launched = host.stand_in(
            &container.dir,
            &[
                "launch", "--kind", "fix", "--name", name, "--kinds", &kinds, "--json",
            ],
            &[],
        );
        assert_eq!(launched.code, 0, "{}", launched.text);
        let answer = launched.answer();
        let place = the_place(&answer);
        assert_eq!(place.base, "main", "the launch of {name}");
        assert_eq!(
            git(
                Path::new(&place.place.0),
                &["rev-parse", &format!("fix/{name}^{{commit}}")]
            ),
            origin.head(),
            "the place of {name} cut from the remote's main"
        );
    }
}

/// A clone of `source` through a bare remote beside it, the host's
/// `origin`, and two commits pushed to that remote after the clone, so
/// the clone's own `main`, which nothing on the host moves, lags it: the
/// clone, and origin's `main` (S92).
fn a_clone_lagging_origin(source: &Repository, label: &str) -> (Repository, String) {
    let remote = source.bare_remote();
    let clone = Repository::clone_of(&remote, Scratch::new(label));
    for file in ["one.rs", "two.rs"] {
        source.write(file, "landed on origin\n");
        source.commit(file);
    }
    source.git(&["push", "--quiet", &remote.to_string_lossy(), "main"]);
    assert_ne!(clone.head(), source.head(), "the clone's main is origin's");
    (clone, source.head())
}

/// The commit a place's branch was cut at, the branch made by the launch.
fn cut_at(place: &flume_core::PlaceStatus) -> String {
    git(
        Path::new(&place.place.0),
        &["rev-parse", &format!("{}^{{commit}}", place.branch)],
    )
}

#[test]
fn a_base_no_merge_queue_lands_is_cut_from_origins_copy() {
    // S92; 9, 56, 126, C.1: in a clone bound to no merge queue, whose own
    // main lags origin's, a place is cut at origin's after flume's fetch,
    // the clone's main left where it was; one on a branch origin lacks at
    // the clone's own; and with origin out of reach, a launch is refused
    // naming the fetch, nothing made. Status says where each base was
    // read.
    let host = Host::new("origin-base");
    let source = repository("origin-base", None);
    let (clone, origins) = a_clone_lagging_origin(&source, "origin-base-clone");
    let own = clone.head();
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &clone.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let answer = launched.answer();
    let place = the_place(&answer);
    assert_eq!(cut_at(place), origins, "the place cut at origin's main");
    assert_eq!(place.base_from, Some(BaseFrom::Origin));
    assert_eq!(clone.head(), own, "the clone's own main moved");
    assert_eq!(
        clone.git(&["rev-parse", "refs/remotes/origin/main"]),
        origins,
        "origin's main as the fetch left it"
    );
    let status = host.flume(&clone.dir, &["status", &place.place.0, "--json"], &[]);
    assert_eq!(
        status.json()["status"]["places"][0]["base_from"],
        json!("origin")
    );
    let person = host.flume(&clone.dir, &["status", &place.place.0], &[]);
    assert!(
        person.text.contains("branch fix/one from main (at origin)"),
        "{}",
        person.text
    );

    clone.git(&["branch", "trial"]);
    let launched = host.stand_in(
        &clone.dir,
        &[
            "launch", "--kind", "fix", "--name", "two", "--base", "trial", "--kinds", &kinds,
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let answer = launched.answer();
    let place = the_place(&answer);
    assert_eq!(cut_at(place), own, "the place cut at the clone's own trial");
    assert_eq!(place.base_from, Some(BaseFrom::OriginLacks));
    let person = host.flume(&clone.dir, &["status", &place.place.0], &[]);
    assert!(
        person
            .text
            .contains("branch fix/two from trial (the host's own, origin holding none)"),
        "{}",
        person.text
    );

    let gone = clone.scratch.path("gone.git");
    clone.git(&["remote", "set-url", "origin", &gone.to_string_lossy()]);
    let refused = host.stand_in(
        &clone.dir,
        &[
            "launch", "--kind", "fix", "--name", "three", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(refused.code, 1, "{}", refused.text);
    let reason = refused
        .answer()
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(
        reason.contains("fetch of main from origin failed"),
        "{reason}"
    );
    assert!(
        !Path::new(&host.place_of(&clone, "fix--three")).exists(),
        "a place was made"
    );
    assert!(
        clone.git(&["branch", "--list", "fix/three"]).is_empty(),
        "a branch was made"
    );
}

#[test]
fn a_repository_with_no_origin_is_cut_from_its_own_base() {
    // 9, 56, C.1: a repository with no remote is cut at its own main, and
    // status says it has no origin.
    let host = Host::new("no-origin");
    let repository = repository("no-origin", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let answer = launched.answer();
    let place = the_place(&answer);
    assert_eq!(cut_at(place), repository.head());
    assert_eq!(place.base_from, Some(BaseFrom::NoOrigin));
}

#[test]
fn a_base_the_merge_queue_lands_is_read_where_it_says() {
    // 9, 126, C.1: bound to a merge queue that lands main, here at the
    // clone's own, a place is cut where the merge queue says, origin's
    // main past it never read, though the binding's read fetched it (C.2).
    let host = Host::new("queue-base");
    let source = repository("queue-base", Some(STAND_IN_QUEUE));
    let (clone, origins) = a_clone_lagging_origin(&source, "queue-base-clone");
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &clone.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let answer = launched.answer();
    let place = the_place(&answer);
    assert_eq!(
        cut_at(place),
        clone.head(),
        "the place cut where the queue says"
    );
    assert_ne!(cut_at(place), origins);
    assert_eq!(place.base_from, Some(BaseFrom::MergeQueue));
    assert_eq!(
        clone.git(&["rev-parse", "refs/remotes/origin/main"]),
        origins,
        "origin's main as the binding's read fetched it"
    );
}

#[test]
fn a_watch_over_a_scratch_repository() {
    // The command-line spec's "A watch over a scratch repository": a
    // caller's settle with a bound steps the submitted place's base once,
    // and status gives the host's watch with its bound, when that settle
    // began and when it ended, and says the places where something is
    // pending settle (56, 208, 225).
    let host = Host::new("settle-every");
    let repository = repository("settle-every", Some(HOLDING_QUEUE));
    let kinds = kinds_file(&host, true);
    let log = host.scratch.path("queue.log");
    let log = log.to_string_lossy();
    let env = [(QUEUE_LOG, log.as_ref())];
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &env,
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    commit_in(&place, "code.txt");
    host.end_agent(&place, "code", 1);
    let settled = host.stand_in(&repository.dir, &["settle", &place, "--json"], &env);
    assert_eq!(settled.code, 0, "{}", settled.text);
    let record = || std::fs::read_to_string(host.scratch.path("queue.log")).unwrap_or_default();
    assert!(
        record().lines().any(|line| line == "submit fix/one"),
        "the place was not submitted: {}",
        record()
    );
    let steps = || {
        record()
            .lines()
            .filter(|line| line.starts_with("step "))
            .count()
    };
    host.resident(&env);
    let before = steps();
    let began = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the clock")
        .as_secs();
    let watched = host.stand_in(
        &repository.dir,
        &["settle", "--every", "2m", "--json"],
        &env,
    );
    assert_eq!(watched.code, 0, "{}", watched.text);
    assert_eq!(steps(), before + 1, "the watch's steps: {}", record());
    assert!(record().ends_with("step main\n"), "{}", record());
    let status = host.stand_in(&repository.dir, &["status", "--json"], &env);
    assert_eq!(status.code, 0, "{}", status.text);
    let watch = &status.json()["status"]["host"];
    assert_eq!(
        watch["watch"],
        json!({"bound": {"seconds": 120}}),
        "{watch}"
    );
    let settled_at = watch["watch_settled"].as_u64().expect("watch_settled");
    assert!(
        (began..began + 60).contains(&settled_at),
        "the watch settled at {settled_at}, the settle began at {began}"
    );
    let ended_at = watch["watch_ended"].as_u64().expect("watch_ended");
    assert!(
        ended_at >= settled_at,
        "the watch's settle ended at {ended_at}, before it began at {settled_at}"
    );
    assert!(watch.get("watch_lapsed").is_none(), "{watch}");
    let person = host.stand_in(&repository.dir, &["status"], &env);
    assert!(
        person.text.contains(
            "a caller keeps watch: the places where something is pending settle every 2m, last at "
        ),
        "{}",
        person.text
    );
}

#[test]
fn needs_from_outside_an_agent() {
    // 41, C.4: no FLUME_AGENT, so nothing is asked of the core.
    let host = Host::new("needs");
    let repository = repository("needs", None);
    let ran = host.flume(&repository.dir, &["needs", "which design?", "--json"], &[]);
    assert_eq!(ran.code, 1, "{}", ran.text);
    let reason = ran
        .answer()
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(reason.contains("FLUME_AGENT"), "{reason}");
    let found = host.flume(
        &repository.dir,
        &["needs", "--found", "more work", "--json"],
        &[("FLUME_AGENT", "flume:x:fix--one:code:1")],
    );
    assert_eq!(found.code, 1, "{}", found.text);
    assert!(found.text.contains("FLUME_SECRET"), "{}", found.text);
    assert!(!host.state().join("places").exists(), "something changed");
}

#[test]
fn a_hold_after_the_launch_and_a_stall_bound() {
    // 7, 47, 130: `flume hold` with its words, refused naming why; a
    // launch's stall bound kept on the place.
    let host = Host::new("hold");
    let repository = repository("hold", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--stall-after",
            "30m",
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "change--one");
    assert_eq!(held_of(&host, &place).state.stall_after, 30 * 60);
    let held = answered(
        &host.flume(
            &repository.dir,
            &[
                "hold",
                "change--one",
                "--stage",
                "code",
                "waiting on the schema card",
                "--json",
            ],
            &[],
        ),
        0,
    );
    let hold = &held["acted"]["status"]["holds"][0];
    assert_eq!(held["acted"]["act"], "hold", "{held}");
    assert_eq!(hold["on"], "stage", "{held}");
    assert_eq!(hold["stage"], "code", "{held}");
    assert_eq!(hold["words"], "waiting on the schema card", "{held}");
    let running = answered(
        &host.flume(
            &repository.dir,
            &["hold", "change--one", "--stage", "construct", "x", "--json"],
            &[],
        ),
        1,
    );
    let reason = running["refused"]["refusal"]["reason"].to_string();
    assert!(reason.contains("construct"), "{reason}");
    let submission = answered(
        &host.flume(
            &repository.dir,
            &["hold", "change--one", "--submission", "x", "--json"],
            &[],
        ),
        1,
    );
    let reason = submission["refused"]["refusal"]["reason"].to_string();
    assert!(reason.contains("does not land"), "{reason}");
    let neither = host.flume(&repository.dir, &["hold", "change--one", "x"], &[]);
    assert_eq!(neither.code, 2, "{}", neither.text);
    let person = host.flume(&repository.dir, &["status", "change--one"], &[]);
    assert!(
        person.text.contains("code on hold since ")
            && person.text.contains(": waiting on the schema card"),
        "{}",
        person.text
    );
    let no_time = host.stand_in(
        &repository.dir,
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "two",
            "--kinds",
            &kinds,
            "--stall-after",
            "soon",
            "--json",
        ],
        &[],
    );
    assert_eq!(no_time.code, 1, "{}", no_time.text);
    assert!(no_time.text.contains("soon is no time"), "{}", no_time.text);
}

/// Moves the place's current stage on: its agent commits and its session
/// ends, and a settle starts the next.
fn deliver(host: &Host, dir: &Path, place: &str, stage: &str) {
    commit_in(place, &format!("{stage}.txt"));
    host.end_agent(place, stage, 1);
    let settled = host.stand_in(dir, &["settle", place, "--json"], &[]);
    assert_eq!(settled.code, 0, "{}", settled.text);
}

#[test]
fn a_launch_takes_its_stream_by_its_repositorys_address() {
    // 56, 143; record 0055: a clone whose origin is on a forge, reached
    // through a bare remote beside it by the host's rewriting, takes the
    // stream the host names for its address, and status gives the
    // address beside its folder.
    let host = Host::new("by-address");
    host.configure("[streams.work]\nagents = 2\n\n[repos.\"acme/acme\"]\nstream = \"work\"\n");
    let source = repository("by-address", None);
    let remote = source.bare_remote();
    let clone = Repository::clone_of(&remote, Scratch::new("by-address-clone"));
    let origin = "git@github.com:acme/acme.git";
    clone.git(&["remote", "set-url", "origin", origin]);
    clone.git(&[
        "config",
        &format!("url.{}.insteadOf", remote.to_string_lossy()),
        origin,
    ]);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &clone.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&clone, "fix--one");
    let status = host.flume(&clone.dir, &["status", &place, "--json"], &[]);
    assert_eq!(status.code, 0, "{}", status.text);
    let answer = status.answer();
    let one = the_place(&answer);
    assert_eq!(
        one.stream,
        StreamHeld {
            name: "work".into(),
            from: StreamFrom::HostRepository,
        }
    );
    assert_eq!(one.address.as_deref(), Some("acme/acme"));
    assert_eq!(one.repository, clone.name());
    assert!(
        !status.text.contains(origin),
        "status gives the origin's URL: {}",
        status.text
    );
}

#[test]
fn a_launch_given_an_address_finds_no_clone() {
    // 4, 143; record 0055: `--repository` names a folder, and flume finds
    // no clone by an address.
    let host = Host::new("no-clone-by-address");
    let kinds = kinds_file(&host, false);
    let empty = host.scratch.path("empty");
    std::fs::create_dir_all(&empty).expect("the empty folder");
    let refused = host.stand_in(
        &empty,
        &[
            "launch",
            "--repository",
            "acme/acme",
            "--kind",
            "fix",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--json",
        ],
        &[],
    );
    assert_ne!(refused.code, 0, "{}", refused.text);
    let folder = empty
        .canonicalize()
        .expect("the empty folder")
        .join("acme/acme");
    let reason = refused
        .answer()
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(
        reason.contains(&*folder.to_string_lossy()),
        "the folder it was given: {reason}"
    );
    assert!(!folder.exists(), "the folder made");
    let count = |dir: PathBuf| std::fs::read_dir(dir).map_or(0, |entries| entries.count());
    assert_eq!(count(host.data().join("places")), 0, "a place made");
    assert_eq!(
        count(StateDir::at(host.state()).places()),
        0,
        "a place held"
    );
}

#[test]
fn a_place_named_from_inside_it() {
    // A place named by `<kind>--<name>` from inside its own worktree.
    let host = Host::new("from-inside");
    let repository = repository("from-inside", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = PathBuf::from(host.place_of(&repository, "fix--one"));
    let ran = host.flume(&place, &["status", "fix--one", "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    let answer = ran.answer();
    assert_eq!(
        the_place(&answer).place,
        PlaceId::new(place.to_string_lossy())
    );
    let by_location = host.flume(
        place.parent().expect("the place's folder"),
        &["status", "./fix--one", "--json"],
        &[],
    );
    assert_eq!(by_location.answer(), answer);
}

/// The directory `project` beside `origin`, holding a bare clone of it,
/// `project.git`, beside its worktree `main`, which it returns.
fn bare_layout(origin: &Repository) -> PathBuf {
    let project = origin.scratch.path("project");
    let bare = project.join("project.git");
    git(
        &origin.scratch.root,
        &[
            "clone",
            "--quiet",
            "--bare",
            &origin.name(),
            &bare.to_string_lossy(),
        ],
    );
    git(&bare, &["remote", "remove", "origin"]);
    let main = project.join("main");
    git(
        &bare,
        &[
            "worktree",
            "add",
            "--quiet",
            &main.to_string_lossy(),
            "main",
        ],
    );
    main
}

#[test]
fn every_place_is_made_under_flumes_own_data_directory() {
    // 3, C.1, C.4, record 0027: a clone's place and a bare repository's
    // each go under flume's own data directory by the repository's id,
    // none in the repository's directory, and neither repository's status
    // shows one.
    let host = Host::new("data-directory");
    let kinds = kinds_file(&host, false);
    let launch = |dir: &Path| {
        let launched = host.stand_in(
            dir,
            &[
                "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
            ],
            &[],
        );
        assert_eq!(launched.code, 0, "{}", launched.text);
        the_place(&launched.answer()).place.0.clone()
    };

    let clone = repository("data-directory", None);
    let place = launch(&clone.dir);
    assert_eq!(place, host.place_of(&clone, "fix--one"));
    assert!(Path::new(&place).starts_with(host.data().join("places")));
    assert!(!Path::new(&place).starts_with(&clone.dir), "{place}");
    assert!(Path::new(&place).join(".git").is_file(), "{place}");
    assert_eq!(
        clone.git(&["status", "--porcelain", "--untracked-files=all"]),
        "",
        "the clone shows its place"
    );

    let origin = repository("data-directory-bare", None);
    let main = bare_layout(&origin);
    let project = main.parent().expect("the project");
    let beside = launch(&main);
    assert_eq!(
        beside,
        host.place_in(&project.to_string_lossy(), "fix--one")
    );
    assert!(Path::new(&beside).starts_with(host.data().join("places")));
    assert!(!Path::new(&beside).starts_with(project), "{beside}");
    assert!(!project.join("places").exists(), "{beside}");
    assert_ne!(beside, place, "two repositories' fix--one share a location");
    assert!(Path::new(&beside).join(".git").is_file(), "{beside}");
    assert_eq!(
        git(&main, &["status", "--porcelain", "--untracked-files=all"]),
        "",
        "the main worktree shows the place"
    );
    for dir in [main.as_path(), Path::new(&beside)] {
        let named = host.flume(dir, &["status", "fix--one", "--json"], &[]);
        assert_eq!(named.code, 0, "{}", named.text);
        assert_eq!(the_place(&named.answer()).place, PlaceId::new(&beside));
    }
}

#[test]
fn a_place_made_in_the_repositorys_directory_is_read_and_tidied_where_it_stands() {
    // Record 0027: a place an earlier flume made at
    // `<repository>/places/<kind>--<name>`, inside a clone's main worktree
    // or beside a bare repository's worktrees, is named by its kind and
    // name, refuses a launch of its kind and name, and is dropped and
    // tidied there, never moved.
    let host = Host::new("made-inside");
    let kinds = kinds_file(&host, false);
    let launch = |dir: &Path, env: &[(&str, &str)]| {
        host.stand_in(
            dir,
            &[
                "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
            ],
            env,
        )
    };
    let clone = repository("made-inside", None);
    let origin = repository("made-beside", None);
    let main = bare_layout(&origin);
    let project = main.parent().expect("the project").to_path_buf();
    for (dir, repository) in [(clone.dir.clone(), clone.dir.clone()), (main, project)] {
        let launched = launch(&dir, &[(PLACES_INSIDE, "1")]);
        assert_eq!(launched.code, 0, "{}", launched.text);
        let inside = format!("{}/places/fix--one", repository.to_string_lossy());
        assert_eq!(the_place(&launched.answer()).place, PlaceId::new(&inside));
        assert_ne!(
            inside,
            host.place_in(&repository.to_string_lossy(), "fix--one")
        );

        let status = host.flume(&dir, &["status", "fix--one", "--json"], &[]);
        assert_eq!(status.code, 0, "{}", status.text);
        assert_eq!(the_place(&status.answer()).place, PlaceId::new(&inside));
        let again = launch(&dir, &[]);
        assert_ne!(again.code, 0, "{}", again.text);
        assert!(again.text.contains("fix/one"), "{}", again.text);
        assert!(
            !Path::new(&host.place_in(&repository.to_string_lossy(), "fix--one")).exists(),
            "{}",
            again.text
        );

        host.end_agent(&inside, "code", 1);
        let dropped = host.stand_in(&dir, &["drop", "fix--one", "--json"], &[]);
        assert_eq!(dropped.code, 0, "{}", dropped.text);
        let tidied = host.stand_in(&dir, &["tidy", "--json"], &[]);
        assert_eq!(tidied.code, 0, "{}", tidied.text);
        assert!(!Path::new(&inside).exists(), "{}", tidied.text);
        assert_eq!(git(&dir, &["branch", "--list", "fix/one"]), "");
    }
    assert!(!host.data().join("places").exists(), "a place was moved");
    // Each place's document alone is kept, tidied, for its ends pending for
    // the recipient its launch took (43, 62).
    let every = host.flume(&clone.dir, &["status", "--json"], &[]);
    let places = every.json()["status"]["places"].clone();
    assert!(
        places.as_array().is_some_and(|places| places
            .iter()
            .all(|place| place["phase"]["tidied"]["pending_for"] == json!([CALLER]))),
        "{places}"
    );
}

/// Every file under `dir` and its bytes.
fn files_under(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut found = vec![];
    let mut todo = vec![dir.to_path_buf()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else {
                found.push((path.clone(), std::fs::read(&path).unwrap_or_default()));
            }
        }
    }
    found.sort();
    found
}

#[test]
fn status_over_mcp() {
    // A scripted MCP session initializes `flume mcp`, lists its tools and
    // calls `status`: the tool's result is the JSON `flume status --json`
    // gives over the same state, and nothing is written (57, 60).
    let host = Host::new("mcp");
    let repository = repository("mcp", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    let status = host.flume(&repository.dir, &["status", &place, "--json"], &[]);
    let every = host.flume(&repository.dir, &["status", "--json"], &[]);
    let before = files_under(&host.state());

    let requests = [
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "test", "version": "1"}}}),
        serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        serde_json::json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
        serde_json::json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "status", "arguments": {"place": place}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "status", "arguments": {}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "status", "arguments": {"place": "/nowhere/places/fix--none"}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 6, "method": "settle"}),
    ];
    let input: String = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect();
    let mut child = host
        .command(Path::new(env!("CARGO_BIN_EXE_flume")), &repository.dir, &[])
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("flume mcp runs");
    child
        .stdin
        .take()
        .expect("its input")
        .write_all(input.as_bytes())
        .expect("the session written");
    let output = child.wait_with_output().expect("flume mcp ends");
    assert!(output.status.success());
    let replies: Vec<Value> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("a JSON-RPC reply"))
        .collect();
    assert_eq!(replies.len(), 6, "one reply a request: {replies:?}");
    assert_eq!(replies[0]["id"], 1);
    assert_eq!(replies[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(replies[0]["result"]["serverInfo"]["name"], "flume");
    let tools = replies[1]["result"]["tools"].as_array().expect("the tools");
    assert_eq!(tools[0]["name"], "status");
    let text = |reply: &Value| {
        reply["result"]["content"][0]["text"]
            .as_str()
            .expect("the tool's text")
            .to_string()
    };
    assert_eq!(text(&replies[2]), status.text.trim());
    assert_eq!(replies[2]["result"]["isError"], false);
    assert_eq!(text(&replies[3]), every.text.trim());
    assert_eq!(replies[4]["result"]["isError"], true);
    assert_eq!(replies[5]["error"]["code"], -32601);
    assert_eq!(files_under(&host.state()), before, "status over MCP wrote");
}

/// A host holding the fix named one, launched under the printed command
/// over a scratch repository, its code agent not yet said started; the
/// repository and the place's location.
fn one_unstarted_fix(label: &str) -> (Host, Repository, String) {
    let host = Host::new(label);
    let repository = repository(label, None);
    let kinds = kinds_file(&host, false);
    let program = format!("program={UNSTARTED}");
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--agent", &program,
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    (host, repository, place)
}

#[test]
fn the_list_through_the_command_line_and_the_core() {
    // 60, 200: `flume list --json` is the core's own list over the same
    // state.
    let (host, repository, _) = one_unstarted_fix("list-same");
    let ran = host.flume(&repository.dir, &["list", "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    let core = core_over(
        &host.state(),
        HostConfig::default(),
        None,
        BuiltIn::with_login(&[("HOME", &host.home().to_string_lossy())]),
    )
    .expect("the core over the same state");
    assert_eq!(ran.answer(), core.list(false));
    let ran = host.flume(&repository.dir, &["list", "--all", "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    assert_eq!(ran.answer(), core.list(true));
}

/// The place's held state as the store holds it.
fn held_of(host: &Host, place: &str) -> Versioned {
    StateDir::at(host.state())
        .read(&PlaceId::new(place))
        .expect("the place's held state")
}

/// A wait's or an inbox's answer, its exit and its JSON as one value.
fn answered(ran: &Ran, code: i32) -> Value {
    assert_eq!(ran.code, code, "{}", ran.text);
    ran.json()
}

#[test]
fn a_wait_exits_by_its_outcome() {
    // 43, design 9: 3 still waiting, 0 delivered and read again, 2 ended
    // short, 1 refused.
    let host = Host::new("wait-exits");
    let repository = repository("wait-exits", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--recipient",
            "ops",
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "change--one");
    let wait = |stage: &str, confirm: bool| {
        let mut args = vec![
            "wait",
            "change--one",
            "--stage",
            stage,
            "--recipient",
            "ops",
            "--timeout",
            "1s",
            "--json",
        ];
        if confirm {
            args.push("--confirm");
        }
        host.stand_in(&repository.dir, &args, &[])
    };
    let waiting = answered(&wait("construct", false), 3);
    assert!(waiting["waited"]["end"].is_null(), "{waiting}");
    assert_eq!(waiting["waited"]["recipient"], "ops");

    deliver(&host, &repository.dir, &place, "construct");
    // Confirmed on receipt, so the next wait reads it (43; weir 103).
    let delivered = answered(&wait("construct", true), 0);
    assert_eq!(delivered["waited"]["delivered"], true, "{delivered}");
    assert_eq!(delivered["waited"]["end"]["state"], "delivered");
    assert_eq!(
        delivered["waited"]["end"]["deliveries"][0]["by"], "wait",
        "{delivered}"
    );
    let read = answered(&wait("construct", false), 0);
    assert_eq!(read["waited"]["delivered"], false, "{read}");
    let person = host.stand_in(
        &repository.dir,
        &[
            "wait",
            "change--one",
            "--stage",
            "construct",
            "--recipient",
            "ops",
        ],
        &[],
    );
    assert_eq!(person.code, 0, "{}", person.text);
    assert!(
        person.text.contains("construct delivered for ops"),
        "{}",
        person.text
    );

    let held = held_of(&host, &place).state;
    let start = held.stages[1].start.clone().expect("code's start");
    let needs = host.stand_in(
        &repository.dir,
        &["needs", "which design?", "--json"],
        &[
            ("FLUME_AGENT", &start.identity.to_string()),
            ("FLUME_SECRET", &start.secret.0),
        ],
    );
    assert_eq!(needs.code, 0, "{}", needs.text);
    let short = answered(&wait("code", false), 2);
    assert_eq!(short["waited"]["end"]["state"], "ended_short", "{short}");
    assert_eq!(short["waited"]["end"]["end"]["words"], "which design?");

    let none = answered(
        &host.stand_in(
            &repository.dir,
            &[
                "wait",
                "change--none",
                "--submission",
                "--recipient",
                "ops",
                "--json",
            ],
            &[],
        ),
        1,
    );
    assert!(none["refused"].is_object(), "{none}");
    let unlanded = answered(
        &host.stand_in(
            &repository.dir,
            &[
                "wait",
                "change--one",
                "--submission",
                "--recipient",
                "ops",
                "--json",
            ],
            &[],
        ),
        1,
    );
    assert!(
        unlanded["refused"]["refusal"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("does not land")),
        "{unlanded}"
    );
    let unnamed = host.stand_in(&repository.dir, &["wait", "change--one"], &[]);
    assert_eq!(unnamed.code, 2, "{}", unnamed.text);
}

#[test]
fn the_recipient_a_wait_and_an_inbox_default_to() {
    // 7, 132, design 7: $FLUME_RECIPIENT, then a role's run's address read
    // from the place $FLUME_PLACE names, and a refusal where none names
    // one, naming each way to; the terminal's session and the standing
    // agent's pane are the terminal profile's to name, tested beside it
    // under profiles/.
    let host = Host::new("recipients");
    let repository = repository("recipients", None);
    let kinds = kinds_file(&host, false);
    carried_by_acme(&host, &["madswan/planner"]);
    for (name, recipient) in [("one", "ops"), ("two", "madswan/planner")] {
        let launched = host.stand_in(
            &repository.dir,
            &[
                "launch",
                "--kind",
                "change",
                "--name",
                name,
                "--kinds",
                &kinds,
                "--recipient",
                recipient,
                "--json",
            ],
            &[],
        );
        assert_eq!(launched.code, 0, "{}", launched.text);
        let place = host.place_of(&repository, &format!("change--{name}"));
        deliver(&host, &repository.dir, &place, "construct");
    }
    let two = host.place_of(&repository, "change--two");
    let store = StateDir::at(host.state());
    let read = held_of(&host, &two);
    let mut role = read.state.clone();
    role.role = Some("madswan/planner".into());
    assert!(matches!(
        store.write(&PlaceId::new(&two), Some(read.version), Some(&role)),
        Written::Written { .. }
    ));

    let unnamed = answered(
        &host.stand_in(&repository.dir, &["inbox", "--json"], &[]),
        1,
    );
    let reason = unnamed["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_default();
    for way in [
        "--recipient",
        "$FLUME_RECIPIENT",
        "a terminal's session",
        "a role's run",
        "a standing agent's pane",
    ] {
        assert!(
            reason.contains(way),
            "the refusal names no {way}: {unnamed}"
        );
    }

    let by_variable = answered(
        &host.stand_in(
            &repository.dir,
            &["inbox", "--confirm", "--json"],
            &[("FLUME_RECIPIENT", "ops"), ("FLUME_PLACE", &two)],
        ),
        0,
    );
    assert_eq!(by_variable["inbox"]["recipient"], "ops", "{by_variable}");
    let ends = by_variable["inbox"]["ends"].as_array().expect("ends");
    assert_eq!(ends.len(), 1, "{by_variable}");
    assert_eq!(ends[0]["kind"], "change");
    assert_eq!(ends[0]["name"], "one");
    assert_eq!(ends[0]["end"]["state"], "delivered");
    assert_eq!(ends[0]["end"]["deliveries"][0]["by"], "inbox");

    let by_role = answered(
        &host.stand_in(
            &repository.dir,
            &["inbox", "--confirm", "--json"],
            &[("FLUME_PLACE", &two)],
        ),
        0,
    );
    assert_eq!(
        by_role["inbox"]["recipient"], "madswan/planner",
        "{by_role}"
    );
    assert_eq!(
        by_role["inbox"]["ends"].as_array().map(Vec::len),
        Some(1),
        "{by_role}"
    );
    let again = answered(
        &host.stand_in(
            &repository.dir,
            &["inbox", "--recipient", "madswan/planner", "--json"],
            &[],
        ),
        0,
    );
    assert_eq!(
        again["inbox"]["ends"].as_array().map(Vec::len),
        Some(0),
        "{again}"
    );
    let person = host.stand_in(&repository.dir, &["inbox", "--recipient", "ops"], &[]);
    assert_eq!(person.code, 0, "{}", person.text);
    assert_eq!(person.text.trim(), "nothing pending for ops");

    let waited = answered(
        &host.stand_in(
            &repository.dir,
            &["wait", "change--one", "--stage", "construct", "--json"],
            &[("FLUME_RECIPIENT", "ops")],
        ),
        0,
    );
    assert_eq!(waited["waited"]["recipient"], "ops", "{waited}");
    assert_eq!(waited["waited"]["delivered"], false, "{waited}");
}

#[test]
fn a_wake_of_a_roles_address() {
    // 132, 193, C.7: the host's source acme declares the planner's run in
    // its roles file, of its own kind `planner`, locked; `flume wake`
    // launches it once in its address's stream, named `<stream>-<role>`,
    // answers the run that stands after, refuses an address no source
    // declares and a name that is no address, each saying why, and the
    // run's inbox collects for its address.
    let host = Host::new("wake");
    let repository = Repository::new("wake");
    repository.write(
        ".config/flume.toml",
        "[profiles]\nplace = \"git-worktree\"\n",
    );
    repository.commit("the binding");
    let acme = kinds_dir_of(&host, "acme");
    write_package(&acme, "planner", 1, false, "Read what is pending for you");
    let roles = host.scratch.path("state/acme/roles.toml");
    std::fs::create_dir_all(roles.parent().unwrap()).expect("the roles file's directory");
    std::fs::write(
        &roles,
        format!(
            "[roles.\"east/planner\"]\nrepository = \"{}\"\nkind = \"planner\"\npriority = 5\n",
            repository.name()
        ),
    )
    .expect("the roles file");
    std::fs::create_dir_all(sources_of(&host)).expect("the sources directory");
    std::fs::write(
        sources_of(&host).join("acme.toml"),
        format!(
            "kinds_dir = \"{}\"\nroles = \"{}\"\n",
            acme.display(),
            roles.display()
        ),
    )
    .expect("the source's file");
    host.configure("[streams.east]\n");
    answered(
        &host.flume(
            &repository.dir,
            &["kinds", "lock", "--host", "acme/planner@1", "--json"],
            &[],
        ),
        0,
    );

    let woke = answered(
        &host.stand_in(&repository.dir, &["wake", "east/planner", "--json"], &[]),
        0,
    );
    assert_eq!(woke["woken"]["woke"], "launched", "{woke}");
    assert_eq!(woke["woken"]["address"], "east/planner", "{woke}");
    let run = &woke["woken"]["place"];
    assert_eq!(run["role"], "east/planner", "{woke}");
    assert_eq!(run["kind"], "planner");
    assert_eq!(run["identity"], "acme/planner@1", "{woke}");
    assert_eq!(run["stream"]["name"], "east", "{woke}");
    assert_eq!(run["name"], "east-planner", "{woke}");
    assert_eq!(run["base"], "main", "the repository's default branch");
    assert_eq!(run["priority"], 5);
    assert_eq!(run["recipients"], serde_json::json!([]));
    let place = host.place_of(&repository, "planner--east-planner");
    assert_eq!(run["place"], place.as_str());

    let again = answered(
        &host.stand_in(&repository.dir, &["wake", "east/planner", "--json"], &[]),
        0,
    );
    assert_ne!(again["woken"]["woke"], "launched", "{again}");
    assert_eq!(again["woken"]["place"]["place"], place.as_str(), "{again}");
    let person = host.stand_in(&repository.dir, &["wake", "east/planner"], &[]);
    assert_eq!(person.code, 0, "{}", person.text);
    assert!(
        person.text.starts_with("east/planner's run"),
        "{}",
        person.text
    );

    let unnamed = answered(
        &host.stand_in(&repository.dir, &["wake", "east/ops", "--json"], &[]),
        1,
    );
    assert!(
        unnamed["refused"]["refusal"]["reason"]
            .as_str()
            .is_some_and(|reason| reason
                == "east/ops wakes nothing: no source of the host declares east/ops"),
        "{unnamed}"
    );
    let no_address = answered(
        &host.stand_in(&repository.dir, &["wake", "planner", "--json"], &[]),
        1,
    );
    assert!(
        no_address["refused"]["refusal"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("<stream>/<role>")),
        "{no_address}"
    );

    let inbox = answered(
        &host.stand_in(
            &repository.dir,
            &["inbox", "--json"],
            &[("FLUME_PLACE", &place)],
        ),
        0,
    );
    assert_eq!(inbox["inbox"]["recipient"], "east/planner", "{inbox}");
}

#[test]
fn needs_to_a_role_and_to_another_streams() {
    // 41, 132, design 4: the command-line spec's "A stop-short addressed
    // by its role" and "A stop-short addressed to another stream". The
    // host's source acme carries east/ops and west/ops, each with the
    // words of what it answers, so a stage's prompt lists them with no run
    // to wake; `flume needs --to ops` from a place in east names east/ops,
    // `--to west/ops` names west/ops, each in its answer, and a name the
    // prompt did not list is refused naming those it did, the stage going
    // on.
    let host = Host::new("needs-to");
    let repository = repository("needs-to", None);
    let kinds = kinds_file(&host, false);
    host.configure("[streams.east]\n");
    carried_answering(
        &host,
        &[
            ("east/ops", "east's live systems"),
            ("west/ops", "west's live systems"),
        ],
    );
    for name in ["one", "two"] {
        let launched = host.stand_in(
            &repository.dir,
            &[
                "launch", "--kind", "change", "--name", name, "--kinds", &kinds, "--stream",
                "east", "--json",
            ],
            &[],
        );
        assert_eq!(launched.code, 0, "{}", launched.text);
    }
    let needs = |place: &str, to: &str, json: bool| {
        let start = held_of(&host, place).state.stages[0]
            .start
            .clone()
            .expect("construct's start");
        let mut words = vec!["needs", "--to", to, "which queue's settings are live?"];
        if json {
            words.push("--json");
        }
        host.stand_in(
            &repository.dir,
            &words,
            &[
                ("FLUME_AGENT", &start.identity.to_string()),
                ("FLUME_SECRET", &start.secret.0),
            ],
        )
    };
    let addressee = |answer: &Value| {
        answer["acted"]["status"]["ends"]
            .as_array()
            .and_then(|ends| ends.last())
            .map(|end| end["end"]["addressee"].clone())
            .unwrap_or_default()
    };

    let one = host.place_of(&repository, "change--one");
    let unlisted = answered(&needs(&one, "design", true), 1);
    let reason = unlisted["refused"]["refusal"]["reason"].to_string();
    assert!(
        reason.contains("`ops`") && reason.contains("`west/ops`"),
        "{reason}"
    );
    let by_role = answered(&needs(&one, "ops", true), 0);
    assert_eq!(addressee(&by_role), "east/ops", "{by_role}");

    let two = host.place_of(&repository, "change--two");
    let person = needs(&two, "west/ops", false);
    assert_eq!(person.code, 0, "{}", person.text);
    assert!(person.text.contains("asking west/ops"), "{}", person.text);
    let status = host.flume(&repository.dir, &["status", "change--two", "--json"], &[]);
    let ends = &status.json()["status"]["places"][0]["ends"];
    assert_eq!(
        ends.as_array()
            .and_then(|ends| ends.last())
            .map(|end| end["end"]["addressee"].clone()),
        Some(json!("west/ops")),
        "{ends}"
    );
}

/// The host's source acme carrying each of `addresses` with the words of
/// what it answers, in its roles file: addresses a stage's prompt lists
/// that wake no run on this host (132, 191; design 11.4).
fn carried_answering(host: &Host, addresses: &[(&str, &str)]) {
    let roles = host.scratch.path("state/acme/roles.toml");
    std::fs::create_dir_all(roles.parent().unwrap()).expect("the roles file's directory");
    let entries: String = addresses
        .iter()
        .map(|(address, answers)| {
            format!("[roles.\"{address}\"]\ncarried = true\nanswers = \"{answers}\"\n")
        })
        .collect();
    std::fs::write(&roles, entries).expect("the roles file");
    std::fs::create_dir_all(sources_of(host)).expect("the sources directory");
    std::fs::write(
        sources_of(host).join("acme.toml"),
        format!(
            "kinds_dir = \"{}\"\nroles = \"{}\"\n",
            kinds_dir_of(host, "acme").display(),
            roles.display()
        ),
    )
    .expect("the source's file");
}

/// The directory the host's source `source` holds its packages in, a
/// scratch directory of the host's own.
fn kinds_dir_of(host: &Host, source: &str) -> PathBuf {
    host.scratch.path(&format!("packages/{source}"))
}

/// The host's sources directory, beside its configuration (191, C.7).
fn sources_of(host: &Host) -> PathBuf {
    host.scratch.path("config/flume/sources")
}

/// The host's source acme declares each of `addresses` carried to another
/// host in its roles file, so a launch naming one is taken, its ends left
/// pending, in a stream no role stands behind (132, 193, C.7).
fn carried_by_acme(host: &Host, addresses: &[&str]) {
    let roles = host.scratch.path("state/acme/roles.toml");
    std::fs::create_dir_all(roles.parent().unwrap()).expect("the roles file's directory");
    let entries: String = addresses
        .iter()
        .map(|address| format!("[roles.\"{address}\"]\ncarried = true\n"))
        .collect();
    std::fs::write(&roles, entries).expect("the roles file");
    std::fs::create_dir_all(sources_of(host)).expect("the sources directory");
    std::fs::write(
        sources_of(host).join("acme.toml"),
        format!(
            "kinds_dir = \"{}\"\nroles = \"{}\"\n",
            kinds_dir_of(host, "acme").display(),
            roles.display()
        ),
    )
    .expect("the source's file");
}

/// The host's lock of kinds, beside its configuration (155, C.7).
fn host_lock(host: &Host) -> PathBuf {
    host.scratch.path("config/flume/flume.lock")
}

/// Declares the host's source `source` by its file, `kinds_dir` its
/// directory (191, C.7).
fn declare_source(host: &Host, source: &str, kinds_dir: &Path) {
    std::fs::create_dir_all(sources_of(host)).expect("the sources directory");
    std::fs::write(
        sources_of(host).join(format!("{source}.toml")),
        format!("kinds_dir = \"{}\"\n", kinds_dir.display()),
    )
    .expect("the source's file");
}

/// Writes the package `<name>@<version>` in `kinds_dir`: a one-stage
/// kind whose agent is the stand-in and whose deliverable commits, its
/// prompt saying `says`; it lands where `lands` (157, 159).
fn write_package(kinds_dir: &Path, name: &str, version: u32, lands: bool, says: &str) {
    let dir = kinds_dir.join(format!("{name}@{version}"));
    std::fs::create_dir_all(dir.join("prompts")).expect("the package's directory");
    std::fs::write(
        dir.join("kind.toml"),
        format!(
            "[kinds.{name}]\nversion = {version}\nbranch = \"{name}\"\nstarts_unready = false\nlands = {lands}\n\n\
             [[kinds.{name}.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{ program = \"{STAND_IN}\" }}]\n"
        ),
    )
    .expect("the package's declaration");
    std::fs::write(
        dir.join("prompts/code.md"),
        format!("{says} {{{{name}}}} in {{{{place}}}}. To stop short: {{{{needs}}}}\n"),
    )
    .expect("the package's prompt");
}

/// The hash of the package `<name>@<version>` of `source` in `kinds_dir`,
/// as the kinds profile reads it (156).
fn hash_of(source: &str, kinds_dir: &Path, name: &str, version: u32) -> String {
    match Weir.read_package(source, &kinds_dir.to_string_lossy(), name, version) {
        PackageRead::Read { package } => package.hash,
        PackageRead::CouldNotRead { reason } => panic!("{name}@{version} not read: {reason}"),
    }
}

/// Each entry of the lock at `path`, `(identity, hash)`, as the kinds
/// profile reads it; none where there is no lock.
fn lock_at(path: &Path) -> Vec<(String, String)> {
    match Weir.read_lock_file(&path.to_string_lossy()) {
        LockRead::None => vec![],
        LockRead::Read { entries } => entries
            .into_iter()
            .map(|entry| (entry.kind.spelled, entry.hash))
            .collect(),
        LockRead::CouldNotRead { reason } => panic!("{} not read: {reason}", path.display()),
    }
}

/// The lock's bytes, none where there is no lock.
fn bytes_of(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

/// A host whose source acme holds `review@1` and `lint@1`, with the
/// repository it runs in.
fn acme_host(label: &str) -> (Host, PathBuf) {
    let host = Host::new(label);
    let acme = kinds_dir_of(&host, "acme");
    write_package(&acme, "review", 1, false, "Review");
    write_package(&acme, "lint", 1, false, "Lint");
    declare_source(&host, "acme", &acme);
    (host, acme)
}

#[test]
fn kinds_lock_two_writers_at_once_and_a_removal() {
    // 74, 155: two `flume kinds lock --host` run at once as two
    // processes, each recording one kind, and both are recorded; then
    // `--remove` twice leaves acme/review@1 alone, the second changing
    // nothing.
    let (host, acme) = acme_host("lock-writers");
    let repository = repository("lock-writers", None);
    let flume = Path::new(env!("CARGO_BIN_EXE_flume"));
    for round in 0..4 {
        let _ = std::fs::remove_file(host_lock(&host));
        let writers: Vec<_> = ["acme/review@1", "acme/lint@1"]
            .into_iter()
            .map(|kind| {
                host.command(flume, &repository.dir, &[])
                    .args(["kinds", "lock", "--host", kind, "--json"])
                    .stdout(std::process::Stdio::null())
                    .spawn()
                    .expect("a writer runs")
            })
            .collect();
        for mut writer in writers {
            assert!(
                writer.wait().expect("a writer's end").success(),
                "round {round}"
            );
        }
        let mut read = lock_at(&host_lock(&host));
        read.sort();
        assert_eq!(
            read,
            vec![
                ("acme/lint@1".to_string(), hash_of("acme", &acme, "lint", 1)),
                (
                    "acme/review@1".to_string(),
                    hash_of("acme", &acme, "review", 1)
                ),
            ],
            "both recorded, round {round}"
        );
    }
    let removed = answered(
        &host.flume(
            &repository.dir,
            &[
                "kinds",
                "lock",
                "--host",
                "--remove",
                "acme/lint@1",
                "--json",
            ],
            &[],
        ),
        0,
    );
    assert_eq!(
        removed["locked"]["removed"],
        serde_json::json!(["acme/lint@1"])
    );
    let after = bytes_of(&host_lock(&host));
    assert_eq!(
        lock_at(&host_lock(&host)),
        vec![(
            "acme/review@1".to_string(),
            hash_of("acme", &acme, "review", 1)
        )]
    );
    std::fs::remove_dir_all(acme.join("lint@1")).expect("lint@1's directory removed");
    let again = host.flume(
        &repository.dir,
        &["kinds", "lock", "--host", "--remove", "acme/lint@1"],
        &[],
    );
    assert_eq!(again.code, 0, "{}", again.text);
    assert_eq!(
        again.text.trim(),
        format!("{} is as it was", host_lock(&host).display())
    );
    assert_eq!(
        bytes_of(&host_lock(&host)),
        after,
        "the second removal changes nothing"
    );
}

#[test]
fn flume_writes_neither_configuration_file() {
    // C.7, core.16, core.45: a launch, its settles, status and its tidy
    // leave both layers' files as they were, their content and their
    // write time.
    let host = Host::new("neither-file");
    host.configure("[streams.work]\nagents = 4\n");
    host.configure_local("[streams.work]\npriority = 30\n");
    let files = [
        host.scratch.path("config/flume/config.toml"),
        host.scratch.path("config/flume/config.local.toml"),
    ];
    let read = || {
        files
            .iter()
            .map(|file| {
                (
                    std::fs::read(file).expect("the file"),
                    std::fs::metadata(file)
                        .and_then(|meta| meta.modified())
                        .expect("its write time"),
                )
            })
            .collect::<Vec<_>>()
    };
    let before = read();
    let repository = repository("neither-file", None);
    let dir = &repository.dir;
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--stream", "work",
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    let settled = host.stand_in(dir, &["settle", &place, "--json"], &[]);
    assert_eq!(settled.code, 0, "{}", settled.text);
    let status = host.flume(dir, &["status", "--json"], &[]);
    assert_eq!(status.code, 0, "{}", status.text);
    host.end_agent(&place, "code", 1);
    let dropped = host.stand_in(dir, &["drop", "fix--one", "--json"], &[]);
    assert_eq!(dropped.code, 0, "{}", dropped.text);
    let tidied = host.stand_in(dir, &["tidy", "--json"], &[]);
    assert_eq!(tidied.code, 0, "{}", tidied.text);
    assert!(!Path::new(&place).exists(), "{}", tidied.text);
    assert_eq!(read(), before, "a layer's file was written");
}

#[test]
fn open_under_the_printed_command() {
    // 187: the printed command holds no session a person can be shown,
    // refused in its JSON, needing no repository.
    let host = Host::new("open-printed");
    host.configure("[streams.work]\nagents = 1\n");
    let ran = host.flume(&host.home(), &["open", "work", "--json"], &[]);
    assert_ne!(ran.code, 0, "{}", ran.text);
    let reason = ran
        .answer()
        .refusal()
        .map(str::to_string)
        .unwrap_or_else(|| panic!("open answered {}", ran.text));
    assert!(
        reason.contains("the printed command holds no session a person can be shown"),
        "{reason}"
    );
}

#[test]
fn a_readers_place_collected_by_its_inbox() {
    // 7, 43, 52: the command-line spec's "A reader's place collected" and
    // "The helps": status gives both recipients before the drop, the
    // inbox naming the place as `boomdrive`, which no pane holds, gives
    // its drop among its ends, each handed to it by the inbox and pending
    // for it, a second with `--confirm` gives them again, each confirmed,
    // and a third gives nothing; a launch naming `--caller` is refused,
    // nothing made.
    let host = Host::new("reader-inbox");
    let repository = repository("reader-inbox", None);
    let dir = &repository.dir;
    let kinds = kinds_file(&host, false);
    let launch = |name: &str, more: &[&str]| {
        let mut args = vec!["launch", "--kind", "fix", "--name", name, "--kinds", &kinds];
        args.extend_from_slice(more);
        args.push("--json");
        host.stand_in(dir, &args, &[])
    };
    let refused = launch("two", &["--caller", "boomdrive"]);
    assert_eq!(refused.code, 2, "{}", refused.text);
    assert!(refused.text.contains("--caller"), "{}", refused.text);
    assert!(!Path::new(&host.place_of(&repository, "fix--two")).exists());
    let launched = launch("one", &["--recipient", "boomdrive", "--recipient", "ops"]);
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    let status = host.flume(dir, &["status", "fix--one", "--json"], &[]);
    assert_eq!(status.code, 0, "{}", status.text);
    let answer = status.answer();
    let one = the_place(&answer);
    assert_eq!(
        one.recipients,
        vec!["boomdrive".to_string(), "ops".to_string()],
        "{}",
        status.text
    );
    host.end_agent(&place, "code", 1);
    let dropped = host.stand_in(dir, &["drop", "fix--one", "--json"], &[]);
    assert_eq!(dropped.code, 0, "{}", dropped.text);
    let inbox = |more: &[&str]| {
        let mut args = vec!["inbox", "fix--one", "--recipient", "boomdrive", "--json"];
        args.extend_from_slice(more);
        answered(&host.stand_in(dir, &args, &[]), 0)
    };
    let collected = inbox(&[]);
    assert_eq!(collected["inbox"]["recipient"], "boomdrive", "{collected}");
    let ends = collected["inbox"]["ends"].as_array().expect("ends");
    assert!(
        ends.iter()
            .all(|end| end["kind"] == "fix" && end["name"] == "one"),
        "{collected}"
    );
    let over = ends
        .iter()
        .map(|end| &end["end"])
        .find(|end| end["key"] == "over/dropped")
        .unwrap_or_else(|| panic!("no drop's end: {collected}"));
    assert_eq!(over["of"], "over", "{collected}");
    assert_eq!(over["over"], "dropped", "{collected}");
    for end in ends {
        let deliveries = end["end"]["deliveries"].as_array().expect("deliveries");
        assert!(
            deliveries
                .iter()
                .any(|delivery| delivery["recipient"] == "boomdrive" && delivery["by"] == "inbox"),
            "{collected}"
        );
        assert_eq!(
            end["end"]["pending_for"],
            json!(["boomdrive", "ops"]),
            "{collected}"
        );
        assert_eq!(end["confirmed"], false, "{collected}");
    }
    let confirmed = inbox(&["--confirm"]);
    let again = confirmed["inbox"]["ends"].as_array().expect("ends");
    assert_eq!(again.len(), ends.len(), "{confirmed}");
    for end in again {
        assert_eq!(end["confirmed"], true, "{confirmed}");
        assert_eq!(end["end"]["pending_for"], json!(["ops"]), "{confirmed}");
    }
    let third = inbox(&[]);
    assert_eq!(third["inbox"]["ends"], json!([]), "{third}");
}

#[test]
fn a_key_confirmed_through_the_binary() {
    // 43; weir 102: the command-line spec's "A key confirmed through the
    // binary": the drop's end's delivery key the inbox gives, confirmed
    // twice as `boomdrive`, is confirmed with when, then confirmed before
    // with the first time, and the next inbox gives nothing.
    let host = Host::new("confirm-key");
    let repository = repository("confirm-key", None);
    let dir = &repository.dir;
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        dir,
        &[
            "launch",
            "--kind",
            "fix",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--recipient",
            "boomdrive",
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "fix--one");
    host.end_agent(&place, "code", 1);
    let dropped = host.stand_in(dir, &["drop", "fix--one", "--json"], &[]);
    assert_eq!(dropped.code, 0, "{}", dropped.text);
    let inbox = || {
        answered(
            &host.stand_in(
                dir,
                &["inbox", "fix--one", "--recipient", "boomdrive", "--json"],
                &[],
            ),
            0,
        )
    };
    let collected = inbox();
    let keys: Vec<String> = collected["inbox"]["ends"]
        .as_array()
        .expect("ends")
        .iter()
        .map(|end| {
            end["end"]["delivery"]
                .as_str()
                .unwrap_or_else(|| panic!("an end with no delivery key: {collected}"))
                .to_string()
        })
        .collect();
    assert!(
        keys.iter()
            .any(|key| key.starts_with("fix--one/") && key.ends_with("/over/dropped")),
        "{collected}"
    );
    let confirm = || {
        let mut args = vec!["confirm"];
        args.extend(keys.iter().map(String::as_str));
        args.extend(["--recipient", "boomdrive", "--json"]);
        answered(&host.stand_in(dir, &args, &[]), 0)
    };
    let first = confirm();
    assert_eq!(first["confirmed"]["recipient"], "boomdrive", "{first}");
    let confirmed = first["confirmed"]["confirmed"].as_array().expect("keys");
    assert_eq!(confirmed.len(), keys.len(), "{first}");
    for (key, answer) in keys.iter().zip(confirmed) {
        assert_eq!(answer["key"], key.as_str(), "{first}");
        assert_eq!(answer["before"], false, "{first}");
        assert!(answer["at"].as_u64().is_some(), "{first}");
    }
    let second = confirm();
    for (first, again) in confirmed
        .iter()
        .zip(second["confirmed"]["confirmed"].as_array().expect("keys"))
    {
        assert_eq!(again["before"], true, "{second}");
        assert_eq!(again["at"], first["at"], "{second}");
    }
    let next = inbox();
    assert_eq!(next["inbox"]["ends"], json!([]), "{next}");
}

#[test]
fn an_answer_naming_no_decision() {
    // 176, section 9: a statechart kind launched over a scratch
    // repository, `flume answer` naming a region whose state carries no
    // decision exits non-zero naming it, and status of the place gives no
    // answer: the command-line spec's "An answer over a scratch
    // repository".
    let host = Host::new("answer");
    let repository = repository("answer", None);
    let dir = host.scratch.path("statechart");
    std::fs::create_dir_all(dir.join("prompts")).expect("the kinds' folder");
    std::fs::write(
        dir.join("prompts/work.md"),
        "Do the work of {{name}} in {{place}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let declaration = dir.join("kinds.toml");
    std::fs::write(
        &declaration,
        format!(
            r#"[kinds.one]
title = "One stage"

[kinds.one.regions.main]
initial = "work"

[kinds.one.regions.main.states.work]
machine = "stage"
params = {{ agents = [{{ program = "{STAND_IN}" }}], deliverables = ["commits"], prompt = "prompts/work.md" }}
transitions = [{{ when = {{ final = "passed" }}, to = "done" }}]

[kinds.one.regions.main.states.done]
final = true
"#
        ),
    )
    .expect("the declaration");
    let kinds = declaration.to_string_lossy().into_owned();
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "one", "--name", "a", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = host.place_of(&repository, "one--a");
    let ran = host.flume(&repository.dir, &["answer", &place, "main", "retry"], &[]);
    assert_ne!(ran.code, 0, "{}", ran.text);
    assert!(
        ran.text.contains("main") && ran.text.contains("no decision"),
        "the refusal names not the region: {}",
        ran.text
    );
    let status = host.flume(&repository.dir, &["status", &place, "--json"], &[]);
    assert_eq!(status.code, 0, "{}", status.text);
    let answer = status.answer();
    let chart = the_place(&answer)
        .statechart
        .as_ref()
        .expect("status gives the place's statechart");
    assert!(chart.answers.is_empty(), "{:?}", chart.answers);
}

/// The replies of a scripted MCP session of `flume mcp` run in `dir` with
/// `env`: initialize, then each of `calls`, one reply a request (section
/// 9; design 12).
fn mcp_session(host: &Host, dir: &Path, env: &[(&str, &str)], calls: &[Value]) -> Vec<Value> {
    let mut command = host.command(Path::new(env!("CARGO_BIN_EXE_flume")), dir, env);
    command.arg("mcp");
    mcp_replies(command, calls)
}

/// The same session with this test binary as flume, with the stand-ins,
/// so a launch over MCP starts its agent under the stand-in terminal.
fn stand_in_mcp_session(
    host: &Host,
    dir: &Path,
    env: &[(&str, &str)],
    calls: &[Value],
) -> Vec<Value> {
    let me = std::env::current_exe().expect("the test binary");
    let mut command = host.command(&me, dir, env);
    command
        .env(AS_FLUME, serde_json::to_string(&["mcp"]).expect("words"))
        .args([
            "--exact",
            "stand_in_flume",
            "--nocapture",
            "--test-threads=1",
            "-q",
        ]);
    mcp_replies(command, calls)
}

/// The replies `command`, a `flume mcp`, gives to initialize and each of
/// `calls`, the test harness's own lines aside.
fn mcp_replies(mut command: std::process::Command, calls: &[Value]) -> Vec<Value> {
    let mut requests = vec![
        serde_json::json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "test", "version": "1"}}}),
        serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    ];
    for (id, call) in calls.iter().enumerate() {
        let mut call = call.clone();
        call["jsonrpc"] = "2.0".into();
        call["id"] = (id + 1).into();
        requests.push(call);
    }
    let input: String = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect();
    let mut child = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("flume mcp runs");
    child
        .stdin
        .take()
        .expect("its input")
        .write_all(input.as_bytes())
        .expect("the session written");
    let output = child.wait_with_output().expect("flume mcp ends");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let replies: Vec<Value> = stdout
        .split_once("running 1 test\n")
        .map_or(&*stdout, |(_, after)| after)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("a JSON-RPC reply"))
        .collect();
    assert_eq!(replies.len(), calls.len() + 1, "{replies:?}");
    replies[1..].to_vec()
}

/// A tool's call of the MCP session.
fn tool_call(name: &str, arguments: Value) -> Value {
    serde_json::json!({"method": "tools/call", "params": {"name": name, "arguments": arguments}})
}

/// A tool's result as its JSON, and whether it is marked an error.
fn tool_result(reply: &Value) -> (Value, bool) {
    let text = reply["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_else(|| panic!("no tool's text: {reply}"));
    (
        serde_json::from_str(text).unwrap_or_else(|error| panic!("{error}: {text}")),
        reply["result"]["isError"].as_bool().expect("isError"),
    )
}

/// A host whose source acme declares the planner's run, `east/planner`,
/// of its own kind `planner`, locked, in the stream `east` the host
/// declares, and the repository it runs in (132, 193, C.7).
fn planner_host(label: &str) -> (Host, Repository) {
    let host = Host::new(label);
    let repository = Repository::new(label);
    repository.write(
        ".config/flume.toml",
        "[profiles]\nplace = \"git-worktree\"\n",
    );
    repository.commit("the binding");
    let acme = kinds_dir_of(&host, "acme");
    write_package(&acme, "planner", 1, false, "Read what is pending for you");
    let roles = host.scratch.path("state/acme/roles.toml");
    std::fs::create_dir_all(roles.parent().unwrap()).expect("the roles file's directory");
    std::fs::write(
        &roles,
        format!(
            "[roles.\"east/planner\"]\nrepository = \"{}\"\nkind = \"planner\"\npriority = 5\n",
            repository.name()
        ),
    )
    .expect("the roles file");
    std::fs::create_dir_all(sources_of(&host)).expect("the sources directory");
    std::fs::write(
        sources_of(&host).join("acme.toml"),
        format!(
            "kinds_dir = \"{}\"\nroles = \"{}\"\n",
            acme.display(),
            roles.display()
        ),
    )
    .expect("the source's file");
    host.configure("[streams.east]\n");
    answered(
        &host.flume(
            &repository.dir,
            &["kinds", "lock", "--host", "acme/planner@1", "--json"],
            &[],
        ),
        0,
    );
    (host, repository)
}

#[test]
fn the_mcp_tools_listed() {
    // The agent program's plugin spec's "The tools listed" (section 9;
    // design 12): status, message, launch, wake, hibernate and resume,
    // each with its input schema, status alone read-only (212, 213).
    let host = Host::new("mcp-tools");
    let repository = repository("mcp-tools", None);
    let replies = mcp_session(
        &host,
        &repository.dir,
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

/// A scratch repository with a fix `fix--one` launched by the stand-in
/// flume, whose kind hibernates nothing (212).
fn a_fix_launched(label: &str) -> (Host, Repository) {
    let host = Host::new(label);
    let repository = repository(label, Some(STAND_IN_QUEUE));
    let kinds = kinds_file(&host, true);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    (host, repository)
}

#[test]
fn mcp_hibernate_refused_as_the_command_line_refuses_it() {
    // The agent program's plugin spec's "A hibernate refused as the
    // command line refuses it" (60, 212, C.2): a fix's kind hibernates
    // nothing, so the tool's result is the refusal `flume hibernate`
    // gives, naming the place, marked an error.
    let (host, repository) = a_fix_launched("mcp-hibernate");
    let replies = mcp_session(
        &host,
        &repository.dir,
        &[],
        &[tool_call(
            "hibernate",
            serde_json::json!({"place": "fix--one", "force": true}),
        )],
    );
    let (over_mcp, error) = tool_result(&replies[0]);
    assert!(error, "{over_mcp}");
    let by_hand = answered(
        &host.flume(
            &repository.dir,
            &["hibernate", "fix--one", "--force", "--json"],
            &[],
        ),
        1,
    );
    assert_eq!(over_mcp, by_hand);
    let reason = serde_json::from_value::<Answer>(over_mcp.clone())
        .expect("an answer")
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    let place = host.place_of(&repository, "fix--one");
    assert!(
        reason.contains(&place) && reason.contains("does not hibernate"),
        "{reason}"
    );
}

#[test]
fn mcp_resume_refused_as_the_command_line_refuses_it() {
    // The agent program's plugin spec's "A resume of nothing hibernating"
    // (60, 213, C.2): the tool's result is the refusal `flume resume`
    // gives, marked an error, and nothing is started.
    let (host, repository) = a_fix_launched("mcp-resume");
    let place = host.place_of(&repository, "fix--one");
    let before = answered(
        &host.flume(&repository.dir, &["status", "fix--one", "--json"], &[]),
        0,
    );
    let replies = mcp_session(
        &host,
        &repository.dir,
        &[],
        &[tool_call(
            "resume",
            serde_json::json!({"place": "fix--one"}),
        )],
    );
    let (over_mcp, error) = tool_result(&replies[0]);
    assert!(error, "{over_mcp}");
    let by_hand = answered(
        &host.flume(&repository.dir, &["resume", "fix--one", "--json"], &[]),
        1,
    );
    assert_eq!(over_mcp, by_hand);
    let reason = serde_json::from_value::<Answer>(over_mcp.clone())
        .expect("an answer")
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(reason.contains(&place), "{reason}");
    let after = answered(
        &host.flume(&repository.dir, &["status", "fix--one", "--json"], &[]),
        0,
    );
    let starts = |status: &Value| status["status"]["places"][0]["stages"][0]["starts"].clone();
    assert!(starts(&before).is_u64(), "{before}");
    assert_eq!(starts(&after), starts(&before), "starts after the resume");
}

#[test]
fn message_over_mcp() {
    // The agent program's plugin spec's "message over MCP" (53, 214,
    // section 9): the tool's result is the JSON `flume message east/planner
    // <words> --json` gives, its key `message/east/planner/<token>`.
    let (host, repository) = planner_host("mcp-message");
    let replies = mcp_session(
        &host,
        &repository.dir,
        &[],
        &[tool_call(
            "message",
            serde_json::json!({"target": "east/planner", "words": "--look at the nightly job"}),
        )],
    );
    let (over_mcp, error) = tool_result(&replies[0]);
    assert!(!error, "{over_mcp}");
    let by_hand = answered(
        &host.flume(
            &repository.dir,
            &["message", "east/planner", "and the weekly one", "--json"],
            &[],
        ),
        0,
    );
    for answer in [&over_mcp, &by_hand] {
        let messaged = &answer["messaged"];
        assert_eq!(messaged["address"], "east/planner", "{answer}");
        assert!(
            messaged["key"]
                .as_str()
                .is_some_and(|key| key.starts_with("message/east/planner/")),
            "{answer}"
        );
    }
    assert_eq!(
        over_mcp
            .as_object()
            .map(|answer| answer.keys().collect::<Vec<_>>()),
        by_hand
            .as_object()
            .map(|answer| answer.keys().collect::<Vec<_>>()),
    );
}

#[test]
fn launch_over_mcp_refused_as_the_command_line_refuses_it() {
    // The agent program's plugin spec's "launch over MCP, refused as the
    // command line refuses it" (4, section 9): a kind no declaration
    // holds, the command line's refusal marked an error, and no place.
    let host = Host::new("mcp-launch-refused");
    let repository = repository("mcp-launch-refused", None);
    let replies = mcp_session(
        &host,
        &repository.dir,
        &[("FLUME_RECIPIENT", CALLER)],
        &[tool_call(
            "launch",
            serde_json::json!({"kind": "nowhere", "name": "one", "brief": "mend it"}),
        )],
    );
    let (over_mcp, error) = tool_result(&replies[0]);
    assert!(error, "{over_mcp}");
    let by_hand = answered(
        &host.flume(
            &repository.dir,
            &[
                "launch", "--kind", "nowhere", "--name", "one", "--brief", "mend it", "--json",
            ],
            &[],
        ),
        1,
    );
    assert_eq!(over_mcp, by_hand);
    assert!(
        !Path::new(&host.place_of(&repository, "nowhere--one")).exists(),
        "a place was made"
    );
}

#[test]
fn a_role_runs_launch_over_mcp_reports_to_its_address() {
    // The role-waking spec's "Through MCP" (7, 132, 214, section 9): the
    // run's session calls flume's MCP `launch` for a place naming no
    // recipient, of a kind of acme's that lands nothing, the server in
    // its environment, and the place's recipients are east/planner from
    // role-run. The server is the stand-in flume, as the wake was, so
    // the launch's agent starts under the stand-in terminal.
    let (host, repository) = planner_host("mcp-role-launch");
    let woke = answered(
        &host.stand_in(&repository.dir, &["wake", "east/planner", "--json"], &[]),
        0,
    );
    let run = woke["woken"]["place"]["place"]
        .as_str()
        .expect("the run's place")
        .to_string();
    let replies = stand_in_mcp_session(
        &host,
        &repository.dir,
        &[("FLUME_PLACE", &run)],
        &[tool_call(
            "launch",
            serde_json::json!({
                "kind": "acme/planner",
                "name": "tidy",
                "brief": "tidy the notes",
            }),
        )],
    );
    let (launched, error) = tool_result(&replies[0]);
    assert!(!error, "{launched}");
    let place = &launched["launched"]["place"];
    assert_eq!(
        place["recipients"],
        serde_json::json!(["east/planner"]),
        "{launched}"
    );
    assert_eq!(
        place["recipients_from"],
        serde_json::json!({"from": "role-run"}),
        "{launched}"
    );
}
