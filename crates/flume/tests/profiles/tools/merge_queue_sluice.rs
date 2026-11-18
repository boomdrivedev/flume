//! The merge-queue profile over sluice (C.6; design 13) against the
//! pinned sluice, on scratch repositories gated by a command and, where a
//! base lands on a remote, a local bare remote: nothing reaches a forge,
//! a credential or the host's own sluice state, sluice's checkouts keep
//! their scratch in the test's own folder, and no multiplexer's session
//! reaches sluice. Each test's queues are moved by the host process of
//! its own state folder, which its first submit starts, and no test or
//! settle steps them; a check waits while the test holds it, so a test
//! reads a submission open for as long as it needs (124, 125, C.6;
//! sluice 135). The merge-queue seam's scenarios run against
//! it; then a submission from a herdr pane's hook, a failed entry
//! collected once, a queue the settles never step decided by sluice's
//! host process, a conflict's base revision, a conflict sluice answers at the submit
//! itself, through the stand-in for sluice, and a settle beside that
//! submit, which the inbox gives nothing, a pull request answered with
//! no entry and the entry taken in from it read to its landing, through
//! the stand-in for sluice on a base that takes pull requests, a branch
//! whose records sluice cannot read passed over by the inbox and
//! read-submission while the place's landing is collected, every branch
//! asked at once read from one overview and one status of each base, a
//! fetch that fails, and what the policy says of a place, its readiness
//! asked of the pinned sluice's equip and its answer read. A base place's
//! branch is opened and closed by the seam's scenarios, an open of a
//! branch the policy names no table for refused and a close of a branch a
//! submission is queued on kept, each in sluice's words (205, 206), and
//! submitted by name from the repository; over a stand-in origin, a base
//! place's branch is made there, a fix lands on it, it lands on main by
//! name, its landing read from sluice's status with no inbox run, and it
//! is closed on the origin and the host (203, S110). A chore submitted
//! with no agent in it lands by sluice's host process and is read and
//! tidied by the settles of a caller keeping watch alone, `flume settle
//! --every 1m`, none of which runs a round of the queue, with no `sluice
//! run` live; and where that host process ended with the chore's check
//! waiting, a settle's read starts another, which lands it (124, 125,
//! 208, S109; sluice 135).

use super::walk::{stage, unlink_user_scratch, user_scratch, Walk, USER_SCRATCH};
use crate::common::*;
use flume::profiles::merge_queue::sluice::{for_place, Sluice, EQUIP};
use flume::profiles::place::git_worktree::{GitWorktree, Login};
use flume::profiles::place_data;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::merge_queue::{self, MergeQueueHarness};
use flume_seams::*;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// What a place submits from its worktree at `head`.
fn from_worktree(head: &Revision) -> SubmitHead {
    SubmitHead::Worktree { head: head.clone() }
}

const POLICY: &str =
    "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.main]\n";

/// The policy with a table for `train/*` beside main's, so sluice opens,
/// lands on and closes `train/a`.
const TRAINS: &str = "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.\"train/*\"]\n\n[branches.main]\n";

/// The policy that lands `train/*` and main on the stand-in origin.
const TRAINS_ON_ORIGIN: &str = "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.\"train/*\"]\nremote = \"origin\"\n\n[branches.main]\nremote = \"origin\"\n";

/// The file in a test's scratch that holds its check while it stands.
const HOLD: &str = "hold";

/// The gate's check: it waits while `hold` stands, looking every 100
/// milliseconds for at most 120 seconds, and then passes, so a test reads
/// a submission open for as long as it holds it.
fn held_check(hold: &Path) -> String {
    format!(
        "i=0\nwhile [ -e '{}' ] && [ \"$i\" -lt 1200 ]; do sleep 0.1; i=$((i + 1)); done\nexit 0\n",
        hold.display()
    )
}

/// A scratch repository gated by sluice, and the place it holds.
struct Harness {
    repository: Repository,
    seam: Sluice,
    home: PathBuf,
    place: PlaceRef,
    locked: RefCell<Vec<PathBuf>>,
}

impl Harness {
    fn new() -> Harness {
        Harness::with_policy(POLICY)
    }

    /// A scratch repository whose policy is `policy`, its check held
    /// from the start: written before the place is cut, so the place's
    /// branch carries it, each run of it waits until the test asks a
    /// submission decided, so sluice's host process decides nothing the
    /// test reads open.
    fn with_policy(policy: &str) -> Harness {
        let repository = Repository::new("sluice");
        repository.write(".config/sluice.toml", policy);
        repository.write("check.sh", &held_check(&repository.scratch.path(HOLD)));
        repository.commit("the gate");
        fs::write(repository.scratch.path(HOLD), "").expect("the hold");
        let home = repository.scratch.path("home");
        fs::create_dir_all(&home).expect("the scratch home");
        let mut seam = Sluice::new()
            .with_env("HOME", Some(&home.to_string_lossy()))
            .with_env(
                "XDG_STATE_HOME",
                Some(&home.join("state").to_string_lossy()),
            )
            .with_env(
                "XDG_CONFIG_HOME",
                Some(&home.join("config").to_string_lossy()),
            );
        for (name, _) in std::env::vars() {
            if name.starts_with("SLUICE_") {
                seam = seam.with_env(&name, None);
            }
        }
        let seam = seam.with_env(
            USER_SCRATCH,
            Some(&user_scratch(&repository).to_string_lossy()),
        );
        let place = Harness::worktree(&repository, "fix/one", "fix--one", "work.rs");
        Harness {
            repository,
            seam,
            home,
            place,
            locked: RefCell::new(vec![]),
        }
    }

