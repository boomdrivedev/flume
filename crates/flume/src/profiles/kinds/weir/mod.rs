//! The kinds profile (C.2; design 7): every declaration, a repository's
//! `.config/flume.toml` at a revision, a launch's file and flume's own
//! built in, read through `weir-kinds`, with no reader of flume's own
//! (core.4). The format's fields are mapped onto the seam's types, every
//! section the format does not name is ignored (23), and a declaration
//! weir refuses is answered as could not be read, naming every field that
//! fails (84). A kind that lands and declares `stages = []` is mapped to a
//! kind with no stages, which weir refuses on a kind that does not land
//! (18, 202; weir 51, 57). Such a kind's `landing_fix`, the identity with
//! its version of the kind its fixes run as, and its `max_landing_fixes`
//! are mapped onto the seam's kind of its fixes and its bound, the
//! identity spelled as weir's crate spells it, which the core resolves at
//! its place's launch as a launch's name (23, 154, 203; weir 52, 57; weir
//! record 0005). A kind's `agents` table, and a `[kinds.<kind>]` table that
//! gives settings to a kind declared elsewhere, are weir's too: each is
//! mapped onto the seam's settings for every stage and for each stage it
//! names, the latter returned apart from the kinds (32, C.2; weir 52,
//! 57, C.4); which declaration they join, and whether the stages they
//! name exist, are the core's to say. Every agents table's `programs`,
//! each program's own settings, is read by flume's one reader of them, a
//! file it names read as a prompt is (195). A prompt is a path relative
//! to its declaration's file, read beside it: at the same revision for a
//! repository's, on disk for a launch's, and from the binary for flume's
//! own, whose versions are each a declaration of its own with its prompts
//! beside it, change@1 to change@5, chore@1 and chore@2, and fix@1 and
//! fix@2 side by side (157; records 0064, 0077).
//!
//! A statechart kind, a table holding `regions`, is read by weir-kinds
//! into its statechart and mapped onto the seam's kind with its
//! statechart: its title, its table spelled again as a declaration of it
//! alone, and each state's stage parameters by the state's path, a
//! state's `prompt` in its `params` read beside the declaration as a
//! simple stage's prompt is; every other key of `params` is the
//! template's, and ignored (21, 23, 84, 175, C.2; weir 94). A held
//! statechart kind's text and the stage template's are read back by
//! weir-kinds into the engine's types at each settle (85, 176).
//!
//! flume's own kinds are packages, each read through weir-kinds' package
//! reader at its identity, `flume/<name>@<version>`, with its content's
//! hash over its declaration's and prompts' paths and bytes (156, 157).
//! A kind as a launch or a role names it is read by weir-kinds' reader of
//! a kind's name, which spells it back, and the identity of a kind of the
//! repository's or a launch's, which lies in no package, is read by
//! weir-kinds' reader of an identity, as is a package's (154; weir C.4).
//! The repository's declaration is [`DECLARATION`] and its lock [`LOCK`],
//! each given the core by its path within the tree (20, 155).
//!
//! A host's source is read through weir-kinds alone (155 to 160, 192; weir
//! 88 to 93, C.4; core.4):
//!
//! - **Its listing**, read-source: every entry of its `kinds_dir`, through
//!   a link or not, that is a directory holding a `kind.toml` and whose
//!   name weir's reader reads, joined to the source's name by `/`, as a
//!   whole identity, in weir's order of identities; every entry of any
//!   other shape is ignored (159).
//! - **A package**, read-package: `<kinds_dir>/<name>@<version>/`, read by
//!   weir-kinds' reader of a package's folder at that identity, with its
//!   content's hash, each prompt read beside its `kind.toml`; a package
//!   weir refuses, or whose agents give an `env` or a program by a
//!   location, is answered as could not be read, naming the package and
//!   every field (157, 158).
//! - **The repository's lock**, [`LOCK`] at a revision as git gives it, and
//!   **a lock on disk**, each read by weir-kinds' lock reader; no file is
//!   no lock (20, 155).
//! - **A lock written**, spelled by weir-kinds' lock writer, written beside
//!   the lock and renamed over it, so a write that fails leaves the lock
//!   as it was and no reader sees half of one (155).
//!
//! [`source_fault`] says which names a host's source may not take: none
//! weir's reader refuses, and none of flume's own source, the
//! repository's or a launch's (154, 191; weir C.4).
//!
//! The repository's binding is the declaration's `[profiles]` table at
//! the default branch: the remote's, after one fetch, where the
//! repository has a remote, and the local default branch otherwise (20,
//! 105, C.2). No other tool's file is read to choose a profile (core.3).
//! Beside `[profiles]`, the repository's `[agents]` is flume's own table
//! of its declaration and no part of any kind: its settings for every
//! kind it runs, read from the sections weir returns unnamed with the
//! kinds at their revision, as a kind's agents table is but naming no
//! stage, stream, environment or pane (21, 32, 195, C.2). A launch's
//! declaration and flume's own are not read for it.

