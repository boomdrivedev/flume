//! flume host's socket (flume-host-serves-status-and-its-subscription-on-
//! its-socket; residents-speak-json-lines-over-a-unix-socket): status, and
//! a subscription to status, served by the host's resident, and the client
//! every other surface asks it through.
//!
//! The socket is `host-<key>.sock` in the user's own directory, `/tmp`
//! resolved and `flume-<uid>`, or `$FLUME_USER_SCRATCH` where set, the
//! key the first 10 hex digits of the SHA-1 of flume's resolved state
//! folder, as the merge queue's resident names its own. Each line is one
//! JSON object.
//! The client says `{"hello":"<client>/<version>"}`; flume host answers
//! `{"version", "protocol", "pid", "since"}`. The client then asks one
//! thing: `{"ask":"status"}`, answered by one `{"document": <answer>}`
//! line, the answer as `flume --json status` prints it, after which flume
//! host closes the connection; or `{"ask":"subscribe"}`, answered by a
//! `document` line at once and another each time the answer differs from
//! the last one sent, until the client closes it. Each ask may name a
//! `place`, by its location, and ask each place's `line`; and, asking no
//! line, may name `repositories` by their folders, each read beside the
//! places under the answer's `repositories`. Or the client asks
//! `{"ask":"settle","place":<location>}`, with `"on_quiet":true` where the
//! agent's own quiet asks it, answered by one `{"taken":true}` line, after
//! which flume host closes the connection and settles the place itself:
//! a settle a hook asks for is a request to flume host, so a pane that
//! closes ends only the client (121; nothing-detached; a-session-in-a-
//! herdr-pane-is-herdrs-child-and-flume-owns-its-place). A line flume
//! host cannot read gets `{"error": <words>}`, and the connection closes.

use crate::resident::{Say, GATHER};
use flume_core::{Answer, Engine};
use flume_seams::PlaceId;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ffi::OsString;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::watch;
use tokio::time::{sleep, timeout, Instant};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

/// The variable the tests and a host name the user's own directory in,
/// which flume host's socket is in.
pub const USER_SCRATCH: &str = "FLUME_USER_SCRATCH";

/// The version of the lines flume host speaks.
pub const PROTOCOL: u32 = 1;

/// How long either side waits for the other's hello or ask, and a client
/// for its answer.
pub const WITHIN: Duration = Duration::from_secs(60);

/// What a client asks after its hello.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "ask", rename_all = "kebab-case")]
pub enum Ask {
    /// One answer, then the connection closes.
    Status(Asked),
    /// An answer at once and one at each change, until the client closes.
    Subscribe(Asked),
    /// One `taken` line, then the connection closes and flume host
    /// settles the place.
    Settle(Settling),
}

/// The place a client asks flume host to settle, by its location, and
/// whether its agent's own quiet asks it, so flume host first reads the
/// agent until it is quiet (121).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settling {
    pub place: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub on_quiet: bool,
}

/// How long flume host, or `flume settle --on-quiet`, reads a place's
/// agent for until it is quiet, before a settle its quiet asks for (121;
/// design 14).
pub const ON_QUIET_WITHIN: Duration = Duration::from_secs(60);

/// How often flume host reads the agent while it waits for its quiet.
pub const QUIET_EVERY: Duration = Duration::from_secs(1);

/// What status a client asks: one place's by its location, or every
/// place's, each place's line alone where `line`; and each repository it
/// names by its folder, read beside the places, which a line asks none of.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asked {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub line: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}

/// The socket flume host answers on for the process whose environment
/// `var` reads, or the words saying why there is none.
pub fn socket_from(var: impl Fn(&str) -> Option<OsString>) -> Result<PathBuf, String> {
    let state = match var("XDG_STATE_HOME").filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(var("HOME").ok_or("HOME is not set")?)
            .join(".local")
            .join("state"),
    }
    .join("flume");
    let state = state.canonicalize().map_err(|error| {
        format!(
            "flume's state folder {} is not there ({error}): flume host has never run as this user",
            state.display()
        )
    })?;
    Ok(user_dir(&var)?.join(format!("host-{}.sock", key(&state))))
}

/// The socket of this process's own flume host.
pub fn socket() -> Result<PathBuf, String> {
    socket_from(|name| std::env::var_os(name))
}