    /// A worktree on `branch` cut from main at `places/<short>`, holding
    /// one commit touching `file`.
    fn worktree(repository: &Repository, branch: &str, short: &str, file: &str) -> PlaceRef {
        let location = repository.dir.join("places").join(short);
        repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            branch,
            &location.to_string_lossy(),
            "main",
        ]);
        fs::write(location.join(file), format!("{branch}\n")).expect("the work written");
        git(&location, &["add", file]);
        git(&location, &["commit", "--quiet", "-m", branch]);
        PlaceRef {
            repository: repository.name(),
            location: location.to_string_lossy().into(),
            branch: branch.into(),
            base: "main".into(),
        }
    }

    /// A commit on main, past the hook sluice protects it with.
    fn on_main(&self, path: &str, text: &str) -> Revision {
        self.repository.write(path, text);
        self.repository.git(&["add", "--all"]);
        self.repository.git(&[
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "-m",
            path,
        ]);
        Revision::new(self.repository.git(&["rev-parse", "main"]))
    }

    /// `path` written with `text` and committed on `branch`, through a
    /// worktree of its own removed after: the branch's new head.
    fn on_branch(&self, branch: &str, path: &str, text: &str) -> Revision {
        let dir = self.repository.dir.join("places/on-branch");
        let at = dir.to_string_lossy().into_owned();
        self.repository
            .git(&["worktree", "add", "--quiet", &at, branch]);
        std::fs::write(dir.join(path), text).expect("the branch's file written");
        git(&dir, &["add", "--all"]);
        git(
            &dir,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                path,
            ],
        );
        let head = git(&dir, &["rev-parse", "HEAD"]);
        self.repository.git(&["worktree", "remove", "--force", &at]);
        Revision::new(head)
    }

    /// sluice run by the test in `dir`, in the scratch home, with no
    /// multiplexer's session: its JSON answer.
    fn sluice(&self, dir: &Path, args: &[&str]) -> Value {
        self.sluice_ran(dir, args).1
    }

    /// sluice run by the test in `dir`, in the scratch home, with no
    /// multiplexer's session: whether it succeeded, and its JSON answer.
    fn sluice_ran(&self, dir: &Path, args: &[&str]) -> (bool, Value) {
        let mut command = Command::new("sluice");
        command.args(args).current_dir(dir).stdin(Stdio::null());
        for (name, _) in std::env::vars() {
            if name.starts_with("SLUICE_")
                || name.starts_with("HERDR_")
                || name.starts_with("REX_")
                || name.starts_with("TMUX")
            {
                command.env_remove(name);
            }
        }
        let output = command
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .env("XDG_CONFIG_HOME", self.home.join("config"))
            .env(USER_SCRATCH, user_scratch(&self.repository))
            .output()
            .expect("sluice runs");
        let answer = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
            panic!(
                "sluice {}: {}{}",
                args.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
        (output.status.success(), answer)
    }

    /// The newest record sluice holds of `branch`'s submissions to main.
    fn record(&self, branch: &str) -> Option<Value> {
        let status = self.sluice(&self.repository.dir, &["status", "main", "--all", "--json"]);
        status["submissions"]
            .as_array()?
            .iter()
            .filter(|record| record["from"]["worktree"] == branch)
            .max_by_key(|record| record["id"].as_u64())
            .cloned()
    }

    /// The check held: each run of it waits until [`Harness::release`].
    fn hold(&self) {
        fs::write(self.repository.scratch.path(HOLD), "").expect("the hold");
    }

    /// The check released, so a run of it waiting ends and passes.
    fn release(&self) {
        let _ = fs::remove_file(self.repository.scratch.path(HOLD));
    }

    /// The check released, and main's status read until `branch`'s
    /// newest submission is decided by sluice's host process, which the
    /// submit started; nothing steps main (124, 125, C.6; sluice 135).
    fn decide(&self, branch: &str) {
        self.release();
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            let state = self
                .record(branch)
                .and_then(|record| record["state"].as_str().map(str::to_string));
            if !matches!(state.as_deref(), Some("queued" | "testing" | "landing")) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "{branch}'s submission stayed {state:?}; {}",
                self.host_process()
            );
            std::thread::sleep(Duration::from_millis(300));
        }
    }

    /// What the test's state folder says of sluice's host process: the
    /// one it records, and the end of its log.
    fn host_process(&self) -> String {
        let host = self.home.join("state/sluice/host");
        let read = |name: &str| fs::read_to_string(host.join(name)).unwrap_or_default();
        let log = read("log");
        let tail: Vec<&str> = log.lines().rev().take(20).collect::<Vec<_>>();
        format!(
            "the host process recorded {}, its log ending:\n{}",
            read("version").trim(),
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        )
    }
}

impl Harness {
    /// The seam with a `sluice` first on its `PATH` that writes each
    /// command it is asked, a line each, then runs the pinned sluice; and
    /// where it writes them.
    fn counting(&self) -> (Sluice, PathBuf) {
        let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|dir| dir.join("sluice"))
            .find(|path| path.is_file())
            .expect("the pinned sluice on PATH");
        let bin = self.repository.scratch.path("counted");
        fs::create_dir_all(&bin).expect("the counting sluice's folder");
        let asked = self.repository.scratch.path("sluice-counted");
        let program = bin.join("sluice");
        fs::write(
            &program,
            format!(
                "#!/bin/sh\necho \"$*\" >>'{}'\nexec '{}' \"$@\"\n",
                asked.display(),
                pinned.display()
            ),
        )
        .expect("the counting sluice");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("it runs");
        let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        )))
        .expect("a PATH");
        let seam = self
            .seam
            .clone()
            .with_env("PATH", Some(&path.to_string_lossy()));
        (seam, asked)
    }
}

/// `branch`'s newest submission to `base`, read through `seam` by
/// read-submission alone until sluice's host process decides it, the
/// profile's step answering once that `base`'s queue moves on its own
/// (124, 125, C.6; sluice 135).
fn read_until_decided(seam: &Sluice, repository: &str, base: &str, branch: &str) -> Submission {
    assert_eq!(
        seam.step(repository, base),
        Stepped::MovesOnItsOwn,
        "a step of {base}"
    );
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        match seam.read_submission(repository, branch).read {
            SubmissionRead::Found { submission } if !submission.state.open() => return submission,
            SubmissionRead::Found { .. } => {}
            other => panic!("read-submission of {branch} gave {other:?}"),
        }
        assert!(
            Instant::now() < deadline,
            "{branch}'s submission to {base} was never decided"
        );
        std::thread::sleep(Duration::from_millis(300));
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        // Released first, so every run ends and the host process of the
        // test's state folder ends with nothing in play.
        self.release();
        for path in self.locked.borrow_mut().drain(..).rev() {
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
        }
        unlink_user_scratch(&self.repository);
    }
}

impl MergeQueueHarness for Harness {
    type Seam = Sluice;

    fn seam(&self) -> &Sluice {
        &self.seam
    }

    fn place(&self) -> (PlaceRef, Revision) {
        let head = git(Path::new(&self.place.location), &["rev-parse", "HEAD"]);
        (self.place.clone(), Revision::new(head))
    }

    fn will_fail(&self, words: &str) {
        self.on_main(
            ".config/sluice.toml",
            &POLICY.replace("sh check.sh", &format!("echo '{words}' >&2; exit 1")),
        );
    }

    fn will_conflict(&self, _place: &PlaceRef, file: &str) {
        self.on_main(file, "the base's own\n");
    }

    fn will_hold(&self) {
        self.hold();
    }

    fn until_decided(&self, place: &PlaceRef) {
        self.decide(&place.branch);
    }

    fn withdraw(&self, place: &PlaceRef) {
        let id = self.record(&place.branch).expect("a submission")["id"].to_string();
        self.sluice(&self.repository.dir, &["withdraw", &id, "--json"]);
        self.release();
    }

