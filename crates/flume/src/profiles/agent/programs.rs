//! The programs flume runs (170, C.4; record 0026; core.22, core.39;
//! design 1, 2): one entry for each kind herdr's pinned release starts,
//! in [`KINDS`]' order, each its program's name and herdr's kind for it,
//! and each way its release takes a model, an effort and a mode, none
//! where no way is checked against the release the tree pins. The tree
//! pins Claude Code alone among agent programs, whose ways are its own
//! profile's, so every entry holds its name and kind and nothing more.
//! The resolver binds `claude` to Claude Code's profile and every other
//! entry to the common profile over it (105, 170).
//!
//! [`KINDS`]: crate::profiles::terminal::herdr::KINDS

use flume_seams::Mode;
use std::path::Path;

/// A program flume runs: its name, the terminal's kind for it, and each
/// way its release takes a setting, none where no way is checked against
/// the release the tree pins (170, C.4; core.39).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Program {
    pub name: &'static str,
    pub kind: &'static str,
    /// The flag its release takes a model by, passed as written (172).
    pub model: Option<&'static str>,
    /// The flag its release takes an effort by, and the words it takes
    /// (197).
    pub effort: Option<Effort>,
    /// The setting each of flume's four maps to on its release (169).
    pub modes: Option<[(Mode, &'static str); 4]>,
}

/// The way a release takes an effort: its flag and the words it takes,
/// each passed as written (197).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effort {
    pub flag: &'static str,
    pub words: &'static [&'static str],
}

/// An entry of its name and kind alone, no way checked against a release
/// the tree pins (170, C.4).
const fn named(name: &'static str) -> Program {
    Program {
        name,
        kind: name,
        model: None,
        effort: None,
        modes: None,
    }
}

/// One entry per kind herdr 0.9.5 starts, in its order, each its
/// program's canonical name and herdr's kind for it; Claude Code's ways
/// are its own profile's, so its entry holds its name and kind as every
/// other does (170, C.4; design 1, 2).
pub const PROGRAMS: [Program; 24] = [
    named("pi"),
    named("claude"),
    named("codex"),
    named("gemini"),
    named("cursor"),
    named("devin"),
    named("agy"),
    named("cline"),
    named("omp"),
    named("mastracode"),
    named("opencode"),
    named("copilot"),
    named("kimi"),
    named("kiro"),
    named("droid"),
    named("amp"),
    named("grok"),
    named("hermes"),
    named("kilo"),
    named("qodercli"),
    named("qwen"),
    named("letta"),
    named("maki"),
    named("muse"),
];

/// The entry `program` names, by its name or by a path whose file name is
/// its name; none for a program no entry names (105, 170).
pub fn entry(program: &str) -> Option<&'static Program> {
    let name = Path::new(program)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(program);
    PROGRAMS.iter().find(|entry| entry.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::terminal::herdr::KINDS;

    /// The agent-programs spec's scenario "herdr's kinds, each with its
    /// entry" (170, C.4).
    #[test]
    fn every_kind_herdr_starts_has_its_entry() {
        assert_eq!(PROGRAMS.len(), KINDS.len());
        for (entry, kind) in PROGRAMS.iter().zip(KINDS) {
            assert_eq!(entry.kind, kind, "an entry in herdr's order");
            assert_eq!(entry.name, kind, "an entry named as its kind");
            // No program's release but Claude Code's is pinned, and
            // Claude Code's ways are its own profile's (core.22, core.39).
            assert_eq!(
                (entry.model, entry.effort, entry.modes),
                (None, None, None),
                "{} names a way no pinned release was checked against",
                entry.name
            );
        }
        for kind in KINDS {
            assert_eq!(entry(kind).map(|entry| entry.kind), Some(kind));
        }
        assert_eq!(
            entry("/opt/amp/bin/amp").map(|entry| entry.name),
            Some("amp")
        );
        for other in ["nonesuch", "agent", "/usr/bin/nonesuch", ""] {
            assert_eq!(entry(other), None, "{other}");
        }
    }
}
