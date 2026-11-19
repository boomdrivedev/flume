//! flume's launcher under the common profile (35, 41, 161, C.4; core.30;
//! agent-launcher spec): a fix launched by flume's own binary with
//! `--agent program=amp` under the printed command, over a scratch state
//! directory, its account giving `AMP_HOME`, and the launcher run as its
//! printed line gives it, or with the environment file that line names,
//! beside a scratch `amp` that writes its process id and its arguments
//! and a link of the kind's name to flume's own binary first on `PATH`,
//! as a terminal's start folder holds one.

use crate::common::{Repository, Scratch};
use flume::profiles::agent::launcher::{self, Word};
use flume::profiles::store::state_dir::StateDir;
use flume_seams::{PlaceId, Store};
use serde_json::Value;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// flume's own binary.
const FLUME: &str = env!("CARGO_BIN_EXE_flume");

/// The value the account gives `AMP_HOME`, under the scratch's home.
const HOME_OF_AMP: &str = "amp-1";

/// A fix launched for amp under the printed command on a host of its own.
struct Launched {
    scratch: Scratch,
    repository: Repository,
    /// The line the printed command printed.
    printed: String,
    /// The place's location.
    place: String,
    /// The start's identity.
    identity: String,
}

impl Launched {
    /// The fix `one` launched with `--agent program=amp` under the
    /// printed command, its stage's agent amp under the host's account.
    fn new(label: &str) -> Launched {
        let scratch = Scratch::new(label);
        for dir in [
            "state",
            "config/flume",
            "home",
            "start",
            "bin",
            "other",
            "runs",
        ] {
            std::fs::create_dir_all(scratch.path(dir)).expect("the host's directory");
        }
        // The start folder a terminal leads PATH with: the kind's name
        // linked to the launcher, flume's own binary.
        symlink(FLUME, scratch.path("start/amp")).expect("the kind's link");
        for dir in ["bin", "other"] {
            amp(&scratch, dir);
        }
        std::fs::write(
            scratch.path("config/flume/config.toml"),
            format!(
                "[host]\npool = \"work\"\n\n[pools.work]\nstrategy = \"fill-first\"\naccounts = [{{ name = \"work-1\", program = \"amp\", env = {{ AMP_HOME = \"{}\" }} }}]\n",
                scratch.path(&format!("home/{HOME_OF_AMP}")).display()
            ),
        )
        .expect("the host's configuration");
        let repository = Repository::new(&format!("{label}-repository"));
        repository.write(
            ".config/flume.toml",
            "[profiles]\nplace = \"git-worktree\"\n",
        );
        repository.commit("the binding");
        let kinds = kinds(&scratch);
        let launched = Launched {
            scratch,
            repository,
            printed: String::new(),
            place: String::new(),
            identity: String::new(),
        };
        let ran = launched.flume(&[
            "launch",
            "--kind",
            "fix",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--agent",
            "program=amp",
            "--json",
        ]);
        assert!(ran.status.success(), "the launch: {}", text(&ran));
        let answer: Value = serde_json::from_slice(&ran.stdout).expect("the launch's JSON");
        let place = &answer["launched"]["place"];
        let agent = &place["agent"];
        Launched {
            printed: agent["printed"]
                .as_str()
                .unwrap_or_else(|| panic!("no command printed: {answer}"))
                .to_string(),
            place: place["place"]
                .as_str()
                .unwrap_or_else(|| panic!("no location: {answer}"))
                .to_string(),
            identity: agent["identity"]
                .as_str()
                .unwrap_or_else(|| panic!("no identity: {answer}"))
                .to_string(),
            ..launched
        }
    }

    /// `PATH` as the pane's shell would give it: the start's folder, then
    /// the scratch's `bin`, then the system's.
    fn path(&self) -> String {
        format!(
            "{}:{}:/usr/bin:/bin",
            self.scratch.path("start").display(),
            self.scratch.path("bin").display()
        )
    }

