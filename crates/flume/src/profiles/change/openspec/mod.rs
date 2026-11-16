//! The change profile (C.3; design 8): a change read at a revision.
//! Whether a standing change's planning is complete is OpenSpec's own
//! reading of it, whatever its schema, through the pinned `openspec
//! status --change <name> --json`, run in a checkout of the revision's
//! `openspec/` made for the read and removed after it; whether it
//! exists, its tasks and whether it is archived are read from its files
//! with git alone. Reading writes nothing to the place or the repository
//! (59, 86, 87). A change lies at `openspec/changes/<name>` in the place
//! (127).

use crate::git::Git;
use flume_seams::{Change, ChangeFacts, ChangeRead, Revision, Task, TaskGroup};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The profile's name in a repository's binding, `change` (105).
pub const NAME: &str = "openspec";

const CHANGES: &str = "openspec/changes";
const ARCHIVE: &str = "openspec/changes/archive";

/// The change profile over OpenSpec.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenSpec;

impl Change for OpenSpec {
    fn read_change(&self, repository: &str, revision: &Revision, name: &str) -> ChangeRead {
        match read(repository, revision, name) {
            Ok(facts) => ChangeRead::Read { facts },
            Err(reason) => ChangeRead::CouldNotRead { reason },
        }
    }

    fn path(&self, name: &str) -> String {
        format!("{CHANGES}/{name}")
    }
}

fn read(repository: &str, revision: &Revision, name: &str) -> Result<ChangeFacts, String> {
    let git = Git::at(repository);
    git.commit(&revision.0)?;
    let directory = format!("{CHANGES}/{name}");
    let exists = !git.files(&revision.0, &directory)?.is_empty();
    let archived_as = archived(&git, revision, name)?;
    let read_from = match (&archived_as, exists) {
        (Some(archived), false) => archived.clone(),
        _ => directory,
    };
    let (tasks, groups) = match git.file(&revision.0, &format!("{read_from}/tasks.md"))? {
        Some(text) => tasks(&text),
        None => (vec![], vec![]),
    };
    // A standing change's planning complete is OpenSpec's own reading,
    // whatever its schema, and not planned where OpenSpec reads no such
    // change (59, C.3). An archived change that no longer stands was
    // planned before it was archived, and OpenSpec reads only a change
    // that stands; a change neither standing nor archived is not planned,
    // with no `openspec` run.
    let planning_complete = if exists {
        status(&git, revision, name)?.unwrap_or(false)
    } else {
        archived_as.is_some()
    };
    Ok(ChangeFacts {
        exists,
        planning_complete,
        tasks,
        archived: archived_as.is_some(),
        groups,
    })
}

/// The archive's directory for the change, `<date>-<name>`, where the
/// revision holds one; the newest where it holds several.
fn archived(git: &Git, revision: &Revision, name: &str) -> Result<Option<String>, String> {
    let suffix = format!("-{name}");
    let mut held: Vec<String> = git
        .names(&revision.0, ARCHIVE)?
        .into_iter()
        .filter(|entry| {
            entry.strip_suffix(&suffix).is_some_and(|date| {
                date.len() == 10
                    && date.bytes().enumerate().all(|(n, b)| {
                        matches!(n, 4 | 7) == (b == b'-') && (b == b'-' || b.is_ascii_digit())
                    })
            })
        })
        .collect();
    held.sort();
    Ok(held.pop().map(|entry| format!("{ARCHIVE}/{entry}")))
}

/// The `- [ ]` and `- [x]` lines of a tasks file, in order, each in
/// its task group: a `## <n>. <title>` heading starts a group holding the
/// tasks under it, the tasks above every heading are a group with no
/// number or title, and any other heading starts no group (86, C.3).
pub fn tasks(text: &str) -> (Vec<Task>, Vec<TaskGroup>) {
    let mut tasks = vec![];
    let mut groups = vec![];
    for line in text.lines() {
        if let Some(group) = heading(line) {
            groups.push(group);
            continue;
        }
        let Some((text, ticked)) = checkbox(line) else {
            continue;
        };
        if groups.is_empty() {
            groups.push(TaskGroup::default());
        }
        tasks.push(Task {
            text,
            ticked,
            group: Some(groups.len() - 1),
        });
    }
    (tasks, groups)
}

/// A `## <n>. <title>` heading's group, its number the digits and its
/// title what follows the dot.
fn heading(line: &str) -> Option<TaskGroup> {
    let rest = line.strip_prefix("## ")?.trim_start();
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    let (number, rest) = rest.split_at(digits);
    let title = rest.strip_prefix('.')?;
    if number.is_empty() || !title.starts_with(char::is_whitespace) {
        return None;
    }
    let title = title.trim();
    (!title.is_empty()).then(|| TaskGroup {
        number: Some(number.to_string()),
        title: Some(title.to_string()),
    })
}

/// A `- [ ]` or `- [x]` line's text and whether it is ticked.
fn checkbox(line: &str) -> Option<(String, bool)> {
    let line = line.trim_start();
    let rest = line
        .strip_prefix("- [")
        .or_else(|| line.strip_prefix("* ["))?;
    let mut chars = rest.chars();
    let mark = chars.next()?;
    let rest = chars.as_str().strip_prefix(']')?;
    let ticked = match mark {
        ' ' => false,
        'x' | 'X' => true,
        _ => return None,
    };
    let text = rest.strip_prefix(' ').unwrap_or(rest).trim_end();
    Some((text.to_string(), ticked))
}

