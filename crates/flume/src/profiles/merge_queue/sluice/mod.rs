//! The merge-queue profile over sluice (C.6; design 13), every command
//! and field it reads checked against the sluice the tree pins, 0.110.0.
//!
//! submit is `sluice submit <base> --no-wait --json` in the place, with
//! every multiplexer's session stripped from its environment, so sluice
//! records the place's worktree as the submitter, pushes into no pane and
//! answers at once; never `--check` to a submit, never `wt submit` (111,
//! 113, 117, core.6). Before each submit, a place's from its worktree and
//! a base place's by name, the profile reads the sluice it runs by `sluice
//! --version`, in the directory the submit runs in: where that names a
//! release before [`MERGES_BY_ITSELF`], the first whose own host process
//! moves its queues, or names no release, as a build of sluice's main says
//! 0.0.0, it runs no `sluice submit` and answers the submit refused, its
//! words naming what sluice said and the release flume needs (111, 112,
//! 203, C.6's submit and version rows; sluice 135; record 0080). Where sluice finds the submission decided as it takes it in,
//! as on a base that takes pull requests, it answers with the entry's
//! record, exiting as the outcome does, so the outcome is read from that
//! answer as the inbox reads one, never as a refusal, with the key of the
//! answer's `delivery` where it is owed, for confirm once flume has acted
//! on it (101, 112, 133; sluice 51; sluice record 0102). On such a
//! base sluice pushes
//! the place's branch and opens or updates its pull request, and where
//! intake has not taken it in answers `{pull_request, branch, state}`
//! with no entry: the request is held, open and undecided, and an intake
//! on whichever host takes the base's requests in takes it in later; a
//! request answered `closed` was closed without landing (124, C.6). An
//! entry intake took in names its place `pull/<n>` as its `from.worktree`
//! and the branch the request was opened from as its `from.head`, and
//! sluice delivers its outcome to the request, so read-submission reads
//! it, and the inbox only where it is still pending for the place (133).
//! inbox is sluice host's `inbox` read on its socket, as `sluice inbox
//! --json` in the place asks it, starting no sluice command while sluice
//! host answers (flume-reads-sluices-records-on-sluice-hosts-socket), and
//! `sluice inbox --json` in the place where it gives no answer, which
//! starts sluice host on a bare install (S109); it hands over each
//! outcome owed to it again until it is confirmed, never `--confirm`,
//! which would confirm it before flume acted on it, or confirm one the
//! profile then drops (C.6's inbox row at boomdrivedev/blueprints
//! `8b73d11` as `fc4323b` amends it; core.58): each record with its
//! entry's `id` and the key of its `delivery` where it is owed, landed
//! with its commit, failed or in
//! conflict with sluice's `notice` as its words, a conflict's files from
//! the record's `conflict.paths` and its base revision from its
//! `tested.on`, never parsed from the notice; a record with no notice
//! says its state, and a conflict's files and the entries its
//! `conflict.with` names; one found in conflict before any test names the
//! base's current revision (112, 133); and a `refused` record, a landing
//! the forge refused that sluice holds, held with its `notice` whole as
//! its words, or what a record with none says (112, 133; sluice 152). An
//! outcome in a state the profile does not read, or whose outcome it
//! cannot read, is left owed, unconfirmed, and passed over by its branch
//! with words naming it, rather than dropped unsaid (133; core.58). Where
//! sluice could not read a branch's records it answers the others'
//! outcomes with that branch in its `unread`, exiting as a failure: the
//! inbox passes that branch over with its words, whose outcomes sluice
//! leaves pending (133). confirm is `sluice confirm <key> --json` in the
//! place, run once flume has acted on the outcome; exit 0, confirmed now
//! or before, and exit 1, nothing owed under the key, are each
//! confirmed, since sluice hands nothing over under it again, and any
//! other exit sluice unread in its words, the outcome left owed for the
//! next inbox (101, 133; sluice 51). read-submission asks sluice host's
//! socket, never a sluice command, for the repository's `overview` and
//! for `status <base> --all` of each branch the overview names, once for
//! every branch of the repository asked at once, so a status of every
//! place asks them once for each repository (58, C.6); the records of a
//! base are kept in the process with the `version` the overview gave
//! them, and asked again only once it moved, so flume host asks sluice
//! host only for records that moved ([`ask`];
//! flume-reads-sluices-records-on-sluice-hosts-socket); for each branch
//! asked, the newest entry submitted from it, by a
//! worktree on it or from a pull request opened from it, a conflict's
//! base revision its `tested.on` where it holds one; a branch whose
//! status sluice refuses, as it refuses one whose policy it no longer
//! reads, is passed over with sluice's words, and every other branch is
//! read still (99, 101). A `refused` record, a landing the forge refused
//! that sluice holds, reads as held with the record's `refused.words`;
//! one that carries no `refused`, as sluice `44e33d4` prints every record
//! but a waiting submit's report, is held still, with [`HELD_UNSAID`] as
//! its words, never read as not understood, never passed over and never
//! read through `sluice wait` or a submit that waits (C.6's
//! read-submission row at `8b73d11` as `04e5a1a` amends it; core.6,
//! core.13, core.58). A submit's answer is never `refused`, its landing
//! coming after the rounds a submit with `--no-wait` does not wait on;
//! were one so, it is answered submitted, open and undecided. step is
//! nothing, as C.6's step row says: it
//! runs no sluice command and answers that the base's queue moves on its
//! own, whichever way runs it, sluice's host process, a daemon on this
//! host or another, or a forge's jobs, since sluice keeps one contract
//! with flume for every way; a submit starts sluice's host process as it
//! takes the entry in, and the inbox starts it again where work waits
//! and none runs, so the profile runs no `sluice step`, `run`,
//! `daemon` or `wait` (124, 125, C.6; sluice 135; sluice record 0093 as
//! amended). The base lives at `<remote>/<base>` after one fetch where
//! `.config/sluice.toml` lands it on a remote, and at the local base
//! otherwise (9, 126, C.1). A place is made ready by sluice's equipping
//! of it, `sluice equip <base> --at <revision> --json` run in the place,
//! the revision the place's launch read, so a retry equips it as its
//! launch did: sluice reads `[tests]` of the base's policy at that
//! revision and runs in the place what it runs before its own runs, the
//! toolchain, then its runner's install of what the place pins, its check
//! of its own version and its setup, and answers each step with its
//! outcome and words, which [`equipped`] reads as the readiness's steps
//! (11, 12, 20, 79, C.1, C.6). The profile reads no `[tests]` key and no
//! `.prototools` for a place: the readiness, `sluice release` as the
//! teardown and the remote the policy lands the base on are declared for
//! the resolver to hand the place profile (core.3). A base place's
//! branch is opened by `sluice branch create <branch> --from <base>
//! --json` and closed by `sluice branch delete <branch> --into <base>
//! --json`, each run in the repository as read-submission is:
//! open answers the revision sluice made the branch at, and any other
//! exit a refusal in sluice's words; close answers removed on exit 0, gone
//! on exit 1 naming the branch itself, and kept in sluice's words
//! otherwise, an `into` sluice cannot find among it, since the branch
//! stands then (205, 206; sluice 104, 105). A base place's branch is
//! submitted by name, `sluice submit <base> --head <branch> --no-wait
//! --json` run in the repository, answered as any submit; sluice records
//! the branch as the entry's `from.head`, so read-submission reads its
//! outcome, and no inbox runs for it (99, 133, 203; sluice 124). A base
//! place's branch is forecast by the same command with `--check` in
//! place of `--no-wait`, run in the repository, which forms the
//! candidate sluice forms for a head not yet submitted, behind every entry
//! already submitted, and submits nothing; [`forecast_of`] reads its
//! answer (203, 210, C.6; sluice 64, 65).

pub mod ask;
pub mod subscription;

use crate::git::Git;
use crate::shell;
use flume_seams::{
    BaseRevision, BranchClosed, BranchOpened, Confirmed, Forecast, Inbox, LandsOn, MergeQueue,
    Owed, PassedOver, PlaceRef, ReadAcross, ReadyStep, Revision, Stepped, Submission,
    SubmissionOutcome, SubmissionRead, SubmissionState, SubmitHead, Submitted,
};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The profile's name in a repository's binding, `merge_queue` (105).
pub const NAME: &str = "sluice";

