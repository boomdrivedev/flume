//! The merge-queue profile over sluice reading the documents sluice
//! `44e33d4` prints, which the pinned 0.93.2 never prints since it holds
//! no landing: a stand-in `sluice` under each test's scratch folder, and a
//! stand-in sluice host on a socket of the test's own for the reads, log
//! each command and ask, a line each, and answer from the fixtures under
//! `fixtures/sluice/`, whose `README.md` names the code each follows (99,
//! 133, C.6; core.22, core.58; flume-reads-sluices-records-on-sluice-
//! hosts-socket).

use crate::common::*;
use flume::profiles::merge_queue::sluice::{Sluice, HELD_UNSAID};
use flume_seams::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// Where the fixtures are.
const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/profiles/fixtures/sluice"
);

/// The words the forge gave for the refused landing, as the fixtures hold
/// them.
const FORGE: &str = "GH013: Repository rule violations found for refs/heads/main. Push cannot contain secrets: a GitHub Personal Access Token in src/config.rs:12";

/// A scratch repository and place, and the profile over a stand-in
/// `sluice` that answers from the fixtures it was given.
struct StandIn {
    repository: Repository,
    seam: Sluice,
    place: PlaceRef,
    log: PathBuf,
}

impl StandIn {
    /// The stand-in answering each command by the fixture named for it:
    /// each `(answer, fixture)` copies `fixture` as the answer the
    /// stand-in gives under `answer`, `overview.json`, `status-<branch
    /// with / as ->.json`, `inbox.json` or `confirm.json`; any other
    /// command is refused in sluice's error document.
    fn new(answers: &[(&str, &str)]) -> StandIn {
        let repository = Repository::new("sluice-documents");
        let folder = repository.scratch.path("stand-in");
        let (bin, given) = (folder.join("bin"), folder.join("answers"));
        for dir in [&bin, &given] {
            fs::create_dir_all(dir).expect("the stand-in's folders");
        }
        for (answer, fixture) in answers {
            fs::copy(format!("{FIXTURES}/{fixture}"), given.join(answer)).expect("a fixture");
        }
        let log = folder.join("asked");
        let program = bin.join("sluice");
        fs::write(
            &program,
            format!(
                r#"#!/bin/sh
printf '%s\n' "$*" >>'{log}'
answers='{answers}'
case "$*" in
  "overview --json") file=overview.json ;;
  "status "*" --all --json") file="status-$(printf '%s' "$2" | tr / -).json" ;;
  "inbox --json") file=inbox.json ;;
  "confirm "*" --json") file=confirm.json ;;
  *) file= ;;
esac
if [ -n "$file" ] && [ -f "$answers/$file" ]; then
  cat "$answers/$file"
  exit 0
fi
printf '{{"error": {{"kind": "refused", "message": "the stand-in answers no %s", "next": []}}}}\n' "$1"
exit 2
"#,
                log = log.display(),
                answers = given.display(),
            ),
        )
        .expect("the stand-in");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("it runs");
        let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        )))
        .expect("a PATH");
        let host = host_of(&folder, &given, &log);
        let seam = Sluice::new()
            .with_env("PATH", Some(&path.to_string_lossy()))
            .with_env(
                "XDG_STATE_HOME",
                Some(&folder.join("state").to_string_lossy()),
            )
            .with_env("SLUICE_USER_SCRATCH", Some(&host.to_string_lossy()));
        let location = repository.scratch.path("places/fix--one");
        fs::create_dir_all(&location).expect("the place's folder");
        let place = PlaceRef {
            repository: repository.dir.to_string_lossy().into_owned(),
            location: location.to_string_lossy().into_owned(),
            branch: "fix/one".into(),
            base: "main".into(),
        };
        StandIn {
            repository,
            seam,
            place,
            log,
        }
    }

    /// Each command the stand-in was asked, in order.
    fn asked(&self) -> Vec<String> {
        fs::read_to_string(&self.log)
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }

    /// read-submission of `branches` at once.
    fn read(&self, branches: &[&str]) -> Vec<ReadAcross> {
        self.seam
            .read_submissions(&self.repository.dir.to_string_lossy(), branches)
    }

    /// Whether the stand-in was asked to wait, or for one submission's
    /// status by its number, which read nothing through a wait (core.6,
    /// core.13).
    fn waited(&self) -> bool {
        self.asked().iter().any(|asked| {
            let mut words = asked.split_whitespace();
            match (words.next(), words.next()) {
                (Some("wait"), _) => true,
                (Some("status"), Some(second)) => second.parse::<u64>().is_ok(),
                (Some("submit"), _) => !asked.contains("--no-wait"),
                _ => false,
            }
        })
    }
}

