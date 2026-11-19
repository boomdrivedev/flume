//! The common agent profile (170, 171, C.4; record 0026; design 5): the
//! profile of every program an entry of [`programs`] names that has no
//! profile of its own, built over that entry and the absolute path of the
//! binary that runs. Its command runs the program the settings name, by
//! its name or its path, in the place, through flume's launcher, that
//! binary by its absolute path with `ack launched --`, with its entry's
//! way of passing a model, an effort and a mode where the settings give
//! one, the account's environment with flume's four words over it, as
//! Claude Code's command sets them, and its entry's kind; it passes no
//! prompt argument, answers no folder-trust question and writes no file.
//! Its way to stop short names the same binary by its absolute path (92;
//! core.60). Its start is read from the launcher's word in the place's
//! own folder ([`launcher`]): started once the word names nothing
//! differing, not before a word or where it names something, unknown
//! where the folder cannot be read; how it began otherwise, the word's;
//! and the process the launcher became, the word's, so the core takes the
//! start only once the terminal reads the program in that process (35,
//! 161, 171, C.4). A model, an effort or a mode its entry names no way to
//! pass is refused naming the program (170, 172, 197; core.39). It builds
//! none of the agent seam's parts, the seam's default, so the core reads
//! its received, asking, own work, ended and last words from its terminal
//! as 171 says, asking it nothing of them, and status names all thirteen
//! degraded with the agent (171, 174; core.2). Every part it reads
//! nothing of answers as the seam's default where it is asked, and
//! nothing of the terminal is read here (93, 171, 195; core.2, core.39):
//! received, asking, own work and ended unknown, last words none,
//! send-back, on-quiet and guard not offered.
//!
//! [`launcher`]: super::launcher
//!
//! [`programs`]: super::programs

use super::claude::{AGENT, PLACE, SECRET, STAGE};
use super::launcher::{self, Word};
use super::programs::Program;
use crate::shell::quote;
use flume_seams::{
    Agent, AgentCommand, Guarded, Identity, ModeEntry, PerProgram, PlaceRef, Reading, Received,
    Secret, Settings, Unsettled,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The common profile over one entry (170, C.4).
#[derive(Clone, Debug)]
pub struct Common {
    entry: &'static Program,
    /// The binary its way to stop short names, by its absolute path
    /// (core.60).
    binary: PathBuf,
}

impl Common {
    /// The common profile over `entry`, its way to stop short naming the
    /// binary that runs (core.60).
    pub fn of_host(entry: &'static Program) -> Common {
        let binary = std::env::current_exe()
            .and_then(fs::canonicalize)
            .unwrap_or_else(|_| PathBuf::from("flume"));
        Common::new(entry, &binary)
    }

    /// The common profile over `entry`, its way to stop short naming
    /// `binary`.
    pub fn new(entry: &'static Program, binary: &Path) -> Common {
        Common {
            entry,
            binary: binary.to_path_buf(),
        }
    }

    /// The entry it runs.
    pub fn entry(&self) -> &'static Program {
        self.entry
    }

    /// The launcher's word of the start `agent` in the place's own
    /// `folder`: none where none stands, why not where it cannot be read
    /// (35, C.4).
    fn word(&self, folder: &str, agent: &Identity) -> Result<Option<Word>, String> {
        launcher::read(folder, &agent.to_string())
    }

    /// Words refusing `what`, a model, an effort or a mode, which its
    /// entry names no way to give its program (170).
    fn no_way(&self, what: &str) -> String {
        format!(
            "the entry for `{}` names no way to give it {what}",
            self.entry.name
        )
    }
}

impl Agent for Common {
    /// The program the settings name, its entry's name where they name
    /// none, in the place, through flume's launcher, the binary by its
    /// absolute path with `ack launched --`: the model, the effort and the
    /// mode its entry names a way for, where the settings give them,
    /// flume's four words over the account's environment, its entry's
    /// kind, and no prompt argument, folder-trust answer or file (88, 170,
    /// C.4; core.60).
    fn command(
        &self,
        _folder: &str,
        settings: &Settings,
        mode: Option<&str>,
        _own: &BTreeMap<String, serde_json::Value>,
        _resume: Option<&str>,
        env: &BTreeMap<String, String>,
        place: &PlaceRef,
        identity: &Identity,
        secret: &Secret,
        _prompt: &str,
    ) -> AgentCommand {
        let mut args = vec![];
        if let (Some(flag), Some(model)) = (
            self.entry.model,
            settings.model.as_ref().and_then(PerProgram::one),
        ) {
            args.extend([flag.to_string(), model.to_string()]);
        }
        if let (Some(effort), Some(word)) = (
            self.entry.effort,
            settings.effort.as_ref().and_then(PerProgram::one),
        ) {
            args.extend([effort.flag.to_string(), word.to_string()]);
        }
        if let (Some(_), Some(mode)) = (self.entry.modes, mode) {
            args.push(mode.to_string());
        }
        let mut env = env.clone();
        env.insert(AGENT.into(), identity.to_string());
        env.insert(SECRET.into(), secret.0.clone());
        env.insert(PLACE.into(), place.location.clone());
        env.insert(STAGE.into(), identity.stage.clone());
        AgentCommand {
            program: settings
                .program
                .clone()
                .unwrap_or_else(|| self.entry.name.to_string()),
            args,
            env,
            prompt_args: vec![],
            folder_trust: None,
            kind: self.kind().map(str::to_string),
            files: BTreeMap::new(),
            launcher: vec![
                self.binary.to_string_lossy().into_owned(),
                "ack".into(),
                "launched".into(),
                "--".into(),
            ],
        }
    }