/// Where a repository's sluice policy lives in its tree.
pub const POLICY: &str = ".config/sluice.toml";

/// The words of a `refused` record that carries none of the forge's, as
/// sluice `44e33d4` prints every record but a waiting submit's report
/// (99, C.6 at `04e5a1a`; core.58).
pub const HELD_UNSAID: &str =
    "the forge refused its landing; sluice's record gives none of its words";

/// The first sluice release whose own host process moves its queues, so
/// work submitted to it is merged by itself: flume hands no work to an
/// earlier one (sluice 135; sluice record 0093; record 0080).
pub const MERGES_BY_ITSELF: (u64, u64, u64) = (0, 97, 0);

/// The release a `sluice --version` that exited 0 names on `line`, its
/// first line, as sluice wrote it and read part by part: the line, trimmed,
/// is exactly `sluice` and a version of three parts joined by `.`, each of
/// ASCII digits alone; anything else, and 0.0.0, which a build of sluice's
/// main says, names none (C.6; sluice record 0111).
fn release(line: &str) -> Option<(&str, (u64, u64, u64))> {
    let mut words = line.split_whitespace();
    let (Some("sluice"), Some(version), None) = (words.next(), words.next(), words.next()) else {
        return None;
    };
    let part = |part: &str| {
        (!part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
            .then(|| part.parse::<u64>().ok())
            .flatten()
    };
    let mut parts = version.split('.');
    let (Some(major), Some(minor), Some(patch), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    let release = (part(major)?, part(minor)?, part(patch)?);
    (release != (0, 0, 0)).then_some((version, release))
}

/// The variables a multiplexer's session is named by: a submission made
/// with one would be recorded as that session's and its outcome pushed
/// into its pane (C.6, core.6).
fn a_session(name: &str) -> bool {
    name.starts_with("HERDR_")
        || name.starts_with("REX_")
        || name.starts_with("ZELLIJ")
        || name == "TMUX"
        || name == "TMUX_PANE"
}

/// One call of sluice's: the program, the folder it runs in, its
/// arguments, and each variable set over the command's own environment,
/// or left out where none.
#[derive(Clone, Copy, Debug)]
pub struct SluiceCall<'a> {
    pub program: &'a Path,
    pub dir: &'a Path,
    pub args: &'a [&'a str],
    pub env: &'a [(String, Option<String>)],
}

