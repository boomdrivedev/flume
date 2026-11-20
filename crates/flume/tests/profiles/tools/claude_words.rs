//! What the Claude Code profile reads of the release the tree pins
//! (design 6 and 7 of record 0020's change; C.4; core.22): the program
//! the `claude` wrapper from `devenv.nix` runs is read as a file, and each
//! source fragment the profile rests on must stand in it byte for byte:
//! the usage-limit words it reads, the credentials the release keeps
//! from its hooks, which no start's record names, its permission modes,
//! which its map of flume's modes rests on (design 10 of record 0024's
//! change), how it keeps and resumes a session, which its carry rests
//! on (design 9 of record 0020's amendment's change), how it names a
//! session it runs, which its live rests on (design 2 of record 0052's
//! change), and how it takes its own settings by `--settings`, the keys
//! the profile reserves and the `attribution` the docs give as their
//! example (design 9 of record 0043's change), `effortLevel` among those
//! keys, and the efforts its `--effort` takes and the variable it reads
//! over them (design 6 and 7 of record 0044's change), the input each of
//! its hook events hands the plugin's hooks and how it reads their
//! answers, the options the command names, and its folder-trust dialog
//! (C.4; core.39). A release that changes any of them, or only renames
//! what its minifier names in those fragments, fails here until the
//! reading is made again. No agent runs.

use flume::profiles::agent::claude::{
    self, carry, effort, hook, limit, live, mode, settings, withheld, wrapper,
};
use std::path::{Path, PathBuf};

/// The Claude Code version `devenv.nix` pins.
fn pinned_version() -> String {
    let nix =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../devenv.nix"))
            .expect("devenv.nix");
    nix.lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("claudeVersion = \"")?
                .split('"')
                .next()
                .map(str::to_string)
        })
        .expect("devenv.nix pins Claude Code's version")
}

/// The program the pinned release's `claude` wrapper runs, its version.
fn pinned_program() -> (PathBuf, String) {
    let version = pinned_version();
    let path = std::env::var("PATH").unwrap_or_default();
    let found =
        wrapper::located("claude", &path).expect("a claude on PATH: run in the devenv shell");
    let resolved = std::fs::canonicalize(&found).unwrap_or_else(|_| found.clone());
    assert!(
        resolved
            .to_string_lossy()
            .contains(&format!("-claude-code-{version}/")),
        "the first claude on PATH, {}, is not the pinned {version}: run in the devenv shell",
        resolved.display()
    );
    let program = wrapper::unwrapped(&resolved)
        .unwrap_or_else(|| {
            panic!(
                "{} is no wrapper of the pinned release's program",
                resolved.display()
            )
        })
        .program;
    (program, version)
}

/// Each of `fragments` the pinned release's program does not hold, as
/// its name and the fragment.
fn missing(fragments: &[(&'static str, String)]) -> (Vec<String>, PathBuf, String) {
    let (program, version, bytes) = pinned_bytes();
    let missing = fragments
        .iter()
        .filter(|(_, fragment)| memchr::memmem::find(bytes, fragment.as_bytes()).is_none())
        .map(|(name, fragment)| format!("{name}: {fragment}"))
        .collect();
    (missing, program.clone(), version.clone())
}

/// The pinned release's program, its version and its bytes, read once
/// for every test of this module.
fn pinned_bytes() -> &'static (PathBuf, String, Vec<u8>) {
    static PINNED: std::sync::OnceLock<(PathBuf, String, Vec<u8>)> = std::sync::OnceLock::new();
    PINNED.get_or_init(|| {
        let (program, version) = pinned_program();
        let bytes = std::fs::read(&program)
            .unwrap_or_else(|error| panic!("{}: {error}", program.display()));
        (program, version, bytes)
    })
}

#[test]
fn the_pinned_release_holds_the_usage_limit_words() {
    let (missing, program, version) = missing(&limit::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the usage-limit reading rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_keeps_its_credentials_from_its_hooks() {
    let (missing, program, version) = missing(&withheld::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the credentials it keeps from its hooks rest on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_holds_the_modes_the_map_rests_on() {
    let (missing, program, version) = missing(&mode::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the map of modes rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_resumes_as_the_carry_reads_it() {
    let (missing, program, version) = missing(&carry::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the carry of a conversation rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_names_its_running_sessions_as_live_reads_them() {
    let (missing, program, version) = missing(&live::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the reading of a running session rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_takes_own_settings_as_the_profile_passes_them() {
    let (missing, program, version) = missing(&settings::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments its own settings' way rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_hands_its_hooks_the_inputs_they_read() {
    let (missing, program, version) = missing(&hook::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the hooks and their inputs rest on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_takes_the_command_the_profile_builds() {
    let (missing, program, version) = missing(&claude::command_fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the command rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_asks_its_folder_trust_as_the_terminal_answers_it() {
    let (missing, program, version) = missing(&claude::trust_fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the folder-trust dialog rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}

#[test]
fn the_pinned_release_takes_the_efforts_the_profile_passes() {
    let (missing, program, version) = missing(&effort::fragments());
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the effort rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}