/// A stand-in sluice host for the folder `folder`'s tests, its socket in
/// a short folder of its own, which it gives: at each connection it reads
/// the hello, answers with a version, then answers one read, `overview`
/// from `overview.json` and `status` of a target from
/// `status-<target with / as ->.json` in `answers`, logging it to `log`
/// as the command it stands for; any other ask is an `error` line. Its
/// thread ends with the test's process.
fn host_of(folder: &std::path::Path, answers: &std::path::Path, log: &std::path::Path) -> PathBuf {
    use std::io::{BufRead, BufReader, Write};
    let state = folder.join("state/sluice");
    fs::create_dir_all(&state).expect("sluice's state folder");
    let user = crate::common::resident::user_scratch();
    let socket =
        flume::profiles::merge_queue::sluice::subscription::socket_from(|name| match name {
            "XDG_STATE_HOME" => Some(folder.join("state").into()),
            "SLUICE_USER_SCRATCH" => Some(user.clone().into()),
            _ => None,
        })
        .expect("the stand-in host's socket");
    let listener = std::os::unix::net::UnixListener::bind(&socket)
        .unwrap_or_else(|error| panic!("no room for {}: {error}", socket.display()));
    let (answers, log) = (answers.to_path_buf(), log.to_path_buf());
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut writer = stream.try_clone().expect("the connection");
            let mut lines = BufReader::new(stream).lines();
            let _hello = lines.next();
            writeln!(writer, r#"{{"version":"0.110.0","pid":1,"since":"now"}}"#).ok();
            let Some(Ok(line)) = lines.next() else {
                continue;
            };
            let ask: serde_json::Value = serde_json::from_str(&line).expect("an ask");
            let (said, file) = match ask["ask"].as_str() {
                Some("overview") => ("overview --json".to_string(), "overview.json".to_string()),
                Some("status") => {
                    let target = ask["target"].as_str().unwrap_or_default();
                    (
                        format!("status {target} --all --json"),
                        format!("status-{}.json", target.replace('/', "-")),
                    )
                }
                _ => {
                    writeln!(writer, r#"{{"error":"the stand-in answers no {line}"}}"#).ok();
                    continue;
                }
            };
            let mut logged = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log)
                .expect("the log");
            writeln!(logged, "{said}").ok();
            let output = match fs::read_to_string(answers.join(&file)) {
                Ok(stdout) => serde_json::json!({"output": {"stdout": stdout, "code": 0}}),
                Err(_) => serde_json::json!({"output": {
                    "stdout": format!(r#"{{"error": {{"kind": "refused", "message": "the stand-in answers no {said}", "next": []}}}}"#),
                    "code": 2,
                }}),
            };
            writeln!(writer, "{output}").ok();
        }
    });
    user
}

/// A submission of `branch` to main, number `id`, held with `words`.
fn held(id: &str, branch: &str, words: &str) -> SubmissionRead {
    SubmissionRead::Found {
        submission: Submission {
            id: id.into(),
            branch: branch.into(),
            target: "main".into(),
            state: SubmissionState::Held {
                words: words.into(),
            },
            request: None,
        },
    }
}

/// The landing the inbox fixtures give, #6 on main.
fn landed() -> SubmissionOutcome {
    SubmissionOutcome::Landed {
        commit: Revision::new("1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d1a2b3c4d"),
    }
}

#[test]
fn a_refused_outcome_is_held_with_its_notice_and_owed_until_confirmed() {
    // merge-queue-profile "A refused outcome" (112, 133, C.6): collected
    // with no --confirm, each outcome owed under its key until flume
    // confirms it once it has acted on it (record 0072; core.58).
    let stand_in = StandIn::new(&[
        ("inbox.json", "inbox-refused.json"),
        ("confirm.json", "confirm.json"),
    ]);
    let notice = format!(
        "[sluice] #7's landing on main was refused by the forge and is held: {FORGE}. \
         Withdraw it or submit again from its place, or the operator pushes it again once \
         the forge takes it."
    );
    assert_eq!(
        stand_in.seam.inbox(&stand_in.place),
        Inbox::Outcomes {
            outcomes: vec![
                Owed {
                    submission: "6".into(),
                    outcome: landed(),
                    key: Some("6:landed".into()),
                },
                Owed {
                    submission: "7".into(),
                    outcome: SubmissionOutcome::Held { words: notice },
                    key: Some("7:refused".into()),
                },
            ],
            passed_over: vec![],
        }
    );
    assert_eq!(
        stand_in.asked(),
        vec!["inbox --json"],
        "collected with no --confirm, confirming nothing"
    );
    assert_eq!(
        stand_in.seam.confirm(&stand_in.place, "7:refused"),
        Confirmed::Confirmed
    );
    assert_eq!(
        stand_in.asked(),
        vec!["inbox --json", "confirm 7:refused --json"],
        "confirmed by its key once asked"
    );
}