    fn land_elsewhere(&self, place: &PlaceRef) {
        let other = self.repository.dir.join("places/other");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "other",
            &other.to_string_lossy(),
            &place.branch,
        ]);
        self.sluice(&other, &["submit", "main", "--no-wait", "--json"]);
        self.decide("other");
        assert_eq!(
            self.record("other").expect("the other submission")["state"],
            "landed",
            "the other branch's landing"
        );
    }

    fn open_submission(&self, place: &PlaceRef) -> Submission {
        self.hold();
        let entry = self.sluice(
            Path::new(&place.location),
            &["submit", "main", "--no-wait", "--json"],
        );
        Submission {
            id: entry["id"].to_string(),
            branch: place.branch.clone(),
            target: "main".into(),
            state: SubmissionState::Waiting,
            request: None,
        }
    }

    fn base_elsewhere(&self, _repository: &str, base: &str) -> Revision {
        let remote = self.repository.bare_remote();
        self.repository
            .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
        self.on_main(
            ".config/sluice.toml",
            &format!("{POLICY}remote = \"origin\"\n"),
        );
        self.repository.git(&["push", "--quiet", "origin", base]);
        let elsewhere = self.repository.scratch.path("elsewhere");
        git(
            &self.repository.scratch.root,
            &[
                "clone",
                "--quiet",
                &remote.to_string_lossy(),
                &elsewhere.to_string_lossy(),
            ],
        );
        fs::write(elsewhere.join("remote.rs"), "remote\n").expect("the remote's work");
        git(&elsewhere, &["add", "remote.rs"]);
        git(&elsewhere, &["commit", "--quiet", "-m", "on the remote"]);
        git(&elsewhere, &["push", "--quiet", "origin", base]);
        Revision::new(git(&elsewhere, &["rev-parse", "HEAD"]))
    }

    fn fetch_fails(&self, _repository: &str, _base: &str) {
        let gone = self.repository.scratch.path("gone.git");
        self.repository
            .git(&["remote", "set-url", "origin", &gone.to_string_lossy()]);
    }

    /// `trial`, made from main, whose policy names a table for main
    /// alone, so sluice lands nothing on it (126, C.6).
    fn base_unlanded(&self, _repository: &str) -> String {
        self.repository.git(&["branch", "trial", "main"]);
        "trial".into()
    }

    fn unreadable(&self) {
        let records = self.repository.dir.join(".git/sluice");
        fs::create_dir_all(&records).expect("sluice's records");
        fs::set_permissions(&records, fs::Permissions::from_mode(0o000))
            .expect("the records locked");
        self.locked.borrow_mut().push(records);
    }

    /// `bolt/x`, its policy naming it, given a submission from `unit/x`,
    /// whose records sluice then cannot read: `branch.json` locked in a
    /// folder no process may write, so sluice's host process, which passes
    /// the branch, cannot write it whole again. The branch is given once
    /// `sluice status bolt/x` refuses it, read until it does within 30
    /// seconds; it and the inbox refuse the branch from then on, as sluice
    /// refused one whose policy it could no longer read on deneb on
    /// 2026-11-04 (133).
    fn refuse_another(&self, _place: &PlaceRef) -> String {
        let other = "bolt/x";
        let at = self.repository.dir.join("places/bolt--x");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            other,
            &at.to_string_lossy(),
            "main",
        ]);
        fs::write(
            at.join(".config/sluice.toml"),
            format!("{POLICY}\n[branches.\"bolt/*\"]\n"),
        )
        .expect("bolt/x's policy");
        git(&at, &["add", ".config/sluice.toml"]);
        git(
            &at,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                "bolt/x's policy",
            ],
        );
        let unit = self.repository.dir.join("places/unit--x");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "unit/x",
            &unit.to_string_lossy(),
            other,
        ]);
        fs::write(unit.join("x.rs"), "unit/x\n").expect("the work written");
        git(&unit, &["add", "x.rs"]);
        git(&unit, &["commit", "--quiet", "-m", "unit/x"]);
        let entry = self.sluice(&unit, &["submit", other, "--no-wait", "--json"]);
        assert!(entry["id"].is_u64(), "bolt/x's submission: {entry}");
        let records = self.repository.dir.join(".git/sluice").join(other);
        let file = records.join("branch.json");
        assert!(file.exists(), "{other}'s records at {file:?}");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o000)).expect("the records locked");
        fs::set_permissions(&records, fs::Permissions::from_mode(0o500))
            .expect("the records' folder locked");
        self.locked.borrow_mut().extend([file, records]);
        let deadline = Instant::now() + Duration::from_secs(30);
        while self
            .sluice_ran(&self.repository.dir, &["status", other, "--all", "--json"])
            .0
        {
            assert!(
                Instant::now() < deadline,
                "sluice read {other}'s status for 30 seconds after its records were locked"
            );
            std::thread::sleep(Duration::from_millis(100));
        }
        other.into()
    }

    fn branch_head(&self, _repository: &str, branch: &str) -> Option<Revision> {
        let head = self
            .repository
            .git(&["branch", "--list", branch, "--format=%(objectname)"]);
        (!head.is_empty()).then(|| Revision::new(head))
    }

    fn change_on(&self, _repository: &str, branch: &str, file: &str) -> Revision {
        self.on_branch(branch, file, "the branch's own\n")
    }

    fn move_base_against(
        &self,
        _repository: &str,
        _base: &str,
        _branch: &str,
        file: &str,
    ) -> Revision {
        self.on_main(file, "the base's own\n")
    }
}

#[test]
fn the_sluice_profile_passes_the_merge_queue_scenarios() {
    pass_all("merge queue", &merge_queue::scenarios(), Harness::new);
}

#[test]
fn the_sluice_profile_passes_the_base_place_scenarios() {
    // The seam's base-place scenarios, over a policy that names
    // `train/*`: a branch opened and closed, and submitted by name from
    // the repository (203, 205, 206).
    pass_all(
        "merge queue's base place",
        &merge_queue::base_place_scenarios(),
        || Harness::with_policy(TRAINS),
    );
}

#[test]
fn a_base_places_branch_over_a_scratch_repository() {
    // A base place's branch through the profile alone, on a scratch
    // repository whose policy lands `train/*` and main on a stand-in
    // origin: sluice makes `train/a` there, a fix cut from it lands on it
    // by sluice's host process, which its submit started, the base
    // place's submission of `train/a` by name from the repository lands
    // on main the same way, no settle stepping either queue, its landing
    // read from sluice's status with no inbox run, and its close removes
    // it on the origin and the host. The profile runs sluice and a fetch
    // alone, so nothing of flume's pushes or moves a branch (14, 124,
    // 125, 202 to 206, S110).
    let harness = Harness::with_policy(TRAINS_ON_ORIGIN);
    // Nothing here is read open, so every check passes as it runs.
    harness.release();
    let repository = harness.repository.name();
    let remote = harness.repository.bare_remote();
    harness
        .repository
        .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
    harness.repository.git(&["fetch", "--quiet", "origin"]);
    let on_origin = |branch: &str| {
        let line = git(
            &harness.repository.dir,
            &[
                "ls-remote",
                &remote.to_string_lossy(),
                &format!("refs/heads/{branch}"),
            ],
        );
        line.split_whitespace().next().map(str::to_string)
    };
    let main = on_origin("main").expect("main on the origin");

    let BranchOpened::Made { revision } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    assert_eq!(revision.0, main, "train/a made at main's revision");
    assert_eq!(
        on_origin("train/a").as_deref(),
        Some(main.as_str()),
        "train/a on the origin"
    );

    harness
        .repository
        .git(&["fetch", "--quiet", "origin", "train/a"]);
    let fix = harness.repository.dir.join("places/fix--a");
    harness.repository.git(&[
        "worktree",
        "add",
        "--quiet",
        "-b",
        "fix/a",
        &fix.to_string_lossy(),
        "origin/train/a",
    ]);
    fs::write(fix.join("a.rs"), "fix/a\n").expect("the fix's work");
    git(&fix, &["add", "a.rs"]);
    git(&fix, &["commit", "--quiet", "-m", "fix/a"]);
    let fix_place = PlaceRef {
        repository: repository.clone(),
        location: fix.to_string_lossy().into(),
        branch: "fix/a".into(),
        base: "train/a".into(),
    };
    let head = Revision::new(git(&fix, &["rev-parse", "HEAD"]));
    let Submitted::Submitted { .. } = harness.seam.submit(&fix_place, &from_worktree(&head)) else {
        panic!("the fix's submit to train/a was refused");
    };
    let landed = read_until_decided(&harness.seam, &repository, "train/a", "fix/a");
    assert!(
        matches!(landed.state, SubmissionState::Landed { .. }),
        "the fix came to {landed:?}"
    );
    let train = on_origin("train/a").expect("train/a on the origin");
    assert_eq!(
        git(
            &harness.repository.dir,
            &[
                "--git-dir",
                &remote.to_string_lossy(),
                "show",
                &format!("{train}:a.rs")
            ],
        ),
        "fix/a",
        "the fix on train/a"
    );

    let (seam, asked) = harness.counting();
    let base_place = PlaceRef {
        repository: repository.clone(),
        location: harness
            .repository
            .dir
            .join("places/train--a")
            .to_string_lossy()
            .into(),
        branch: "train/a".into(),
        base: "main".into(),
    };
    let submission = match seam.submit(
        &base_place,
        &SubmitHead::Branch {
            branch: "train/a".into(),
            revision: Revision::new(train.clone()),
        },
    ) {
        Submitted::Submitted { submission } => submission,
        other => panic!("train/a's submit by name answered {other:?}"),
    };
    assert_eq!(
        (submission.branch.as_str(), submission.target.as_str()),
        ("train/a", "main"),
        "the submission by name"
    );
    let landed = read_until_decided(&seam, &repository, "main", "train/a");
    assert_eq!(landed.id, submission.id, "the landing read by its branch");
    assert!(
        matches!(landed.state, SubmissionState::Landed { .. }),
        "train/a came to {landed:?}"
    );
    let main = on_origin("main").expect("main on the origin");
    assert_eq!(
        git(
            &harness.repository.dir,
            &[
                "--git-dir",
                &remote.to_string_lossy(),
                "show",
                &format!("{main}:a.rs")
            ],
        ),
        "fix/a",
        "train/a's work on main"
    );
    let asked = fs::read_to_string(&asked).expect("sluice was asked");
    assert!(
        asked
            .lines()
            .next()
            .is_some_and(|line| line == "submit main --head train/a --no-wait --json"),
        "the submit by name: {asked}"
    );
    assert!(
        !asked.lines().any(|line| line.starts_with("inbox")),
        "an inbox ran for a submission by name: {asked}"
    );
    let worktrees = harness.repository.git(&["worktree", "list", "--porcelain"]);
    assert!(
        !worktrees.contains("branch refs/heads/train/a"),
        "a worktree on train/a: {worktrees}"
    );
    assert!(
        !Path::new(&base_place.location).exists(),
        "something stands at the base place's location"
    );

    assert_eq!(
        harness.seam.close(&repository, "train/a", "main"),
        BranchClosed::Removed,
        "train/a's close into main"
    );
    assert_eq!(on_origin("train/a"), None, "train/a on the origin");
    assert_eq!(
        harness.branch_head(&repository, "train/a"),
        None,
        "train/a on the host"
    );
}