/// The user's own directory: `$FLUME_USER_SCRATCH`, else `/tmp` resolved
/// and `flume-<uid>`.
fn user_dir(var: &impl Fn(&str) -> Option<OsString>) -> Result<PathBuf, String> {
    match var(USER_SCRATCH).filter(|dir| !dir.is_empty()) {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => {
            let tmp = Path::new("/tmp")
                .canonicalize()
                .map_err(|error| format!("resolving /tmp: {error}"))?;
            // SAFETY: geteuid reads the process's effective user and
            // cannot fail.
            Ok(tmp.join(format!("flume-{}", unsafe { libc::geteuid() })))
        }
    }
}

/// The key a folder names its socket by: the first 10 hex digits of the
/// SHA-1 of its path's bytes, as the merge queue's resident keys its own.
fn key(folder: &Path) -> String {
    use sha1::{Digest, Sha1};
    use std::os::unix::ffi::OsStrExt as _;
    Sha1::digest(folder.as_os_str().as_bytes())
        .iter()
        .take(5)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The words a client gives where no flume host answers on `socket`.
fn unanswered(socket: &Path, error: impl std::fmt::Display) -> String {
    format!(
        "flume host answers on no socket at {} ({error}); the host's service manager runs flume host",
        socket.display()
    )
}

/// Asks the flume host answering on `socket` for `asked`'s status: the
/// answer's JSON as `flume --json status` prints it, or the words saying
/// why none came. It builds no core and starts no process.
pub fn status_at(socket: &Path, asked: &Asked) -> Result<String, String> {
    let answer = ask_at(socket, &Ask::Status(asked.clone()))?;
    answer
        .get("document")
        .map(Value::to_string)
        .ok_or_else(|| format!("flume host gave no document: {answer}"))
}

/// Asks the flume host answering on `socket` to settle the place
/// `settling` names; the words saying why it did not take the ask. It
/// waits for no settle, builds no core and starts no process
/// (nothing-detached).
pub fn settle_at(socket: &Path, settling: &Settling) -> Result<(), String> {
    let answer = ask_at(socket, &Ask::Settle(settling.clone()))?;
    match answer.get("taken") {
        Some(Value::Bool(true)) => Ok(()),
        _ => Err(format!("flume host did not take the settle: {answer}")),
    }
}

/// Asks this host's flume host to settle the place `settling` names; the
/// words saying why it did not take the ask.
pub fn settle(settling: &Settling) -> Result<(), String> {
    socket().and_then(|socket| settle_at(&socket, settling))
}

/// Says the hello to the flume host answering on `socket`, asks `ask`,
/// and gives the first line of its answer; the words saying why none came,
/// or its refusal.
fn ask_at(socket: &Path, ask: &Ask) -> Result<Value, String> {
    use std::io::{BufRead, Write};
    let stream =
        std::os::unix::net::UnixStream::connect(socket).map_err(|e| unanswered(socket, e))?;
    stream
        .set_read_timeout(Some(WITHIN))
        .and_then(|()| stream.set_write_timeout(Some(WITHIN)))
        .map_err(|error| format!("bounding the ask of flume host: {error}"))?;
    let mut writer = stream
        .try_clone()
        .map_err(|error| format!("asking flume host: {error}"))?;
    let mut lines = std::io::BufReader::new(stream).lines();
    let mut send = |line: &Value| {
        writeln!(writer, "{line}").map_err(|error| format!("writing to flume host: {error}"))
    };
    let mut next = |what: &str| -> Result<Value, String> {
        let line = lines
            .next()
            .ok_or_else(|| format!("flume host closed the connection before {what}"))?
            .map_err(|error| format!("reading {what} from flume host: {error}"))?;
        serde_json::from_str(&line)
            .map_err(|error| format!("flume host's {what}, {line}, is not JSON: {error}"))
    };
    send(&json!({ "hello": format!("flume/{}", crate::VERSION) }))?;
    let named = next("its hello")?;
    if named.get("version").and_then(Value::as_str).is_none() {
        return Err(format!("flume host's hello names no version: {named}"));
    }
    send(&serde_json::to_value(ask).map_err(|e| e.to_string())?)?;
    let answer = next("its answer")?;
    if let Some(words) = answer.get("error") {
        let words = words
            .as_str()
            .map_or_else(|| words.to_string(), str::to_string);
        return Err(format!("flume host refused the ask: {words}"));
    }
    Ok(answer)
}

/// The status `asked` names, from this host's flume host: its answer, or
/// the refusal naming why none came.
pub fn status(asked: &Asked) -> Answer {
    let read = socket().and_then(|socket| status_at(&socket, asked));
    match read.and_then(|json| {
        serde_json::from_str::<Answer>(&json)
            .map_err(|error| format!("flume host's answer, {json}, is not flume's: {error}"))
    }) {
        Ok(answer) => answer,
        Err(words) => Answer::refused(words),
    }
}

/// How flume host reads the status a client asks.
pub trait Reads: Send + Sync + 'static {
    /// The answer's JSON, as `flume --json status` prints it.
    fn read(&self, asked: &Asked) -> impl Future<Output = String> + Send;
}