/// How the profile runs a call of sluice's: as a process of its program
/// for flume's binary ([`Process`]), or as a test's recorded answers; its
/// exit code, none where a signal ended it, its output and its errors, or
/// why it could not run.
pub trait SluiceRuns: Send + Sync + std::fmt::Debug {
    fn run(&self, call: &SluiceCall<'_>) -> Result<(Option<i32>, String, String), String>;
}

/// sluice run as a process of its program in the call's folder, asking
/// for no credential, with the call's variables over the command's own
/// environment and no terminal session's (C.6).
#[derive(Clone, Copy, Debug, Default)]
pub struct Process;

impl SluiceRuns for Process {
    fn run(&self, call: &SluiceCall<'_>) -> Result<(Option<i32>, String, String), String> {
        let mut command = Command::new(call.program);
        command
            .args(call.args)
            .current_dir(call.dir)
            .stdin(Stdio::null())
            .env("GIT_TERMINAL_PROMPT", "0");
        for (name, value) in call.env {
            match value {
                Some(value) => command.env(name, value),
                None => command.env_remove(name),
            };
        }
        for (name, _) in std::env::vars_os() {
            if a_session(&name.to_string_lossy()) {
                command.env_remove(&name);
            }
        }
        for (name, _) in call.env {
            if a_session(name) {
                command.env_remove(name);
            }
        }
        let output = command
            .output()
            .map_err(|error| format!("sluice could not be run: {error}"))?;
        Ok((
            output.status.code(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

/// The merge queue through sluice.
#[derive(Clone, Debug)]
pub struct Sluice {
    program: PathBuf,
    env: Vec<(String, Option<String>)>,
    runs: std::sync::Arc<dyn SluiceRuns>,
}

impl Default for Sluice {
    fn default() -> Self {
        Sluice::new()
    }
}

impl Sluice {
    /// sluice as `PATH` finds it.
    pub fn new() -> Sluice {
        Sluice {
            program: "sluice".into(),
            env: vec![],
            runs: std::sync::Arc::new(Process),
        }
    }

    /// Its calls of sluice run by `runs`, in place of a process of its
    /// program.
    pub fn with_runs(mut self, runs: std::sync::Arc<dyn SluiceRuns>) -> Sluice {
        self.runs = runs;
        self
    }

    /// sluice run with `name` set to `value`, or left out where none,
    /// over the command's own environment.
    pub fn with_env(mut self, name: &str, value: Option<&str>) -> Sluice {
        self.env.push((name.to_string(), value.map(str::to_string)));
        self
    }

    /// sluice run in `dir` with `args`: whether it succeeded, its output
    /// and its errors, or why it could not run.
    fn run(&self, dir: &Path, args: &[&str]) -> Result<(bool, String, String), String> {
        let (code, stdout, stderr) = self.exited(dir, args)?;
        Ok((code == Some(0), stdout, stderr))
    }

    /// sluice run in `dir` with `args`: its exit code, none where a
    /// signal ended it, its output and its errors, or why it could not
    /// run.
    fn exited(&self, dir: &Path, args: &[&str]) -> Result<(Option<i32>, String, String), String> {
        self.runs.run(&SluiceCall {
            program: &self.program,
            dir,
            args,
            env: &self.env,
        })
    }

    /// Whether the sluice run in `dir` merges work by itself: where
    /// `sluice --version` names a release from [`MERGES_BY_ITSELF`] on, and
    /// otherwise the words a submit is refused with, what sluice said and
    /// the release flume needs (C.6; sluice 135; record 0080).
    fn merges_by_itself(&self, dir: &Path) -> Result<(), String> {
        let said = match self.exited(dir, &["--version"]) {
            Ok((Some(0), stdout, _)) => {
                let line = stdout.lines().next().unwrap_or_default();
                match release(line) {
                    Some((_, release)) if release >= MERGES_BY_ITSELF => return Ok(()),
                    Some((version, _)) => {
                        format!("sluice {version} is too old to merge this work by itself")
                    }
                    None => format!("sluice --version said \"{line}\", which names no release"),
                }
            }
            Ok((_, stdout, stderr)) => {
                let said = refusal(&stdout, &stderr);
                let said = match said.is_empty() {
                    true => "it said nothing",
                    false => said.as_str(),
                };
                format!("sluice --version gave no release: {said}")
            }
            Err(reason) => format!("sluice --version gave no release: {reason}"),
        };
        let (major, minor, patch) = MERGES_BY_ITSELF;
        Err(format!(
            "{said}; flume needs sluice {major}.{minor}.{patch} or later, the first release whose own process moves its queues"
        ))
    }

    /// A decided record's outcome: landed with its commit, failed with
    /// its words, or in conflict with the files contested, its words and
    /// the base's revision it was tested on, or where it was found in
    /// conflict before any test, the base's current revision where it
    /// lives, else its tracking copy's (112, 133).
    fn outcome(&self, record: &Value, place: &PlaceRef) -> Result<SubmissionOutcome, String> {
        match state(record)? {
            SubmissionState::Landed { commit } => Ok(SubmissionOutcome::Landed { commit }),
            SubmissionState::Failed { words } => Ok(SubmissionOutcome::Failed { words }),
            SubmissionState::Conflict {
                files,
                base: tested,
                words,
            } => {
                let base = match tested {
                    Some(base) => base,
                    None => match self.base_revision(&place.repository, &place.base) {
                        BaseRevision::At { revision } => revision,
                        BaseRevision::Refused { reason } => base_at(&place.repository, &place.base)
                            .ok_or_else(|| {
                                format!("{words}, and the base's revision is unread: {reason}")
                            })?,
                        BaseRevision::Unlanded => base_at(&place.repository, &place.base)
                            .ok_or_else(|| {
                                format!("{words}, and the policy lands no base {}", place.base)
                            })?,
                    },
                };
                Ok(SubmissionOutcome::Conflict { files, base, words })
            }
            other => Err(format!("sluice gave a submission undecided: {other:?}")),
        }
    }

    /// The variable `name` as sluice would see it: this profile's own
    /// setting where it gives one, else the process's.
    fn var(&self, name: &str) -> Option<std::ffi::OsString> {
        match self.env.iter().rev().find(|(set, _)| set == name) {
            Some((_, value)) => value.as_ref().map(Into::into),
            None => std::env::var_os(name),
        }
    }

    /// sluice host's JSON answer to `ask`, a read naming the folder `dir`,
    /// asked on its socket, or its words where it refused or could not be
    /// read (flume-reads-sluices-records-on-sluice-hosts-socket).
    fn host_json(&self, ask: &Value) -> Result<Value, String> {
        let read = ask["ask"].as_str().unwrap_or_default().to_string();
        let output = self.host_output(ask)?;
        let answer: Option<Value> = serde_json::from_str(output.stdout.trim()).ok();
        match answer {
            Some(answer) if output.code == 0 && answer.get("error").is_none() => Ok(answer),
            _ => Err(words(&[&read], &output.stdout, &output.stderr)),
        }
    }

    /// sluice host's output for `ask`, a read asked on its socket, or the
    /// words saying why it gave none
    /// (flume-reads-sluices-records-on-sluice-hosts-socket).
    fn host_output(&self, ask: &Value) -> Result<ask::Output, String> {
        let read = ask["ask"].as_str().unwrap_or_default().to_string();
        let socket = subscription::socket_from(|name| self.var(name))
            .map_err(|reason| format!("sluice {read}: {reason}"))?;
        ask::ask(&socket, ask).map_err(|reason| format!("sluice {read}: {reason}"))
    }

    /// Each branch sluice keeps a queue for in the repository, with the
    /// version of its records, as sluice host's overview gives them.
    fn bases(&self, folder: &str) -> Result<Vec<(String, Value)>, String> {
        let overview = self.host_json(&ask::overview(folder))?;
        Ok(overview
            .get("branches")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|branch| {
                let name = branch.get("branch").and_then(Value::as_str)?;
                Some((name.to_string(), branch["version"].clone()))
            })
            .collect())
    }
}

/// sluice's answer to a forecast of `<branch>` onto `<base>`, `submit
/// <base> --check --head <branch> --json`, exiting `code`, as the seam's
/// forecast: exit 0 with `conflict` null is clear; exit 2 with
/// `conflict` is in conflict, its `paths` the files and each of its
/// `with[]` an entry ahead it meets, `#<id>` with the branch it came
/// from, none where it meets the base itself; and anything else could not
/// be read, in sluice's words. The head and the base's revision are the
/// answer's `head` and `onto`, and the words sluice's text form, under
/// which the two lines a conflicting submission's words carry name the
/// files contested and the base's revision, so a fix reads either alike
/// (112, 210, C.6; sluice 64, 65).
fn forecast_of(
    code: Option<i32>,
    stdout: &str,
    stderr: &str,
    args: &[&str],
    base: &str,
) -> Forecast {
    let unread = || Forecast::CouldNotRead {
        reason: words(args, stdout, stderr),
    };
    let Some(answer) = serde_json::from_str::<Value>(stdout.trim())
        .ok()
        .filter(|answer| answer.get("error").is_none())
    else {
        return unread();
    };
    let (Some(head), Some(onto)) = (text(&answer, "head"), text(&answer, "onto")) else {
        return unread();
    };
    let (head, onto) = (Revision::new(head), Revision::new(onto));
    match (code, answer.get("conflict")) {
        (Some(0), Some(Value::Null)) => Forecast::Clear { head, onto },
        (Some(2), Some(conflict @ Value::Object(_))) => {
            let files: Vec<String> = conflict
                .get("paths")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect();
            let with: Vec<String> = conflict
                .get("with")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|entry| {
                    let named = entry
                        .get("from")
                        .and_then(|from| text(from, "head").or_else(|| text(from, "worktree")));
                    let id = id(entry).unwrap_or_default();
                    match named {
                        Some(named) => format!("#{id} ({named})"),
                        None => format!("#{id}"),
                    }
                })
                .collect();
            let met = if with.is_empty() {
                base.to_string()
            } else {
                with.join(", ")
            };
            let contested = files.join(", ");
            let words = format!(
                "would conflict: {contested} with {met}\n\nThe files contested: {contested}.\nThe base's revision the merge queue tested against: {onto}."
            );
            Forecast::Conflict {
                head,
                onto,
                files,
                with,
                words,
            }
        }
        _ => unread(),
    }
}

/// The branch's newest submission as sluice records it, or none.
fn newest_read(branch: &str, newest: Option<Value>) -> SubmissionRead {
    let Some(record) = newest else {
        return SubmissionRead::None;
    };
    match (id(&record), state(&record)) {
        (Some(id), Ok(state)) => SubmissionRead::Found {
            submission: Submission {
                id,
                branch: branch.into(),
                target: text(&record, "branch").unwrap_or_default(),
                state,
                request: request(&record),
            },
        },
        (None, _) => SubmissionRead::CouldNotRead {
            reason: format!("sluice gave a submission with no id: {record}"),
        },
        (_, Err(reason)) => SubmissionRead::CouldNotRead { reason },
    }
}

/// sluice's own words for a refusal, naming the command refused.
fn words(args: &[&str], stdout: &str, stderr: &str) -> String {
    format!(
        "sluice {}: {}",
        args.first().copied().unwrap_or_default(),
        refusal(stdout, stderr)
    )
}

/// What sluice said of a refusal: its JSON error's message where it gave
/// one, its errors otherwise, and what it printed where it gave neither.
fn refusal(stdout: &str, stderr: &str) -> String {
    let message = serde_json::from_str::<Value>(stdout.trim())
        .ok()
        .and_then(|answer| {
            answer
                .get("error")?
                .get("message")?
                .as_str()
                .map(str::to_string)
        });
    message.unwrap_or_else(|| {
        let stderr = stderr.trim();
        if stderr.is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.to_string()
        }
    })
}

fn text(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

/// The key of a decided record's `delivery` while its reader has not
/// confirmed it, `owed` or `expired`; none where sluice keeps no delivery
/// for it, as for a submission by name, or it is done, as a pull
/// request's post confirms it (sluice 51; sluice record 0102).
fn owed(record: &Value) -> Option<String> {
    let delivery = record.get("delivery")?;
    match text(delivery, "state").as_deref() {
        Some("owed" | "expired") => text(delivery, "key"),
        _ => None,
    }
}

/// A record's id as flume names a submission.
fn id(record: &Value) -> Option<String> {
    match record.get("id")? {
        Value::Number(number) => Some(number.to_string()),
        Value::String(id) => Some(id.clone()),
        _ => None,
    }
}

/// The pull request an answer or a record names: a submit's
/// `pull_request`, or an entry's submitter `pull/<n>`, as sluice names
/// the place of an entry intake took in from a request.
fn request(record: &Value) -> Option<String> {
    if let Some(number) = record.get("pull_request").and_then(Value::as_u64) {
        return Some(number.to_string());
    }
    let from = record.get("from").and_then(|from| text(from, "worktree"))?;
    let number = from.strip_prefix("pull/")?;
    number.parse::<u64>().is_ok().then(|| number.to_string())
}

/// Whether a record was submitted from `branch`: by a worktree on it, or
/// by intake from a pull request opened from it, which sluice records as
/// `from.head` (99).
fn submitted_from(record: &Value, branch: &str) -> bool {
    let from = record.get("from");
    let by = |key| from.and_then(|from| text(from, key));
    by("worktree").as_deref() == Some(branch) || by("head").as_deref() == Some(branch)
}

/// Each branch an answer of sluice's names in its `unread`, as `sluice
/// inbox --json` names a branch whose records it could not read, with
/// sluice's words.
fn unread(answer: &Value) -> Vec<PassedOver> {
    answer
        .get("unread")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|unread| {
            Some(PassedOver {
                branch: text(unread, "branch")?,
                words: format!("sluice inbox: {}", text(unread, "error")?),
            })
        })
        .collect()
}

/// The files a conflict record contests.
fn contested(record: &Value) -> Vec<String> {
    record
        .get("conflict")
        .and_then(|conflict| conflict.get("paths"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// The entries a conflict record conflicts with, each by its id and
/// what submitted it, as sluice's forecast names them.
fn contested_with(record: &Value) -> Vec<String> {
    record
        .get("conflict")
        .and_then(|conflict| conflict.get("with"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let from = entry.get("from");
            let by = |key| from.and_then(|from| text(from, key)).unwrap_or_default();
            Some(format!(
                "#{} ({}, {})",
                id(entry)?,
                by("agent"),
                by("worktree")
            ))
        })
        .collect()
}

/// A decided record's words: sluice's notice, or where a record holds
/// none, its state and the checks that failed, or the files contested
/// and the entries it conflicts with.
fn said(record: &Value) -> String {
    text(record, "notice").unwrap_or_else(|| {
        let state = text(record, "state").unwrap_or_default();
        let named = match state.as_str() {
            "conflict" => {
                let with = contested_with(record);
                let files = contested(record).join(", ");
                match with.is_empty() {
                    true => files,
                    false => format!("{files} with {}", with.join(", ")),
                }
            }
            _ => record
                .get("failed")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|check| text(check, "task"))
                .collect::<Vec<String>>()
                .join(", "),
        };
        let branch = text(record, "branch").unwrap_or_default();
        match named.is_empty() {
            true => format!("{state} on {branch}"),
            false => format!("{state} on {branch}: {named}"),
        }
    })
}

/// A record's state as the seam names it, or the state sluice gave that
/// the seam has no name for.
fn state(record: &Value) -> Result<SubmissionState, String> {
    let name = text(record, "state").unwrap_or_default();
    Ok(match name.as_str() {
        "queued" | "waiting" => SubmissionState::Waiting,
        // Tested and passed, or landing: under the merge queue's hand
        // still, undecided until it lands (sluice 0.110.0's states).
        "testing" | "passed" | "landing" => SubmissionState::UnderTest,
        "landed" => SubmissionState::Landed {
            commit: Revision::new(text(record, "landed_as").ok_or("a landing names no commit")?),
        },
        "failed" => SubmissionState::Failed {
            words: said(record),
        },
        "conflict" => SubmissionState::Conflict {
            files: contested(record),
            base: record
                .get("tested")
                .and_then(|tested| text(tested, "on"))
                .map(Revision::new),
            words: said(record),
        },
        "withdrawn" => SubmissionState::Withdrawn,
        // 152: held, the forge having refused its landing.
        "refused" => SubmissionState::Held {
            words: record
                .get("refused")
                .and_then(|refused| text(refused, "words"))
                .unwrap_or_else(|| HELD_UNSAID.into()),
        },
        other => return Err(format!("sluice gave a submission in the state {other:?}")),
    })
}

/// A branch's settings in the policy: its own table, or the first pattern
/// with one `*` that names it.
fn branch_table<'a>(policy: &'a toml::Table, base: &str) -> Option<&'a toml::Table> {
    let branches = policy.get("branches")?.as_table()?;
    if let Some(table) = branches.get(base).and_then(toml::Value::as_table) {
        return Some(table);
    }
    branches.iter().find_map(|(pattern, table)| {
        let (before, after) = pattern.split_once('*')?;
        (base.len() >= before.len() + after.len()
            && base.starts_with(before)
            && base.ends_with(after))
        .then(|| table.as_table())
        .flatten()
    })
}

/// The policy at the base's revision: the local base's, else a remote's
/// tracking copy's; none where neither holds one.
fn policy(repository: &str, base: &str) -> Result<Option<toml::Table>, String> {
    let git = Git::at(repository);
    let mut revisions = vec![format!("refs/heads/{base}")];
    revisions.extend(
        git.remotes()?
            .into_iter()
            .map(|remote| format!("refs/remotes/{remote}/{base}")),
    );
    for revision in revisions {
        if git.commit(&revision).is_err() {
            continue;
        }
        if let Some(text) = git.file(&revision, POLICY)? {
            return text
                .parse::<toml::Table>()
                .map(Some)
                .map_err(|error| format!("{POLICY} at {revision}: {error}"));
        }
    }
    Ok(None)
}

/// Where the policy lands a base (126, C.6).
#[derive(Clone, Debug, PartialEq, Eq)]
enum Lands {
    /// On the remote the base's table names.
    On(String),
    /// On the local branch, the base's table naming no remote.
    Locally,
    /// Nowhere: the policy names no table for the base, or no revision of
    /// it holds a policy, so sluice lands no such base.
    Not,
}

/// Where the policy at the base's revision lands `base`.
fn lands(repository: &str, base: &str) -> Result<Lands, String> {
    let policy = policy(repository, base)?;
    let Some(table) = policy
        .as_ref()
        .and_then(|policy| branch_table(policy, base))
    else {
        return Ok(Lands::Not);
    };
    Ok(match table.get("remote").and_then(toml::Value::as_str) {
        Some(remote) => Lands::On(remote.to_string()),
        None => Lands::Locally,
    })
}

/// The remote the policy lands `base` on, none where it lands it locally
/// or not at all.
pub fn base_remote(repository: &str, base: &str) -> Result<Option<String>, String> {
    Ok(match lands(repository, base)? {
        Lands::On(remote) => Some(remote),
        Lands::Locally | Lands::Not => None,
    })
}

/// The step a place's readiness names where sluice's equip answered no
/// step it could read.
pub const EQUIP: &str = "sluice equip";

/// What sluice says of a place, for the resolver to hand the place
/// profile, which no profile calls (core.2): its readiness, sluice's
/// equipping of it, whose answer [`equipped`] reads; `sluice release`, its
/// teardown; and the remote the base lands on (C.1, C.6, core.3).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ForPlace {
    /// The readiness, as a shell line: `sluice equip <base> --at
    /// <revision> --json`, the revision the place's launch read; none
    /// where no revision was given, as for a teardown.
    pub equip: Option<String>,
    /// The teardown, as a shell line.
    pub teardown: String,
    pub base_remote: Option<String>,
}

/// What sluice says of a place on `base` in `repository`, its readiness
/// asked at `revision`, the revision its launch read, where one is given
/// (12, 20). Only where the base lives is read of the policy here: what
/// equips the place is sluice's to read, at that revision.
pub fn for_place(
    repository: &str,
    base: &str,
    revision: Option<&Revision>,
) -> Result<ForPlace, String> {
    Ok(ForPlace {
        equip: revision
            .map(|revision| shell::words(["sluice", "equip", base, "--at", &revision.0, "--json"])),
        teardown: "sluice release".into(),
        base_remote: base_remote(repository, base)?,
    })
}

/// The readiness's steps from what `sluice equip --json` printed, whether
/// it passed, on its output and on its errors: each of its `steps[]`, in
/// order, with its outcome and its words cut as any step's are. Where it
/// answered no step it could read, a refusal, an answer that is not its
/// form or a sluice that could not run, they are one failed step,
/// [`EQUIP`], with sluice's message, else its errors, else what it
/// printed; where it failed with every step it answered passed, that step
/// follows them (12, 79, C.6).
pub fn equipped(passed: bool, printed: &str, errors: &str) -> Vec<ReadyStep> {
    let answer = serde_json::from_str::<Value>(printed.trim()).ok();
    let steps = answer
        .as_ref()
        .filter(|answer| answer.get("error").is_none())
        .and_then(|answer| answer.get("steps")?.as_array())
        .and_then(|steps| {
            steps
                .iter()
                .map(|step| {
                    Some(ReadyStep {
                        step: text(step, "step")?,
                        passed: step.get("passed")?.as_bool()?,
                        words: shell::tail(step.get("words")?.as_str()?),
                    })
                })
                .collect::<Option<Vec<ReadyStep>>>()
        });
    let refused = || ReadyStep {
        step: EQUIP.into(),
        passed: false,
        words: shell::tail(&refusal(printed, errors)),
    };
    match steps {
        Some(steps) if passed || steps.iter().any(|step| !step.passed) => steps,
        Some(mut steps) => {
            steps.push(refused());
            steps
        }
        None => vec![refused()],
    }
}

/// The base's revision where its tracking copy or the local branch holds
/// it, with no fetch.
fn base_at(repository: &str, base: &str) -> Option<Revision> {
    let git = Git::at(repository);
    let at = match base_remote(repository, base).ok()? {
        Some(remote) => format!("refs/remotes/{remote}/{base}"),
        None => format!("refs/heads/{base}"),
    };
    git.commit(&at).ok().map(Revision::new)
}

impl MergeQueue for Sluice {
    fn submit(&self, place: &PlaceRef, head: &SubmitHead) -> Submitted {
        // A base place's branch is submitted by name from the repository,
        // which holds no worktree of it, and every other place from its
        // worktree (203; sluice 124).
        let git = Git::at(&place.repository);
        let (dir, args) = match head {
            SubmitHead::Branch { branch, .. } => (
                git.path(),
                vec![
                    "submit",
                    place.base.as_str(),
                    "--head",
                    branch.as_str(),
                    "--no-wait",
                    "--json",
                ],
            ),
            SubmitHead::Worktree { .. } => (
                Path::new(&place.location),
                vec!["submit", place.base.as_str(), "--no-wait", "--json"],
            ),
        };
        // No work is handed to a sluice too old to merge it by itself, its
        // version read where the submit would run (C.6; record 0080).
        if let Err(reason) = self.merges_by_itself(dir) {
            return Submitted::Refused { reason };
        }
        let (success, stdout, stderr) = match self.run(dir, &args) {
            Ok(output) => output,
            Err(reason) => return Submitted::Refused { reason },
        };
        // A submission sluice found decided as it took it in is answered
        // with its record, exiting as the outcome does, and confirmed for
        // the place below.
        let entry = serde_json::from_str::<Value>(stdout.trim())
            .ok()
            .filter(|entry| entry.get("error").is_none())
            .filter(|entry| success || entry.get("state").is_some());
        let Some(entry) = entry else {
            return Submitted::Refused {
                reason: words(&args, &stdout, &stderr),
            };
        };
        // On a base that takes pull requests, sluice answers the request
        // it opened or updated and no entry where intake has not taken it
        // in, which a later intake does; and a request closed without
        // landing, exiting as a failure does (124, C.6).
        let Some(id) = id(&entry) else {
            return match (request(&entry), success) {
                (Some(request), true) => Submitted::Requested { request },
                (Some(request), false) => Submitted::Refused {
                    reason: format!(
                        "sluice submit: pull request #{request} is {}",
                        text(&entry, "state").unwrap_or_default()
                    ),
                },
                (None, _) => Submitted::Refused {
                    reason: format!("sluice submit answered no entry: {entry}"),
                },
            };
        };
        let state = match entry.get("state") {
            Some(_) => match state(&entry) {
                Ok(state) => state,
                Err(reason) => return Submitted::Refused { reason },
            },
            None => SubmissionState::Waiting,
        };
        let submission = Submission {
            id,
            branch: place.branch.clone(),
            target: place.base.clone(),
            state,
            request: request(&entry),
        };
        if !matches!(
            submission.state,
            SubmissionState::Landed { .. }
                | SubmissionState::Failed { .. }
                | SubmissionState::Conflict { .. }
        ) {
            return Submitted::Submitted { submission };
        }
        // sluice hands a decided answer over by the submit and keeps it
        // owed until its reader confirms it, which flume does by its key
        // once it has acted on it; until then an inbox may hand it over
        // too (112, 133; sluice 51; sluice record 0102).
        match self.outcome(&entry, place) {
            Ok(outcome) => Submitted::Decided {
                submission,
                outcome,
                key: owed(&entry),
            },
            Err(reason) => Submitted::Refused { reason },
        }
    }

