//! A start's environment compared with its record, as an agent's word
//! that it started compares it (161; core.10, core.30): each variable the
//! start's record names that the environment lacks, and each it holds
//! whose digest keyed by the start's secret differs from the record's, by
//! name alone, never a value. Claude Code's started word and flume's
//! launcher each compare through it, and each compares its program, and
//! Claude Code's its session, beside it; and each, finding its start began
//! otherwise, starts a settle of the place detached through it.

use flume_core::variable_digest;
use flume_seams::{Secret, StartRecord, Unsettled};
use std::path::Path;

/// How the environment `variable` reads began otherwise than `recorded`
/// names it under `secret`: each variable it finds unset, each whose
/// digest keyed by `secret` differs, by name alone, in the record's order,
/// and nothing else (161; core.30).
pub fn variables(
    recorded: &StartRecord,
    secret: &Secret,
    variable: impl Fn(&str) -> Option<String>,
) -> Unsettled {
    let mut unsettled = Unsettled::default();
    for (name, digest) in &recorded.env {
        match variable(name) {
            None => unsettled.missing.push(name.clone()),
            Some(value) if &variable_digest(secret, name, &value) != digest => {
                unsettled.different.push(name.clone());
            }
            Some(_) => {}
        }
    }
    unsettled
}

/// `flume settle <place>` run by `binary`, with `--on-quiet` where
/// `on_quiet`, in a session of its own that outlives the hook or the
/// launcher that starts it and the agent's terminal, its output nowhere,
/// as a start found begun otherwise starts one (161). A process group of
/// its own is not enough: a terminal closing the agent's pane, as herdr
/// 0.9.5 does once the settle ends the agent, ends every process of the
/// pane's session, so a settle killed between claiming its place's submit
/// and recording the merge queue's entry would leave the place submitted
/// with no entry to read, never stepped again (46, 111).
pub fn settle_detached(binary: &Path, place: &str, on_quiet: bool) {
    use std::process::{Command, Stdio};
    let mut args = vec!["settle", place];
    if on_quiet {
        args.push("--on-quiet");
    }
    let mut command = Command::new(binary);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let _ = in_a_session_of_its_own(command);
}

/// `command` started as the leader of a session of its own, which no
/// terminal's session the caller runs in reaches.
fn in_a_session_of_its_own(
    mut command: std::process::Command,
) -> std::io::Result<std::process::Child> {
    use std::os::unix::process::CommandExt;
    // SAFETY: setsid is async-signal-safe and touches no memory of the
    // parent's; it runs in the child between fork and exec.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    command.spawn()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_detached_settle_leads_a_session_of_its_own() {
        let mut command = std::process::Command::new("/bin/sleep");
        command.arg("5");
        let mut child = in_a_session_of_its_own(command).expect("sleep starts");
        let pid = child.id() as libc::pid_t;
        // SAFETY: getsid reads the session of a process this test started.
        let (session, ours) = unsafe { (libc::getsid(pid), libc::getsid(0)) };
        let _ = child.kill();
        let _ = child.wait();
        assert_eq!(session, pid, "the child leads its own session");
        assert_ne!(session, ours, "the child shares the test's session");
    }
}