/// How flume host settles the place a client asks it to.
pub trait Settles: Send + Sync + 'static {
    /// Whether the place's agent is quiet: neither working nor asking,
    /// nor waiting on own work; a place with no agent is quiet (121).
    fn quiet(&self, place: &str) -> impl Future<Output = bool> + Send;
    /// Settles the place, as its agent's own quiet starts a settle where
    /// `on_quiet` (121, 212).
    fn settle(&self, place: &str, on_quiet: bool) -> impl Future<Output = ()> + Send;
}

/// The reads and settles of the host's own core: each builds the core
/// afresh on tokio's blocking pool, as a settle does, and asks it status
/// or a settle (a-residents-core-stays-synchronous-behind-a-tokio-edge).
pub struct CoreReads {
    build: crate::commands::Build,
}

impl CoreReads {
    pub fn new(build: crate::commands::Build) -> CoreReads {
        CoreReads { build }
    }
}

impl Reads for CoreReads {
    async fn read(&self, asked: &Asked) -> String {
        let build = self.build.clone();
        let asked = asked.clone();
        tokio::task::spawn_blocking(move || read_core(&*build, &asked))
            .await
            .unwrap_or_else(|error| {
                Answer::refused(format!("the status's task ended: {error}")).to_json()
            })
    }
}

impl Settles for CoreReads {
    async fn quiet(&self, place: &str) -> bool {
        let build = self.build.clone();
        let place = PlaceId::new(place);
        tokio::task::spawn_blocking(move || {
            build().map_or(true, |core| crate::commands::quiet(&core, &place))
        })
        .await
        .unwrap_or(true)
    }

    async fn settle(&self, place: &str, on_quiet: bool) {
        let build = self.build.clone();
        let place = PlaceId::new(place);
        let _ = tokio::task::spawn_blocking(move || {
            let Ok(core) = build() else {
                return;
            };
            if on_quiet {
                core.settle_on_quiet(&place);
            } else {
                core.settle(Some(&place));
            }
        })
        .await;
    }
}

/// `asked`'s status from the core `build` gives, as its JSON.
pub fn read_core(build: &dyn Fn() -> Result<Engine, String>, asked: &Asked) -> String {
    let place = asked.place.as_deref().map(PlaceId::new);
    if asked.line && !asked.repositories.is_empty() {
        return Answer::refused(
            "a status of lines names no repository: ask the repositories without line",
        )
        .to_json();
    }
    match build() {
        Ok(core) if asked.line => core.lines(place.as_ref()),
        Ok(core) => core.status_of(place.as_ref(), &asked.repositories),
        Err(words) => Answer::refused(words),
    }
    .to_json()
}

/// When flume host started, for its hello.
fn since() -> &'static str {
    static SINCE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SINCE.get_or_init(|| {
        chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now())
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    })
}

/// Binds flume host's socket at `socket`, its directory made the user's
/// alone, a socket no process answers on taken away first; the words
/// saying why not. The caller holds the resident's lock, so no other
/// flume host's socket is taken away.
pub fn bind(socket: &Path) -> Result<UnixListener, String> {
    use std::os::unix::fs::PermissionsExt as _;
    let dir = socket
        .parent()
        .ok_or_else(|| format!("the socket {} has no folder", socket.display()))?;
    std::fs::create_dir_all(dir).map_err(|error| format!("making {}: {error}", dir.display()))?;
    let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
    if socket.exists() {
        std::fs::remove_file(socket)
            .map_err(|error| format!("taking away {}: {error}", socket.display()))?;
    }
    UnixListener::bind(socket).map_err(|error| {
        format!(
            "no room for flume host's socket at {}: {error}",
            socket.display()
        )
    })
}