#[test]
fn an_outcome_in_a_state_flume_does_not_read_is_left_owed_and_named() {
    // merge-queue-profile "An outcome in a state flume does not read"
    // (133; core.58).
    let stand_in = StandIn::new(&[("inbox.json", "inbox-parked.json")]);
    assert_eq!(
        stand_in.seam.inbox(&stand_in.place),
        Inbox::Outcomes {
            outcomes: vec![Owed {
                submission: "6".into(),
                outcome: landed(),
                key: Some("6:landed".into()),
            }],
            passed_over: vec![PassedOver {
                branch: "main".into(),
                words: "#8: sluice gave an outcome in the state \"parked\"".into(),
            }],
        }
    );
    assert_eq!(
        stand_in.asked(),
        vec!["inbox --json"],
        "8:parked left owed, nothing confirmed"
    );
}

#[test]
fn a_refused_record_is_held_with_its_refused_words() {
    // merge-queue-profile "A refused record with its words" (99, C.6).
    let stand_in = StandIn::new(&[
        ("overview.json", "overview.json"),
        ("status-main.json", "status-main-refused-words.json"),
        ("status-train-x.json", "status-train-x.json"),
    ]);
    let read = stand_in.read(&["fix/one"]);
    assert_eq!(
        read,
        vec![ReadAcross {
            read: held("7", "fix/one", FORGE),
            passed_over: vec![],
        }]
    );
    assert!(
        !stand_in.waited(),
        "read through a wait: {:?}",
        stand_in.asked()
    );
}

#[test]
fn a_refused_record_without_them_is_held_saying_sluices_record_gives_none() {
    // merge-queue-profile "A refused record without its words": the
    // place's and the base place's, by `from.head`, each held, neither
    // unread nor passed over (99, 203, C.6; core.58).
    let stand_in = StandIn::new(&[
        ("overview.json", "overview.json"),
        ("status-main.json", "status-main.json"),
        ("status-train-x.json", "status-train-x.json"),
    ]);
    assert_eq!(
        stand_in.read(&["fix/one", "train/x"]),
        vec![
            ReadAcross {
                read: held("7", "fix/one", HELD_UNSAID),
                passed_over: vec![],
            },
            ReadAcross {
                read: held("9", "train/x", HELD_UNSAID),
                passed_over: vec![],
            },
        ]
    );
    assert!(
        HELD_UNSAID.contains("gives none"),
        "the words say sluice's record gives none: {HELD_UNSAID}"
    );
    assert!(
        !stand_in.waited(),
        "read through a wait: {:?}",
        stand_in.asked()
    );
}

#[test]
fn a_bases_records_are_asked_again_only_once_their_version_moved() {
    // merge-queue-profile "Records kept until their version moves": each
    // read asks sluice host the overview, and the status of a base only
    // where its records' version moved since the records kept were read
    // (flume-reads-sluices-records-on-sluice-hosts-socket).
    let stand_in = StandIn::new(&[
        ("overview.json", "overview.json"),
        ("status-main.json", "status-main.json"),
        ("status-train-x.json", "status-train-x.json"),
    ]);
    let first = stand_in.read(&["fix/one"]);
    assert_eq!(stand_in.read(&["fix/one"]), first, "the same records");
    let statuses = |asked: &[String]| {
        asked
            .iter()
            .filter(|asked| asked.starts_with("status "))
            .count()
    };
    assert_eq!(
        stand_in.asked(),
        vec![
            "overview --json",
            "status main --all --json",
            "status train/x --all --json",
            "overview --json",
        ],
        "the second read asked no status"
    );
    let given = stand_in
        .log
        .parent()
        .expect("the stand-in's folder")
        .join("answers");
    let overview = fs::read_to_string(given.join("overview.json")).expect("the overview");
    fs::write(
        given.join("overview.json"),
        overview.replacen("\"version\": 41", "\"version\": 42", 1),
    )
    .expect("main's records moved");
    assert_eq!(stand_in.read(&["fix/one"]), first);
    let asked = stand_in.asked();
    assert_eq!(
        statuses(&asked),
        3,
        "main asked again, train/x not: {asked:?}"
    );
    assert_eq!(
        asked.last().map(String::as_str),
        Some("status main --all --json")
    );
}