#[test]
fn a_base_places_branch_forecast_over_a_scratch_repository() {
    // The merge-queue-profile spec's "A base place's branch forecast over
    // a scratch repository": sluice lands main and `train/*` on a stand-in
    // origin; flume opens `train/a`, a fix lands a change to README.md on
    // it, and the forecast of `train/a` onto main is clear at main's
    // revision; a change to README.md lands on main, and the next is in
    // conflict on README.md at main's new revision, meeting main itself.
    // Neither leaves an entry of `train/a` in main's records. Each
    // landing is sluice's host process's, which its submit started, no
    // settle stepping its queue (124, 125, 203, 210; sluice 64, 135).
    let harness = Harness::with_policy(TRAINS_ON_ORIGIN);
    // Nothing here is read open, so every check passes as it runs.
    harness.release();
    let repository = harness.repository.name();
    let remote = harness.repository.bare_remote();
    harness
        .repository
        .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
    harness.repository.git(&["fetch", "--quiet", "origin"]);
    let on_origin = |branch: &str| {
        let line = git(
            &harness.repository.dir,
            &[
                "ls-remote",
                &remote.to_string_lossy(),
                &format!("refs/heads/{branch}"),
            ],
        );
        line.split_whitespace().next().map(Revision::new)
    };
    let BranchOpened::Made { .. } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    // A place on `from`, cut where the origin holds it, landing a change
    // to README.md written `text` on it.
    let lands = |name: &str, from: &str, text: &str| {
        harness
            .repository
            .git(&["fetch", "--quiet", "origin", from]);
        let location = harness.repository.dir.join("places").join(name);
        harness.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            name,
            &location.to_string_lossy(),
            &format!("origin/{from}"),
        ]);
        fs::write(location.join("README.md"), text).expect("the work written");
        git(&location, &["add", "README.md"]);
        git(&location, &["commit", "--quiet", "-m", name]);
        let place = PlaceRef {
            repository: repository.clone(),
            location: location.to_string_lossy().into(),
            branch: name.into(),
            base: from.into(),
        };
        let head = Revision::new(git(&location, &["rev-parse", "HEAD"]));
        let Submitted::Submitted { .. } = harness.seam.submit(&place, &from_worktree(&head)) else {
            panic!("{name}'s submit to {from} was refused");
        };
        let landed = read_until_decided(&harness.seam, &repository, from, name);
        assert!(
            matches!(landed.state, SubmissionState::Landed { .. }),
            "{name} came to {landed:?}"
        );
    };
    let no_entry = || {
        let status = harness.sluice(
            &harness.repository.dir,
            &["status", "main", "--all", "--json"],
        );
        let entries: Vec<&Value> = status["submissions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|record| {
                record["from"]["head"] == "train/a" || record["from"]["worktree"] == "train/a"
            })
            .collect();
        assert!(entries.is_empty(), "train/a's entries on main: {entries:?}");
    };

    lands("fix-a", "train/a", "the branch's own\n");
    let head = on_origin("train/a").expect("train/a on the origin");
    let main = on_origin("main").expect("main on the origin");
    assert_eq!(
        harness.seam.forecast(&repository, "train/a", "main"),
        Forecast::Clear {
            head: head.clone(),
            onto: main,
        },
        "the forecast while the two merge"
    );
    no_entry();

    lands("other", "main", "the base's own\n");
    let main = on_origin("main").expect("main on the origin");
    match harness.seam.forecast(&repository, "train/a", "main") {
        Forecast::Conflict {
            head: at,
            onto,
            files,
            with,
            words,
        } => {
            assert_eq!(at, head, "the head forecast");
            assert_eq!(onto, main, "main's new revision");
            assert_eq!(files, vec!["README.md".to_string()], "the files contested");
            assert!(with.is_empty(), "a conflict with main met {with:?}");
            assert!(
                words.contains("README.md") && words.contains(&main.0),
                "the words: {words}"
            );
        }
        other => panic!("the forecast once main moved: {other:?}"),
    }
    no_entry();
}

#[test]
fn an_open_of_a_branch_the_policy_names_no_table_for_is_refused_in_sluices_words() {
    let harness = Harness::new();
    let repository = harness.repository.name();
    match harness.seam.open(&repository, "other/a", "main") {
        BranchOpened::Refused { reason } => assert!(
            reason.starts_with("sluice branch create: ")
                && reason.contains("no [branches] table")
                && reason.contains("other/a"),
            "the refusal is not sluice's words: {reason}"
        ),
        other => panic!("an open the policy names no table for answered {other:?}"),
    }
    assert_eq!(
        harness.branch_head(&repository, "other/a"),
        None,
        "a branch made"
    );
}

