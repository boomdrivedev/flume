//! The subscription `flume host` holds to `sluice host` (record 0082;
//! one-subscription-per-plugin; residents-ask-each-other-by-socket):
//! one connection to sluice host's socket, never a sluice command, which
//! follows the repository of each place a change in sluice can move.
//!
//! The socket is where sluice host keeps it: in the user's own directory,
//! `/tmp` resolved and `sluice-<uid>`, or `$SLUICE_USER_SCRATCH` where
//! set, as `host-<key>.sock`, the key the first 10 hex digits of the
//! SHA-1 of sluice's state folder, `$XDG_STATE_HOME/sluice` or
//! `~/.local/state/sluice`, resolved. Nothing answering there is no sluice
//! host: the subscription ends saying so, and flume starts none, which is
//! the host's service manager's to start (clients-never-fetch-or-start-a-
//! tool; the-service-manager-starts-each-resident).
//!
//! On the connection flume says `{"hello":"flume/<version>"}`, reads
//! sluice host's answer, asks `{"ask":"subscribe"}`, and sends
//! `{"add":"<folder>"}` and `{"remove":"<folder>"}` as the repositories it
//! follows change, each folder the repository's as the profile's commands
//! run in it. sluice host sends a `document` line for a folder when it is
//! added and each time its overview changes. flume reads of each document
//! only each branch's name, its records' `version`, which moves on every
//! write, its `revision` and whether it is `gone`: a document whose
//! reading differs from the folder's last is a change heard, and the
//! first document of a folder, sent as it is added, is none, since a
//! settle has just read it. An `error` line, or a line flume cannot read,
//! ends the subscription with its words.

use crate::git::Git;
use crate::resident::Subscribes;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;
use tokio::sync::{watch, Notify};
use tokio_util::sync::CancellationToken;

/// The variable sluice's tests name the user's own directory in, which
/// sluice host's socket is in.
pub const USER_SCRATCH: &str = "SLUICE_USER_SCRATCH";

/// How long flume waits for sluice host to answer its hello.
pub const HELLO_WAIT: Duration = Duration::from_secs(10);

/// The socket sluice host answers on for the process whose environment
/// `var` reads, or the words saying why there is none.
pub fn socket_from(var: impl Fn(&str) -> Option<OsString>) -> Result<PathBuf, String> {
    let state = match var("XDG_STATE_HOME").filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(var("HOME").ok_or("HOME is not set")?)
            .join(".local")
            .join("state"),
    }
    .join("sluice");
    let state = state.canonicalize().map_err(|error| {
        format!(
            "sluice's state folder {} is not there ({error}): sluice host has never run as this user",
            state.display()
        )
    })?;
    let user = match var(USER_SCRATCH).filter(|dir| !dir.is_empty()) {
        Some(dir) => PathBuf::from(dir),
        None => {
            let tmp = Path::new("/tmp")
                .canonicalize()
                .map_err(|error| format!("resolving /tmp: {error}"))?;
            // SAFETY: geteuid reads the process's effective user and
            // cannot fail.
            tmp.join(format!("sluice-{}", unsafe { libc::geteuid() }))
        }
    };
    Ok(user.join(format!("host-{}.sock", key(&state))))
}

/// sluice's key of a folder: the first 10 hex digits of the SHA-1 of its
/// path's bytes.
fn key(folder: &Path) -> String {
    use sha1::{Digest, Sha1};
    use std::os::unix::ffi::OsStrExt as _;
    Sha1::digest(folder.as_os_str().as_bytes())
        .iter()
        .take(5)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The folder sluice is asked about for `repository`, the one the
/// profile's commands run in.
fn folder_of(repository: &str) -> String {
    Git::at(repository).path().to_string_lossy().into_owned()
}

/// What flume reads of a folder's overview: each branch's name, records'
/// version, revision and whether it is gone, and whether the folder
/// itself is.
pub fn reading(overview: &Value) -> Vec<(String, Value, Value, bool)> {
    let mut read: Vec<_> = overview
        .get("branches")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|branch| {
            (
                branch["branch"].as_str().unwrap_or_default().to_string(),
                branch["version"].clone(),
                branch["revision"].clone(),
                branch["gone"].as_bool().unwrap_or(false),
            )
        })
        .collect();
    if overview["gone"].as_bool() == Some(true) {
        read.push((String::new(), Value::Null, Value::Null, true));
    }
    read
}

