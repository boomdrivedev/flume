//! flume host for a test's scratch host
//! (a-tool-has-no-mode-that-only-its-tests-use): the resident run as a
//! host's service manager runs it, with its watch, its socket in a short
//! folder of its own under the system's temporary directory, which every
//! command of the test names by `FLUME_USER_SCRATCH`. It ends at its
//! drop, and a test whose resident would not end fails (a-leaked-child-
//! fails-its-test).

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

static COUNT: AtomicUsize = AtomicUsize::new(0);

/// The variable flume's commands find their resident's socket by.
pub const USER_SCRATCH: &str = "FLUME_USER_SCRATCH";

/// The watch's bound the tests' resident keeps: long enough that it
/// settles only at once and at each change its merge queue's subscription
/// hears, so a test settles by its own commands.
pub const EVERY: &str = "1h";

/// Whether a command's `words` ask status, which flume host answers.
pub fn asks_status(words: &[&str]) -> bool {
    let mut words = words.iter();
    while let Some(word) = words.next() {
        match *word {
            "--repository" => {
                words.next();
            }
            word if word.starts_with('-') => {}
            word => return word == "status",
        }
    }
    false
}

/// A short folder for a scratch host's resident's socket, made now.
pub fn user_scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "fr-{}-{}",
        std::process::id(),
        COUNT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the resident's folder");
    dir.canonicalize().expect("the resident's folder's path")
}

/// A running resident: `flume host`, or the `sluice daemon` a host's
/// service manager runs beside it.
pub struct Resident {
    child: std::sync::Mutex<Child>,
    /// The folder it is answered in, taken away at its end: flume host's.
    dir: Option<PathBuf>,
    log: PathBuf,
}

impl Resident {
    /// Starts `command`, a `flume host` whose environment names `dir` as
    /// its `FLUME_USER_SCRATCH`, and waits until its socket answers; panics
    /// with what it said where it ended or never answered.
    pub fn start(mut command: Command, dir: &Path) -> Resident {
        command.env(USER_SCRATCH, dir);
        let answers = |dir: &Path| {
            std::fs::read_dir(dir)
                .into_iter()
                .flatten()
                .flatten()
                .filter(|entry| entry.file_name().to_string_lossy().ends_with(".sock"))
                .any(|entry| std::os::unix::net::UnixStream::connect(entry.path()).is_ok())
        };
        let watched = dir.to_path_buf();
        Resident::until(command, dir.join("host.log"), Some(dir), move || {
            answers(&watched)
        })
    }

    /// Starts `command`, its errors logged to `log`, and waits until
    /// `ready`; `dir`, where given, taken away at its end.
    pub fn until(
        mut command: Command,
        log: PathBuf,
        dir: Option<&Path>,
        ready: impl Fn() -> bool,
    ) -> Resident {
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(std::fs::File::create(&log).expect("the resident's log"))
            .spawn()
            .expect("the resident starts");
        let resident = Resident {
            child: std::sync::Mutex::new(child),
            dir: dir.map(Path::to_path_buf),
            log: log.clone(),
        };
        let dir = log.parent().unwrap_or(Path::new("/")).to_path_buf();
        let until = Instant::now() + Duration::from_secs(60);
        loop {
            if ready() {
                return resident;
            }
            if let Ok(Some(status)) = resident.child().try_wait() {
                panic!(
                    "the resident ended ({status}) before it answered: {}",
                    std::fs::read_to_string(&log).unwrap_or_default()
                );
            }
            if Instant::now() > until {
                panic!(
                    "the resident gave no socket in {} within a minute: {}",
                    dir.display(),
                    std::fs::read_to_string(&log).unwrap_or_default()
                );
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn child(&self) -> std::sync::MutexGuard<'_, Child> {
        self.child
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Its process id.
    pub fn pid(&self) -> u32 {
        self.child().id()
    }

    /// Ends it at once, as a crash would, and waits for its end.
    pub fn kill(&self) {
        let mut child = self.child();
        let _ = child.kill();
        let _ = child.wait();
    }

    /// What it said on its errors so far.
    pub fn said(&self) -> String {
        std::fs::read_to_string(&self.log).unwrap_or_default()
    }
}

impl Drop for Resident {
    fn drop(&mut self) {
        let child = self
            .child
            .get_mut()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Ok(Some(_)) = child.try_wait() {
            if let Some(dir) = &self.dir {
                let _ = std::fs::remove_dir_all(dir);
            }
            return;
        }
        // SAFETY: a signal to the child this guard started and still holds.
        unsafe {
            libc::kill(child.id() as libc::pid_t, libc::SIGTERM);
        }
        // A sluice daemon ends its runs before it ends.
        let until = Instant::now() + Duration::from_secs(60);
        let ended = loop {
            match child.try_wait() {
                Ok(Some(_)) => break true,
                Ok(None) if Instant::now() < until => std::thread::sleep(Duration::from_millis(20)),
                _ => break false,
            }
        };
        if !ended {
            let _ = child.kill();
            let _ = child.wait();
        }
        if let Some(dir) = &self.dir {
            let _ = std::fs::remove_dir_all(dir);
        }
        if !ended && !std::thread::panicking() {
            panic!("the resident did not end within a minute of its SIGTERM");
        }
    }
}

/// The short folder of the resident of the host whose flume state folder
/// is under `state_home`, the same for each test that shares that host:
/// the first 12 hex digits of the SHA-1 of the state folder's resolved
/// path, made now.
pub fn user_scratch_of(state_home: &Path) -> PathBuf {
    use sha1::{Digest, Sha1};
    use std::os::unix::ffi::OsStrExt as _;
    let state = state_home.join("flume");
    std::fs::create_dir_all(&state).expect("flume's state folder");
    let state = state.canonicalize().expect("flume's state folder's path");
    let key: String = Sha1::digest(state.as_os_str().as_bytes())
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let dir = std::env::temp_dir().join(format!("fr-{key}"));
    std::fs::create_dir_all(&dir).expect("the resident's folder");
    dir.canonicalize().expect("the resident's folder's path")
}

/// The resident whose socket is in `dir`, shared by each test of this
/// process that shares its host: the one running, else one started by
/// `command`. It ends once the last test holding it lets it go.
pub fn shared(dir: &Path, command: impl FnOnce() -> Command) -> std::sync::Arc<Resident> {
    shared_until(dir, command, None)
}

/// The resident keyed by `key`, as [`shared`] gives it, started by
/// `command` and ready once a connection to `socket` is taken, where
/// given: a `sluice daemon`, its log beside `key`.
pub fn shared_until(
    key: &Path,
    command: impl FnOnce() -> Command,
    socket: Option<&Path>,
) -> std::sync::Arc<Resident> {
    let dir = key;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock, Weak};
    static RUNNING: OnceLock<Mutex<HashMap<PathBuf, Weak<Resident>>>> = OnceLock::new();
    let mut running = RUNNING
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(resident) = running.get(dir).and_then(Weak::upgrade) {
        return resident;
    }
    let resident = match socket {
        None => {
            std::fs::create_dir_all(dir).expect("the resident's folder");
            Arc::new(Resident::start(command(), dir))
        }
        Some(socket) => {
            let socket = socket.to_path_buf();
            let log = PathBuf::from(format!("{}.log", dir.display()));
            Arc::new(Resident::until(command(), log, None, move || {
                std::os::unix::net::UnixStream::connect(&socket).is_ok()
            }))
        }
    };
    running.insert(dir.to_path_buf(), Arc::downgrade(&resident));
    resident
}