#[test]
fn a_close_sluice_refuses_keeps_the_branch_in_its_words() {
    let harness = Harness::with_policy(TRAINS);
    let repository = harness.repository.name();
    let BranchOpened::Made { revision } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    let fix = harness.repository.dir.join("places/fix--a");
    harness.repository.git(&[
        "worktree",
        "add",
        "--quiet",
        "-b",
        "fix/a",
        &fix.to_string_lossy(),
        "train/a",
    ]);
    fs::write(fix.join("a.rs"), "fix/a\n").expect("the fix's work");
    git(&fix, &["add", "a.rs"]);
    git(&fix, &["commit", "--quiet", "-m", "fix/a"]);
    let entry = harness.sluice(&fix, &["submit", "train/a", "--no-wait", "--json"]);
    assert!(entry["id"].is_u64(), "the fix's submission: {entry}");
    // While sluice's host process passes train/a, sluice refuses a close
    // as a step holding the branch and says to ask again, which the test
    // does until it answers for the submission held open on it.
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match harness.seam.close(&repository, "train/a", "main") {
            BranchClosed::Kept { reason } if reason.contains("a step holds train/a") => {
                assert!(Instant::now() < deadline, "a step held train/a: {reason}");
                std::thread::sleep(Duration::from_millis(100));
            }
            BranchClosed::Kept { reason } => {
                assert!(
                    reason.starts_with("sluice branch delete: ")
                        && reason.contains("queued on train/a"),
                    "the refusal is not sluice's words: {reason}"
                );
                break;
            }
            other => panic!("a close with a submission queued answered {other:?}"),
        }
    }
    assert_eq!(
        harness.branch_head(&repository, "train/a"),
        Some(revision),
        "the branch a close was refused"
    );
}

#[test]
fn a_submission_from_a_herdr_panes_hook_names_the_place_and_no_session() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    let seam = harness
        .seam
        .clone()
        .with_env("HERDR_PANE_ID", Some("w1:p1"))
        .with_env("HERDR_SOCKET_PATH", Some("/nowhere/herdr.sock"))
        .with_env("HERDR_ENV", Some("1"))
        .with_env("TMUX", Some("/nowhere/tmux,1,0"))
        .with_env("TMUX_PANE", Some("%1"));
    let Submitted::Submitted { submission } = seam.submit(&place, &from_worktree(&head)) else {
        panic!("the submit was refused");
    };
    assert_eq!(submission.state, SubmissionState::Waiting, "the answer");
    let record = harness.record("fix/one").expect("sluice's entry");
    assert_eq!(
        record["from"]["path"].as_str(),
        Some(place.location.as_str()),
        "the entry's submitter"
    );
    assert!(
        record["from"].get("session").is_none(),
        "the entry names a session: {record}"
    );
    assert!(
        matches!(record["state"].as_str(), Some("queued" | "testing")),
        "the entry before any verdict: {record}"
    );
}

#[test]
fn a_submission_from_a_rex_block_names_no_session() {
    // C.6; core.6: the merge-queue-profile scenario "A submission from a
    // Rex block": a submission from a process whose environment carries
    // REX_SERVER, REX_SESSION and REX_BLOCK, as a hook's in a Rex block
    // does, names the place's worktree as its submitter and no session, and
    // answers before any verdict.
    let harness = Harness::new();
    let (place, head) = harness.place();
    let seam = harness
        .seam
        .clone()
        .with_env("REX_SERVER", Some("unix:///nowhere/rex.sock"))
        .with_env(
            "REX_SESSION",
            Some("session:01a1d7f4-c985-7f17-ade1-a7707e974087"),
        )
        .with_env(
            "REX_BLOCK",
            Some("block:01a1d7f4-c986-716b-904e-a3b017e8d18f"),
        );
    let Submitted::Submitted { submission } = seam.submit(&place, &from_worktree(&head)) else {
        panic!("the submit was refused");
    };
    assert_eq!(submission.state, SubmissionState::Waiting, "the answer");
    let record = harness.record("fix/one").expect("sluice's entry");
    assert_eq!(
        record["from"]["path"].as_str(),
        Some(place.location.as_str()),
        "the entry's submitter"
    );
    assert!(
        record["from"].get("session").is_none(),
        "the entry names a session: {record}"
    );
    assert!(
        matches!(record["state"].as_str(), Some("queued" | "testing")),
        "the entry before any verdict: {record}"
    );
}

#[test]
fn a_failed_entry_is_collected_once_with_sluices_notice() {
    // The inbox collects with no `--confirm` and then confirms what it
    // read by one `sluice confirm` of each outcome's `delivery.key`, which
    // the pinned sluice gives and takes: the first collect gives the
    // failure with sluice's notice and confirms it, so the second gives
    // nothing (101, 133, C.6; sluice 51; core.58).
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.will_fail("test x failed");
    harness.seam.submit(&place, &from_worktree(&head));
    harness.decide("fix/one");
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
        panic!("the inbox could not be read");
    };
    match outcomes.as_slice() {
        [SubmissionOutcome::Failed { words }] => assert!(
            words.starts_with("[sluice] ") && words.contains("test x failed"),
            "the words are not sluice's notice: {words}"
        ),
        other => panic!("the first collect gave {other:?}"),
    }
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the second collect, the first having confirmed the failure by its key"
    );
}

#[test]
fn every_branch_asked_at_once_is_read_from_one_overview_and_one_status_of_each_base() {
    let harness = Harness::new();
    let (one, head) = harness.place();
    let two = Harness::worktree(&harness.repository, "fix/two", "fix--two", "two.rs");
    let two_head = Revision::new(git(Path::new(&two.location), &["rev-parse", "HEAD"]));
    let submitted = |place: &PlaceRef, head: &Revision| match harness
        .seam
        .submit(place, &from_worktree(head))
    {
        Submitted::Submitted { submission } => submission.id,
        other => panic!("the submit gave {other:?}"),
    };
    let ids = [submitted(&one, &head), submitted(&two, &two_head)];
    let real = Command::new("sh")
        .args(["-c", "command -v sluice"])
        .output()
        .expect("sh runs");
    let real = String::from_utf8_lossy(&real.stdout).trim().to_string();
    let bin = harness.repository.scratch.path("counting");
    fs::create_dir_all(&bin).expect("the counting sluice's folder");
    let ran = harness.repository.scratch.path("sluice-ran");
    let counting = bin.join("sluice");
    fs::write(
        &counting,
        format!(
            "#!/bin/sh\necho \"$1\" >>'{}'\nexec '{real}' \"$@\"\n",
            ran.display()
        ),
    )
    .expect("the counting sluice");
    fs::set_permissions(&counting, fs::Permissions::from_mode(0o755)).expect("it runs");
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let read = seam.read_submissions(&one.repository, &["fix/one", "fix/two", "fix/none"]);
    let found: Vec<Option<(String, String)>> = read
        .iter()
        .map(|across| match &across.read {
            SubmissionRead::Found { submission } => {
                Some((submission.id.clone(), submission.branch.clone()))
            }
            SubmissionRead::None => None,
            other => panic!("read-submission gave {other:?}"),
        })
        .collect();
    assert_eq!(
        found,
        vec![
            Some((ids[0].clone(), "fix/one".to_string())),
            Some((ids[1].clone(), "fix/two".to_string())),
            None
        ],
        "each branch's own submission"
    );
    let ran = fs::read_to_string(&ran).expect("sluice ran");
    assert_eq!(
        ran.lines().collect::<Vec<_>>(),
        vec!["overview", "status"],
        "sluice's commands, once for the three branches"
    );
}