/// The subscription of the host's sluice host, its socket found at each
/// hold, or the one given.
#[derive(Clone, Debug, Default)]
pub struct Subscription {
    socket: Option<PathBuf>,
}

impl Subscription {
    /// The subscription of the sluice host this process's environment
    /// names.
    pub fn of_host() -> Subscription {
        Subscription { socket: None }
    }

    /// The subscription of the sluice host answering on `socket`.
    pub fn at(socket: PathBuf) -> Subscription {
        Subscription {
            socket: Some(socket),
        }
    }

    fn socket(&self) -> Result<PathBuf, String> {
        match &self.socket {
            Some(socket) => Ok(socket.clone()),
            None => socket_from(|name| std::env::var_os(name)),
        }
    }
}

/// One subscription's connection, past its hello and its ask.
struct Held {
    lines: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
    /// Each folder followed, by the repository it is of.
    folders: BTreeMap<String, String>,
    /// Each folder's last reading.
    read: BTreeMap<String, Vec<(String, Value, Value, bool)>>,
}

impl Held {
    async fn send(&mut self, line: &Value) -> Result<(), String> {
        let mut bytes = line.to_string().into_bytes();
        bytes.push(b'\n');
        self.writer
            .write_all(&bytes)
            .await
            .map_err(|error| format!("writing to sluice host: {error}"))
    }

    /// The folders followed brought to those of `repositories`.
    async fn follow(&mut self, repositories: &BTreeSet<String>) -> Result<(), String> {
        let wanted: BTreeMap<String, String> = repositories
            .iter()
            .map(|repository| (folder_of(repository), repository.clone()))
            .collect();
        let gone: Vec<String> = self
            .folders
            .keys()
            .filter(|folder| !wanted.contains_key(*folder))
            .cloned()
            .collect();
        for folder in gone {
            self.send(&serde_json::json!({ "remove": folder })).await?;
            self.folders.remove(&folder);
            self.read.remove(&folder);
        }
        for (folder, repository) in wanted {
            if !self.folders.contains_key(&folder) {
                self.send(&serde_json::json!({ "add": folder })).await?;
                self.folders.insert(folder, repository);
            }
        }
        Ok(())
    }

    /// Whether `line` says a change in a folder followed, or the words
    /// that end the subscription.
    fn heard(&mut self, line: &str) -> Result<bool, String> {
        let sent: Value = serde_json::from_str(line).map_err(|error| {
            format!("sluice host sent a line flume cannot read ({error}): {line}")
        })?;
        if let Some(words) = sent.get("error") {
            return Err(format!("sluice host could not read flume's line: {words}"));
        }
        let Some(document) = sent.get("document") else {
            return Ok(false);
        };
        let Some(folder) = document["folder"].as_str() else {
            return Ok(false);
        };
        if !self.folders.contains_key(folder) {
            return Ok(false);
        }
        let now = reading(&document["overview"]);
        Ok(self
            .read
            .insert(folder.to_string(), now.clone())
            .is_some_and(|before| before != now))
    }
}

/// Connects to `socket` and says hello, then asks a subscription.
async fn connect(socket: &Path) -> Result<Held, String> {
    let stream = UnixStream::connect(socket).await.map_err(|error| {
        format!(
            "sluice host answers on no socket at {} ({error})",
            socket.display()
        )
    })?;
    let (reader, writer) = stream.into_split();
    let mut held = Held {
        lines: BufReader::new(reader).lines(),
        writer,
        folders: BTreeMap::new(),
        read: BTreeMap::new(),
    };
    held.send(&serde_json::json!({ "hello": format!("flume/{}", crate::VERSION) }))
        .await?;
    let named = tokio::time::timeout(HELLO_WAIT, held.lines.next_line())
        .await
        .map_err(|_| {
            format!(
                "sluice host gave no answer to flume's hello within {} seconds",
                HELLO_WAIT.as_secs()
            )
        })?
        .map_err(|error| format!("reading sluice host's hello: {error}"))?
        .ok_or("sluice host closed the connection at its hello")?;
    let named: Value = serde_json::from_str(&named)
        .map_err(|error| format!("sluice host's hello, {named}, is not JSON: {error}"))?;
    if named.get("version").and_then(Value::as_str).is_none() {
        return Err(format!("sluice host's hello names no version: {named}"));
    }
    held.send(&serde_json::json!({ "ask": "subscribe" }))
        .await?;
    Ok(held)
}