/// Serves `listener` until `stop` is cancelled: each connection on a task
/// of its own, a subscription read again at each move of `changed`, the
/// moves that come within [`GATHER`] of the first making one read, and a
/// settle asked run on its connection's task. It ends once every
/// connection's task has (workers-end-with-their-owner).
pub async fn serve<R: Reads + Settles>(
    listener: UnixListener,
    reads: Arc<R>,
    changed: watch::Receiver<u64>,
    stop: CancellationToken,
    say: Say,
) {
    let tasks = TaskTracker::new();
    loop {
        tokio::select! {
            _ = stop.cancelled() => break,
            accepted = listener.accept() => match accepted {
                Ok((stream, _)) => {
                    let reads = reads.clone();
                    let changed = changed.clone();
                    let stop = stop.clone();
                    let say = say.clone();
                    tasks.spawn(async move {
                        if let Err(words) = connection(stream, &*reads, changed, stop).await {
                            say(&format!("a client's connection ended: {words}"));
                        }
                    });
                }
                Err(error) => say(&format!("accepting a client: {error}")),
            },
        }
    }
    tasks.close();
    tasks.wait().await;
}

/// One client's connection: its hello, then its one ask.
async fn connection<R: Reads + Settles>(
    stream: UnixStream,
    reads: &R,
    mut changed: watch::Receiver<u64>,
    stop: CancellationToken,
) -> Result<(), String> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    let mut next = async || -> Result<Option<String>, String> {
        timeout(WITHIN, lines.next_line())
            .await
            .map_err(|_| format!("no line within {} seconds", WITHIN.as_secs()))?
            .map_err(|error| format!("reading a client: {error}"))
    };
    let hello = next().await?.ok_or("the client closed before its hello")?;
    if serde_json::from_str::<Value>(&hello)
        .ok()
        .and_then(|hello| hello.get("hello").cloned())
        .is_none()
    {
        let words = format!("flume host reads a hello first, and read {hello}");
        send(&mut writer, &json!({ "error": words })).await?;
        return Err(words);
    }
    send(
        &mut writer,
        &json!({
            "version": crate::VERSION,
            "protocol": PROTOCOL,
            "pid": std::process::id(),
            "since": since(),
        }),
    )
    .await?;
    let line = next().await?.ok_or("the client closed before its ask")?;
    let ask = match serde_json::from_str::<Ask>(&line) {
        Ok(ask) => ask,
        Err(error) => {
            let words = format!("flume host cannot read the line {line}: {error}");
            send(&mut writer, &json!({ "error": words })).await?;
            return Err(words);
        }
    };
    match ask {
        Ask::Settle(settling) => {
            send(&mut writer, &json!({ "taken": true })).await?;
            drop(writer);
            settle_asked(reads, &settling, &stop).await;
            Ok(())
        }
        Ask::Status(asked) => {
            let document = reads.read(&asked).await;
            send(&mut writer, &document_line(&document)).await
        }
        Ask::Subscribe(asked) => {
            changed.borrow_and_update();
            let mut last = reads.read(&asked).await;
            send(&mut writer, &document_line(&last)).await?;
            loop {
                tokio::select! {
                    _ = stop.cancelled() => return Ok(()),
                    line = next() => match line {
                        Ok(None) => return Ok(()),
                        Ok(Some(_)) => {}
                        Err(_) => return Ok(()),
                    },
                    moved = changed.changed() => {
                        if moved.is_err() {
                            return Ok(());
                        }
                        tokio::select! {
                            _ = stop.cancelled() => return Ok(()),
                            _ = sleep(GATHER) => {}
                        }
                        changed.borrow_and_update();
                        let now = reads.read(&asked).await;
                        if now != last {
                            send(&mut writer, &document_line(&now)).await?;
                            last = now;
                        }
                    }
                }
            }
        }
    }
}