use crate::git::{tree_path, Git, REMOTE};
use crate::own::{self, Reader};
use flume_seams::{
    Binding, BindingHeld, BindingRead, ChartAgents, ChartStage, Charts, Deliverable, HeldFrom,
    Join, Kind, KindIdentity, KindNamed, KindSettings, KindStatechart, Kinds, KindsRead, LockEntry,
    LockRead, PackageRead, Packaged, Revision, Settings, SourceRead, Stage,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io::ErrorKind;
use std::path::Path;
use weir_kinds::toml::{Table, Value};
use weir_kinds::{End, Refusal};

/// Where a repository declares its kinds and its binding, in its tree.
pub const DECLARATION: &str = ".config/flume.toml";

/// Where a repository's lock lies, in its tree and in a working copy
/// (155, C.2).
pub const LOCK: &str = ".config/flume.lock";

/// The source of flume's own kinds: flume's name, as weir's identity
/// names a tool's own source (154; weir C.4).
pub const OWN_SOURCE: &str = "flume";

/// One version of one of flume's own kinds, built into the binary: the
/// identity it is held at, as weir's reader is given it, its declaration
/// and each prompt it names by its path within the version's own
/// directory (22, 156, 157, C.2; design 7).
struct Own {
    identity: &'static str,
    declaration: &'static str,
    prompts: &'static [(&'static str, &'static str)],
}

/// Where a package declares its kind, at its root (157; weir C.4).
const KIND_TOML: &str = "kind.toml";

/// flume's own kinds, one declaration per version, each in a directory of
/// its own under `own/`, `<name>@<version>/kind.toml` with its prompts
/// beside it under `prompts/`: change@1 to change@5 side by side, then
/// chore@1 and chore@2, then fix@1 and fix@2, each name's versions in
/// ascending order so its last is its newest. change@4, change@5, chore@2
/// and fix@2 give each stage's agent `full` and say in each prompt that
/// stopping short is its one way to ask, and change@5 works each change
/// through OpenSpec's own commands (22, 154, 157, 162, C.2, C.3; records
/// 0022, 0054, 0064, 0077).
const OWN: &[Own] = &[
    Own {
        identity: "flume/change@1",
        declaration: include_str!("own/change@1/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@1/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@1/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@1/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/change@2",
        declaration: include_str!("own/change@2/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@2/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@2/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@2/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/change@3",
        declaration: include_str!("own/change@3/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@3/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@3/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@3/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/change@4",
        declaration: include_str!("own/change@4/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@4/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@4/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@4/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/change@5",
        declaration: include_str!("own/change@5/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@5/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@5/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@5/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/chore@1",
        declaration: include_str!("own/chore@1/kind.toml"),
        prompts: &[(
            "prompts/code.md",
            include_str!("own/chore@1/prompts/code.md"),
        )],
    },
    Own {
        identity: "flume/chore@2",
        declaration: include_str!("own/chore@2/kind.toml"),
        prompts: &[(
            "prompts/code.md",
            include_str!("own/chore@2/prompts/code.md"),
        )],
    },
    Own {
        identity: "flume/fix@1",
        declaration: include_str!("own/fix@1/kind.toml"),
        prompts: &[("prompts/code.md", include_str!("own/fix@1/prompts/code.md"))],
    },
    Own {
        identity: "flume/fix@2",
        declaration: include_str!("own/fix@2/kind.toml"),
        prompts: &[("prompts/code.md", include_str!("own/fix@2/prompts/code.md"))],
    },
];

/// The kinds profile over weir's kind format.
#[derive(Clone, Copy, Debug, Default)]
pub struct Weir;

impl Kinds for Weir {
    fn read_kinds(&self, repository: &str, revision: &Revision) -> KindsRead {
        let git = Git::at(repository);
        if let Err(reason) = git.commit(&revision.0) {
            return KindsRead::CouldNotRead { reason };
        }
        let text = match git.file(&revision.0, DECLARATION) {
            Ok(Some(text)) => text,
            Ok(None) => {
                return KindsRead::Read {
                    kinds: vec![],
                    settings: BTreeMap::new(),
                    agents: Box::default(),
                }
            }
            Err(reason) => return KindsRead::CouldNotRead { reason },
        };
        read_in_tree(
            &text,
            &|at| git.file(&revision.0, at),
            &revision.to_string(),
        )
    }

    fn read_declaration(&self, declaration: &str) -> KindsRead {
        let file = Path::new(declaration);
        let text = match std::fs::read_to_string(file) {
            Ok(text) => text,
            Err(error) => {
                return KindsRead::CouldNotRead {
                    reason: format!("{declaration} could not be read: {error}"),
                }
            }
        };
        let directory = file.parent().unwrap_or(Path::new(""));
        read(&text, Whose::Launch, &|path| {
            let at = directory.join(path);
            std::fs::read_to_string(&at)
                .map(|text| (at.display().to_string(), text))
                .map_err(|error| format!("{} could not be read: {error}", at.display()))
        })
    }

    /// Each version of flume's own kinds as a package, read through
    /// weir-kinds' package reader over its declaration and prompts by
    /// their paths within its version's directory, at the identity it is
    /// held at, with its content's hash over those paths and bytes; four
    /// versions of `change` and two each of `chore` and `fix` among them
    /// (22, 156, 157).
    fn flumes_own(&self) -> Vec<Packaged> {
        OWN.iter()
            .map(|own| {
                own_package(own).unwrap_or_else(|reason| {
                    panic!(
                        "flume's own {} is built in and reads: {reason}",
                        own.identity
                    )
                })
            })
            .collect()
    }

    fn read_binding(&self, repository: &str) -> BindingRead {
        match default_revision(repository) {
            Ok(revision) => binding_at(repository, &revision),
            Err(reason) => BindingRead::CouldNotRead { reason },
        }
    }

    /// `.config/flume.toml`'s `[profiles]` at the default branch as the
    /// repository holds it, fetching nothing: `origin/<branch>` as the
    /// last fetch left it, else the repository's own branch (20, 105;
    /// core.15).
    fn read_binding_held(&self, repository: &str) -> BindingHeld {
        match binding_held(repository) {
            Ok(held) => held,
            Err(reason) => BindingHeld::CouldNotRead { reason },
        }
    }

    /// The held text and the stage template's, each read by weir-kinds to
    /// exactly one statechart kind, the template keyed by its name; refused
    /// naming which could not be read and why (21, 85, 176).
    fn read_statechart(&self, kind: &KindStatechart, template: &str) -> Result<Charts, String> {
        let chart = one_statechart(&kind.text)
            .map_err(|why| format!("the held statechart kind \"{}\" {why}", kind.title))?;
        let template =
            one_statechart(template).map_err(|why| format!("the stage template {why}"))?;
        Ok(Charts {
            chart,
            templates: BTreeMap::from([(template.name.clone(), template)]),
        })
    }

    fn read_named(&self, text: &str) -> Result<KindNamed, String> {
        let named: weir_kinds::NamedKind = text
            .parse()
            .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
        let version = named
            .version()
            .map(|version| {
                u32::try_from(version)
                    .map_err(|_| format!("`{text}` names a version more than flume counts"))
            })
            .transpose()?;
        Ok(KindNamed {
            source: named.source().map(str::to_string),
            name: named.name().to_string(),
            version,
            spelled: named.to_string(),
        })
    }

    fn identity_of(&self, source: &str, name: &str, version: u32) -> Result<KindIdentity, String> {
        let held = held_at(source, &format!("{name}@{version}"))?;
        let mut problems = vec![];
        let identity = map_identity(&held, &mut problems);
        if problems.is_empty() {
            Ok(identity)
        } else {
            Err(problems.join("; "))
        }
    }

    fn declaration_path(&self) -> Option<String> {
        Some(DECLARATION.into())
    }

    fn lock_path(&self) -> Option<String> {
        Some(LOCK.into())
    }

    fn read_source(&self, source: &str, kinds_dir: &str) -> SourceRead {
        let dir = Path::new(kinds_dir);
        let unread = |words: String| SourceRead::CouldNotRead {
            reason: format!("{kinds_dir}, the directory of the source {source}, {words}"),
        };
        if let Err(refusal) = weir_kinds::check_name(source) {
            return SourceRead::CouldNotRead {
                reason: format!("the source {refusal}"),
            };
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return unread("does not exist".into())
            }
            Err(error) => return unread(format!("could not be read: {error}")),
        };
        let mut held = BTreeSet::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => return unread(format!("could not be read: {error}")),
            };
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            let Ok(identity) = held_at(source, &name) else {
                continue;
            };
            if entry.path().join(KIND_TOML).is_file() {
                held.insert(identity);
            }
        }
        let kinds = held
            .iter()
            .filter_map(|identity| {
                let mut problems = vec![];
                let identity = map_identity(identity, &mut problems);
                problems.is_empty().then_some(identity)
            })
            .collect();
        SourceRead::Read { kinds }
    }

    fn read_package(&self, source: &str, kinds_dir: &str, name: &str, version: u32) -> PackageRead {
        let unread = |reason: String| PackageRead::CouldNotRead { reason };
        let entry = format!("{name}@{version}");
        let held = match held_at(source, &entry) {
            Ok(held) => held,
            Err(reason) => return unread(reason),
        };
        let dir = Path::new(kinds_dir).join(&entry);
        if !dir.exists() {
            return unread(format!(
                "the source {source} holds no {entry}: {} does not exist",
                dir.display()
            ));
        }
        let refused_at = |problems: String| {
            unread(format!(
                "the package {held} at {}: {problems}",
                dir.display()
            ))
        };
        let package = match weir_kinds::read_package_dir(&dir, &held) {
            Ok(package) => package,
            Err(refusal) => return refused_at(refused(&refusal)),
        };
        let mut problems = vec![];
        let identity = map_identity(&package.identity, &mut problems);
        let kind = map_kind(
            &package.kind,
            &|path: &str| in_package(&dir, path),
            &mut problems,
        );
        if !problems.is_empty() {
            return refused_at(problems.join("; "));
        }
        PackageRead::Read {
            package: Box::new(Packaged {
                identity,
                kind,
                hash: package.hash.to_string(),
                dir: Some(dir.to_string_lossy().into_owned()),
            }),
        }
    }

    fn read_lock_at(&self, repository: &str, revision: &Revision) -> LockRead {
        let git = Git::at(repository);
        if let Err(reason) = git.commit(&revision.0) {
            return LockRead::CouldNotRead { reason };
        }
        match git.file(&revision.0, LOCK) {
            Ok(None) => LockRead::None,
            Ok(Some(text)) => lock_read(&text, &format!("{LOCK} at {revision}")),
            Err(reason) => LockRead::CouldNotRead { reason },
        }
    }

    fn read_lock_file(&self, path: &str) -> LockRead {
        match std::fs::read_to_string(path) {
            Ok(text) => lock_read(&text, path),
            Err(error) if error.kind() == ErrorKind::NotFound => LockRead::None,
            Err(error) => LockRead::CouldNotRead {
                reason: format!("{path} could not be read: {error}"),
            },
        }
    }

    fn write_lock(&self, path: &str, entries: &[LockEntry]) -> Result<(), String> {
        let mut lock = weir_kinds::Lock::default();
        for entry in entries {
            let identity: weir_kinds::Identity = entry
                .kind
                .spelled
                .parse()
                .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
            let hash: weir_kinds::ContentHash = entry
                .hash
                .parse()
                .map_err(|refusal: weir_kinds::HashRefusal| refusal.to_string())?;
            if let Some(given) = lock
                .entries
                .get(&identity)
                .filter(|given| given.hash != hash)
            {
                return Err(format!(
                    "{identity} is given at two hashes, {} and {hash}, and a lock records an identity once",
                    given.hash
                ));
            }
            lock.entries.insert(
                identity,
                weir_kinds::LockEntry {
                    hash,
                    sections: Table::new(),
                },
            );
        }
        write_beside(Path::new(path), &weir_kinds::spell_lock(&lock))
            .map_err(|reason| format!("the lock {path} could not be written: {reason}"))
    }
}

/// Why `name` cannot be the name of a source the host declares: one weir's
/// reader refuses as a name, or the source of flume's own kinds, the
/// repository's or a launch's; none where it can be (154, 191; weir C.4).
pub fn source_fault(name: &str) -> Option<String> {
    if let Err(refusal) = weir_kinds::check_name(name) {
        return Some(format!("its name {refusal}"));
    }
    let whose = match name {
        OWN_SOURCE => "flume's own kinds",
        weir_kinds::REPO => "the repository's kinds",
        weir_kinds::LAUNCH => "a launch's kinds",
        _ => return None,
    };
    Some(format!(
        "its name `{name}` is the source of {whose}, which no host declares"
    ))
}

/// The identity a package of the source `source` is held at, its
/// directory named `entry`, `<name>@<version>`, as weir's reader reads the
/// two joined (159; weir C.4).
fn held_at(source: &str, entry: &str) -> Result<weir_kinds::Identity, String> {
    format!("{source}/{entry}")
        .parse()
        .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())
}

/// A lock's text, `named` as its reader names it, read by weir's lock
/// reader (155; weir 91, 93).
fn lock_read(text: &str, named: &str) -> LockRead {
    let lock = match weir_kinds::read_lock(text) {
        Ok(lock) => lock,
        Err(refusal) => {
            return LockRead::CouldNotRead {
                reason: format!("{named}: {}", refused(&refusal)),
            }
        }
    };
    let mut problems = vec![];
    let entries = lock
        .entries
        .iter()
        .map(|(identity, entry)| LockEntry {
            kind: map_identity(identity, &mut problems),
            hash: entry.hash.to_string(),
        })
        .collect();
    if problems.is_empty() {
        LockRead::Read { entries }
    } else {
        LockRead::CouldNotRead {
            reason: format!("{named}: {}", problems.join("; ")),
        }
    }
}

/// `text` written whole at `file`: written beside it, under a name of its
/// own that begins with `.`, and renamed over it, its directory made where
/// it is missing; the file as it was where any step fails (155).
fn write_beside(file: &Path, text: &str) -> Result<(), String> {
    let name = file
        .file_name()
        .ok_or_else(|| format!("{} names no file", file.display()))?;
    if let Some(dir) = file.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    }
    let beside = file.with_file_name(format!(
        ".{}.{}.tmp",
        name.to_string_lossy(),
        std::process::id()
    ));
    let written = std::fs::write(&beside, text)
        .and_then(|()| std::fs::rename(&beside, file))
        .map_err(|error| error.to_string());
    if written.is_err() {
        let _ = std::fs::remove_file(&beside);
    }
    written
}

/// One version of flume's own kinds read as a package (156, 157).
fn own_package(own: &Own) -> Result<Packaged, String> {
    let held: weir_kinds::Identity = own
        .identity
        .parse()
        .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
    let files = std::iter::once((KIND_TOML, own.declaration)).chain(own.prompts.iter().copied());
    let package = weir_kinds::read_package(files, &held).map_err(|refusal| refused(&refusal))?;
    let mut problems = vec![];
    let identity = map_identity(&package.identity, &mut problems);
    let kind = map_kind(
        &package.kind,
        &|path: &str| {
            own.prompts
                .iter()
                .find(|(at, _)| *at == path)
                .map(|(at, text)| (at.to_string(), text.to_string()))
                .ok_or_else(|| format!("{path} is not built in"))
        },
        &mut problems,
    );
    if !problems.is_empty() {
        return Err(problems.join("; "));
    }
    Ok(Packaged {
        identity,
        kind,
        hash: package.hash.to_string(),
        dir: None,
    })
}

/// A file of the package at `dir` by its path within it, with the path
/// read: a path that leaves the package, by `..`, an absolute path or a
/// link, is refused, so a prompt or a program's settings file reads
/// nothing outside it (157, 195; core.34, core.50).
fn in_package(dir: &Path, path: &str) -> Result<(String, String), String> {
    let outside = || format!("{path} lies outside the package");
    if Path::new(path).is_absolute() {
        return Err(outside());
    }
    let within = tree_path("", path).ok_or_else(outside)?;
    let at = dir.join(within);
    let unread = |error: std::io::Error| format!("{} could not be read: {error}", at.display());
    let real = at.canonicalize().map_err(unread)?;
    let root = dir.canonicalize().map_err(unread)?;
    if !real.starts_with(&root) {
        return Err(outside());
    }
    std::fs::read_to_string(&real)
        .map(|text| (at.display().to_string(), text))
        .map_err(unread)
}

/// An identity as weir's reader gave it, mapped onto the seam's with its
/// parts and weir's spelling (154; weir 88, C.4).
fn map_identity(identity: &weir_kinds::Identity, problems: &mut Vec<String>) -> KindIdentity {
    let version = u32::try_from(identity.version()).unwrap_or_else(|_| {
        problems.push(format!("{identity}: its version is more than flume counts"));
        0
    });
    KindIdentity {
        source: identity.source().to_string(),
        name: identity.name().to_string(),
        version,
        spelled: identity.to_string(),
    }
}

/// The revision of the repository's default branch (20, C.2): the
/// remote's after one fetch of it where the repository has the remote,
/// and the local branch's otherwise; why there is none where git names
/// no default branch.
pub fn default_revision(repository: &str) -> Result<Revision, String> {
    let git = Git::at(repository);
    let branch = git
        .default_branch()
        .map_err(|reason| format!("{repository} names no default branch: {reason}"))?;
    if !git.remotes()?.iter().any(|remote| remote == REMOTE) {
        return git.commit(&format!("refs/heads/{branch}")).map(Revision);
    }
    let tracking = git.fetch_branch(REMOTE, &branch)?;
    git.commit(&tracking).map(Revision)
}

/// The binding at the default branch as the repository holds it, with
/// the declaration's path where that revision holds one.
fn binding_held(repository: &str) -> Result<BindingHeld, String> {
    let git = Git::at(repository);
    let branch = git
        .default_branch()
        .map_err(|reason| format!("{repository} names no default branch: {reason}"))?;
    let tracking = format!("refs/remotes/{REMOTE}/{branch}");
    let origin = git.remotes()?.iter().any(|remote| remote == REMOTE);
    let (revision, from) = match git.commit(&tracking) {
        Ok(revision) if origin => (revision, HeldFrom::Origin),
        _ => (
            git.commit(&format!("refs/heads/{branch}"))?,
            HeldFrom::Local,
        ),
    };
    let revision = Revision(revision);
    let text = git.file(&revision.0, DECLARATION)?;
    let binding = match &text {
        Some(text) => {
            profiles(text).map_err(|reason| format!("{DECLARATION} at {revision}: {reason}"))?
        }
        None => Binding::default(),
    };
    Ok(BindingHeld::Read {
        branch,
        revision,
        from,
        declaration: text.map(|_| DECLARATION.to_string()),
        binding,
    })
}

/// The `[profiles]` table of the repository's declaration at `revision`:
/// each seam it names, none where it names none or there is no
/// declaration (105, C.2, core.3).
pub fn binding_at(repository: &str, revision: &Revision) -> BindingRead {
    let git = Git::at(repository);
    let text = match git.file(&revision.0, DECLARATION) {
        Ok(Some(text)) => text,
        Ok(None) => {
            return BindingRead::Read {
                binding: Binding::default(),
            }
        }
        Err(reason) => return BindingRead::CouldNotRead { reason },
    };
    match profiles(&text) {
        Ok(binding) => BindingRead::Read { binding },
        Err(reason) => BindingRead::CouldNotRead {
            reason: format!("{DECLARATION} at {revision}: {reason}"),
        },
    }
}

/// The binding a declaration's `[profiles]` table names. The table is
/// flume's own, beside the kinds, so it is read whether or not the kinds
/// read.
fn profiles(text: &str) -> Result<Binding, String> {
    let root: Table = text
        .parse()
        .map_err(|error: weir_kinds::toml::de::Error| error.message().trim().to_string())?;
    let Some(table) = root.get("profiles") else {
        return Ok(Binding::default());
    };
    let Some(table) = table.as_table() else {
        return Err("profiles: must be a table naming a profile for each seam".into());
    };
    let mut problems = vec![];
    let mut named = |seam: &str| match table.get(seam) {
        None => None,
        Some(Value::String(name)) if !name.is_empty() => Some(name.clone()),
        Some(_) => {
            problems.push(format!("profiles.{seam}: must be a profile's name"));
            None
        }
    };
    let binding = Binding {
        place: named("place"),
        change: named("change"),
        merge_queue: named("merge_queue"),
    };
    if problems.is_empty() {
        Ok(binding)
    } else {
        Err(problems.join("; "))
    }
}

/// Whose declaration is read: a repository's, which alone gives
/// `[agents]` for every kind, or a launch's; flume's own and a host's
/// source's are packages, read by weir's package reader (21, 32, 157).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Whose {
    Repository,
    Launch,
}