static CHECKOUTS: AtomicUsize = AtomicUsize::new(0);

/// A checkout of a revision's `openspec/`, made for one read and
/// removed when dropped.
struct Checkout(PathBuf);

impl Drop for Checkout {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl Checkout {
    fn of(git: &Git, revision: &Revision) -> Result<Checkout, String> {
        let directory = std::env::temp_dir().join(format!(
            "flume-openspec-{}-{}",
            std::process::id(),
            CHECKOUTS.fetch_add(1, Ordering::Relaxed)
        ));
        let checkout = Checkout(directory);
        let _ = std::fs::remove_dir_all(&checkout.0);
        std::fs::create_dir_all(&checkout.0)
            .map_err(|error| format!("a checkout for the read could not be made: {error}"))?;
        let archive = git.bytes(&["archive", "--format=tar", &revision.0, "--", "openspec"])?;
        let mut tar = Command::new("tar")
            .arg("-x")
            .arg("-C")
            .arg(&checkout.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("tar could not run: {error}"))?;
        {
            use std::io::Write;
            let mut stdin = tar.stdin.take().ok_or("tar took no input")?;
            stdin
                .write_all(&archive)
                .map_err(|error| format!("the checkout could not be written: {error}"))?;
        }
        let done = tar
            .wait_with_output()
            .map_err(|error| format!("tar could not run: {error}"))?;
        if !done.status.success() {
            return Err(format!(
                "the checkout could not be written: {}",
                String::from_utf8_lossy(&done.stderr).trim()
            ));
        }
        Ok(checkout)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

/// Whether OpenSpec reads the change's planning complete at the
/// revision, through the pinned `openspec status`; `None` where it reads
/// no such change (59).
pub fn status(git: &Git, revision: &Revision, name: &str) -> Result<Option<bool>, String> {
    let checkout = Checkout::of(git, revision)?;
    let output = Command::new("openspec")
        .args(["status", "--change", name, "--json"])
        .current_dir(checkout.path())
        .env("OPENSPEC_TELEMETRY", "0")
        .env("OPENSPEC_NO_UPDATE_CHECK", "1")
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("openspec could not run: {error}"))?;
    let read: serde_json::Value = match serde_json::from_slice(&output.stdout) {
        Ok(read) => read,
        Err(_) => {
            return Err(format!(
                "openspec status --change {name} gave no answer: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    };
    match read.get("isPlanningComplete").and_then(|v| v.as_bool()) {
        Some(complete) => Ok(Some(complete)),
        None if !output.status.success() => Ok(None),
        None => Err(format!(
            "openspec status --change {name} named no isPlanningComplete"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(text: &str, ticked: bool, group: usize) -> Task {
        Task {
            text: text.into(),
            ticked,
            group: Some(group),
        }
    }

    fn group(number: &str, title: &str) -> TaskGroup {
        TaskGroup {
            number: Some(number.into()),
            title: Some(title.into()),
        }
    }

    #[test]
    fn a_tasks_file_gives_its_checkbox_lines_in_order() {
        let text = "# Tasks\n\n## 1. One\n\n- [ ] 1.1 first\n- [x] 1.2 second\n  - [X] 1.3 nested\nnot - [ ] a task\n- [~] 1.4 neither\n";
        assert_eq!(
            tasks(text),
            (
                vec![
                    task("1.1 first", false, 0),
                    task("1.2 second", true, 0),
                    task("1.3 nested", true, 0),
                ],
                vec![group("1", "One")]
            )
        );
    }

    #[test]
    fn groups_are_read_from_their_headings() {
        let text = "# Tasks\n\n## 1. Parser\n\n- [x] 1.1 the grammar\n- [ ] 1.2 the parser\n\n## 2. Tests\n\n- [ ] 2.1 its tests\n";
        assert_eq!(
            tasks(text),
            (
                vec![
                    task("1.1 the grammar", true, 0),
                    task("1.2 the parser", false, 0),
                    task("2.1 its tests", false, 1),
                ],
                vec![group("1", "Parser"), group("2", "Tests")]
            )
        );
    }

    #[test]
    fn tasks_above_every_heading_are_a_group_of_their_own() {
        let text = "# Tasks\n\n- [ ] first\n\n## 1. Parser\n\n- [ ] 1.1 the parser\n";
        assert_eq!(
            tasks(text),
            (
                vec![task("first", false, 0), task("1.1 the parser", false, 1)],
                vec![TaskGroup::default(), group("1", "Parser")]
            )
        );
        assert_eq!(
            tasks("- [x] alone\n"),
            (vec![task("alone", true, 0)], vec![TaskGroup::default()]),
            "a file with no heading"
        );
        assert_eq!(tasks("# Tasks\n"), (vec![], vec![]), "no task, no group");
    }

    #[test]
    fn any_other_heading_starts_no_group() {
        let text = "## 1. Parser\n- [ ] 1.1 a\n### 1.2 detail\n## Notes\n## 2.no space\n## 3.\n# 4. Top\n##5. Tight\n- [ ] 1.2 b\n";
        assert_eq!(
            tasks(text),
            (
                vec![task("1.1 a", false, 0), task("1.2 b", false, 0)],
                vec![group("1", "Parser")]
            )
        );
        assert_eq!(
            heading("## 12.  Many  words "),
            Some(group("12", "Many  words"))
        );
    }
}