    fn inbox(&self, place: &PlaceRef) -> Inbox {
        // Each outcome owed is handed over again until flume confirms it
        // once it has acted on it, so one a settle could not act on comes
        // to the next; never `--confirm`, which would confirm on receipt
        // one the profile then drops (124, 133; sluice 51; sluice record
        // 0102; core.58). It is asked of sluice host on its socket, as
        // `sluice inbox --json` in the place asks it, starting no sluice
        // command while sluice host answers
        // (flume-reads-sluices-records-on-sluice-hosts-socket;
        // residents-ask-each-other-by-socket). Where it gives no answer,
        // `sluice inbox --json` runs in the place, which starts sluice host
        // on a bare install, as sluice's commands do, and refuses where a
        // service manager owns it (S109; sluice 135;
        // the-service-manager-starts-each-resident).
        let args = ["inbox", "--json"];
        let agent = self.var("SLUICE_AGENT");
        let ask = ask::inbox(&place.location, agent.as_deref().and_then(|a| a.to_str()));
        let read = match self.host_output(&ask) {
            Ok(output) => Ok((output.code == 0, output.stdout, output.stderr)),
            Err(_) => self.run(Path::new(&place.location), &args),
        };
        let (success, stdout, stderr) = match read {
            Ok(output) => output,
            Err(reason) => return Inbox::CouldNotRead { reason },
        };
        let answer = serde_json::from_str::<Value>(stdout.trim()).ok();
        let records = answer
            .as_ref()
            .and_then(|answer| answer.get("outcomes").and_then(Value::as_array).cloned());
        let mut passed_over = answer.as_ref().map(unread).unwrap_or_default();
        // sluice exits as a failure where it passed a branch over, having
        // answered every other branch's outcomes.
        let Some(records) =
            records.filter(|records| success || !records.is_empty() || !passed_over.is_empty())
        else {
            return Inbox::CouldNotRead {
                reason: words(&args, &stdout, &stderr),
            };
        };
        let mut outcomes = vec![];
        for record in &records {
            let reading = match text(record, "state").as_deref() {
                Some("landed" | "failed" | "conflict") => self.outcome(record, place),
                // 152: a landing the forge refused, held, with the notice
                // whole as its words (C.6's inbox row at `fc4323b`).
                Some("refused") => Ok(SubmissionOutcome::Held {
                    words: said(record),
                }),
                state => Err(format!(
                    "sluice gave an outcome in the state {:?}",
                    state.unwrap_or_default()
                )),
            };
            let handed = match id(record) {
                Some(submission) => reading.map(|outcome| Owed {
                    submission,
                    outcome,
                    key: owed(record),
                }),
                None => Err(format!("sluice gave an outcome with no id: {record}")),
            };
            match handed {
                Ok(handed) => outcomes.push(handed),
                // Left owed, unconfirmed, and passed over by its branch, so
                // status names it and a later inbox hands it over again
                // (133; core.58).
                Err(reason) => passed_over.push(PassedOver {
                    branch: text(record, "branch").unwrap_or_default(),
                    words: match id(record) {
                        Some(id) => format!("#{id}: {reason}"),
                        None => reason,
                    },
                }),
            }
        }
        Inbox::Outcomes {
            outcomes,
            passed_over,
        }
    }