/// A repository's kinds from its declaration's `text`, each file the
/// declaration names read by `file` from its path in the repository's tree
/// at the revision `at` names; a path outside the tree, or one the
/// revision lacks, fails the read naming it (84, 157).
fn read_in_tree(
    text: &str,
    file: &dyn Fn(&str) -> Result<Option<String>, String>,
    at: &str,
) -> KindsRead {
    let directory = Path::new(DECLARATION)
        .parent()
        .map(|parent| parent.to_string_lossy().into_owned())
        .unwrap_or_default();
    read(text, Whose::Repository, &|path| {
        let in_tree = tree_path(&directory, path)
            .ok_or_else(|| format!("{path} lies outside the repository"))?;
        let text = file(&in_tree)?.ok_or_else(|| format!("{in_tree} is not in {at}"))?;
        Ok((in_tree, text))
    })
}

/// Every kind a declaration's text holds, its prompts and its agents
/// tables' settings files read by `beside` from their paths, the settings
/// it gives kinds it does not declare, and, for a repository's, its
/// settings for every kind; or every field that fails it (84, 32, 157).
fn read(text: &str, whose: Whose, beside: Reader) -> KindsRead {
    let declaration = match weir_kinds::read_kinds(text) {
        Ok(declaration) => declaration,
        Err(refusal) => {
            return KindsRead::CouldNotRead {
                reason: refused(&refusal),
            }
        }
    };
    let mut problems = vec![];
    let mut kinds: Vec<Kind> = declaration
        .kinds
        .values()
        .map(|kind| map_kind(kind, beside, &mut problems))
        .collect();
    if !declaration.statecharts.is_empty() {
        let tables = weir_kinds::toml::from_str::<Table>(text).unwrap_or_default();
        kinds.extend(
            declaration
                .statecharts
                .values()
                .map(|kind| map_statechart(kind, &tables, beside, &mut problems)),
        );
        kinds.sort_by(|a, b| a.name.cmp(&b.name));
    }
    let settings = declaration
        .settings
        .values()
        .map(|given| {
            let at = format!("kinds.{}.agents", given.name);
            (
                given.name.clone(),
                map_agents(&given.agents, &at, beside, &mut problems),
            )
        })
        .collect();
    let agents = match whose {
        Whose::Repository => every_kind(&declaration.sections, beside, &mut problems),
        Whose::Launch => Settings::default(),
    };
    if problems.is_empty() {
        KindsRead::Read {
            kinds,
            settings,
            agents: Box::new(agents),
        }
    } else {
        KindsRead::CouldNotRead {
            reason: problems.join("; "),
        }
    }
}

/// A repository's `[agents]`, its settings for every kind it runs, among
/// the declaration's `sections`: read as a kind's agents table is, with a
/// `stages`, a `streams`, an `env` or a `beside` failing it naming the
/// field, since a stage's name is its kind's, a stream and an environment
/// the host's and a pane the launch's (21, 32, 107, 144, 195, C.2).
fn every_kind(sections: &Table, beside: Reader, problems: &mut Vec<String>) -> Settings {
    let Some(agents) = sections.get("agents") else {
        return Settings::default();
    };
    let Some(agents) = agents.as_table() else {
        problems.push("agents: must be a table of settings for every kind".into());
        return Settings::default();
    };
    for (key, words) in [
        (
            "stages",
            "a repository's settings for every kind name no stage, since a stage's name is its kind's: a stage's settings go in [kinds.<kind>.agents.stages.<stage>]",
        ),
        (
            "streams",
            "a repository's settings name no stream, since a stream is the host's: a stream's settings go in the host's [agents.streams.<stream>]",
        ),
        (
            "beside",
            "a repository's settings for every kind name no pane to open beside, which is the launch's to name",
        ),
    ] {
        if agents.contains_key(key) {
            problems.push(format!("agents.{key}: {words}"));
        }
    }
    map_agent(agents, "agents", beside, problems)
}

fn refused(refusal: &Refusal) -> String {
    refusal
        .problems
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ")
}