impl Subscribes for Subscription {
    async fn hold(
        &self,
        mut follow: watch::Receiver<BTreeSet<String>>,
        heard: Arc<Notify>,
        stop: CancellationToken,
    ) -> String {
        // Nothing is asked of sluice host while there is nothing to follow.
        tokio::select! {
            _ = stop.cancelled() => return "stopped".into(),
            followed = follow.wait_for(|set| !set.is_empty()) => if followed.is_err() {
                return "the resident stopped following".into();
            }
        }
        let socket = match self.socket() {
            Ok(socket) => socket,
            Err(words) => return words,
        };
        let mut held = match connect(&socket).await {
            Ok(held) => held,
            Err(words) => return words,
        };
        loop {
            let wanted = follow.borrow_and_update().clone();
            if let Err(words) = held.follow(&wanted).await {
                return words;
            }
            tokio::select! {
                _ = stop.cancelled() => return "stopped".into(),
                changed = follow.changed() => if changed.is_err() {
                    return "the resident stopped following".into();
                },
                line = held.lines.next_line() => match line {
                    Ok(Some(line)) => match held.heard(&line) {
                        Ok(true) => heard.notify_one(),
                        Ok(false) => {}
                        Err(words) => return words,
                    },
                    Ok(None) => return "sluice host closed the subscription".into(),
                    Err(error) => return format!("reading sluice host: {error}"),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::net::UnixListener;

    /// A folder of the test's own, short enough for a socket's path.
    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("fss-{test}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the test's scratch");
        dir
    }

    fn overview(branches: &[(&str, u64)]) -> Value {
        json!({
            "schema": "sluice/overview@1",
            "branches": branches
                .iter()
                .map(|(branch, version)| json!({"branch": branch, "version": version}))
                .collect::<Vec<_>>(),
        })
    }

    #[test]
    fn the_socket_is_sluices_for_its_state_folder() {
        let dir = scratch("socket");
        let state = dir.join("state");
        std::fs::create_dir_all(state.join("sluice")).unwrap();
        let resolved = state.join("sluice").canonicalize().unwrap();
        let socket = socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(state.clone().into()),
            USER_SCRATCH => Some(dir.clone().into()),
            _ => None,
        })
        .unwrap();
        use sha1::{Digest, Sha1};
        use std::os::unix::ffi::OsStrExt as _;
        let digest = Sha1::digest(resolved.as_os_str().as_bytes());
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(socket, dir.join(format!("host-{}.sock", &hex[..10])));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn no_state_folder_is_no_sluice_host() {
        let dir = scratch("nostate");
        let words = socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(dir.join("none").into()),
            _ => None,
        })
        .unwrap_err();
        assert!(words.contains("sluice host has never run"), "{words}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_reading_holds_each_branchs_version_revision_and_gone() {
        let a = overview(&[("main", 3), ("bolt/a", 9)]);
        let mut b = a.clone();
        b["branches"][1]["candidates"] = json!([{"head": "abc"}]);
        b["room"] = json!({"queue": 4});
        assert_eq!(reading(&a), reading(&b), "nothing flume reads moved");
        let mut c = a.clone();
        c["branches"][0]["version"] = json!(4);
        assert_ne!(reading(&a), reading(&c), "a records' version moved");
        let mut d = a.clone();
        d["branches"][0]["revision"] = json!("def");
        assert_ne!(reading(&a), reading(&d), "a branch's revision moved");
    }

    /// A stand-in sluice host on `socket`: it answers one connection's
    /// hello, records every line after it and sends each line `send`
    /// hands it; its lines received.
    async fn stand_in(
        socket: &Path,
    ) -> (
        tokio::sync::mpsc::UnboundedSender<Value>,
        tokio::sync::mpsc::UnboundedReceiver<Value>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = UnixListener::bind(socket).unwrap_or_else(|error| {
            panic!(
                "no room for the stand-in's socket {}: {error}",
                socket.display()
            )
        });
        let (send, mut to_send) = tokio::sync::mpsc::unbounded_channel::<Value>();
        let (got, received) = tokio::sync::mpsc::unbounded_channel::<Value>();
        let handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (reader, mut writer) = stream.into_split();
            let mut lines = BufReader::new(reader).lines();
            let hello = lines.next_line().await.unwrap().unwrap();
            got.send(serde_json::from_str(&hello).unwrap()).unwrap();
            writer
                .write_all(b"{\"version\":\"0.105.1\",\"pid\":1,\"since\":\"now\"}\n")
                .await
                .unwrap();
            loop {
                tokio::select! {
                    line = lines.next_line() => match line {
                        Ok(Some(line)) => { got.send(serde_json::from_str(&line).unwrap()).unwrap(); }
                        _ => return,
                    },
                    sent = to_send.recv() => match sent {
                        Some(sent) => {
                            let mut bytes = sent.to_string().into_bytes();
                            bytes.push(b'\n');
                            if writer.write_all(&bytes).await.is_err() { return; }
                        }
                        None => return,
                    },
                }
            }
        });
        (send, received, handle)
    }

    async fn next(received: &mut tokio::sync::mpsc::UnboundedReceiver<Value>) -> Value {
        tokio::time::timeout(Duration::from_secs(5), received.recv())
            .await
            .expect("a line within five seconds")
            .expect("the stand-in ended")
    }

    #[tokio::test]
    async fn it_follows_hears_and_ends_over_sluice_hosts_socket() {
        let dir = scratch("hold");
        let socket = dir.join("h.sock");
        let (send, mut received, server) = stand_in(&socket).await;
        let one = dir.join("one");
        let two = dir.join("two");
        for repository in [&one, &two] {
            std::fs::create_dir_all(repository.join(".git")).unwrap();
        }
        let (one, two) = (
            one.to_string_lossy().into_owned(),
            two.to_string_lossy().into_owned(),
        );
        let (follow, following) = watch::channel(BTreeSet::from([one.clone()]));
        let heard = Arc::new(Notify::new());
        let stop = CancellationToken::new();
        let holding = {
            let (heard, stop) = (heard.clone(), stop.clone());
            tokio::spawn(async move { Subscription::at(socket).hold(following, heard, stop).await })
        };
        assert_eq!(
            next(&mut received).await,
            json!({"hello": format!("flume/{}", crate::VERSION)})
        );
        assert_eq!(next(&mut received).await, json!({"ask": "subscribe"}));
        assert_eq!(next(&mut received).await, json!({"add": one}));

        // The document sent as the folder is added is no change.
        send.send(json!({"document": {"folder": one, "overview": overview(&[("main", 1)])}}))
            .unwrap();
        // A document whose reading is unmoved is no change either.
        send.send(json!({"document": {"folder": one, "overview": overview(&[("main", 1)])}}))
            .unwrap();
        // One for a folder not followed is passed over.
        send.send(json!({"document": {"folder": two, "overview": overview(&[("main", 1)])}}))
            .unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(300), heard.notified())
                .await
                .is_err(),
            "a change heard where none came"
        );
        send.send(json!({"document": {"folder": one, "overview": overview(&[("main", 2)])}}))
            .unwrap();
        tokio::time::timeout(Duration::from_secs(5), heard.notified())
            .await
            .expect("the change heard");

        follow.send_replace(BTreeSet::from([two.clone()]));
        assert_eq!(next(&mut received).await, json!({"remove": one}));
        assert_eq!(next(&mut received).await, json!({"add": two}));

        send.send(json!({"error": "no such ask"})).unwrap();
        let words = tokio::time::timeout(Duration::from_secs(5), holding)
            .await
            .expect("the hold ended")
            .unwrap();
        assert!(words.contains("no such ask"), "{words}");
        drop(send);
        let _ = server.await;
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn nothing_is_asked_while_there_is_nothing_to_follow_and_no_host_ends_it() {
        let dir = scratch("none");
        let socket = dir.join("h.sock");
        let (follow, following) = watch::channel(BTreeSet::new());
        let stop = CancellationToken::new();
        let holding = {
            let (stop, socket) = (stop.clone(), socket.clone());
            tokio::spawn(async move {
                Subscription::at(socket)
                    .hold(following, Arc::new(Notify::new()), stop)
                    .await
            })
        };
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(!holding.is_finished(), "it asked with nothing to follow");
        follow.send_replace(BTreeSet::from(["/repository".to_string()]));
        let words = tokio::time::timeout(Duration::from_secs(5), holding)
            .await
            .expect("the hold ended")
            .unwrap();
        assert!(
            words.contains("answers on no socket") && words.contains(&*socket.to_string_lossy()),
            "{words}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
