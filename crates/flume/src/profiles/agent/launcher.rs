//! flume's launcher, `flume ack launched -- <program> [<argument>…]`
//! (35, 161, C.4; core.17, core.30; record 0026; design 2): run by a
//! terminal flume opened in the program's place, under the start's
//! identity and secret, it reads the start's record from the core,
//! compares each variable the record names, by its digest keyed by the
//! secret, and the program it is to run with the record's, and writes its
//! word once in the place's own folder, `launched/<identity>.json`, the
//! identity's `/` written `%`: its process, the program's absolute path,
//! and how the start began otherwise, none where nothing differed. Where
//! nothing differed it answers the program for its caller to become in
//! the same process, its arguments whole; where anything did, it runs
//! nothing and starts a settle of the place detached; where a word
//! already stands for the start, or no start is under its identity and
//! secret, it runs nothing and writes nothing. It is flume's own and
//! names no program (core.1).

use super::claude::{AGENT, PLACE, SECRET};
use super::start;
use flume_core::Engine;
use flume_seams::{Secret, Unsettled};
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// The folder under the place's own folder that holds the launchers'
/// words.
pub const DIR: &str = "launched";

/// A launcher's word of its start: the process it became, the program's
/// absolute path, and how the start began otherwise, none where nothing
/// differed, each variable by its name alone (35, 161; core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word {
    pub pid: u32,
    pub program: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsettled: Option<Unsettled>,
}

/// Where the word of the start `identity` lies under the place's own
/// `folder`.
pub fn path(folder: &str, identity: &str) -> PathBuf {
    Path::new(folder)
        .join(DIR)
        .join(format!("{}.json", identity.replace('/', "%")))
}

/// The word of the start `identity`, read in the place's own `folder`:
/// none where no word stands; why not where it cannot be read.
pub fn read(folder: &str, identity: &str) -> Result<Option<Word>, String> {
    let at = path(folder, identity);
    match std::fs::read_to_string(&at) {
        Ok(text) => serde_json::from_str(&text)
            .map(Some)
            .map_err(|error| format!("{}: {error}", at.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if Path::new(folder).is_dir() {
                Ok(None)
            } else {
                Err(format!("{folder}: the place's own folder cannot be read"))
            }
        }
        Err(error) => Err(format!("{}: {error}", at.display())),
    }
}

/// The word of the start `identity` written in the place's own `folder`,
/// once: where one already stands it is left as it stands, and the write
/// refused naming the process that word names (core.17).
pub fn write(folder: &str, identity: &str, word: &Word) -> Result<(), String> {
    let at = path(folder, identity);
    if let Some(dir) = at.parent() {
        std::fs::create_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    }
    let text = serde_json::to_string(word).expect("a word is written as JSON");
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&at)
    {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(match read(folder, identity) {
                Ok(Some(first)) => already(identity, first.pid),
                Ok(None) => format!("{}: a word stands for the start", at.display()),
                Err(why) => why,
            });
        }
        Err(error) => return Err(format!("{}: {error}", at.display())),
    };
    file.write_all(text.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("{}: {error}", at.display()))
}

/// Why a launcher that finds the word of its start written runs nothing,
/// naming the process the first became (35, 161).
fn already(identity: &str, pid: u32) -> String {
    format!(
        "{identity} began in the process {pid}, through its own launcher; a start is launched once, so this runs nothing"
    )
}

/// The program `program` names, found as the command finds it: a path as
/// given, made canonical; a name, the first file of that name on `path`'s
/// folders whose canonical path is not `binary`'s, so a link of the kind's
/// name to the launcher itself is passed over; none where none is found
/// (C.4, C.5).
pub fn find(program: &str, path: &str, binary: &Path) -> Option<PathBuf> {
    let canonical = |file: &Path| file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
    if program.contains('/') {
        let file = Path::new(program);
        return file.is_file().then(|| canonical(file));
    }
    let own = canonical(binary);
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .map(|dir| Path::new(dir).join(program))
        .filter(|candidate| candidate.is_file())
        .map(|candidate| canonical(&candidate))
        .find(|candidate| *candidate != own)
}

/// What the launcher's environment says of its start.
#[derive(Clone, Debug, Default)]
pub struct Launching {
    pub agent: Option<String>,
    pub secret: Option<String>,
    pub place: Option<String>,
}

fn variable(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

impl Launching {
    /// The start this process's environment names.
    pub fn of_process() -> Launching {
        Launching {
            agent: variable(AGENT),
            secret: variable(SECRET),
            place: variable(PLACE),
        }
    }
}

/// The program the launcher becomes, by its absolute path, with its
/// arguments whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Become {
    pub program: PathBuf,
    pub args: Vec<String>,
}