    fn confirm(&self, place: &PlaceRef, key: &str) -> Confirmed {
        let args = ["confirm", key, "--json"];
        match self.exited(Path::new(&place.location), &args) {
            Ok((Some(0 | 1), _, _)) => Confirmed::Confirmed,
            Ok((_, stdout, stderr)) => Confirmed::CouldNotRead {
                reason: words(&args, &stdout, &stderr),
            },
            Err(reason) => Confirmed::CouldNotRead { reason },
        }
    }

    fn read_submissions(&self, repository: &str, branches: &[&str]) -> Vec<ReadAcross> {
        let folder = Git::at(repository).path().to_string_lossy().into_owned();
        let bases = match self.bases(&folder) {
            Ok(bases) => bases,
            Err(reason) => {
                return branches
                    .iter()
                    .map(|_| {
                        SubmissionRead::CouldNotRead {
                            reason: reason.clone(),
                        }
                        .into()
                    })
                    .collect()
            }
        };
        let names: Vec<String> = bases.iter().map(|(name, _)| name.clone()).collect();
        let mut newest: Vec<Option<(u64, Value)>> = vec![None; branches.len()];
        let mut passed_over = vec![];
        for (base, version) in bases {
            let status = match ask::kept_at(&folder, &base, &version) {
                Some(kept) => kept,
                None => match self.host_json(&ask::status(&folder, &base)) {
                    Ok(status) => {
                        ask::keep(&folder, &names, &base, &version, &status);
                        status
                    }
                    Err(words) => {
                        passed_over.push(PassedOver {
                            branch: base,
                            words,
                        });
                        continue;
                    }
                },
            };
            for record in status
                .get("submissions")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let number = record.get("id").and_then(Value::as_u64).unwrap_or_default();
                for (branch, newest) in branches.iter().zip(newest.iter_mut()) {
                    if submitted_from(record, branch)
                        && newest.as_ref().is_none_or(|(id, _)| number > *id)
                    {
                        *newest = Some((number, record.clone()));
                    }
                }
            }
        }
        branches
            .iter()
            .zip(newest)
            .map(|(branch, newest)| ReadAcross {
                read: newest_read(branch, newest.map(|(_, record)| record)),
                passed_over: passed_over.clone(),
            })
            .collect()
    }