#[test]
fn a_queue_the_settles_never_step_is_decided_by_sluices_host_process() {
    // The merge-queue-profile scenarios "A queue the settles never step"
    // and "A queue nothing drives": with no `sluice run` or `sluice
    // daemon` live, the place's entry, its check held, is read open; once
    // the hold goes, the profile's step, read-submission and inbox, asked
    // as a settle asks them, find it landed by sluice's host process,
    // which the submit started, each step answering that main's queue
    // moves on its own, and every sluice command the profile ran after
    // the submit a read (124, 125, C.6; sluice 135).
    let harness = Harness::new();
    let (place, head) = harness.place();
    let (seam, asked) = harness.counting();
    harness.hold();
    let Submitted::Submitted { .. } = seam.submit(&place, &from_worktree(&head)) else {
        panic!("the submit was refused");
    };
    match seam.read_submission(&place.repository, &place.branch).read {
        SubmissionRead::Found { submission } => assert!(
            submission.state.open(),
            "the held entry read {submission:?}"
        ),
        other => panic!("read-submission of the held entry gave {other:?}"),
    }
    harness.release();
    let deadline = Instant::now() + Duration::from_secs(90);
    let decided = loop {
        assert_eq!(
            seam.step(&place.repository, &place.base),
            Stepped::MovesOnItsOwn,
            "a settle's step"
        );
        let read = seam.read_submission(&place.repository, &place.branch).read;
        assert!(
            matches!(seam.inbox(&place), Inbox::Outcomes { .. }),
            "the inbox could not be read"
        );
        match read {
            SubmissionRead::Found { submission } if !submission.state.open() => break submission,
            SubmissionRead::Found { .. } => {}
            other => panic!("read-submission gave {other:?}"),
        }
        assert!(
            Instant::now() < deadline,
            "the entry was never decided; {}",
            harness.host_process()
        );
        std::thread::sleep(Duration::from_millis(300));
    };
    assert!(
        matches!(decided.state, SubmissionState::Landed { .. }),
        "the entry came to {decided:?}"
    );
    let asked = fs::read_to_string(&asked).expect("sluice was asked");
    let mut commands = asked.lines();
    assert!(
        commands
            .next()
            .is_some_and(|line| line.starts_with("submit main ")),
        "the submit: {asked}"
    );
    for line in commands {
        assert!(
            matches!(
                line.split_whitespace().next(),
                Some("inbox" | "confirm" | "overview" | "status")
            ),
            "a sluice command after the submit: {line}"
        );
    }
}

#[test]
fn a_conflicts_base_revision_and_words_are_sluices() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    let base = harness.on_main("work.rs", "the base's own\n");
    harness.seam.submit(&place, &from_worktree(&head));
    harness.decide("fix/one");
    let record = harness.record("fix/one").expect("sluice's entry");
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
        panic!("the inbox could not be read");
    };
    match outcomes.as_slice() {
        [SubmissionOutcome::Conflict {
            files,
            base: tested,
            words,
        }] => {
            let expected = record["tested"]["on"]
                .as_str()
                .map_or(base.clone(), Revision::new);
            assert_eq!(tested, &expected, "the base revision");
            assert_eq!(files, &vec!["work.rs".to_string()], "the files contested");
            assert!(
                words.starts_with("[sluice] ") && words.contains("work.rs"),
                "the words are not sluice's notice: {words}"
            );
        }
        other => panic!("a conflicting entry came to {other:?}"),
    }
}

#[test]
fn a_conflict_sluice_answers_at_the_submit_is_its_outcome_never_a_refusal() {
    // sluice answers the submit with the entry's record, in conflict with
    // the one queued ahead of it, exiting as a conflict does, and the
    // profile confirms its delivery: submit answers the outcome, and no
    // inbox gives it (sluice 51; sluice record 0102).
    let harness = Harness::new();
    let (place, head) = harness.place();
    let other = Harness::worktree(&harness.repository, "other", "other", "work.rs");
    harness.sluice(
        Path::new(&other.location),
        &["submit", "main", "--no-wait", "--json"],
    );
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_sluice(&bin);
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let answer = seam.submit(&place, &from_worktree(&head));
    let record = harness.record("fix/one").expect("sluice's entry");
    assert_eq!(record["state"], "conflict", "{record}");
    let Submitted::Decided {
        submission,
        outcome: SubmissionOutcome::Conflict { files, base, words },
    } = answer
    else {
        panic!("a conflict at the submit was answered {answer:?}");
    };
    assert_eq!(submission.id, record["id"].to_string(), "the entry");
    assert!(
        matches!(submission.state, SubmissionState::Conflict { .. }),
        "{submission:?}"
    );
    assert_eq!(files, vec!["work.rs".to_string()], "the files contested");
    let ahead = harness.record("other").expect("the entry ahead")["id"].to_string();
    assert!(
        words.starts_with("[sluice] ")
            && words.contains(&format!("conflicts on main with #{ahead} in work.rs")),
        "the words are not sluice's notice: {words}"
    );
    let expected = record["tested"]["on"].as_str().map_or_else(
        || Revision::new(harness.repository.git(&["rev-parse", "main"])),
        Revision::new,
    );
    assert_eq!(base, expected, "the base revision");
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, once the answer delivered the outcome"
    );
}