/// The settle a client asked: where its agent's quiet asked it, the agent
/// read each [`QUIET_EVERY`] until it is quiet or [`ON_QUIET_WITHIN`] has
/// passed, a stop meanwhile ending it with no settle; then the place
/// settled, which runs to its end (121; workers-end-with-their-owner).
async fn settle_asked<S: Settles>(settles: &S, settling: &Settling, stop: &CancellationToken) {
    if settling.on_quiet {
        let until = Instant::now() + ON_QUIET_WITHIN;
        while !settles.quiet(&settling.place).await && Instant::now() < until {
            tokio::select! {
                _ = stop.cancelled() => return,
                _ = sleep(QUIET_EVERY) => {}
            }
        }
        if stop.is_cancelled() {
            return;
        }
    }
    settles.settle(&settling.place, settling.on_quiet).await;
}

/// The `document` line of an answer's JSON.
fn document_line(answer: &str) -> Value {
    let document = serde_json::from_str::<Value>(answer).unwrap_or_else(|_| json!(answer));
    json!({ "document": document })
}

/// Writes `line` and a newline to the client.
async fn send(writer: &mut tokio::net::unix::OwnedWriteHalf, line: &Value) -> Result<(), String> {
    let mut bytes = line.to_string().into_bytes();
    bytes.push(b'\n');
    writer
        .write_all(&bytes)
        .await
        .map_err(|error| format!("writing to a client: {error}"))
}