/// The seam's kind of a kind weir-kinds read, every problem it finds
/// pushed onto `problems`. Its label is none: a simple kind's label waits
/// on weir's statement naming the field and weir-kinds reading it, since
/// weir 52 and C.4 name none and weir 94 gives a statechart kind's
/// `label` another meaning, and flume reads no field of the format
/// itself; until then only the host's configuration gives a place a
/// label (180; record 0033; core.4). Its kind of fixes is the kind's
/// `landing_fix` as weir-kinds reads it, an identity with its version
/// spelled by weir's `Display`, and its bound is the kind's
/// `max_landing_fixes`, a count past what flume counts a problem naming
/// the field; nothing else feeds either, so a kind's `fix` and
/// `max_fixes`, which weir returns as sections, are ignored (23; weir 52,
/// 57; weir record 0005; core.4).
fn map_kind(kind: &weir_kinds::Kind, beside: Reader, problems: &mut Vec<String>) -> Kind {
    let at = format!("kinds.{}", kind.name);
    let version = u32::try_from(kind.version).unwrap_or_else(|_| {
        problems.push(format!("{at}.version: is more than flume counts"));
        0
    });
    Kind {
        name: kind.name.clone(),
        version,
        branch_prefix: kind.branch.clone(),
        starts_unready: kind.starts_unready,
        lands: kind.lands,
        agents: map_agents(&kind.agents, &format!("{at}.agents"), beside, problems),
        stages: kind
            .stages
            .iter()
            .enumerate()
            .map(|(n, stage)| map_stage(stage, &format!("{at}.stages[{n}]"), beside, problems))
            .collect(),
        label: None,
        fix: kind.landing_fix.as_ref().map(ToString::to_string),
        fix_limit: kind.max_landing_fixes.map(|times| {
            u32::try_from(times).unwrap_or_else(|_| {
                problems.push(format!("{at}.max_landing_fixes: is more than flume counts"));
                0
            })
        }),
        statechart: None,
    }
}

/// The seam's kind of a statechart kind weir-kinds read, every problem it
/// finds pushed onto `problems`: its title, label, branch, whether it
/// lands, its landing fix spelled as a simple kind's is and its bound; its
/// table, taken from `tables`, the declaration as parsed, spelled again as
/// a declaration of it alone, which the kinds seam reads back into the
/// engine's types at each settle; and each state's stage parameters by its
/// path, its entries' settings read as a simple stage's agents are, its
/// deliverables as a simple stage's, and its `prompt`, a string in the
/// state's own `params`, read beside the declaration as a simple stage's
/// prompt is, `<at>.params.prompt` named where it cannot be read. Every
/// other key of `params` is the template's, and ignored; its stages are
/// none, and it starts ready with no settings of its own (19, 21, 23, 84,
/// 175, C.2; core.4).
fn map_statechart(
    kind: &weir_kinds::StatechartKind,
    tables: &Table,
    beside: Reader,
    problems: &mut Vec<String>,
) -> Kind {
    let name = kind.name();
    let at = format!("kinds.{name}");
    let version = u32::try_from(kind.version()).unwrap_or_else(|_| {
        problems.push(format!("{at}.version: is more than flume counts"));
        0
    });
    let table = tables
        .get("kinds")
        .and_then(Value::as_table)
        .and_then(|kinds| kinds.get(name))
        .and_then(Value::as_table)
        .cloned()
        .unwrap_or_default();
    let alone = Table::from_iter([(
        "kinds".to_string(),
        Value::Table(Table::from_iter([(name.to_string(), Value::Table(table))])),
    )]);
    let text = weir_kinds::toml::to_string(&alone).unwrap_or_else(|error| {
        problems.push(format!("{at}: could not be spelled again: {error}"));
        String::new()
    });
    let stages = kind
        .params
        .iter()
        .map(|(path, params)| {
            let at = state_at(&at, path);
            let state = state_of(&kind.statechart, path);
            let stage = map_chart_stage(params, state, &at, beside, problems);
            (path.clone(), stage)
        })
        .collect();
    Kind {
        name: name.to_string(),
        version,
        branch_prefix: kind.branch.clone(),
        starts_unready: false,
        lands: kind.lands,
        agents: KindSettings::default(),
        stages: vec![],
        label: kind.label.clone(),
        fix: kind.landing_fix.as_ref().map(ToString::to_string),
        fix_limit: kind.max_landing_fixes.map(|times| {
            u32::try_from(times).unwrap_or_else(|_| {
                problems.push(format!("{at}.max_landing_fixes: is more than flume counts"));
                0
            })
        }),
        statechart: Some(KindStatechart {
            title: kind.title.clone(),
            text,
            stages,
        }),
    }
}

/// The one statechart kind `text` declares, read by weir-kinds; or why it
/// gives none: refused by weir, or declaring other than one statechart
/// kind (21, 85).
fn one_statechart(text: &str) -> Result<weir_kinds::weir_engine::Statechart, String> {
    let declaration = weir_kinds::read_kinds(text)
        .map_err(|refusal| format!("could not be read: {}", refused(&refusal)))?;
    let mut statecharts = declaration.statecharts.into_values();
    match (
        statecharts.next(),
        statecharts.next(),
        declaration.kinds.is_empty(),
    ) {
        (Some(kind), None, true) => Ok(kind.statechart),
        _ => Err("holds other than one statechart kind".into()),
    }
}

/// Where the state at `path`, `<region>.<state>` and so on down, lies in
/// its kind's table at `at`: `kinds.persona` and `stages.test` give
/// `kinds.persona.regions.stages.states.test`.
fn state_at(at: &str, path: &str) -> String {
    let mut spelled = at.to_string();
    for (n, part) in path.split('.').enumerate() {
        let field = if n % 2 == 0 { "regions" } else { "states" };
        spelled.push_str(&format!(".{field}.{part}"));
    }
    spelled
}

/// The state at `path` in `chart`, through its states' own regions.
fn state_of<'a>(
    chart: &'a weir_kinds::weir_engine::Statechart,
    path: &str,
) -> Option<&'a weir_kinds::weir_engine::State> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut regions = &chart.regions;
    let mut found = None;
    for pair in parts.chunks(2) {
        let [region, state] = pair else {
            return None;
        };
        let state = regions.get(*region)?.states.get(*state)?;
        regions = &state.regions;
        found = Some(state);
    }
    found
}

/// A state's stage parameters at `at`, its `prompt` read from the state's
/// own `params` by `beside` (23, 175, C.2).
fn map_chart_stage(
    params: &weir_kinds::Params,
    state: Option<&weir_kinds::weir_engine::State>,
    at: &str,
    beside: Reader,
    problems: &mut Vec<String>,
) -> ChartStage {
    let agents = match &params.agents {
        None => ChartAgents::Entries(vec![]),
        Some(weir_kinds::ParamAgents::Entries(entries)) => ChartAgents::Entries(
            entries
                .iter()
                .enumerate()
                .map(|(n, agent)| {
                    map_agent(agent, &format!("{at}.params.agents[{n}]"), beside, problems)
                })
                .collect(),
        ),
        Some(weir_kinds::ParamAgents::Rule { rule, pattern }) => ChartAgents::Rule {
            rule: rule.clone(),
            pattern: pattern.clone(),
        },
    };
    let join = match params.join {
        None | Some(weir_kinds::Join::All) => Join::All,
        Some(weir_kinds::Join::Any) => Join::Any,
        Some(weir_kinds::Join::Count(n)) => Join::Count(u32::try_from(n).unwrap_or_else(|_| {
            problems.push(format!("{at}.params.join: is more than flume counts"));
            0
        })),
    };
    let send_back_limit = params.max_send_backs.map(|times| {
        u32::try_from(times).unwrap_or_else(|_| {
            problems.push(format!(
                "{at}.params.max_send_backs: is more than flume counts"
            ));
            0
        })
    });
    let prompt = match state
        .and_then(|state| state.params.as_ref())
        .and_then(|params| params.get("prompt"))
    {
        None => None,
        Some(Value::String(path)) => match beside(path) {
            Ok((_, text)) => Some(text),
            Err(reason) => {
                problems.push(format!("{at}.params.prompt: {reason}"));
                None
            }
        },
        Some(_) => {
            problems.push(format!(
                "{at}.params.prompt: must be the path of the prompt's file, a string"
            ));
            None
        }
    };
    ChartStage {
        agents,
        join,
        deliverables: params
            .deliverables
            .iter()
            .flatten()
            .map(map_deliverable)
            .collect(),
        prompt,
        on_fail: params.on_fail.clone(),
        send_back_limit,
    }
}

fn map_stage(
    stage: &weir_kinds::Stage,
    at: &str,
    beside: Reader,
    problems: &mut Vec<String>,
) -> Stage {
    let text = beside(&stage.prompt)
        .map(|(_, text)| text)
        .unwrap_or_else(|reason| {
            problems.push(format!("{at}.prompt: {reason}"));
            String::new()
        });
    let (conversation, deliverables) = match &stage.end {
        End::Conversation => (true, vec![]),
        End::Deliverable(deliverable) => (false, vec![map_deliverable(deliverable)]),
    };
    let send_back_limit = stage.max_send_backs.map(|times| {
        u32::try_from(times).unwrap_or_else(|_| {
            problems.push(format!("{at}.max_send_backs: is more than flume counts"));
            0
        })
    });
    Stage {
        name: stage.name.clone(),
        prompt: text,
        conversation,
        deliverables,
        agents: stage
            .agents
            .iter()
            .enumerate()
            .map(|(n, agent)| map_agent(agent, &format!("{at}.agents[{n}]"), beside, problems))
            .collect(),
        waits: stage.wait,
        quiet_limit: stage.quiet.map(|quiet| quiet.as_secs()),
        send_back_to: stage.on_fail.clone(),
        send_back_limit,
        makes: stage.makes.clone(),
        // weir has refused a repeat on a stage that does not deliver tasks
        // (weir 57); a way of repeating weir names and flume does not run
        // fails here to compile, never to read as not repeating.
        repeats: match stage.repeat {
            None => false,
            Some(weir_kinds::Repeat::PerTaskGroup) => true,
        },
        // weir gives `hibernate` as written, `None` where it is left out,
        // since the format gives no default (weir 53, 58); flume reads a
        // stage that leaves it out as not hibernating (212), and never
        // reads its sections for it (core.4).
        hibernates: stage.hibernate.unwrap_or(false),
    }
}

