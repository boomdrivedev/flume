//! flume host's socket (flume-host-serves-status-and-its-subscription-on-
//! its-socket; residents-speak-json-lines-over-a-unix-socket): status, and
//! a subscription to status, served by the host's resident, and the client
//! every other surface asks it through.
//!
//! The socket is `host-<key>.sock` in the user's own directory, `/tmp`
//! resolved and `flume-<uid>`, or `$FLUME_USER_SCRATCH` where set, the
//! key the first 10 hex digits of the SHA-1 of flume's resolved state
//! folder, as sluice host names its own. Each line is one JSON object.
//! The client says `{"hello":"<client>/<version>"}`; flume host answers
//! `{"version", "protocol", "pid", "since"}`. The client then asks one
//! thing: `{"ask":"status"}`, answered by one `{"document": <answer>}`
//! line, the answer as `flume --json status` prints it, after which flume
//! host closes the connection; or `{"ask":"subscribe"}`, answered by a
//! `document` line at once and another each time the answer differs from
//! the last one sent, until the client closes it. Each ask may name a
//! `place`, by its location, and ask each place's `line`. A line flume
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
use tokio::time::{sleep, timeout};
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
}

/// What status a client asks: one place's by its location, or every
/// place's, each place's line alone where `line`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asked {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub line: bool,
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
/// SHA-1 of its path's bytes, as sluice keys its own.
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
    send(&serde_json::to_value(Ask::Status(asked.clone())).map_err(|e| e.to_string())?)?;
    let answer = next("its answer")?;
    if let Some(words) = answer.get("error") {
        let words = words
            .as_str()
            .map_or_else(|| words.to_string(), str::to_string);
        return Err(format!("flume host refused the ask: {words}"));
    }
    answer
        .get("document")
        .map(Value::to_string)
        .ok_or_else(|| format!("flume host gave no document: {answer}"))
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

/// The reads of the host's own core: each builds the core afresh on
/// tokio's blocking pool, as a settle does, and asks it status
/// (a-residents-core-stays-synchronous-behind-a-tokio-edge).
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

/// `asked`'s status from the core `build` gives, as its JSON.
pub fn read_core(build: &dyn Fn() -> Result<Engine, String>, asked: &Asked) -> String {
    let place = asked.place.as_deref().map(PlaceId::new);
    match build() {
        Ok(core) if asked.line => core.lines(place.as_ref()),
        Ok(core) => core.status(place.as_ref()),
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
/// moves that come within [`GATHER`] of the first making one read. It
/// ends once every connection's task has (workers-end-with-their-owner).
pub async fn serve<R: Reads>(
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
async fn connection<R: Reads>(
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

    /// Reads that give the answer the test holds, counting themselves.
    #[derive(Default)]
    struct Held {
        answer: Mutex<String>,
        reads: Mutex<Vec<Asked>>,
    }

    impl Reads for Held {
        async fn read(&self, asked: &Asked) -> String {
            self.reads.lock().unwrap().push(asked.clone());
            self.answer.lock().unwrap().clone()
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
    fn the_socket_is_named_as_sluice_names_its_own() {
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
                line: true
            })
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
                place: None,
                line: true
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
        let mut lines = client(&served.socket, r#"{"ask":"settle"}"#).await;
        line(&mut lines).await.unwrap();
        let error = line(&mut lines).await.unwrap();
        assert!(
            error["error"].as_str().unwrap().contains("settle"),
            "{error}"
        );
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
}
