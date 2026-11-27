//! The reads the sluice profile asks of sluice host on its socket
//! (flume-reads-sluices-records-on-sluice-hosts-socket;
//! residents-ask-each-other-by-socket): one connection for each ask, never
//! a sluice command. flume says `{"hello":"flume/<version>"}`, reads
//! sluice host's answer, asks one read, `overview`, `status` or `inbox`,
//! naming the folder the profile's commands run in, and reads its `output`, whose
//! `stdout` is what the command prints. An `error` line, no answer within
//! [`WITHIN`] or no socket gives its words; flume starts no sluice host,
//! which is the host's service manager's to start
//! (the-service-manager-starts-each-resident).
//!
//! The records of each base are kept in the process with the `version`
//! the overview gave for that base: sluice moves `version` at each write
//! of a branch's records, so while it stands a status read answers from
//! the records kept and asks no `status` (one-read-in-flight).

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

/// How long flume waits for each line sluice host answers.
pub const WITHIN: Duration = Duration::from_secs(60);

/// What sluice host's `output` holds: the command's code, what it printed
/// and what it said on its errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// One read asked of the sluice host answering on `socket`, `ask` its
/// request after the hello; the words saying why it gave no output.
pub fn ask(socket: &Path, ask: &Value) -> Result<Output, String> {
    let stream = UnixStream::connect(socket).map_err(|error| {
        format!(
            "sluice host answers on no socket at {} ({error})",
            socket.display()
        )
    })?;
    stream
        .set_read_timeout(Some(WITHIN))
        .and_then(|()| stream.set_write_timeout(Some(WITHIN)))
        .map_err(|error| format!("bounding the ask of sluice host: {error}"))?;
    let mut writer = stream
        .try_clone()
        .map_err(|error| format!("asking sluice host: {error}"))?;
    let mut lines = BufReader::new(stream).lines();
    let mut next = |what: &str| -> Result<Value, String> {
        let line = lines
            .next()
            .ok_or_else(|| format!("sluice host closed the connection before {what}"))?
            .map_err(|error| format!("reading {what} from sluice host: {error}"))?;
        serde_json::from_str(&line)
            .map_err(|error| format!("sluice host's {what}, {line}, is not JSON: {error}"))
    };
    let mut send = |line: &Value| -> Result<(), String> {
        let mut bytes = line.to_string().into_bytes();
        bytes.push(b'\n');
        writer
            .write_all(&bytes)
            .map_err(|error| format!("writing to sluice host: {error}"))
    };
    send(&json!({ "hello": format!("flume/{}", crate::VERSION) }))?;
    let named = next("its hello")?;
    if named.get("version").and_then(Value::as_str).is_none() {
        return Err(format!("sluice host's hello names no version: {named}"));
    }
    send(ask)?;
    let answer = next("its answer")?;
    if let Some(words) = answer.get("error") {
        let words = words
            .as_str()
            .map_or_else(|| words.to_string(), str::to_string);
        return Err(format!("sluice host refused the ask: {words}"));
    }
    let output = answer
        .get("output")
        .ok_or_else(|| format!("sluice host gave no output: {answer}"))?;
    Ok(Output {
        code: output
            .get("code")
            .and_then(Value::as_i64)
            .and_then(|code| i32::try_from(code).ok())
            .unwrap_or(-1),
        stdout: output
            .get("stdout")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        stderr: output
            .get("stderr")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join("\n"),
    })
}

/// The ask of the inbox of the place at `folder`, as `sluice inbox
/// --json` run there asks it: its worktree the folder itself, confirming
/// nothing, and `agent`, the `SLUICE_AGENT` the command would carry, as
/// its caller where one is set (133; sluice 51).
pub fn inbox(folder: &str, agent: Option<&str>) -> Value {
    let mut ask = json!({ "ask": "inbox", "folder": folder, "worktree": folder, "json": true });
    if let Some(agent) = agent {
        ask["caller"] = json!({ "SLUICE_AGENT": agent });
    }
    ask
}

/// The ask of a folder's overview.
pub fn overview(folder: &str) -> Value {
    json!({ "ask": "overview", "folder": folder, "json": true })
}

/// The ask of every submission of `base`, as `status <base> --all`.
pub fn status(folder: &str, base: &str) -> Value {
    json!({ "ask": "status", "folder": folder, "target": base, "all": true, "json": true })
}

/// The records kept of each base, by its folder and its name: the
/// version the overview gave them, and the records.
type Kept = BTreeMap<(String, String), (Value, Value)>;

/// The records of each base kept in this process.
fn kept() -> &'static Mutex<Kept> {
    static KEPT: OnceLock<Mutex<Kept>> = OnceLock::new();
    KEPT.get_or_init(Default::default)
}

/// The records of `base` in `folder` kept at `version`, where they are.
pub fn kept_at(folder: &str, base: &str, version: &Value) -> Option<Value> {
    if version.is_null() {
        return None;
    }
    let kept = kept()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    kept.get(&(folder.to_string(), base.to_string()))
        .filter(|(at, _)| at == version)
        .map(|(_, records)| records.clone())
}

/// Keeps the records of `base` in `folder` read at `version`, and lets go
/// of the bases of `folder` that `bases` no longer names.
pub fn keep(folder: &str, bases: &[String], base: &str, version: &Value, records: &Value) {
    let mut kept = kept()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    kept.retain(|(at, name), _| at != folder || bases.contains(name));
    if !version.is_null() {
        kept.insert(
            (folder.to_string(), base.to_string()),
            (version.clone(), records.clone()),
        );
    }
}