fn map_deliverable(deliverable: &weir_kinds::Deliverable) -> Deliverable {
    match deliverable {
        weir_kinds::Deliverable::Commits => Deliverable::Commits,
        weir_kinds::Deliverable::Change => Deliverable::ChangeWritten,
        weir_kinds::Deliverable::Tasks => Deliverable::TasksTicked,
        weir_kinds::Deliverable::Archived => Deliverable::ChangeArchived,
        weir_kinds::Deliverable::Report(path) => Deliverable::Report { path: path.clone() },
        weir_kinds::Deliverable::Committed(path) => {
            Deliverable::CommittedReport { path: path.clone() }
        }
    }
}

/// A kind's `agents` table: the settings for every stage, and for each
/// stage its `stages` table names (32; weir 52, C.4).
fn map_agents(
    agents: &weir_kinds::KindAgents,
    at: &str,
    beside: Reader,
    problems: &mut Vec<String>,
) -> KindSettings {
    KindSettings {
        every_stage: map_agent(&agents.every_stage, at, beside, problems),
        stages: agents
            .stages
            .iter()
            .map(|(stage, table)| {
                let at = format!("{at}.stages.{stage}");
                (stage.clone(), map_agent(table, &at, beside, problems))
            })
            .collect(),
    }
}

/// An agent's settings, its table as written: `program`, `model`,
/// `effort`, `instructions`, `beside`, `mode` and `programs` (32, 107,
/// 162, 195, 197), the effort its program's own word, kept as written
/// for the agent's profile to take or refuse, any other value, a table
/// of programs among it, failing it naming the field (197, C.2), the
/// mode one of flume's four or a table of programs, read as the
/// host's configuration reads it, any other failing it naming the field
/// (169, C.2), and `programs` each program's own settings, read by
/// flume's one reader of them, a file an entry names read by `beside` as
/// a prompt is, so never outside the repository or the package, any part
/// not as it reads them failing it naming the field (157, 195, C.2;
/// core.34); an `env` fails it, naming the field, since an agent's
/// environment is its account's alone (144, C.2), and so does a
/// `keep_open`, since how long an agent is kept open for an answer trades
/// the host's room and its accounts' and is the host's or the launch's to
/// give (158, 198, C.2), and a program named by a location, a `/` in it,
/// since a location is the host's (158); every other key is the runner's,
/// and ignored.
fn map_agent(agent: &Table, at: &str, beside: Reader, problems: &mut Vec<String>) -> Settings {
    let mut text = |name: &str| match agent.get(name) {
        None => None,
        Some(Value::String(value)) => Some(value.clone()),
        Some(_) => {
            problems.push(format!("{at}.{name}: must be a string"));
            None
        }
    };
    let program = text("program");
    let model = text("model");
    let effort = text("effort");
    let instructions = text("instructions");
    let pane = text("beside");
    let mode = agent.get("mode").and_then(|mode| {
        crate::mode::read(mode)
            .map_err(|reason| problems.push(format!("{at}.mode: {reason}")))
            .ok()
    });
    let programs = agent
        .get("programs")
        .and_then(|programs| {
            own::read(programs, at, beside)
                .map_err(|fault| problems.push(fault))
                .ok()
        })
        .unwrap_or_default();
    if agent.contains_key("env") {
        problems.push(format!(
            "{at}.env: an agent's environment is its account's to give, and no declaration gives one"
        ));
    }
    if agent.contains_key("keep_open") {
        problems.push(format!(
            "{at}.keep_open: how long an agent is kept open for an answer is the host's or the launch's to give, and no declaration gives one"
        ));
    }
    if agent.contains_key("hibernate_after") {
        problems.push(format!(
            "{at}.hibernate_after: how long a conversation may idle before it hibernates is the host's or the launch's to give, and no declaration gives one"
        ));
    }
    if let Some(program) = program.as_deref().filter(|program| program.contains('/')) {
        problems.push(format!(
            "{at}.program: `{program}` names a program by a location, which is the host's, and a declaration names its program by name"
        ));
    }
    Settings {
        program,
        model,
        effort,
        instructions,
        beside: pane,
        mode,
        // A kind names nothing a host owns, a session among it (158).
        session: None,
        programs,
        // Nor how long an agent is kept open for an answer, nor how long a
        // conversation may idle before it hibernates, each refused above
        // (158, 198, 212).
        keep_open: None,
        hibernate_after: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_seams::{Mode, ModeSetting, NamedFile};
    use std::cell::RefCell;

    /// A repository's tree for the declaration's reading: files written as a
    /// working tree holds them, and what each commit took, which a read
    /// gives.
    #[derive(Default)]
    struct Tree {
        written: RefCell<BTreeMap<String, String>>,
        committed: RefCell<BTreeMap<String, String>>,
    }

    impl Tree {
        fn new(_label: &str) -> Tree {
            Tree::default()
        }

        fn write(&self, path: &str, text: &str) {
            self.written.borrow_mut().insert(path.into(), text.into());
        }

        /// Takes what is written as the tree's revision, and names it.
        fn commit(&self, _message: &str) -> String {
            *self.committed.borrow_mut() = self.written.borrow().clone();
            "the-revision".into()
        }

        /// The repository's kinds as its declaration reads at the revision.
        fn read(&self) -> KindsRead {
            let tree = self.committed.borrow();
            let Some(text) = tree.get(DECLARATION) else {
                return KindsRead::Read {
                    kinds: vec![],
                    settings: BTreeMap::new(),
                    agents: Box::default(),
                };
            };
            read_in_tree(text, &|at| Ok(tree.get(at).cloned()), "the-revision")
        }

        fn could_not_read(&self) -> String {
            match self.read() {
                KindsRead::CouldNotRead { reason } => reason,
                other => panic!("the declaration read as {other:?}"),
            }
        }
    }

    /// The words a declaration's `keep_open` is refused in (design 1).
    const KEEP_OPEN_REFUSED: &str =
        "how long an agent is kept open for an answer is the host's or the launch's to give";

    /// The words a declaration's `hibernate_after` is refused in (212).
    const HIBERNATE_AFTER_REFUSED: &str =
    "how long a conversation may idle before it hibernates is the host's or the launch's to give";

    /// weir C.4's persona, its test stage's rule flume's `every-file`, with a
    /// prompt beside each stage's parameters under `prompts/` (175, S76; weir
    /// C.4).
    const PERSONA: &str = r#"[kinds.persona]
version = 1
title = "Persona test"
branch = "persona"
lands = true

[kinds.persona.regions.stages]
initial = "build"

[kinds.persona.regions.stages.states.build]
machine = "stage"
params = { agents = [{ model = "opus" }], join = "all", on_fail = "build", max_send_backs = 2, deliverables = ["commits"], prompt = "prompts/build.md" }
transitions = [
  { when = { final = "passed" }, to = "test" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.test]
machine = "stage"
params = { agents = { rule = "every-file", pattern = "personas/*.md" }, join = "all", on_fail = "build", max_send_backs = 2, deliverables = ["committed:tests/personas.md"], prompt = "prompts/test.md" }
transitions = [
  { when = { final = "passed" }, to = "passed" },
  { when = { final = "sent-back" }, to = "build" },
  { when = { final = "stopped" }, to = "stopped" },
]

[kinds.persona.regions.stages.states.passed]
final = true

[kinds.persona.regions.stages.states.stopped]
decision = { kind = "stalled", answers = ["retry", "drop"] }
transitions = [
  { when = { response = "retry" }, to = "build" },
  { when = { response = "drop" }, to = "dropped" },
]

[kinds.persona.regions.stages.states.dropped]
final = true
"#;

    /// persona beside a simple kind `bump`, in one repository's declaration.
    fn declaring_persona(name: &str) -> Tree {
        let repository = Tree::new(name);
        repository.write(
        DECLARATION,
        &format!(
            "{PERSONA}\n[kinds.bump]\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/build.md\"\ndeliverables = [\"commits\"]\nagents = [{{}}]\n"
        ),
    );
        repository.write(".config/prompts/build.md", "Build the personas. {{brief}}");
        repository.write(".config/prompts/test.md", "Test {{file}}.");
        repository.commit("persona beside bump");
        repository
    }

    #[test]
    fn a_declaration_weir_refuses_names_every_field_that_fails() {
        let repository = Tree::new("refused");
        repository.write(
        DECLARATION,
        "[kinds.bump]\nversion = 0\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"p.md\"\nquiet = \"1h30m\"\n",
    );
        repository.commit("refused");
        match repository.read() {
            KindsRead::CouldNotRead { reason } => {
                for field in [
                    "kinds.bump.version",
                    "kinds.bump.stages[0].deliverables",
                    "kinds.bump.stages[0].quiet",
                ] {
                    assert!(reason.contains(field), "{field} is not named in {reason}");
                }
            }
            other => panic!("a refused declaration read as {other:?}"),
        }
    }

    #[test]
    fn a_repositorys_agents_table_naming_a_stage_or_an_environment_refuses_the_launch() {
        // 4, 32, C.2: `stages`, `streams`, `env` and `beside` in a
        // repository's `[agents]` each fail the read naming the field, and
        // the launch is refused with nothing made.
        for (given, field) in [
            ("stages = { code = { model = \"opus\" } }", "agents.stages"),
            (
                "streams = { work = { model = \"opus\" } }",
                "agents.streams",
            ),
            ("env = { X = \"1\" }", "agents.env"),
            ("beside = \"pane\"", "agents.beside"),
        ] {
            let repository = Tree::new("every-kind-refused");
            repository.write(DECLARATION, &format!("[agents]\n{given}\n"));
            repository.commit("a repository's agents flume refuses");
            let reason = repository.could_not_read();
            assert!(reason.contains(&format!("{field}:")), "{given}: {reason}");
        }
    }

    #[test]
    fn an_effort_for_every_kind_of_a_repository() {
        // 32, 197, C.2: the repository's `[agents]` gives an effort for every
        // kind, apart from its table for fix, which gives its own.
        let repository = Tree::new("every-kind-effort");
        repository.write(
            DECLARATION,
            r#"[agents]
effort = "medium"

[kinds.fix.agents]
effort = "high"
"#,
        );
        repository.commit("the repository's efforts");
        let KindsRead::Read {
            settings, agents, ..
        } = repository.read()
        else {
            panic!("the declaration did not read");
        };
        assert_eq!(agents.effort.as_deref(), Some("medium"));
        assert_eq!(settings["fix"].every_stage.effort.as_deref(), Some("high"));
    }

    #[test]
    fn an_effort_for_every_stage_and_one_for_a_stage() {
        // 23, 197, C.2: a kind's agents table gives an effort for every
        // stage, and a stage's agents another, each its program's own word
        // kept as written.
        let repository = Tree::new("declared-effort");
        repository.write(
            DECLARATION,
            r#"[kinds.bump.agents]
effort = "high"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ effort = "xhigh" }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("an effort for bump and one for its code");
        let KindsRead::Read { kinds, .. } = repository.read() else {
            panic!("the declaration did not read");
        };
        let bump = &kinds[0];
        assert_eq!(bump.agents.every_stage.effort.as_deref(), Some("high"));
        assert_eq!(bump.stages[0].agents[0].effort.as_deref(), Some("xhigh"));
    }

    #[test]
    fn a_repositorys_keep_open_refuses_the_launch_naming_the_field() {
        // 4, 158, 198, C.2: the kinds-profile spec's "A repository's
        // keep_open": `keep_open` in a repository's table for a kind it does
        // not declare fails the read naming the field and that the host or the
        // launch gives it, and a launch of a fix is refused with nothing made;
        // so does one in its table for every kind, and in a kind's own and a
        // stage's of a kind it declares.
        for (given, _, field) in [
            (
                "[kinds.fix.agents]\nkeep_open = \"2h\"\n",
                "fix",
                "kinds.fix.agents.keep_open",
            ),
            ("[agents]\nkeep_open = \"2h\"\n", "fix", "agents.keep_open"),
            (
                "[kinds.bump.agents.stages.code]\nkeep_open = \"2h\"\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\n\
             deliverables = [\"commits\"]\n",
                "bump",
                "kinds.bump.agents.stages.code.keep_open",
            ),
        ] {
            let repository = Tree::new("declared-keep-open");
            repository.write(DECLARATION, given);
            repository.write(".config/prompts/bump-code.md", "{{brief}}");
            repository.commit("a keep_open in the declaration");
            let reason = repository.could_not_read();
            assert!(
                reason.contains(&format!("{field}: {KEEP_OPEN_REFUSED}")),
                "{given}: {reason}"
            );
        }
    }

    #[test]
    fn a_mode_for_every_stage_and_a_table_for_one() {
        // 23, 162, C.2: a kind's agents table gives one of flume's four for
        // every stage, and a stage's a table of programs, kept as written.
        let repository = Tree::new("declared-mode");
        repository.write(
            DECLARATION,
            r#"[kinds.bump.agents]
mode = "edits"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ mode = { claude = "auto" } }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("a mode for bump and a table for its code");
        let KindsRead::Read { kinds, .. } = repository.read() else {
            panic!("the declaration did not read");
        };
        let bump = &kinds[0];
        assert_eq!(
            bump.agents.every_stage.mode,
            Some(ModeSetting::Flume(Mode::Edits))
        );
        assert_eq!(
            bump.stages[0].agents[0].mode,
            Some(ModeSetting::Programs(BTreeMap::from([(
                "claude".to_string(),
                "auto".to_string()
            )])))
        );
    }

    #[test]
    fn an_entry_flume_cannot_take_refuses_the_launch_naming_the_field() {
        // 4, 195, C.2: an entry that is no table, and a program's own key
        // beside its file rather than under `settings`, each fail the read
        // naming the field, and the launch is refused with nothing made.
        let repository = Tree::new("own-entry");
        repository.write(
            DECLARATION,
            r#"[kinds.change.agents]
programs = { claude = "x" }

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ programs = { claude = { tui = "fullscreen" } } }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("entries flume cannot take");
        let reason = repository.could_not_read();
        for field in [
            "kinds.change.agents.programs.claude:",
            "kinds.bump.stages[0].agents[0].programs.claude.tui:",
            "go under settings",
        ] {
            assert!(
                reason.contains(field),
                "the read names not {field}: {reason}"
            );
        }
    }

    #[test]
    fn an_effort_that_is_no_word_refuses_the_launch_naming_the_field() {
        // 4, 197, C.2: a number and a table of programs each fail the read,
        // naming the table's `effort`, in a repository's table for every kind,
        // in its table for a kind it does not declare, in a stage's of a
        // kind's own and in its agents; the launch is refused with nothing
        // made.
        let repository = Tree::new("declared-bad-effort");
        repository.write(
            DECLARATION,
            r#"[agents]
effort = { claude = "high" }

[kinds.change.agents]
effort = 3

[kinds.bump.agents.stages.code]
effort = { claude = "high" }

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ effort = 3 }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("efforts that are no word");
        let reason = repository.could_not_read();
        for field in [
            "agents.effort:",
            "kinds.change.agents.effort",
            "kinds.bump.agents.stages.code.effort",
            "kinds.bump.stages[0].agents[0].effort",
        ] {
            assert!(
                reason.contains(field),
                "the read names not {field}: {reason}"
            );
        }
    }

    #[test]
    fn hibernate_through_weir_a_conversation_read() {
        // 23, 212, C.2; weir 53, 58; core.4: the kinds-profile spec's "A
        // hibernating conversation read": a declaration whose conversation
        // stage gives `hibernate = true` is read, through weir's `Stage`, with
        // that stage hibernating, and the stage that writes `false` and the
        // one that leaves it out not.
        let repository = Tree::new("declared-hibernate");
        repository.write(
            DECLARATION,
            "[[kinds.research.stages]]\nname = \"talk\"\n\
         prompt = \"prompts/talk.md\"\nconversation = true\nhibernate = true\n\n\
         [[kinds.research.stages]]\nname = \"code\"\n\
         prompt = \"prompts/code.md\"\ndeliverables = [\"commits\"]\n\
         agents = [{}]\nhibernate = false\n\n\
         [[kinds.research.stages]]\nname = \"review\"\n\
         prompt = \"prompts/review.md\"\ndeliverables = [\"commits\"]\n\
         agents = [{}]\n",
        );
        for stage in ["talk", "code", "review"] {
            repository.write(&format!(".config/prompts/{stage}.md"), "{{brief}}");
        }
        repository.commit("a conversation that hibernates");
        let KindsRead::Read { kinds, .. } = repository.read() else {
            panic!("the declaration did not read");
        };
        let research = kinds
            .iter()
            .find(|kind| kind.name == "research")
            .expect("research declared");
        let hibernates: Vec<(&str, bool)> = research
            .stages
            .iter()
            .map(|stage| (stage.name.as_str(), stage.hibernates))
            .collect();
        assert_eq!(
            hibernates,
            vec![("talk", true), ("code", false), ("review", false)],
            "the conversation that writes hibernate hibernates, and every other stage not"
        );
    }

    #[test]
    fn a_repositorys_hibernate_after_refuses_the_launch_naming_the_field() {
        // 4, 158, 212, C.2: the kinds-profile spec's "A repository's hibernate
        // time": `hibernate_after` in a declared kind's own agents table fails
        // the read naming the field and that the host or the launch gives it,
        // and a launch of that kind is refused with nothing made; so does one
        // in a stage's table of a kind it declares, in a repository's table
        // for a kind it does not declare, and in its table for every kind.
        let research = "[[kinds.research.stages]]\nname = \"talk\"\n\
         prompt = \"prompts/talk.md\"\nconversation = true\n";
        for (given, _, field) in [
            (
                format!("[kinds.research.agents]\nhibernate_after = \"2h\"\n\n{research}"),
                "research",
                "kinds.research.agents.hibernate_after",
            ),
            (
                format!(
                    "[kinds.research.agents.stages.talk]\nhibernate_after = \"2h\"\n\n{research}"
                ),
                "research",
                "kinds.research.agents.stages.talk.hibernate_after",
            ),
            (
                "[kinds.fix.agents]\nhibernate_after = \"2h\"\n".to_string(),
                "fix",
                "kinds.fix.agents.hibernate_after",
            ),
            (
                "[agents]\nhibernate_after = \"2h\"\n".to_string(),
                "fix",
                "agents.hibernate_after",
            ),
        ] {
            let repository = Tree::new("declared-hibernate-after");
            repository.write(DECLARATION, &given);
            repository.write(".config/prompts/talk.md", "{{brief}}");
            repository.commit("a hibernate_after in the declaration");
            let reason = repository.could_not_read();
            assert!(
                reason.contains(&format!("{field}: {HIBERNATE_AFTER_REFUSED}")),
                "{given}: {reason}"
            );
        }
    }

    #[test]
    fn a_kind_that_lands_with_no_stages_beside_one_that_does_not_and_settings() {
        // The kinds-profile spec's "Three tables over a scratch repository":
        // weir's crate reads `stages = []` on a kind that lands as a kind with
        // no stages, refuses it on one that does not, naming its stages, and
        // reads a table that leaves `stages` out and holds `agents` alone as
        // settings for a kind declared elsewhere, flume's own fix; a refusal
        // refuses the declaration whole (18, 23, 57, 202; weir 51, 57; core.4).
        let repository = Tree::new("no-stages");
        let declared = "[kinds.train]\nbranch = \"train\"\nlands = true\nstages = []\n\n\
                    [kinds.fix.agents]\nmodel = \"opus\"\n";
        repository.write(DECLARATION, declared);
        repository.commit("a kind that lands with no stages");
        let KindsRead::Read {
            kinds, settings, ..
        } = repository.read()
        else {
            panic!("a kind that lands with no stages did not read");
        };
        assert_eq!(
            kinds
                .iter()
                .map(|kind| (kind.name.as_str(), kind.lands, kind.stages.len()))
                .collect::<Vec<_>>(),
            vec![("train", true, 0)],
            "train, a kind that lands with no stages"
        );
        assert_eq!(kinds[0].branch_prefix, "train");
        assert_eq!(
            settings.keys().collect::<Vec<_>>(),
            vec!["fix"],
            "fix's table, settings for a kind declared elsewhere"
        );
        assert_eq!(settings["fix"].every_stage.model.as_deref(), Some("opus"));
        repository.write(
            DECLARATION,
            &format!("{declared}\n[kinds.idle]\nlands = false\nstages = []\n"),
        );
        repository.commit("a kind that does not land with none");
        match repository.read() {
            KindsRead::CouldNotRead { reason } => {
                assert!(reason.contains("kinds.idle.stages"), "{reason}");
                assert!(
                    !reason.contains("kinds.train") && !reason.contains("kinds.fix"),
                    "a table that reads named in {reason}"
                );
            }
            other => panic!("a kind that does not land with no stages read as {other:?}"),
        }
    }

    #[test]
    fn a_repositorys_agents_for_every_kind() {
        // S102, 21, 32, 195, C.2: the repository's `[agents]`, beside
        // `[profiles]` and no part of any kind, gives its settings for every
        // kind, apart from its settings for one kind; a launch's declaration
        // is not read for it.
        let repository = Tree::new("every-kind");
        let declaration = r#"[profiles]
place = "git-worktree"

[agents]
model = "opus"
mode = "edits"
instructions = "Mind the tagline."

[agents.programs.claude.settings]
attribution = { commit = "Assisted-by: Claude Code", pr = "" }

[kinds.fix.agents]
model = "sonnet"
"#;
        repository.write(DECLARATION, declaration);
        repository.commit("the repository's agents");
        let KindsRead::Read {
            kinds,
            settings,
            agents,
        } = repository.read()
        else {
            panic!("the declaration did not read");
        };
        assert!(kinds.is_empty());
        assert_eq!(agents.model.as_deref(), Some("opus"));
        assert_eq!(agents.mode, Some(ModeSetting::Flume(Mode::Edits)));
        assert_eq!(agents.instructions.as_deref(), Some("Mind the tagline."));
        assert_eq!(
            agents.programs["claude"].settings["attribution"],
            serde_json::json!({ "commit": "Assisted-by: Claude Code", "pr": "" })
        );
        assert_eq!(settings["fix"].every_stage.model.as_deref(), Some("sonnet"));
        assert!(settings["fix"].every_stage.programs.is_empty(), "apart");
        let launched = read(
            &format!("{declaration}\n[agents.stages.code]\nmodel = \"x\"\n"),
            Whose::Launch,
            &|path| Err(format!("{path} is not read")),
        );
        let KindsRead::Read { agents, .. } = launched else {
            panic!("the launch's declaration did not read");
        };
        assert_eq!(
            *agents,
            Settings::default(),
            "a launch's [agents] is not read"
        );
    }

    #[test]
    fn own_settings_for_every_stage_and_for_one() {
        // 23, 157, 195, C.2: a kind's agents table, a stage's agents and a
        // repository's table for a kind it does not declare each give
        // `programs`, kept as JSON values, a stage's file read beside the
        // declaration at the revision, not as the working tree holds it.
        let repository = Tree::new("own-settings");
        repository.write(
            DECLARATION,
            r#"[kinds.change.agents.stages.verify.programs.claude.settings]
statusLine = { type = "command", command = "verify-line" }

[kinds.bump.agents.programs.claude.settings]
tui = "fullscreen"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ programs = { claude = { file = "claude-bump.json" } } }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.write(
            ".config/claude-bump.json",
            r#"{ "statusLine": { "type": "command", "command": "bump-line" } }"#,
        );
        repository.commit("own settings for bump and its code");
        repository.write(".config/claude-bump.json", r#"{ "tui": "default" }"#);
        let KindsRead::Read {
            kinds, settings, ..
        } = repository.read()
        else {
            panic!("the declaration did not read");
        };
        let bump = &kinds[0];
        assert_eq!(
            bump.agents.every_stage.programs["claude"].settings,
            BTreeMap::from([("tui".to_string(), serde_json::json!("fullscreen"))])
        );
        let code = &bump.stages[0].agents[0].programs["claude"];
        assert!(code.settings.is_empty(), "{code:?}");
        assert_eq!(
            code.file,
            Some(NamedFile {
                written: "claude-bump.json".into(),
                read: ".config/claude-bump.json".into(),
                keys: BTreeMap::from([(
                    "statusLine".to_string(),
                    serde_json::json!({ "type": "command", "command": "bump-line" })
                )]),
                left_out: vec![],
            }),
            "read at the revision"
        );
        assert_eq!(
            settings["change"].for_stage("verify").programs["claude"].settings["statusLine"],
            serde_json::json!({ "type": "command", "command": "verify-line" })
        );
    }

    #[test]
    fn a_session_in_an_agents_table_gives_no_setting() {
        // 32, 158, C.2: a kind names nothing a host owns, a session among
        // it: `session` in an agents table, for every stage and for one, in
        // a repository's table for a kind it does not declare, in a kind's
        // own and in a stage's agents, reads as no setting, and the settings
        // beside it are read.
        let repository = Tree::new("declared-session");
        repository.write(
            DECLARATION,
            r#"[kinds.change.agents]
session = "x"
model = "opus"

[kinds.change.agents.stages.verify]
session = "y"
instructions = "Read every task."

[kinds.bump.agents]
session = "x"
model = "sonnet"

[kinds.bump.agents.stages.code]
session = "y"
model = "haiku"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ session = "z", instructions = "keep it short" }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("a session for every stage and for one");
        let KindsRead::Read {
            kinds, settings, ..
        } = repository.read()
        else {
            panic!("the declaration did not read");
        };
        let change = &settings["change"];
        assert_eq!(change.every_stage.session, None, "change's every stage");
        assert_eq!(change.for_stage("verify").session, None, "change's verify");
        assert_eq!(change.every_stage.model.as_deref(), Some("opus"));
        assert_eq!(
            change.for_stage("verify").instructions.as_deref(),
            Some("Read every task.")
        );
        let bump = &kinds[0];
        assert_eq!(bump.agents.every_stage.session, None, "bump's every stage");
        assert_eq!(bump.agents.for_stage("code").session, None, "bump's code");
        assert_eq!(bump.stages[0].agents[0].session, None, "code's own agent");
        assert_eq!(bump.agents.every_stage.model.as_deref(), Some("sonnet"));
        assert_eq!(
            bump.agents.for_stage("code").model.as_deref(),
            Some("haiku")
        );
        assert_eq!(
            bump.stages[0].agents[0].instructions.as_deref(),
            Some("keep it short")
        );
    }

    #[test]
    fn a_repositorys_settings_for_a_kind_it_does_not_declare_and_a_kinds_own() {
        let repository = Tree::new("settings");
        repository.write(
            DECLARATION,
            r#"[kinds.change.agents]
model = "opus"

[kinds.change.agents.stages.verify]
instructions = "Read every task."

[kinds.bump.agents]
program = "agent"
model = "sonnet"

[kinds.bump.agents.stages.code]
model = "haiku"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ instructions = "keep it short" }]
"#,
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("settings for change@1, and bump's own");
        let KindsRead::Read {
            kinds, settings, ..
        } = repository.read()
        else {
            panic!("the declaration did not read");
        };
        assert_eq!(
            kinds
                .iter()
                .map(|kind| kind.name.as_str())
                .collect::<Vec<_>>(),
            vec!["bump"],
            "settings declare no kind"
        );
        assert_eq!(
            settings.keys().collect::<Vec<_>>(),
            vec!["change"],
            "settings apart from the kinds"
        );
        let change = &settings["change"];
        assert_eq!(change.every_stage.model.as_deref(), Some("opus"));
        assert_eq!(
            change.for_stage("verify").instructions.as_deref(),
            Some("Read every task.")
        );
        let bump = &kinds[0];
        assert_eq!(bump.agents.every_stage.program.as_deref(), Some("agent"));
        assert_eq!(
            bump.agents.for_stage("code").model.as_deref(),
            Some("haiku")
        );
        assert_eq!(
            bump.stages[0].agents[0].instructions.as_deref(),
            Some("keep it short"),
            "the stage's own beside the kind's"
        );
    }

    #[test]
    fn a_statechart_kind_is_read_through_weirs_crate() {
        // The kinds-profile spec's "Persona read" and "A simple kind and a
        // statechart kind side by side": weir's crate reads persona beside a
        // simple kind, each in its format, its statechart's text the kind's
        // table alone, and refuses a table holding both `stages` and
        // `regions`, naming the field (84, 85, 175, 177; weir 95; core.4).
        let repository = declaring_persona("persona");
        let KindsRead::Read { kinds, .. } = repository.read() else {
            panic!("persona did not read");
        };
        assert_eq!(
            kinds
                .iter()
                .map(|kind| (kind.name.as_str(), kind.is_statechart()))
                .collect::<Vec<_>>(),
            vec![("bump", false), ("persona", true)],
            "each kind in its format"
        );
        let persona = &kinds[1];
        assert_eq!(
            (
                persona.branch_prefix.as_str(),
                persona.lands,
                persona.version
            ),
            ("persona", true, 1)
        );
        assert!(persona.stages.is_empty(), "{:?}", persona.stages);
        let chart = persona.statechart.as_ref().expect("persona's statechart");
        assert_eq!(chart.title, "Persona test");
        assert!(
            chart.text.contains("[kinds.persona]") && !chart.text.contains("[kinds.bump"),
            "{}",
            chart.text
        );
        assert_eq!(
            chart.stages.keys().collect::<Vec<_>>(),
            vec!["stages.build", "stages.test"]
        );
        let build = &chart.stages["stages.build"];
        match &build.agents {
            ChartAgents::Entries(entries) => {
                assert_eq!(
                    entries
                        .iter()
                        .map(|agent| agent.model.as_deref())
                        .collect::<Vec<_>>(),
                    vec![Some("opus")]
                );
            }
            other => panic!("build's agents read as {other:?}"),
        }
        assert_eq!(build.deliverables, vec![Deliverable::Commits]);
        assert_eq!(
            (build.on_fail.as_deref(), build.send_back_limit),
            (Some("build"), Some(2))
        );
        let test = &chart.stages["stages.test"];
        assert_eq!(
            test.agents,
            ChartAgents::Rule {
                rule: "every-file".into(),
                pattern: Some("personas/*.md".into()),
            }
        );
        assert_eq!(test.join, Join::All);
        assert_eq!(
            test.deliverables,
            vec![Deliverable::CommittedReport {
                path: "tests/personas.md".into()
            }]
        );
        assert_eq!(test.prompt.as_deref(), Some("Test {{file}}."));
        // A table holding both stages and regions.
        repository.write(
        DECLARATION,
        "[kinds.both]\nstages = []\nlands = true\n\n[kinds.both.regions.r]\ninitial = \"s\"\n\n[kinds.both.regions.r.states.s]\nfinal = true\n",
    );
        repository.commit("stages and regions");
        match repository.read() {
            KindsRead::CouldNotRead { reason } => {
                assert!(reason.contains("kinds.both.regions"), "{reason}");
            }
            other => panic!("a table of stages and regions read as {other:?}"),
        }
    }

    /// The kind `train`, landing with no stages, `more` its further
    /// fields, beside its landing fix `mend`, a one-stage kind.
    fn trained(more: &str) -> String {
        format!(
            "[kinds.train]\nbranch = \"train\"\nlands = true\nstages = []\n{more}\n\n\
             [kinds.mend]\nversion = 1\nbranch = \"mend\"\nlands = true\n\n\
             [[kinds.mend.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{}}]\n"
        )
    }

    /// A tree whose declaration is `declaration`, its stages' prompt
    /// beside it, committed.
    fn declaring_trained(declaration: &str) -> Tree {
        let repository = Tree::new("trained");
        repository.write(DECLARATION, declaration);
        repository.write(".config/prompts/code.md", "{{brief}}");
        repository.commit("the declaration");
        repository
    }

    /// Each kind's landing fix and its bound, by the kind's name.
    fn landing_fixes(repository: &Tree) -> BTreeMap<String, (Option<String>, Option<u32>)> {
        match repository.read() {
            KindsRead::Read { kinds, .. } => kinds
                .into_iter()
                .map(|kind| (kind.name, (kind.fix, kind.fix_limit)))
                .collect(),
            other => panic!("the declaration read as {other:?}"),
        }
    }

    #[test]
    fn a_kind_with_no_stages_names_its_landing_fix_and_its_bound() {
        // The kinds-profile spec's "A landing fix and its bound": weir's
        // crate reads a kind's `landing_fix` and `max_landing_fixes`, and
        // the profile maps them onto the kind of its fixes, spelled as weir
        // spells the identity, and its bound; a kind that gives neither
        // names neither (23; weir 52; weir record 0005; core.4).
        let declared = format!(
            "{}\n[kinds.yard]\nbranch = \"yard\"\nlands = true\nstages = []\n",
            trained("landing_fix = \"repo/mend@1\"\nmax_landing_fixes = 2")
        );
        let none = (None, None);
        assert_eq!(
            landing_fixes(&declaring_trained(&declared)),
            BTreeMap::from([
                ("mend".to_string(), none.clone()),
                (
                    "train".to_string(),
                    (Some("repo/mend@1".to_string()), Some(2))
                ),
                ("yard".to_string(), none),
            ]),
            "each kind's landing fix and its bound"
        );
    }

    #[test]
    fn a_landing_fix_weir_refuses_is_named_in_weirs_words() {
        // The kinds-profile spec's "A landing fix weir refuses": a bare
        // name, a name with no version, and either field on a kind with
        // stages are answered as could not be read in weir's words, naming
        // the field (57; weir record 0005).
        for named in ["mend", "repo/mend"] {
            let reason =
                declaring_trained(&trained(&format!("landing_fix = \"{named}\""))).could_not_read();
            assert!(
                reason.contains("kinds.train.landing_fix") && reason.contains("is no identity"),
                "{named} not refused as no identity: {reason}"
            );
        }
        let reason = declaring_trained(&format!(
            "{}\n[kinds.bump]\nbranch = \"bump\"\n\
             landing_fix = \"flume/fix@1\"\nmax_landing_fixes = 1\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{}}]\n",
            trained("")
        ))
        .could_not_read();
        for field in ["kinds.bump.landing_fix", "kinds.bump.max_landing_fixes"] {
            assert!(
                reason.contains(&format!("{field}: is on a kind with stages")),
                "{field} not refused as on a kind with stages: {reason}"
            );
        }
    }

    #[test]
    fn a_landing_fix_bound_past_what_flume_counts_is_refused() {
        // The kinds-profile spec's "A bound past what flume counts": weir
        // reads the count, and the profile refuses the declaration naming
        // the field (4, 84).
        let reason = declaring_trained(&trained(
            "landing_fix = \"repo/mend@1\"\nmax_landing_fixes = 4294967296",
        ))
        .could_not_read();
        assert!(
            reason.contains("kinds.train.max_landing_fixes: is more than flume counts"),
            "{reason}"
        );
    }

    #[test]
    fn fix_and_max_fixes_are_sections_naming_no_landing_fix() {
        // The kinds-profile spec's "The fields' old spellings": weir returns
        // `fix` and `max_fixes` as sections, and flume reads no fix and no
        // bound from them (23; weir record 0005; core.4).
        assert_eq!(
            landing_fixes(&declaring_trained(&trained(
                "fix = \"repo/mend@1\"\nmax_fixes = 2"
            )))
            .get("train"),
            Some(&(None, None)),
            "train's landing fix and its bound"
        );
    }

    #[test]
    fn a_mode_that_is_neither_refuses_the_read_naming_the_field() {
        // 4, 169, C.2: a program's own value given bare, a list and a table
        // naming no program each fail the read, naming the table's `mode`,
        // in a repository's table for a kind it does not declare, in a
        // stage's of a kind's own and in its agents table.
        let repository = Tree::new("declared-bad-mode");
        repository.write(
            DECLARATION,
            "[kinds.change.agents]\nmode = \"auto\"\n\n[kinds.bump.agents.stages.code]\nmode = {}\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{ mode = [\"edits\"] }]\n",
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("modes that are neither");
        let reason = repository.could_not_read();
        for field in [
            "kinds.change.agents.mode",
            "kinds.bump.agents.stages.code.mode",
            "kinds.bump.stages[0].agents[0].mode",
            "`auto`",
        ] {
            assert!(
                reason.contains(field),
                "the read names not {field}: {reason}"
            );
        }
    }

    #[test]
    fn an_environment_in_any_declaration_refuses_the_read_naming_the_field() {
        // C.2, 158: an agent's environment is its account's alone (144), so
        // an `env` in a repository's agents table for a kind it does not
        // declare, in a kind's own, or in a launch's declaration fails the
        // read, naming the field.
        let repository = Tree::new("declared-env");
        repository.write(
            DECLARATION,
            "[kinds.change.agents]\nenv = { A = \"b\" }\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{ env = { B = \"c\" } }]\n",
        );
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("environments in the declaration");
        let reason = repository.could_not_read();
        for field in [
            "kinds.change.agents.env",
            "kinds.bump.stages[0].agents[0].env",
        ] {
            assert!(
                reason.contains(field),
                "the read names not {field}: {reason}"
            );
        }
        let launched = read(
            "[kinds.bump.agents.stages.code]\nenv = { C = \"d\" }\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\ndeliverables = [\"commits\"]\n",
            Whose::Launch,
            &|path| Ok((path.to_string(), "{{brief}}".to_string())),
        );
        match launched {
            KindsRead::CouldNotRead { reason } => assert!(
                reason.contains("kinds.bump.agents.stages.code.env"),
                "the launch's declaration's read names not the field: {reason}"
            ),
            other => panic!("a launch's declaration giving an environment read as {other:?}"),
        }
    }

    #[test]
    fn a_prompt_the_revision_lacks_is_named() {
        // 84, 157: a stage's prompt the declaration's revision lacks fails
        // the read, naming the field and the prompt's path in the tree.
        let repository = Tree::new("no-prompt");
        repository.write(
            DECLARATION,
            "[kinds.bump]\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/gone.md\"\ndeliverables = [\"commits\"]\nagents = [{}]\n",
        );
        repository.commit("no prompt");
        let reason = repository.could_not_read();
        assert!(
            reason.contains("kinds.bump.stages[0].prompt")
                && reason.contains(".config/prompts/gone.md"),
            "{reason}"
        );
    }

    #[test]
    fn a_repository_with_no_declaration_declares_no_kind() {
        let repository = Tree::new("none");
        repository.commit("nothing declared");
        assert_eq!(
            repository.read(),
            KindsRead::Read {
                kinds: vec![],
                settings: Default::default(),
                agents: Box::default(),
            }
        );
    }

    #[test]
    fn a_binding_names_each_seams_profile_as_written() {
        // C.2: `[profiles]` names each seam's profile as written, a profile
        // flume does not hold among them, which the core refuses at the
        // launch naming those there are; a declaration with no `[profiles]`
        // binds none; and flume's own repository binds git-worktree,
        // openspec and sluice.
        assert_eq!(
            profiles("[profiles]\nplace = \"worktrunk\"\n\n[kinds.notes]\n"),
            Ok(Binding {
                place: Some("worktrunk".into()),
                ..Binding::default()
            })
        );
        assert_eq!(profiles("[kinds.notes]\n"), Ok(Binding::default()));
        assert_eq!(
            profiles("[profiles]\nplace = 3\n"),
            Err("profiles.place: must be a profile's name".into())
        );
        assert_eq!(
            profiles(include_str!("../../../../../../.config/flume.toml")),
            Ok(Binding {
                place: Some("git-worktree".into()),
                change: Some("openspec".into()),
                merge_queue: Some("sluice".into()),
            })
        );
    }
}