    /// Nothing: sluice's queue moves on its own once submitted, whichever
    /// way runs it, and the settles' reads start sluice's host process
    /// where work waits and none runs, so no sluice command is run (124,
    /// 125, C.6; sluice 135).
    fn step(&self, _repository: &str, _base: &str) -> Stepped {
        Stepped::MovesOnItsOwn
    }

    /// The base on the remote its table names after one fetch, or the
    /// local base where its table names none; a base the policy names no
    /// table for is one sluice lands not, which the core reads at origin
    /// (9, 126, C.1, C.6).
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        let git = Git::at(repository);
        let at = match lands(repository, base) {
            Ok(Lands::On(remote)) => match git.fetch_branch(&remote, base) {
                Ok(tracking) => tracking,
                Err(reason) => return BaseRevision::Refused { reason },
            },
            Ok(Lands::Locally) => format!("refs/heads/{base}"),
            Ok(Lands::Not) => return BaseRevision::Unlanded,
            Err(reason) => return BaseRevision::Refused { reason },
        };
        match git.commit(&at) {
            Ok(revision) => BaseRevision::At {
                revision: Revision::new(revision),
            },
            Err(reason) => BaseRevision::Refused { reason },
        }
    }

    /// Whether the policy, as the repository holds `base` and each
    /// remote's copy of it, names a table for `base`: read with no fetch
    /// and nothing run, so it moves nothing (126; core.15).
    fn lands_on(&self, repository: &str, base: &str) -> LandsOn {
        match policy(repository, base) {
            Ok(None) => LandsOn::Not {
                reason: format!("no {POLICY} at {base} or a remote's {base}"),
            },
            Ok(Some(policy)) if branch_table(&policy, base).is_none() => LandsOn::Not {
                reason: format!("{POLICY} names no table for {base} under [branches]"),
            },
            Ok(Some(_)) => LandsOn::Lands,
            Err(reason) => LandsOn::CouldNotRead { reason },
        }
    }

    /// `sluice branch create <branch> --from <base> --json` in the
    /// repository: the revision sluice made the branch at, on the remote
    /// the policy names or locally; any other answer, a policy that names
    /// no table for the branch, a branch present or a base sluice cannot
    /// find among it, a refusal in sluice's words (205; sluice 104).
    fn open(&self, repository: &str, branch: &str, base: &str) -> BranchOpened {
        let args = ["branch", "create", branch, "--from", base, "--json"];
        let (code, stdout, stderr) = match self.exited(Git::at(repository).path(), &args) {
            Ok(output) => output,
            Err(reason) => return BranchOpened::Refused { reason },
        };
        let made = serde_json::from_str::<Value>(stdout.trim())
            .ok()
            .filter(|answer| code == Some(0) && answer.get("error").is_none())
            .and_then(|answer| text(&answer, "created"));
        match made {
            Some(revision) => BranchOpened::Made {
                revision: Revision::new(revision),
            },
            None => BranchOpened::Refused {
                reason: branch_words("create", &stdout, &stderr),
            },
        }
    }

    /// `sluice branch delete <branch> --into <into> --json` in the
    /// repository: removed on exit 0; gone on exit 1 where sluice says it
    /// finds no such branch; kept in sluice's words otherwise, a
    /// submission still queued on it, work of it `into` lacks, a step
    /// holding it or an `into` sluice cannot find among it, and a sluice
    /// that could not run, since the branch stands then (206; sluice 105).
    fn close(&self, repository: &str, branch: &str, into: &str) -> BranchClosed {
        let args = ["branch", "delete", branch, "--into", into, "--json"];
        let (code, stdout, stderr) = match self.exited(Git::at(repository).path(), &args) {
            Ok(output) => output,
            Err(reason) => return BranchClosed::Kept { reason },
        };
        match code {
            Some(0) => BranchClosed::Removed,
            Some(1) if gone(branch, &stdout, &stderr) => BranchClosed::Gone,
            _ => BranchClosed::Kept {
                reason: branch_words("delete", &stdout, &stderr),
            },
        }
    }

    fn forecast(&self, repository: &str, branch: &str, base: &str) -> Forecast {
        let git = Git::at(repository);
        let args = ["submit", base, "--check", "--head", branch, "--json"];
        match self.exited(git.path(), &args) {
            Ok((code, stdout, stderr)) => forecast_of(code, &stdout, &stderr, &args, base),
            Err(reason) => Forecast::CouldNotRead { reason },
        }
    }
}

/// sluice's words for a refusal of `sluice branch <command>`.
fn branch_words(command: &str, stdout: &str, stderr: &str) -> String {
    format!("sluice branch {command}: {}", refusal(stdout, stderr))
}