    /// Yes once the launcher's word names nothing differing, no before a
    /// word or where it names something, unknown where the place's own
    /// folder cannot be read (35, 135, C.4).
    fn started(&self, folder: &str, agent: &Identity) -> Reading {
        match self.word(folder, agent) {
            Ok(Some(Word {
                unsettled: None, ..
            })) => Reading::Yes,
            Ok(_) => Reading::No,
            Err(_) => Reading::Unknown,
        }
    }

    /// It starts its program through flume's launcher (35, C.4).
    fn launches(&self) -> bool {
        true
    }

    /// The process the launcher's word names, none before a word or where
    /// the folder cannot be read (35, C.4).
    fn launched(&self, folder: &str, agent: &Identity) -> Option<u32> {
        self.word(folder, agent).ok().flatten().map(|word| word.pid)
    }

    /// How the launcher's word says the start began otherwise, none where
    /// it names nothing, before a word, or where the folder cannot be read
    /// (161, C.4).
    fn started_without(&self, folder: &str, agent: &Identity) -> Option<Unsettled> {
        self.word(folder, agent).ok().flatten()?.unsettled
    }

    fn received(&self, _folder: &str, _agent: &Identity, _text: &str) -> Received {
        Received::Unknown
    }

    fn asking(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn own_work(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn ended(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn last_words(&self, _folder: &str, _agent: &Identity) -> Option<String> {
        None
    }

    /// The value its entry maps each of flume's four to; every entry
    /// refused naming the program where its entry names no way to give a
    /// mode, and a program's own value refused, since no entry lists one
    /// (169, 170).
    fn mode(&self, entry: &ModeEntry) -> Result<String, String> {
        let Some(modes) = self.entry.modes else {
            return Err(self.no_way("a mode"));
        };
        match entry {
            ModeEntry::Flume(mode) => modes
                .iter()
                .find(|(each, _)| each == mode)
                .map(|(_, value)| value.to_string())
                .ok_or_else(|| self.no_way(&format!("the mode `{mode}`"))),
            ModeEntry::Own(value) => Err(format!(
                "the entry for `{}` names no value of its own, `{value}` among them",
                self.entry.name
            )),
        }
    }

    /// Every name, passed as written, where its entry names a way to give
    /// a model; every name refused naming the program otherwise (170,
    /// 172).
    fn model(&self, _name: &str) -> Result<(), String> {
        match self.entry.model {
            Some(_) => Ok(()),
            None => Err(self.no_way("a model")),
        }
    }

    /// Its entry's kind, by which the terminal starts it (170, C.4).
    fn kind(&self) -> Option<&str> {
        Some(self.entry.kind)
    }

    /// Each word its entry's release takes, any other refused naming it,
    /// the program and the words; every word refused naming the program
    /// where its entry names no way to give an effort (170, 197).
    fn effort(&self, word: &str) -> Result<(), String> {
        let Some(effort) = self.entry.effort else {
            return Err(self.no_way("an effort"));
        };
        if effort.words.contains(&word) {
            Ok(())
        } else {
            Err(format!(
                "`{word}` is no effort `{}` takes, which are {}",
                self.entry.name,
                effort.words.join(", ")
            ))
        }
    }

    /// `flume needs`, naming the binary that started the agent by its
    /// absolute path (92; core.60).
    fn stop_short(&self, _agent: &Identity) -> String {
        let flume = quote(&self.binary.to_string_lossy());
        format!(
            "run `{flume} needs \"<words>\"` when you cannot go on without an answer, saying what you need, or `{flume} needs --found \"<words>\"` when you found work a stage before this one must do again, saying what you found; then stop."
        )
    }

    /// `--to <name>` given to `flume needs`, naming the binary as
    /// [`Common::stop_short`] does (41, 92, 132).
    fn stop_short_to(&self) -> String {
        let flume = quote(&self.binary.to_string_lossy());
        format!(
            "name it with `--to`, as `{flume} needs --to <name> \"<words>\"` or `{flume} needs --found --to <name> \"<words>\"`, the name written as listed here"
        )
    }

    fn guard(&self, _folder: &str, _agent: &Identity) -> Guarded {
        Guarded::NotOffered
    }
}

#[cfg(test)]
mod tests {
    use super::super::programs::{entry, PROGRAMS};
    use super::*;
    use flume_seams::{Carry, Live, Mode, Offered, Part, Spent, Windows};

    const BINARY: &str = "/opt/flume/bin/flume";

    fn amp() -> Common {
        Common::new(entry("amp").expect("amp's entry"), Path::new(BINARY))
    }

    fn identity() -> Identity {
        Identity {
            repository: "/r".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
            resume: 0,
        }
    }

    fn place() -> PlaceRef {
        PlaceRef {
            repository: "/r".into(),
            location: "/r/places/fix--one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    /// The agent-programs spec's scenario "amp's command" (88, 92, C.4).
    #[test]
    fn amps_command() {
        let settings = Settings {
            program: Some("amp".into()),
            ..Settings::default()
        };
        let env = BTreeMap::from([("AMP_HOME".to_string(), "/scratch/amp-1".to_string())]);
        let command = amp().command(
            "/state/fix--one",
            &settings,
            None,
            &BTreeMap::new(),
            None,
            &env,
            &place(),
            &identity(),
            &Secret("s3".into()),
            "mend it",
        );
        assert_eq!(command.program, "amp");
        assert!(command.args.is_empty(), "{:?}", command.args);
        assert!(command.prompt_args.is_empty(), "{:?}", command.prompt_args);
        assert_eq!(command.kind.as_deref(), Some("amp"));
        assert_eq!(command.folder_trust, None);
        assert!(command.files.is_empty());
        assert_eq!(
            command.launcher,
            vec![BINARY, "ack", "launched", "--"],
            "the launcher's words"
        );
        assert_eq!(
            command.env,
            BTreeMap::from([
                ("AMP_HOME".to_string(), "/scratch/amp-1".to_string()),
                ("FLUME_AGENT".to_string(), identity().to_string()),
                ("FLUME_SECRET".to_string(), "s3".to_string()),
                ("FLUME_PLACE".to_string(), "/r/places/fix--one".to_string()),
                ("FLUME_STAGE".to_string(), "code".to_string()),
            ])
        );
        let way = amp().stop_short(&identity());
        assert!(
            way.contains(&format!("`{BINARY} needs \"<words>\"`"))
                && way.contains(&format!("`{BINARY} needs --found \"<words>\"`")),
            "{way}"
        );
        assert!(amp()
            .stop_short_to()
            .contains(&format!("`{BINARY} needs --to")));
        // A path a shell would split is quoted.
        let spaced = Common::new(entry("amp").expect("amp"), Path::new("/opt/my flume/flume"));
        assert!(
            spaced
                .stop_short(&identity())
                .contains("`'/opt/my flume/flume' needs"),
            "{}",
            spaced.stop_short(&identity())
        );
        // A program named by its path runs by that path.
        let by_path = amp().command(
            "/state/fix--one",
            &Settings {
                program: Some("/opt/amp/bin/amp".into()),
                ..Settings::default()
            },
            None,
            &BTreeMap::new(),
            None,
            &BTreeMap::new(),
            &place(),
            &identity(),
            &Secret("s3".into()),
            "mend it",
        );
        assert_eq!(by_path.program, "/opt/amp/bin/amp");
        assert_eq!(by_path.launcher, vec![BINARY, "ack", "launched", "--"]);
        assert!(amp().launches(), "amp starts through the launcher");
    }

    /// The agent-programs spec's scenario "Nothing read of amp" (93, 171,
    /// 174, 195): every reading but its start's, which the launcher's word
    /// gives.
    #[test]
    fn nothing_is_read_of_amp() {
        let amp = amp();
        let (folder, agent) = ("/state/fix--one", identity());
        assert_eq!(amp.received(folder, &agent, "mend it"), Received::Unknown);
        assert_eq!(amp.asking(folder, &agent), Reading::Unknown);
        assert_eq!(amp.own_work(folder, &agent), Reading::Unknown);
        assert_eq!(amp.ended(folder, &agent), Reading::Unknown);
        assert_eq!(amp.last_words(folder, &agent), None);
        assert_eq!(amp.spent(folder, &agent), Spent::Unknown);
        assert_eq!(amp.windows(folder, &agent), Windows::Unknown);
        assert_eq!(amp.reports_windows(), Offered::NotOffered);
        assert_eq!(
            amp.carry(folder, &agent, &BTreeMap::new()),
            Carry::NotOffered
        );
        assert!(matches!(
            amp.live(folder, &agent, &[]),
            Live::Unknown { .. }
        ));
        assert_eq!(amp.conversation(folder, &agent), None);
        assert_eq!(amp.mode_read_back(), Offered::NotOffered);
        assert_eq!(amp.send_back(), Offered::NotOffered);
        assert_eq!(amp.on_quiet(), Offered::NotOffered);
        assert_eq!(amp.guard(folder, &agent), Guarded::NotOffered);
        assert_eq!(
            amp.parts().degraded(),
            [
                Part::Received,
                Part::Asking,
                Part::OwnWork,
                Part::Ended,
                Part::LastWords,
                Part::Spent,
                Part::Windows,
                Part::SendBack,
                Part::Guard,
                Part::OnQuiet,
                Part::Mode,
                Part::Carry,
                Part::OwnSettings,
            ],
            "all thirteen degraded, in 171's order"
        );
        assert_eq!(amp.reserved(), vec![]);
        assert_eq!(amp.withheld(), Vec::<String>::new());
        assert_eq!(amp.effort_variable(), None);
        assert_eq!(amp.notes(folder, &agent), Vec::<String>::new());
        assert!(amp
            .own(&BTreeMap::from([("x".to_string(), serde_json::json!(1))]))
            .is_err());
        assert_eq!(amp.own(&BTreeMap::new()), Ok(()));
    }

    /// The agent-programs spec's scenario "amp's start read from the
    /// launcher's word" (35, 161, C.4).
    #[test]
    fn amps_start_is_read_from_the_launchers_word() {
        let amp = amp();
        let dir = std::env::temp_dir().join(format!(
            "flume-common-word-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_nanos())
                .unwrap_or_default()
        ));
        fs::create_dir_all(&dir).expect("a scratch folder");
        let folder = dir.display().to_string();
        let (first, second) = (
            identity(),
            Identity {
                start: 2,
                ..identity()
            },
        );
        assert_eq!(amp.started(&folder, &first), Reading::No, "no word");
        assert_eq!(amp.launched(&folder, &first), None);
        assert_eq!(amp.started_without(&folder, &first), None);
        launcher::write(
            &folder,
            &first.to_string(),
            &Word {
                pid: 41,
                program: "/scratch/bin/amp".into(),
                unsettled: None,
            },
        )
        .expect("the first start's word");
        assert_eq!(amp.started(&folder, &first), Reading::Yes);
        assert_eq!(amp.launched(&folder, &first), Some(41));
        assert_eq!(
            amp.started_without(&folder, &first),
            None,
            "begun as recorded"
        );
        let home = Unsettled {
            different: vec!["AMP_HOME".into()],
            ..Unsettled::default()
        };
        launcher::write(
            &folder,
            &second.to_string(),
            &Word {
                pid: 41,
                program: "/scratch/bin/amp".into(),
                unsettled: Some(home.clone()),
            },
        )
        .expect("the second start's word");
        assert_eq!(amp.started(&folder, &second), Reading::No);
        assert_eq!(amp.started_without(&folder, &second), Some(home.clone()));
        assert_eq!(home.words(), "with another AMP_HOME");
        assert_eq!(amp.launched(&folder, &second), Some(41));
        let gone = dir.join("gone").display().to_string();
        assert_eq!(amp.started(&gone, &first), Reading::Unknown);
        assert_eq!(amp.launched(&gone, &first), None);
        assert_eq!(amp.started_without(&gone, &first), None);
        let _ = fs::remove_dir_all(&dir);
    }

    /// The agent-programs spec's scenario "An entry of name and kind"
    /// (170, 172, 197).
    #[test]
    fn an_entry_of_name_and_kind_refuses_a_model_an_effort_and_a_mode() {
        for program in PROGRAMS.iter().filter(|program| program.name != "claude") {
            let common = Common::new(program, Path::new(BINARY));
            let named = format!("`{}`", program.name);
            for refused in [
                common.model("opus"),
                common.effort("high"),
                common.mode(&ModeEntry::Flume(Mode::Edits)).map(|_| ()),
            ] {
                let words = refused.expect_err(program.name);
                assert!(words.contains(&named), "{words}");
            }
            assert_eq!(common.kind(), Some(program.kind));
        }
        assert_eq!(
            amp().model("opus"),
            Err("the entry for `amp` names no way to give it a model".into())
        );
    }
}