/// Moves `changed` at each event in flume's state folder `state` that is
/// not its log or a lock, until `stop` is cancelled: each command of
/// flume's writes its places' state there
/// (residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop).
/// The words saying why it watches nothing.
pub fn watch_state(
    state: &Path,
    changed: watch::Sender<u64>,
) -> Result<notify::RecommendedWatcher, String> {
    use notify::Watcher as _;
    let mut watcher = notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
        let Ok(event) = event else {
            return;
        };
        let counts = event.paths.iter().any(|path| {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_default();
            name != "flume.log" && !name.ends_with(".lock")
        });
        if counts {
            changed.send_modify(|moves| *moves = moves.wrapping_add(1));
        }
    })
    .map_err(|error| format!("watching flume's state folder: {error}"))?;
    watcher
        .watch(state, notify::RecursiveMode::Recursive)
        .map_err(|error| format!("watching {}: {error}", state.display()))?;
    Ok(watcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Reads that give the answer the test holds, counting themselves;
    /// and settles that record when they ran, the agent read quiet from
    /// its `quiet_at`th read on, and never where it holds none.
    #[derive(Default)]
    struct Held {
        answer: Mutex<String>,
        reads: Mutex<Vec<Asked>>,
        quiet_at: Mutex<Option<usize>>,
        quiet_reads: Mutex<Vec<(String, Instant)>>,
        settles: Mutex<Vec<(String, bool, Instant)>>,
        settled: tokio::sync::Notify,
    }

    impl Reads for Held {
        async fn read(&self, asked: &Asked) -> String {
            self.reads.lock().unwrap().push(asked.clone());
            self.answer.lock().unwrap().clone()
        }
    }

    impl Settles for Held {
        async fn quiet(&self, place: &str) -> bool {
            let mut reads = self.quiet_reads.lock().unwrap();
            reads.push((place.to_string(), Instant::now()));
            self.quiet_at
                .lock()
                .unwrap()
                .is_some_and(|at| reads.len() >= at)
        }

        async fn settle(&self, place: &str, on_quiet: bool) {
            self.settles
                .lock()
                .unwrap()
                .push((place.to_string(), on_quiet, Instant::now()));
            self.settled.notify_one();
        }
    }

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("fsv-{test}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the test's scratch");
        dir
    }

    fn quiet() -> Say {
        Arc::new(|_| {})
    }

    struct Served {
        socket: PathBuf,
        held: Arc<Held>,
        changed: watch::Sender<u64>,
        stop: CancellationToken,
        handle: tokio::task::JoinHandle<()>,
    }

    fn served(test: &str) -> Served {
        let socket = scratch(test).join("host.sock");
        let listener = bind(&socket).expect("the socket");
        let held = Arc::new(Held::default());
        *held.answer.lock().unwrap() = r#"{"status":{"n":1}}"#.into();
        let (changed, following) = watch::channel(0);
        let stop = CancellationToken::new();
        let handle = tokio::spawn(serve(
            listener,
            held.clone(),
            following,
            stop.clone(),
            quiet(),
        ));
        Served {
            socket,
            held,
            changed,
            stop,
            handle,
        }
    }

    async fn client(socket: &Path, ask: &str) -> tokio::io::Lines<BufReader<UnixStream>> {
        let mut stream = UnixStream::connect(socket).await.expect("connected");
        stream
            .write_all(format!("{{\"hello\":\"test/1\"}}\n{ask}\n").as_bytes())
            .await
            .unwrap();
        BufReader::new(stream).lines()
    }

    async fn line(lines: &mut tokio::io::Lines<BufReader<UnixStream>>) -> Option<Value> {
        lines
            .next_line()
            .await
            .unwrap()
            .map(|line| serde_json::from_str(&line).unwrap())
    }

    #[test]
    fn the_socket_is_named_by_its_state_folders_digest() {
        let dir = scratch("socket");
        let state = dir.join("state");
        std::fs::create_dir_all(state.join("flume")).unwrap();
        let resolved = state.join("flume").canonicalize().unwrap();
        let socket = socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(state.clone().into()),
            USER_SCRATCH => Some(dir.clone().into()),
            _ => None,
        })
        .unwrap();
        assert_eq!(socket, dir.join(format!("host-{}.sock", key(&resolved))));
        let words = socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(dir.join("none").into()),
            _ => None,
        })
        .unwrap_err();
        assert!(words.contains("flume host has never run"), "{words}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn asks_read_back_as_written() {
        let ask: Ask = serde_json::from_str(r#"{"ask":"status"}"#).unwrap();
        assert_eq!(ask, Ask::Status(Asked::default()));
        let ask: Ask =
            serde_json::from_str(r#"{"ask":"subscribe","place":"/p","line":true}"#).unwrap();
        assert_eq!(
            ask,
            Ask::Subscribe(Asked {
                place: Some("/p".into()),
                line: true,
                ..Asked::default()
            })
        );
        let ask: Ask =
            serde_json::from_str(r#"{"ask":"subscribe","repositories":["/r","/s"]}"#).unwrap();
        assert_eq!(
            ask,
            Ask::Subscribe(Asked {
                repositories: vec!["/r".into(), "/s".into()],
                ..Asked::default()
            })
        );
    }

    #[test]
    fn a_line_names_no_repository() {
        let asked = Asked {
            line: true,
            repositories: vec!["/r".into()],
            ..Asked::default()
        };
        let built = std::sync::atomic::AtomicBool::new(false);
        let answer = read_core(
            &|| {
                built.store(true, std::sync::atomic::Ordering::SeqCst);
                Err("no core".into())
            },
            &asked,
        );
        assert!(answer.contains("names no repository"), "{answer}");
        assert!(
            !built.load(std::sync::atomic::Ordering::SeqCst),
            "no core is built"
        );
    }

    #[tokio::test]
    async fn a_status_is_one_document_then_the_end() {
        let served = served("status");
        let mut lines = client(&served.socket, r#"{"ask":"status","line":true}"#).await;
        let named = line(&mut lines).await.unwrap();
        assert_eq!(named["version"], crate::VERSION);
        assert_eq!(named["protocol"], PROTOCOL);
        assert_eq!(named["pid"], std::process::id());
        assert_eq!(
            line(&mut lines).await.unwrap(),
            json!({"document": {"status": {"n": 1}}})
        );
        assert_eq!(line(&mut lines).await, None, "flume host closed it");
        assert_eq!(
            served.held.reads.lock().unwrap().clone(),
            vec![Asked {
                line: true,
                ..Asked::default()
            }]
        );
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[tokio::test]
    async fn the_client_reads_the_document() {
        let served = served("client");
        let socket = served.socket.clone();
        let read = tokio::task::spawn_blocking(move || status_at(&socket, &Asked::default()))
            .await
            .unwrap();
        assert_eq!(read.unwrap(), r#"{"status":{"n":1}}"#);
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[test]
    fn no_flume_host_is_named_with_its_socket() {
        let socket = scratch("none").join("host.sock");
        let words = status_at(&socket, &Asked::default()).unwrap_err();
        assert!(words.contains(&socket.display().to_string()), "{words}");
        assert!(words.contains("service manager runs flume host"), "{words}");
    }

    #[tokio::test]
    async fn an_ask_it_cannot_read_is_an_error() {
        let served = served("error");
        let mut lines = client(&served.socket, r#"{"ask":"tidy"}"#).await;
        line(&mut lines).await.unwrap();
        let error = line(&mut lines).await.unwrap();
        assert!(error["error"].as_str().unwrap().contains("tidy"), "{error}");
        assert_eq!(line(&mut lines).await, None);
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn a_subscription_gets_each_change_once() {
        let served = served("subscribe");
        let mut lines = client(&served.socket, r#"{"ask":"subscribe"}"#).await;
        line(&mut lines).await.unwrap();
        assert_eq!(
            line(&mut lines).await.unwrap(),
            json!({"document": {"status": {"n": 1}}})
        );
        // A move with the same answer sends nothing; one with another
        // answer sends it, the moves within the gather making one read.
        served.changed.send_modify(|n| *n += 1);
        sleep(GATHER * 2).await;
        *served.held.answer.lock().unwrap() = r#"{"status":{"n":2}}"#.into();
        for _ in 0..5 {
            served.changed.send_modify(|n| *n += 1);
        }
        assert_eq!(
            line(&mut lines).await.unwrap(),
            json!({"document": {"status": {"n": 2}}})
        );
        assert_eq!(
            served.held.reads.lock().unwrap().len(),
            3,
            "the first read, the unchanged one and one for the five moves"
        );
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    /// Asks `ask` of `served` and reads its answer: the `taken` line, and
    /// then the connection's end.
    async fn taken(served: &Served, ask: &str) {
        let mut lines = client(&served.socket, ask).await;
        line(&mut lines).await.unwrap();
        assert_eq!(line(&mut lines).await.unwrap(), json!({"taken": true}));
        assert_eq!(line(&mut lines).await, None, "flume host closed it");
    }

    #[tokio::test(start_paused = true)]
    async fn a_settle_is_taken_then_settled_with_no_read_of_the_agent() {
        let served = served("settle");
        let began = Instant::now();
        taken(&served, r#"{"ask":"settle","place":"/p"}"#).await;
        served.held.settled.notified().await;
        assert_eq!(
            served.held.settles.lock().unwrap().clone(),
            vec![("/p".to_string(), false, began)]
        );
        assert!(served.held.quiet_reads.lock().unwrap().is_empty());
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn a_settle_on_quiet_waits_for_the_agents_quiet() {
        let served = served("on-quiet");
        *served.held.quiet_at.lock().unwrap() = Some(3);
        let began = Instant::now();
        taken(&served, r#"{"ask":"settle","place":"/p","on_quiet":true}"#).await;
        served.held.settled.notified().await;
        let reads: Vec<Duration> = served
            .held
            .quiet_reads
            .lock()
            .unwrap()
            .iter()
            .map(|(place, at)| {
                assert_eq!(place, "/p");
                *at - began
            })
            .collect();
        assert_eq!(reads, [0, 1, 2].map(Duration::from_secs));
        assert_eq!(
            served.held.settles.lock().unwrap().clone(),
            vec![("/p".to_string(), true, began + QUIET_EVERY * 2)]
        );
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn an_agent_never_quiet_is_settled_at_the_bound() {
        let served = served("never-quiet");
        let began = Instant::now();
        taken(&served, r#"{"ask":"settle","place":"/p","on_quiet":true}"#).await;
        served.held.settled.notified().await;
        assert_eq!(
            served.held.settles.lock().unwrap().clone(),
            vec![("/p".to_string(), true, began + ON_QUIET_WITHIN)]
        );
        served.stop.cancel();
        served.handle.await.unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn a_stop_during_the_wait_for_quiet_settles_nothing() {
        let served = served("stopped");
        taken(&served, r#"{"ask":"settle","place":"/p","on_quiet":true}"#).await;
        sleep(QUIET_EVERY * 5).await;
        assert!(!served.held.quiet_reads.lock().unwrap().is_empty());
        served.stop.cancel();
        served.handle.await.unwrap();
        assert!(
            served.held.settles.lock().unwrap().is_empty(),
            "no settle after the stop"
        );
    }

    #[tokio::test]
    async fn the_client_asks_a_settle() {
        let served = served("settle-client");
        let socket = served.socket.clone();
        let asked = Settling {
            place: "/p".into(),
            on_quiet: false,
        };
        tokio::task::spawn_blocking(move || settle_at(&socket, &asked))
            .await
            .unwrap()
            .expect("flume host took the settle");
        served.held.settled.notified().await;
        assert_eq!(served.held.settles.lock().unwrap()[0].0, "/p");
        served.stop.cancel();
        served.handle.await.unwrap();
    }
}