    /// A process run in `dir` over the host's scratch and nothing of the
    /// caller's flume or terminal session.
    fn command(&self, program: &str, dir: &Path) -> Command {
        let mut command = Command::new(program);
        command.current_dir(dir);
        for (name, _) in std::env::vars() {
            if name.starts_with("FLUME_") || name.starts_with("HERDR_") || name.starts_with("REX_")
            {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.scratch.path("state"))
            .env("XDG_CONFIG_HOME", self.scratch.path("config"))
            .env("XDG_DATA_HOME", self.scratch.path("home/.local/share"))
            .env("HOME", self.scratch.path("home"))
            .env("FLUME_RECIPIENT", "caller")
            .env("PATH", self.path());
        command
    }

    /// flume's own binary, run in the repository with `words`.
    fn flume(&self, words: &[&str]) -> Output {
        self.command(FLUME, &self.repository.dir)
            .args(words)
            .output()
            .expect("flume runs")
    }

    /// The printed line run as a person runs it, `more` after it.
    fn run_printed(&self, more: &str) -> Output {
        self.command("/bin/sh", &self.repository.dir)
            .arg("-c")
            .arg(format!("{} {more}", self.printed))
            .output()
            .expect("the printed line runs")
    }

    /// The launcher run in the place under the environment file the
    /// printed line names, `env` over it, given `program`.
    fn run_launcher(&self, env: &[&str], program: &str) -> Output {
        self.command("/bin/sh", Path::new(&self.place))
            .args([
                "-c",
                "set -a && . \"$0\" && set +a && exec \"$@\"",
                &self.env_file().to_string_lossy(),
                "env",
            ])
            .args(env)
            .args([FLUME, "ack", "launched", "--", program])
            .output()
            .expect("the launcher runs")
    }

    /// The environment file the printed line names.
    fn env_file(&self) -> PathBuf {
        let dir = Path::new(&self.folder()).join("printed");
        std::fs::read_dir(&dir)
            .expect("the printed command's folder")
            .flatten()
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|ext| ext == "env"))
            .expect("the printed command's environment")
    }

    /// The place's own folder in the store.
    fn folder(&self) -> String {
        StateDir::at(self.scratch.path("state/flume")).folder(&PlaceId::new(&self.place))
    }

    /// The launcher's word of the start, none where none stands.
    fn word(&self) -> Option<Word> {
        launcher::read(&self.folder(), &self.identity).expect("the word read")
    }

    /// Each run of the scratch amp in `dir`: its process id, then its
    /// arguments.
    fn runs(&self, dir: &str) -> Vec<String> {
        std::fs::read_to_string(self.scratch.path(&format!("runs/{dir}")))
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    }

    /// The place's status as JSON, after a settle of it.
    fn status(&self) -> String {
        let settled = self.flume(&["settle", &self.place, "--json"]);
        assert!(settled.status.success(), "the settle: {}", text(&settled));
        let status = self.flume(&["status", &self.place, "--json"]);
        assert!(status.status.success(), "the status: {}", text(&status));
        String::from_utf8_lossy(&status.stdout).into_owned()
    }
}

/// A scratch amp in the scratch's `dir`, which writes one line to
/// `runs/<dir>` each time it runs: its process id and its arguments.
fn amp(scratch: &Scratch, dir: &str) {
    let at = scratch.path(&format!("{dir}/amp"));
    std::fs::write(
        &at,
        format!(
            "#!/bin/sh\necho \"$$ $*\" >> '{}'\n",
            scratch.path(&format!("runs/{dir}")).display()
        ),
    )
    .expect("the scratch amp");
    std::fs::set_permissions(&at, std::fs::Permissions::from_mode(0o755))
        .expect("the scratch amp's mode");
}