#[test]
fn a_settle_beside_the_stand_ins_submit_collects_at_most_the_outcome_its_answer_gives() {
    // While the stand-in's first submit waits on sluice's host process,
    // which decides it, a settle beside it collects the place's inbox, as
    // the agent's Stop hook and a caller's settle do, stepping nothing
    // (124, 125). The conflict
    // reaches flume in the submit's answer with its own words. sluice
    // keeps it owed until the profile confirms it after the answer, so an
    // inbox in between may hand it over too, once, then confirming by key
    // what it read; the core acts on it once whichever gives it first
    // (112, 133; sluice 51; sluice record 0102; core.58). Once the answer
    // or the inbox confirmed it, the inbox gives nothing.
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.on_main("work.rs", "the base's own\n");
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_sluice(&bin);
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let (answer, collected) = std::thread::scope(|scope| {
        let submit = scope.spawn(|| seam.submit(&place, &from_worktree(&head)));
        let mut collected = vec![];
        while !submit.is_finished() {
            if let Inbox::Outcomes { outcomes, .. } = seam.inbox(&place) {
                collected.extend(outcomes);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        (submit.join().expect("the submit"), collected)
    });
    let Submitted::Decided {
        outcome: SubmissionOutcome::Conflict { files, words, .. },
        ..
    } = answer
    else {
        panic!("the submit was answered {answer:?}");
    };
    assert!(
        collected.len() <= 1
            && collected.iter().all(|outcome| matches!(
                outcome,
                SubmissionOutcome::Conflict { files: beside, .. } if beside == &files
            )),
        "the inbox beside the submit gave more than the answer's outcome once: {collected:?}"
    );
    assert_eq!(files, vec!["work.rs".to_string()], "the files contested");
    assert!(
        words.starts_with("[sluice] ") && words.contains("conflicts on main in work.rs"),
        "the record's own words, sluice's notice: {words}"
    );
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, once the answer delivered the outcome"
    );
}

#[test]
fn a_pull_request_with_no_entry_is_held_and_the_entry_taken_in_from_it_is_read_to_its_landing() {
    // On a base that takes pull requests, sluice answers the submit with
    // the request and no entry: submit answers it held, never refused.
    // Once an intake takes it in, read-submission finds the entry sluice
    // records from `pull/<n>`, the place's branch its `from.head`, and
    // reads it to its landing; the inbox gives nothing, the outcome
    // having gone to the request.
    let harness = Harness::new();
    let (place, head) = harness.place();
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_pull_requests(&bin);
    let path = std::env::join_paths(std::iter::once(bin.clone()).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let request = super::walk::PULL_REQUEST.to_string();
    assert_eq!(
        seam.submit(&place, &from_worktree(&head)),
        Submitted::Requested {
            request: request.clone()
        }
    );
    assert_eq!(
        seam.read_submission(&place.repository, &place.branch),
        SubmissionRead::None.into(),
        "before it is taken in"
    );
    let mut take_in = Command::new(bin.join("sluice"));
    for (name, _) in std::env::vars() {
        if name.starts_with("SLUICE_")
            || name.starts_with("HERDR_")
            || name.starts_with("REX_")
            || name.starts_with("TMUX")
        {
            take_in.env_remove(name);
        }
    }
    let took = take_in
        .arg("take-in")
        .current_dir(&harness.repository.dir)
        .env("PATH", &path)
        .env("HOME", &harness.home)
        .env("XDG_STATE_HOME", harness.home.join("state"))
        .env("XDG_CONFIG_HOME", harness.home.join("config"))
        .env(USER_SCRATCH, user_scratch(&harness.repository))
        .stdin(Stdio::null())
        .output()
        .expect("the stand-in takes the request in");
    assert!(took.status.success(), "{took:?}");
    let entry = harness.record("fix/one").expect("the entry taken in");
    let SubmissionRead::Found { submission } =
        seam.read_submission(&place.repository, &place.branch).read
    else {
        panic!("the entry taken in is not read");
    };
    assert_eq!(submission.id, entry["id"].to_string(), "the entry");
    assert_eq!(submission.request.as_deref(), Some(request.as_str()));
    assert_eq!(submission.target, "main");
    assert!(submission.state.open(), "{submission:?}");
    harness.decide("fix/one");
    let landed = harness.record("fix/one").expect("the entry decided");
    assert_eq!(landed["state"], "landed", "{landed}");
    assert_eq!(
        seam.read_submission(&place.repository, &place.branch).read,
        SubmissionRead::Found {
            submission: Submission {
                state: SubmissionState::Landed {
                    commit: Revision::new(landed["landed_as"].as_str().expect("its commit")),
                },
                ..submission
            }
        },
        "the landing"
    );
    assert_eq!(
        seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, the outcome delivered to the request"
    );
}

#[test]
fn a_branch_whose_records_sluice_cannot_read_is_passed_over_and_the_landing_collected() {
    // sluice answers the inbox with every other branch's outcomes and the
    // branch it could not read in its `unread`, exiting as a failure:
    // the inbox passes that branch over with sluice's words, and the
    // place's landing on main is collected once (133).
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.seam.submit(&place, &from_worktree(&head));
    let other = harness.refuse_another(&place);
    harness.decide("fix/one");
    let landed = harness.record("fix/one").expect("the entry decided");
    assert_eq!(landed["state"], "landed", "{landed}");
    let commit = Revision::new(landed["landed_as"].as_str().expect("its commit"));
    let Inbox::Outcomes {
        outcomes,
        passed_over,
    } = harness.seam.inbox(&place)
    else {
        panic!("the inbox past {other} could not be read");
    };
    assert_eq!(
        outcomes,
        vec![SubmissionOutcome::Landed {
            commit: commit.clone()
        }],
        "the landing collected past {other}"
    );
    let branches: Vec<&str> = passed_over
        .iter()
        .map(|over| over.branch.as_str())
        .collect();
    assert_eq!(branches, vec![other.as_str()], "{passed_over:?}");
    assert!(
        passed_over[0].words.starts_with("sluice inbox: "),
        "{passed_over:?}"
    );
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
        panic!("the second inbox past {other} could not be read");
    };
    assert_eq!(outcomes, vec![], "the second collect");
    let across = harness
        .seam
        .read_submission(&place.repository, &place.branch);
    assert_eq!(
        across
            .passed_over
            .iter()
            .map(|over| over.branch.as_str())
            .collect::<Vec<_>>(),
        vec![other.as_str()],
        "{across:?}"
    );
    assert!(
        matches!(
            across.read,
            SubmissionRead::Found {
                submission: Submission {
                    state: SubmissionState::Landed { .. },
                    ..
                }
            }
        ),
        "{across:?}"
    );
}

#[test]
fn a_fetch_that_fails_is_answered_with_its_words() {
    let harness = Harness::new();
    let (place, _) = harness.place();
    harness.base_elsewhere(&place.repository, "main");
    harness.fetch_fails(&place.repository, "main");
    match harness.seam.base_revision(&place.repository, "main") {
        BaseRevision::Refused { reason } => assert!(
            reason.contains("fetch") && reason.contains("origin"),
            "the refusal names not the fetch: {reason}"
        ),
        other => panic!("a failed fetch gave {other:?}"),
    }
}

#[test]
fn what_the_policy_says_of_a_place() {
    let harness = Harness::new();
    let repository = harness.repository.name();
    let launched = harness.on_main(
        ".config/sluice.toml",
        "[tests]\nrunner = \"moon\"\ntoolchain = [\"devenv\", \"print-dev-env\"]\n\n[branches.main]\nremote = \"origin\"\n",
    );
    let equip = format!("sluice equip main --at {} --json", launched.0);
    let place = for_place(&repository, "main", Some(&launched)).expect("the policy");
    assert_eq!(place.equip.as_deref(), Some(equip.as_str()));
    assert_eq!(place.teardown, "sluice release");
    assert_eq!(place.base_remote.as_deref(), Some("origin"));
    let data =
        place_data(&repository, Some("sluice"), "main", Some(&launched)).expect("the place's data");
    let readiness: Vec<(&str, &str, bool)> = data
        .readiness
        .iter()
        .map(|step| {
            (
                step.name.as_str(),
                step.command.as_str(),
                step.reading.is_some(),
            )
        })
        .collect();
    assert_eq!(
        readiness,
        vec![(EQUIP, equip.as_str(), true)],
        "a readiness of more than sluice's equip, read"
    );
    let teardown: Vec<&str> = data
        .teardown
        .iter()
        .map(|step| step.command.as_str())
        .collect();
    assert_eq!(teardown, vec!["sluice release"]);
    assert_eq!(data.base_remote.as_deref(), Some("origin"));

    // The pinned sluice equips the place as the step asks it, at the
    // revision named, and its answer is read as the readiness's steps.
    let equipping = harness.on_main(
        ".config/sluice.toml",
        "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\ntoolchain = [\"printf\", \"export READY=1\\\\n\"]\n\n[branches.main]\n",
    );
    let data = place_data(&repository, Some("sluice"), "main", Some(&equipping))
        .expect("the place's data");
    // The login environment gives the harness's state folder and user
    // scratch, as a walk's login shell's sluice does, so the step's sluice
    // reaches the harness's host process, its socket in the test's scratch
    // (core.51).
    let path = std::env::var("PATH").unwrap_or_default();
    let profile = GitWorktree::with_login(
        data,
        Login::given(&[
            ("HOME", &harness.home.to_string_lossy()),
            ("PATH", &path),
            ("LANG", "C"),
            (
                "XDG_STATE_HOME",
                &harness.home.join("state").to_string_lossy(),
            ),
            (
                USER_SCRATCH,
                &user_scratch(&harness.repository).to_string_lossy(),
            ),
        ]),
    );
    let readiness = profile.ready(&harness.place, &equipping);
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    assert_eq!(steps, vec![("toolchain", true)], "{readiness:?}");
}

/// A `sluice` first on the walk's `PATH` that writes each command it is
/// asked, a line each after the process that asked it, then runs the
/// pinned sluice as the test's own `PATH` finds it.
fn counting_sluice(walk: &Walk) -> PathBuf {
    let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("sluice"))
        .find(|path| path.is_file())
        .expect("the pinned sluice on PATH");
    let asked = walk.repository.scratch.path("sluice-asked");
    let program = walk.repository.scratch.path("bin").join("sluice");
    fs::write(
        &program,
        format!(
            "#!/bin/sh\necho \"$PPID $*\" >>'{}'\nexec '{}' \"$@\"\n",
            asked.display(),
            pinned.display()
        ),
    )
    .expect("the counting sluice");
    fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("it runs");
    asked
}