/// Whether `sluice branch delete` said it finds no `branch`: it exits 1
/// both for the branch gone and for an `into` it cannot find, and says
/// `no branch <name>` naming which, so only its words naming the branch
/// itself read gone (sluice 105).
fn gone(branch: &str, stdout: &str, stderr: &str) -> bool {
    refusal(stdout, stderr) == format!("no branch {branch}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_in_flight_is_read_undecided_in_each_state_sluice_names() {
        for (name, read) in [
            ("queued", SubmissionState::Waiting),
            ("waiting", SubmissionState::Waiting),
            ("testing", SubmissionState::UnderTest),
            ("passed", SubmissionState::UnderTest),
            ("landing", SubmissionState::UnderTest),
        ] {
            assert_eq!(
                state(&serde_json::json!({ "state": name })),
                Ok(read),
                "{name}"
            );
        }
    }

    /// sluice's recorded answers: `--version` with the line `says`,
    /// nothing where it is empty, exiting `code`; a submit with the entry
    /// 7; anything else refused, exiting 1; each call but `--version`
    /// logged as its words.
    #[derive(Debug, Default)]
    struct Answers {
        says: String,
        code: i32,
        ran: std::sync::Mutex<Vec<String>>,
    }

    impl SluiceRuns for Answers {
        fn run(&self, call: &SluiceCall<'_>) -> Result<(Option<i32>, String, String), String> {
            if call.args.first() == Some(&"--version") {
                let line = match self.says.is_empty() {
                    true => String::new(),
                    false => format!("{}\n", self.says),
                };
                return Ok((Some(self.code), line, String::new()));
            }
            self.ran.lock().expect("the log").push(call.args.join(" "));
            if call.args.first() == Some(&"submit") {
                return Ok((
                    Some(0),
                    "{\"id\":7,\"branch\":\"main\",\"ahead\":[]}\n".into(),
                    String::new(),
                ));
            }
            Ok((Some(1), String::new(), String::new()))
        }
    }

    #[test]
    fn a_step_runs_no_sluice_and_says_the_queue_moves_on_its_own() {
        // 125, C.6: step is nothing, and no call of sluice's is made.
        let (seam, ran) = stand_in("sluice 0.112.0", 0);
        assert_eq!(seam.step("/r", "main"), Stepped::MovesOnItsOwn);
        assert!(ran_lines(&ran).is_empty(), "a sluice command was run");
    }

    /// A scratch folder of the test's own, named for it and the process.
    fn scratch(test: &str) -> PathBuf {
        std::env::temp_dir().join(format!("fsl-{test}-{}", std::process::id()))
    }

    /// The seam over sluice's recorded answers, answering `--version`
    /// with the line `says` exiting `code`, and those answers.
    fn stand_in(says: &str, code: i32) -> (Sluice, std::sync::Arc<Answers>) {
        let answers = std::sync::Arc::new(Answers {
            says: says.into(),
            code,
            ..Answers::default()
        });
        let seam = Sluice::new().with_runs(answers.clone());
        (seam, answers)
    }

    /// The place `fix/one` onto main, its repository and its worktree
    /// `scratch`.
    fn a_place(scratch: &Path) -> PlaceRef {
        let at = scratch.to_string_lossy().into_owned();
        PlaceRef {
            repository: at.clone(),
            location: at,
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    /// The commands sluice was asked but `--version`, in order.
    fn ran_lines(ran: &Answers) -> Vec<String> {
        ran.ran.lock().expect("the log").clone()
    }

    /// A place's head from its worktree.
    fn from_its_worktree() -> SubmitHead {
        SubmitHead::Worktree {
            head: Revision::new("aaa111"),
        }
    }

    #[test]
    fn a_sluice_before_its_host_process_is_handed_no_work() {
        // C.6; sluice 135; record 0080: a sluice before 0.97.0 has no host
        // process to move its queues, so neither a place's submit nor a
        // base place's by name is handed to it.
        let scratch = scratch("too-old");
        let (seam, ran) = stand_in("sluice 0.95.1", 0);
        let place = a_place(&scratch);
        let by_name = SubmitHead::Branch {
            branch: "train/a".into(),
            revision: Revision::new("aaa111"),
        };
        for head in [from_its_worktree(), by_name] {
            assert_eq!(
                seam.submit(&place, &head),
                Submitted::Refused {
                    reason: "sluice 0.95.1 is too old to merge this work by itself; flume needs sluice 0.97.0 or later, the first release whose own process moves its queues".into(),
                },
                "{head:?}"
            );
        }
        let ran_lines = ran_lines(&ran);
        assert!(
            !ran_lines.iter().any(|line| line.starts_with("submit")),
            "sluice 0.95.1 was handed the work: {ran_lines:?}"
        );
        let _ = std::fs::remove_dir_all(scratch);
    }

    #[test]
    fn a_sluice_that_names_no_release_is_handed_no_work() {
        // C.6; sluice record 0111: a build of sluice's main says 0.0.0, and
        // a pre-release, a version alone or nothing names no release.
        let scratch = scratch("no-release");
        let says = [
            ("sluice 0.0.0", 0, "sluice 0.0.0"),
            ("sluice 0.97.0-rc.1", 0, "sluice 0.97.0-rc.1"),
            ("sluice", 0, "said \"sluice\""),
            ("", 64, "it said nothing"),
        ];
        for (n, (line, code, named)) in says.into_iter().enumerate() {
            let at = scratch.join(n.to_string());
            let (seam, ran) = stand_in(line, code);
            let Submitted::Refused { reason } = seam.submit(&a_place(&at), &from_its_worktree())
            else {
                panic!("{line:?} exiting {code} was handed the work");
            };
            assert!(
                reason.contains(named) && reason.contains("0.97.0"),
                "{line:?}: {reason}"
            );
            let ran_lines = ran_lines(&ran);
            assert!(
                !ran_lines.iter().any(|line| line.starts_with("submit")),
                "{line:?} was handed the work: {ran_lines:?}"
            );
        }
        let _ = std::fs::remove_dir_all(scratch);
    }

    #[test]
    fn a_sluice_with_its_host_process_is_handed_the_work() {
        // 111, C.6: a release is compared part by part as numbers, so
        // 0.100.0 and 1.0.0 are each later than 0.97.0.
        let scratch = scratch("handed");
        for (n, line) in ["sluice 0.97.0", "sluice 0.100.0", "sluice 1.0.0"]
            .into_iter()
            .enumerate()
        {
            let at = scratch.join(n.to_string());
            let (seam, ran) = stand_in(line, 0);
            let Submitted::Submitted { submission } =
                seam.submit(&a_place(&at), &from_its_worktree())
            else {
                panic!("{line:?} was handed no work");
            };
            assert_eq!(submission.id, "7", "{line:?}");
            assert_eq!(
                ran_lines(&ran),
                vec!["submit main --no-wait --json"],
                "{line:?}"
            );
        }
        let _ = std::fs::remove_dir_all(scratch);
    }

    /// The steps a reading gave, each with whether it passed.
    fn outcomes(steps: &[ReadyStep]) -> Vec<(&str, bool)> {
        steps
            .iter()
            .map(|step| (step.step.as_str(), step.passed))
            .collect()
    }

    #[test]
    fn the_steps_sluice_equipped_with() {
        let long = format!("{}moon setup: the toolchain is unknown", "x".repeat(5000));
        let answer = serde_json::json!({
            "branch": "main",
            "revision": "2eb676c",
            "steps": [
                {"step": "toolchain", "passed": true, "words": "  activated\n"},
                {"step": "proto install", "passed": true, "words": "installed moon 2.6.1"},
                {"step": "moon --version", "passed": true, "words": "moon 2.6.1"},
                {"step": "moon setup", "passed": false, "words": long},
            ],
        });
        let steps = equipped(false, &answer.to_string(), "");
        assert_eq!(
            outcomes(&steps),
            vec![
                ("toolchain", true),
                ("proto install", true),
                ("moon --version", true),
                ("moon setup", false),
            ]
        );
        assert_eq!(steps[0].words, "activated");
        assert_eq!(steps[3].words, shell::tail(&long));
        assert!(steps[3].words.len() < long.len(), "the words were not cut");
        assert!(steps[3].words.ends_with("the toolchain is unknown"));
    }

    #[test]
    fn a_close_reads_gone_only_where_sluice_names_the_branch_itself() {
        let not_found = |name: &str| {
            serde_json::json!({"error": {"kind": "not_found", "message": format!("no branch {name}"), "next": []}})
                .to_string()
        };
        assert!(gone("train/a", &not_found("train/a"), ""));
        assert!(
            !gone("train/a", &not_found("main"), ""),
            "an into not found"
        );
        assert!(!gone("train/a", "", "sluice: could not read the policy"));
    }

    /// The arguments a forecast of `train/a` onto main runs sluice with.
    const FORECAST: [&str; 6] = ["submit", "main", "--check", "--head", "train/a", "--json"];

    /// sluice's forecast of `train/a` onto main as the profile reads it,
    /// exiting `code` with `answer`.
    fn forecast_read(code: i32, answer: &serde_json::Value) -> Forecast {
        forecast_of(Some(code), &answer.to_string(), "", &FORECAST, "main")
    }

    #[test]
    fn a_clear_forecast() {
        let answer = serde_json::json!({
            "conflict": null, "head": "aaa111", "onto": "c0ffee", "ahead": [3],
        });
        assert_eq!(
            forecast_read(0, &answer),
            Forecast::Clear {
                head: Revision::new("aaa111"),
                onto: Revision::new("c0ffee"),
            }
        );
    }

    #[test]
    fn a_conflicts_words() {
        // The merge-queue-profile spec's "A conflict's words": a conflict
        // with the base itself, its words naming README.md and c0ffee in
        // the lines a conflicting submission's words carry (112, 210).
        let answer = serde_json::json!({
            "conflict": {"paths": ["README.md"], "with": []},
            "head": "aaa111", "onto": "c0ffee", "ahead": [],
        });
        assert_eq!(
            forecast_read(2, &answer),
            Forecast::Conflict {
                head: Revision::new("aaa111"),
                onto: Revision::new("c0ffee"),
                files: vec!["README.md".into()],
                with: vec![],
                words: "would conflict: README.md with main\n\nThe files contested: README.md.\nThe base's revision the merge queue tested against: c0ffee.".into(),
            }
        );
    }

    #[test]
    fn a_conflict_with_entries_ahead() {
        let answer = serde_json::json!({
            "conflict": {
                "paths": ["README.md", "src/lib.rs"],
                "with": [
                    {"id": 7, "from": {"agent": "a", "worktree": "unit/x", "path": "/w/x"}},
                    {"id": 9, "from": {"agent": "", "worktree": "pull/4", "head": "unit/y"}},
                ],
            },
            "head": "aaa111", "onto": "c0ffee", "ahead": [7, 9],
        });
        let Forecast::Conflict {
            files, with, words, ..
        } = forecast_read(2, &answer)
        else {
            panic!("no conflict read");
        };
        assert_eq!(files, vec!["README.md", "src/lib.rs"]);
        assert_eq!(with, vec!["#7 (unit/x)", "#9 (unit/y)"]);
        assert!(
            words.starts_with(
                "would conflict: README.md, src/lib.rs with #7 (unit/x), #9 (unit/y)\n\n"
            ),
            "{words}"
        );
    }

    #[test]
    fn a_forecast_refused() {
        let refused = serde_json::json!({
            "error": {"kind": "refused", "message": "no branch train/a to forecast", "next": ["sluice overview"]},
        });
        assert_eq!(
            forecast_read(2, &refused),
            Forecast::CouldNotRead {
                reason: "sluice submit: no branch train/a to forecast".into(),
            }
        );
        assert_eq!(
            forecast_of(
                Some(1),
                "",
                "the reader's document is torn",
                &FORECAST,
                "main"
            ),
            Forecast::CouldNotRead {
                reason: "sluice submit: the reader's document is torn".into(),
            }
        );
        let clear_but_failed = serde_json::json!({
            "conflict": null, "head": "aaa111", "onto": "c0ffee", "ahead": [],
        });
        assert!(matches!(
            forecast_read(2, &clear_but_failed),
            Forecast::CouldNotRead { .. }
        ));
    }

    #[test]
    fn nothing_to_equip_is_no_step() {
        let answer = r#"{"branch": "main", "revision": "2eb676c", "steps": []}"#;
        assert_eq!(equipped(true, answer, ""), vec![]);
    }

    #[test]
    fn an_answer_with_no_steps() {
        let refused = r#"{"error": {"kind": "refused", "message": "no .config/sluice.toml at 2eb676c", "next": []}}"#;
        let unread = "equipping /r/places/fix--one for main at 2eb676c";
        let readings = [
            (
                equipped(false, refused, "a warning"),
                "no .config/sluice.toml at 2eb676c",
            ),
            (equipped(false, unread, "unexpected"), "unexpected"),
            (equipped(true, unread, ""), unread),
            (
                equipped(false, "", "bash: line 1: sluice: command not found"),
                "bash: line 1: sluice: command not found",
            ),
        ];
        for (steps, words) in readings {
            assert_eq!(outcomes(&steps), vec![(EQUIP, false)], "{steps:?}");
            assert_eq!(steps[0].words, words);
        }
    }

    #[test]
    fn a_failure_after_every_step_passed_is_sluices() {
        let answer = r#"{"steps": [{"step": "toolchain", "passed": true, "words": ""}]}"#;
        let steps = equipped(
            false,
            answer,
            "sluice: the run directory could not be removed",
        );
        assert_eq!(outcomes(&steps), vec![("toolchain", true), (EQUIP, false)]);
        assert_eq!(
            steps[1].words,
            "sluice: the run directory could not be removed"
        );
    }

    #[test]
    fn the_inbox_is_asked_of_sluice_host_on_its_socket_and_starts_no_sluice() {
        // 133; flume-reads-sluices-records-on-sluice-hosts-socket: a
        // stand-in sluice host on a socket of the test's own answers the
        // inbox, and the seam's `sluice` is no program at all, so a command
        // started for the inbox would leave it unread.
        use std::io::{BufRead, BufReader, Write};
        let dir = std::env::temp_dir().join(format!("fsl-inbox-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let state = dir.join("state");
        std::fs::create_dir_all(state.join("sluice")).expect("sluice's state folder");
        let socket = subscription::socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(state.clone().into()),
            subscription::USER_SCRATCH => Some(dir.clone().into()),
            _ => None,
        })
        .expect("the stand-in's socket");
        let listener = std::os::unix::net::UnixListener::bind(&socket)
            .unwrap_or_else(|error| panic!("no room for {}: {error}", socket.display()));
        let record = serde_json::json!({
            "id": 6,
            "branch": "main",
            "state": "landed",
            "landed_as": "1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d",
            "delivery": {"key": "6:landed", "state": "owed"},
        });
        let answer = serde_json::json!({ "outcomes": [record] }).to_string();
        let host = std::thread::spawn(move || {
            let (stream, _) = listener.accept().expect("a connection");
            let mut writer = stream.try_clone().expect("the connection");
            let mut lines = BufReader::new(stream).lines();
            let _hello = lines.next();
            writeln!(writer, r#"{{"version":"0.110.0","pid":1,"since":"now"}}"#).unwrap();
            let ask: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
            let output = serde_json::json!({"output": {"stdout": answer, "code": 0}});
            writeln!(writer, "{output}").unwrap();
            ask
        });
        let seam = Sluice {
            program: dir.join("no-sluice"),
            ..Sluice::new()
        }
        .with_env("XDG_STATE_HOME", Some(&state.to_string_lossy()))
        .with_env(subscription::USER_SCRATCH, Some(&dir.to_string_lossy()))
        .with_env("SLUICE_AGENT", Some("fix--one"));
        let location = dir.join("places/fix--one").to_string_lossy().into_owned();
        let place = PlaceRef {
            repository: dir.join("repository").to_string_lossy().into_owned(),
            location: location.clone(),
            branch: "fix/one".into(),
            base: "main".into(),
        };
        assert_eq!(
            seam.inbox(&place),
            Inbox::Outcomes {
                outcomes: vec![Owed {
                    submission: "6".into(),
                    outcome: SubmissionOutcome::Landed {
                        commit: Revision::new("1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d"),
                    },
                    key: Some("6:landed".into()),
                }],
                passed_over: vec![],
            }
        );
        assert_eq!(
            host.join().expect("the stand-in host"),
            serde_json::json!({
                "ask": "inbox",
                "folder": location,
                "worktree": location,
                "json": true,
                "caller": {"SLUICE_AGENT": "fix--one"},
            }),
            "the ask `sluice inbox --json` in the place makes, confirming nothing"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_inbox_runs_sluice_inbox_where_no_sluice_host_answers() {
        // S109; sluice 135; the-service-manager-starts-each-resident:
        // with no sluice host on its socket, the inbox is `sluice inbox
        // --json` in the place, which starts one on a bare install. The
        // seam's `sluice` is no program at all, so the words are of the
        // command that could not run, not of the socket.
        let dir = std::env::temp_dir().join(format!("fsl-no-host-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let state = dir.join("state");
        std::fs::create_dir_all(state.join("sluice")).expect("sluice's state folder");
        let seam = Sluice {
            program: dir.join("no-sluice"),
            ..Sluice::new()
        }
        .with_env("XDG_STATE_HOME", Some(&state.to_string_lossy()))
        .with_env(subscription::USER_SCRATCH, Some(&dir.to_string_lossy()));
        let place = PlaceRef {
            repository: dir.join("repository").to_string_lossy().into_owned(),
            location: dir.to_string_lossy().into_owned(),
            branch: "fix/one".into(),
            base: "main".into(),
        };
        let Inbox::CouldNotRead { reason } = seam.inbox(&place) else {
            panic!("an inbox read with no sluice host and no sluice");
        };
        assert!(
            reason.starts_with("sluice could not be run"),
            "the command was run: {reason}"
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