/// A declaration of the fix, one code stage delivering commits.
fn kinds(scratch: &Scratch) -> String {
    let dir = scratch.path("kinds");
    std::fs::create_dir_all(dir.join("prompts")).expect("the kinds' folder");
    std::fs::write(
        dir.join("prompts/work.md"),
        "Do the work of the {{kind}} {{name}} in {{place}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let file = dir.join("kinds.toml");
    std::fs::write(
        &file,
        "[kinds.fix]\nversion = 1\nbranch = \"fix\"\nstarts_unready = true\nlands = false\n\n[[kinds.fix.stages]]\nname = \"code\"\nprompt = \"prompts/work.md\"\ndeliverables = [\"commits\"]\nagents = [{}]\n",
    )
    .expect("the declaration");
    file.to_string_lossy().into_owned()
}

/// What a process printed, both streams.
fn text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// The agent-launcher spec's scenario "amp becomes the start's" (35, 161,
/// C.4).
#[test]
fn amp_becomes_the_starts() {
    let launched = Launched::new("launcher-becomes");
    let flume = Path::new(FLUME).canonicalize().expect("flume's path");
    assert!(
        launched
            .printed
            .contains(&format!("{} ack launched -- amp", flume.display())),
        "the printed line runs no launcher: {}",
        launched.printed
    );
    let ran = launched.run_printed("x y");
    assert!(ran.status.success(), "the launcher: {}", text(&ran));
    let runs = launched.runs("bin");
    let word = launched.word().expect("no word written");
    assert_eq!(runs, vec![format!("{} x y", word.pid)], "amp's runs");
    assert_eq!(word.unsettled, None, "the word names something differing");
    assert_eq!(
        Path::new(&word.program),
        launched.scratch.path("bin/amp").canonicalize().unwrap(),
        "the program's path"
    );
    assert!(launched.runs("other").is_empty(), "another amp ran");
}

/// The agent-launcher spec's scenario "A second launcher under one start"
/// (35, 161).
#[test]
fn a_second_launcher_under_one_start() {
    let launched = Launched::new("launcher-second");
    let first = launched.run_printed("");
    assert!(
        first.status.success(),
        "the first launcher: {}",
        text(&first)
    );
    let word = launched.word().expect("no word written");
    let second = launched.run_printed("");
    assert!(!second.status.success(), "the second launcher exited 0");
    let said = String::from_utf8_lossy(&second.stderr);
    assert!(
        said.starts_with("refused: ") && said.contains(&format!("the process {}", word.pid)),
        "the second launcher names not the first's process: {said}"
    );
    assert_eq!(launched.word(), Some(word), "the word is the first's");
    assert_eq!(launched.runs("bin").len(), 1, "amp ran again");
}

/// The agent-launcher spec's scenario "A variable with another value"
/// (161; core.30).
#[test]
fn a_variable_with_another_value() {
    let launched = Launched::new("launcher-variable");
    let elsewhere = launched.scratch.path("elsewhere").display().to_string();
    let ran = launched.run_launcher(&[&format!("AMP_HOME={elsewhere}")], "amp");
    assert!(!ran.status.success(), "the launcher exited 0");
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(
        said.contains("with another AMP_HOME"),
        "the launcher says not why: {said}"
    );
    assert!(launched.runs("bin").is_empty(), "amp ran");
    let word = launched.word().expect("no word written");
    let unsettled = word.unsettled.expect("the word names nothing differing");
    assert_eq!(unsettled.different, vec!["AMP_HOME".to_string()]);
    let written = std::fs::read_to_string(launcher::path(&launched.folder(), &launched.identity))
        .expect("the word's file");
    assert!(
        !written.contains(&elsewhere) && !written.contains(HOME_OF_AMP),
        "the word holds a value: {written}"
    );
    let status = launched.status();
    let answer: Value = serde_json::from_str(status.trim()).expect("the status's JSON");
    let place = &answer["status"]["places"][0];
    assert_eq!(
        place["unsettled"][0]["different"],
        serde_json::json!(["AMP_HOME"]),
        "status reads the start begun otherwise: {status}"
    );
    assert_eq!(place["agent"]["started"], "no", "{status}");
    assert!(
        !status.contains(&elsewhere) && !status.contains(HOME_OF_AMP),
        "status holds a value: {status}"
    );
}

/// The agent-launcher spec's scenario "Another program" (161, C.4).
#[test]
fn another_program() {
    let launched = Launched::new("launcher-program");
    let other = launched.scratch.path("other/amp").display().to_string();
    let ran = launched.run_launcher(&[], &other);
    assert!(!ran.status.success(), "the launcher exited 0");
    assert!(launched.runs("bin").is_empty(), "the record's amp ran");
    assert!(launched.runs("other").is_empty(), "the other amp ran");
    let word = launched.word().expect("no word written");
    let unsettled = word.unsettled.expect("the word names nothing differing");
    assert!(unsettled.program, "the word says not the program differed");
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(said.contains("under another program"), "{said}");
}

/// The agent-launcher spec's scenario "No start under its identity and
/// secret" (41, 161).
#[test]
fn no_start_under_its_identity_and_secret() {
    let launched = Launched::new("launcher-secret");
    let ran = launched.run_launcher(&["FLUME_SECRET=another"], "amp");
    assert!(!ran.status.success(), "the launcher exited 0");
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(
        said.contains("the secret is not") && said.contains("start's"),
        "the launcher says not the secret is not the start's: {said}"
    );
    assert_eq!(launched.word(), None, "a word was written");
    assert!(launched.runs("bin").is_empty(), "amp ran");
}