/// The launcher of the start `launching` names, given `program` and its
/// `args`, in the process `pid`: the start's record read from the core
/// built by `build`, given its identity and secret; each variable it names
/// compared with `env` and `program`, found on `path` past `binary`, with
/// the record's found so; its word written once; and the program to
/// become where nothing differed. Where anything differed the word says
/// what, a settle of the place is started detached through `binary`, and
/// the answer says why it runs nothing; where a word stands, or no start
/// is under its identity and secret, nothing is written, and the answer
/// says why (35, 41, 161, C.4; core.17, core.30).
#[allow(clippy::too_many_arguments)]
pub fn launch(
    launching: &Launching,
    program: &str,
    args: &[String],
    build: &dyn Fn() -> Result<Engine, String>,
    binary: &Path,
    env: impl Fn(&str) -> Option<String>,
    path: &str,
    pid: u32,
) -> Result<Become, String> {
    let (Some(agent), Some(secret)) = (&launching.agent, &launching.secret) else {
        return Err(format!(
            "this agent was not started by flume: {AGENT} or {SECRET} is not set, so nothing runs"
        ));
    };
    let (folder, recorded) = build()?.start_record(agent, secret)?;
    let Some(found) = find(program, path, binary) else {
        return Err(format!(
            "no program `{program}` is found to run for {agent}, so nothing runs"
        ));
    };
    let mut unsettled = start::variables(&recorded, &Secret(secret.clone()), env);
    if let Some(named) = find(&recorded.program, path, binary) {
        unsettled.program = named != found;
    }
    let differed = unsettled != Unsettled::default();
    let word = Word {
        pid,
        program: found.to_string_lossy().into_owned(),
        unsettled: differed.then(|| unsettled.clone()),
    };
    write(&folder, agent, &word)?;
    if differed {
        if let Some(place) = &launching.place {
            start::settle_detached(binary, place, false);
        }
        return Err(format!(
            "{agent} began {}, not as its start recorded, so it is not its stage's agent and nothing runs",
            unsettled.words()
        ));
    }
    Ok(Become {
        program: found,
        args: args.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};

    /// A scratch folder of the test's own, empty.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "flume-launcher-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_nanos())
                .unwrap_or_default()
        ));
        std::fs::create_dir_all(&dir).expect("a scratch folder");
        dir
    }

    fn executable(at: &Path) {
        std::fs::write(at, "#!/bin/sh\n").expect("a scratch program");
        std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755))
            .expect("a scratch program's mode");
    }

    /// The agent-launcher spec's scenario "A program found past the
    /// launcher itself" (C.4, C.5).
    #[test]
    fn a_program_found_past_the_launcher_itself() {
        let dir = scratch("found");
        let (start, bin) = (dir.join("start"), dir.join("bin"));
        std::fs::create_dir_all(&start).unwrap();
        std::fs::create_dir_all(&bin).unwrap();
        let launcher = dir.join("flume");
        executable(&launcher);
        symlink(&launcher, start.join("amp")).unwrap();
        executable(&bin.join("amp"));
        let path = format!("{}:{}", start.display(), bin.display());
        let found = find("amp", &path, &launcher).expect("amp found");
        assert_eq!(found, bin.join("amp").canonicalize().unwrap());
        assert_ne!(
            found,
            launcher.canonicalize().unwrap(),
            "the launcher itself"
        );
        // The launcher alone on PATH is never found.
        assert_eq!(find("amp", &start.display().to_string(), &launcher), None);
        // A path is taken as given.
        let given = bin.join("amp").display().to_string();
        assert_eq!(find(&given, "", &launcher), Some(found));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The word's half of the agent-launcher spec's scenario "A second
    /// launcher under one start" (35, 161; core.17).
    #[test]
    fn the_word_is_written_once() {
        let dir = scratch("once");
        let folder = dir.display().to_string();
        let identity = "flume:/r:fix--one:code:1";
        assert_eq!(read(&folder, identity), Ok(None), "before a word");
        let first = Word {
            pid: 41,
            program: "/scratch/bin/amp".into(),
            unsettled: None,
        };
        write(&folder, identity, &first).expect("the first word");
        assert_eq!(read(&folder, identity), Ok(Some(first.clone())));
        assert_eq!(
            path(&folder, identity),
            dir.join("launched/flume:%r:fix--one:code:1.json")
        );
        let second = Word {
            pid: 42,
            ..first.clone()
        };
        let refused = write(&folder, identity, &second).expect_err("a second word was written");
        assert!(refused.contains("the process 41"), "{refused}");
        assert_eq!(
            read(&folder, identity),
            Ok(Some(first)),
            "the word is the first's"
        );
        // A word written with what differed names each variable alone.
        let other = "flume:/r:fix--one:code:2";
        let differed = Word {
            pid: 43,
            program: "/scratch/bin/amp".into(),
            unsettled: Some(Unsettled {
                different: vec!["AMP_HOME".into()],
                ..Unsettled::default()
            }),
        };
        write(&folder, other, &differed).unwrap();
        assert_eq!(read(&folder, other), Ok(Some(differed)));
        // A folder that cannot be read reads why.
        let gone = dir.join("gone").display().to_string();
        assert!(read(&gone, identity).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