/// The commands the counting sluice was asked, in order, each with the
/// process that asked it.
fn asked(at: &Path) -> Vec<(i32, String)> {
    fs::read_to_string(at)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let (by, command) = line.split_once(' ')?;
            Some((by.parse().ok()?, command.to_string()))
        })
        .collect()
}

/// Waits until the process that submitted main, the settle the agent's
/// Stop hook started detached once the agent was quiet (121), has ended,
/// so nothing but the test's own commands moves the place after it.
fn until_the_quiet_settle_ends(log: &Path) {
    let deadline = Instant::now() + Duration::from_secs(120);
    let settle = loop {
        if let Some((by, _)) = asked(log)
            .into_iter()
            .find(|(_, command)| command.starts_with("submit main"))
        {
            break by;
        }
        assert!(Instant::now() < deadline, "nothing submitted main");
        std::thread::sleep(Duration::from_millis(200));
    };
    loop {
        // SAFETY: signal 0 to a process of the test's own user only asks
        // whether it stands.
        if unsafe { libc::kill(settle, 0) } != 0 {
            let error = std::io::Error::last_os_error();
            assert_eq!(
                error.raw_os_error(),
                Some(libc::ESRCH),
                "the settle that submitted main, {settle}: {error}"
            );
            return;
        }
        assert!(
            Instant::now() < deadline,
            "the settle that submitted main, {settle}, never ended"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

#[test]
fn s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone() {
    // S109: a chore's agent commits and ends, and the settle its quiet
    // starts submits it; from then on the test runs nothing but a
    // caller's watch, `flume settle --every 1m`, and no `sluice run` or
    // `sluice daemon` is live. The chore's check is held until the
    // watch's first settle has read its submission open; sluice's host
    // process, which the submit started, then lands it, no settle running
    // a round of the queue, with no agent started after its code stage,
    // and the settle that reads the landing tidies it (46, 121, 124, 125,
    // 208, C.6; sluice 135).
    let walk = Walk::new("watch-chore");
    let log = counting_sluice(&walk);
    walk.hold();
    let place = walk.launch("chore", "one", &["--brief", "tidy the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'tidy the parser'" },
        ]),
    );
    until_the_quiet_settle_ends(&log);
    let submitted = walk.status(&place);
    assert_eq!(submitted["submission"]["submits"], 1, "{submitted}");
    let open = |when: &str| {
        let entry = walk.submissions();
        assert_eq!(entry.len(), 1, "{entry:?}");
        assert!(
            matches!(
                entry[0]["state"].as_str(),
                Some("queued" | "testing" | "landing")
            ),
            "the chore was decided {when}: {entry:?}"
        );
    };
    open("before the watch");
    let watched = asked(&log).len();
    let began = Instant::now();
    let mut settles = 0;
    loop {
        walk.flume(&["settle", "--every", "1m"]);
        settles += 1;
        let status = walk.status(&place);
        if settles == 1 {
            assert!(!status.is_null(), "the chore was tidied while held");
            open("at the watch's first settle");
            walk.release();
        }
        if status.is_null() {
            break;
        }
        assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
        assert!(status["agent"].is_null(), "an agent of the chore: {status}");
        assert_eq!(
            walk.host()["watch"],
            json!({"bound": {"seconds": 60}}),
            "the host's watch"
        );
        assert!(
            began.elapsed() < Duration::from_secs(120),
            "the chore never landed after {settles} settles: {status}"
        );
        std::thread::sleep(Duration::from_millis(300));
    }

    // No settle, the quiet one or the watch's, ran a round of sluice's
    // queue or waited on it, and the watch submitted nothing.
    let asked = asked(&log);
    let moving: Vec<&(i32, String)> = asked
        .iter()
        .filter(|(_, command)| {
            matches!(
                command.split_whitespace().next(),
                Some("step" | "run" | "daemon" | "wait")
            )
        })
        .collect();
    assert!(
        moving.is_empty(),
        "a sluice command that moves a queue: {moving:?}"
    );
    assert!(
        !asked[watched..]
            .iter()
            .any(|(_, command)| command.starts_with("submit")),
        "a submit under the watch: {asked:?}"
    );

    // sluice landed flume's one submission of the chore on the remote, and
    // the settle that read it tidied the place and its branch.
    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{submissions:?}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(!Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("chore/one"), "the place's branch is left");
}

/// The pid of the host process the walk's state folder records, as the
/// pinned sluice writes it in `host/version`.
fn host_pid(walk: &Walk) -> Option<i32> {
    let version = fs::read_to_string(walk.home().join("state/sluice/host/version")).ok()?;
    let named: Value = serde_json::from_str(&version).ok()?;
    i32::try_from(named["pid"].as_u64()?).ok()
}

#[test]
fn s109_a_host_process_ended_with_work_in_play_is_started_again_by_a_settles_read() {
    // S109 and sluice 135: the chore's check is held, and sluice's host
    // process, which its submit started, is ended while the check waits,
    // its run outliving it. The hold then goes, and the watch's first
    // settle reads the submission, which starts a host process again;
    // that one lands the chore, and a settle reads the landing and tidies
    // it, no settle running a round of the queue (124, 208, S109).
    let walk = Walk::new("watch-restart");
    let log = counting_sluice(&walk);
    walk.hold();
    let place = walk.launch("chore", "one", &["--brief", "tidy the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'tidy the parser'" },
        ]),
    );
    until_the_quiet_settle_ends(&log);
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let entry = walk.submissions();
        assert_eq!(entry.len(), 1, "{entry:?}");
        if entry[0]["state"] == "testing" {
            break;
        }
        assert!(
            Instant::now() < deadline && entry[0]["state"] == "queued",
            "the chore's held check never ran: {entry:?}"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
    // Once its round has recorded the run started, as sluice's own test
    // of a host process killed under a run waits.
    std::thread::sleep(Duration::from_secs(1));
    let ended = host_pid(&walk).expect("the host process the state folder records");
    // SAFETY: a signal to the host process of the walk's own state folder.
    unsafe { libc::kill(ended, libc::SIGKILL) };
    let deadline = Instant::now() + Duration::from_secs(30);
    // SAFETY: signal 0 only asks whether the process stands.
    while unsafe { libc::kill(ended, 0) } == 0 {
        assert!(
            Instant::now() < deadline,
            "the host process {ended} never ended"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    walk.release();

    let watched = asked(&log).len();
    walk.flume(&["settle", "--every", "1m"]);
    let started = host_pid(&walk);
    assert!(
        started.is_some_and(|pid| pid != ended),
        "the settle's read started no host process: {started:?} after {ended}"
    );
    let began = Instant::now();
    loop {
        let status = walk.status(&place);
        if status.is_null() {
            break;
        }
        assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
        assert!(
            began.elapsed() < Duration::from_secs(120),
            "the chore never landed: {status}"
        );
        std::thread::sleep(Duration::from_millis(300));
        walk.flume(&["settle", "--every", "1m"]);
    }

    let asked = asked(&log);
    assert!(
        !asked
            .iter()
            .any(|(_, command)| command.split_whitespace().next() == Some("step")),
        "a settle stepped the queue: {asked:?}"
    );
    assert!(
        !asked[watched..]
            .iter()
            .any(|(_, command)| command.starts_with("submit")),
        "a submit under the watch: {asked:?}"
    );
    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{submissions:?}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(!Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("chore/one"), "the place's branch is left");
}
