//! An agent's settings, resolved at its start from the eight layers
//! clause 32 orders and from nothing else (design 3; records 0017, 0043;
//! core.30): the launch's, the host's for the repository, for its kind
//! and then for every kind, the host's for the kind, the host's for the
//! place's stream, the host's for every agent, the repository's, for a
//! kind it does not declare and then for every kind, the kind's
//! declaration, and flume's default. Within a layer a stage's own come
//! before those for every stage, and those before a repository's for
//! every kind, and every setting is taken whole. A program's own
//! settings are taken key by key, each top-level key whole from the
//! first table that gives it, a table's `settings` before its file, and
//! only the agent's program's (195). The host's layers are read as they
//! stand at the start, their own settings checked then, every other
//! layer as the launch read it. An agent's environment is its account's
//! alone, each value's `~` and variables read against the host user's
//! login environment (29, 144). How long it is kept open for an answer,
//! `keep_open`, is taken whole as the model is, flume's own hour, written
//! `1h`, in the default layer (198), and how long a conversation whose
//! kind hibernates it may idle before it is hibernated, `hibernate_after`,
//! likewise, its own hour beside it (212). Its mode, where a layer gives one, is
//! mapped by its agent profile to its program's setting, or the stage
//! refused naming the mode, the program, the stage and the layer (162,
//! 169); its model, where a layer gives one, its program's own name or a
//! table of programs taken whole, the agent's entry its program's, is
//! asked of its profile, or the stage refused naming the model as
//! written, the program, the stage and the layer, a table naming no entry
//! for its program refused so, and nothing is asked where no layer gives
//! one (172); its effort, where a layer gives one, its program's own word
//! or a table of programs taken whole as the model is, the agent's entry
//! its program's, is asked of its profile, or the stage refused naming
//! the effort as written, the program, the stage and the layer, a table
//! naming no entry for its program refused so, and nothing is asked
//! where no layer gives one (197); and its own settings are taken by its
//! profile, or the stage refused naming each key with its layer and its
//! file (195, 196). flume's default names the program only for an agent
//! with no pool: where no layer above it names one and an account is
//! chosen, the account's program stands in its place, at the layer
//! `account`, and its own settings are read for that program (32 item 8,
//! 173; design 1).

use crate::own::{fold, Table};
use crate::{Engine, HIBERNATE_AFTER, KEEP_OPEN};
use flume_seams::{
    Account, Agent, Deliverable, Given, Kind, KindSettings, Layer, Mode, ModeEntry, OwnFile,
    PerProgram, PlaceRecord, ProgramSettings, Settings, Stage, Terminal,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;

/// One of the eight layers of a stage's settings: its tables, most
/// specific first, a stage's own, then the table for every stage, then,
/// in a repository's two layers, the table for every kind (32).
type Tables = (Layer, Vec<Settings>);

/// A stage's settings with each value as its layer wrote it, and the
/// layer of each, by `program`, `model`, `effort`, `mode`,
/// `instructions`, `keep_open`, `hibernate_after` or
/// `programs.<program>.settings.<key>`, an own setting's with its file
/// (32, 56, 195, 197, 198, 212); the agent's
/// program's own settings; and the layers they were folded from.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Resolved {
    pub settings: Settings,
    pub from: BTreeMap<String, Given>,
    /// The agent's program's own settings, none where no layer gives its
    /// program an entry (195).
    pub own: Option<Own>,
    layers: Vec<Tables>,
}

/// A program's own settings as the layers resolved them: each key whole,
/// its value as written, with its layer and its file where it came from
/// one, and each file a layer named for the program, most specific first
/// (195).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Own {
    pub keys: BTreeMap<String, Value>,
    pub from: BTreeMap<String, Given>,
    pub files: Vec<OwnFile>,
}

impl Resolved {
    /// Where the setting `key` came from, as a refusal names it.
    fn layer_of(&self, key: &str) -> Layer {
        self.from
            .get(key)
            .map_or(Layer::Default, |given| given.layer)
    }

    /// Whether a layer above flume's default names the agent's program, so
    /// no account's program stands in for it (32 item 8, 173; design 1).
    pub(crate) fn names_program(&self) -> bool {
        self.from
            .get("program")
            .is_some_and(|given| !matches!(given.layer, Layer::Default | Layer::Account))
    }

    /// `program`'s own settings as the layers give them, by its name, key
    /// by key from the most general table to the most specific; none
    /// where no table gives it an entry (32, 195; design 4).
    pub(crate) fn own_for(&self, program: &str) -> Option<Own> {
        let program = program_name(program);
        let mut own: Option<Own> = None;
        for (layer, tables) in self.layers.iter().rev() {
            for table in tables.iter().rev() {
                let Some(entry) = table.programs.get(program) else {
                    continue;
                };
                let own = own.get_or_insert_with(Own::default);
                if let Some(file) = &entry.file {
                    own.files.insert(
                        0,
                        OwnFile {
                            layer: *layer,
                            written: file.written.clone(),
                            read: file.read.clone(),
                            left_out: file.left_out.clone(),
                        },
                    );
                }
                for (key, folded) in fold(entry) {
                    own.keys.insert(key.to_string(), folded.value.clone());
                    own.from.insert(
                        format!("programs.{program}.settings.{key}"),
                        Given {
                            layer: *layer,
                            value: folded.value.to_string(),
                            file: folded.file.map(|file| file.read.clone()),
                        },
                    );
                }
            }
        }
        own
    }
}

/// A stage's agent at its start: its settings as written with their
/// layers, the agent profile that runs its program, the value that
/// profile passes for its mode, and its entries of its model and its
/// effort, which that profile took, each none where no layer gives one
/// (162, 169, 172, 197).
pub(crate) struct Resolution {
    pub resolved: Resolved,
    pub agent: Arc<dyn Agent>,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub effort: Option<String>,
}

impl Resolution {
    /// The settings the agent's command is given: those resolved, with
    /// the model and the effort each the agent's entry, given whole, so a
    /// profile reads one name and one word and never a table (172, 197;
    /// design 4).
    pub(crate) fn command_settings(&self) -> Settings {
        Settings {
            model: self.model.clone().map(PerProgram::One),
            effort: self.effort.clone().map(PerProgram::One),
            ..self.resolved.settings.clone()
        }
    }

    /// The agent's program's own settings as the layers resolved them,
    /// which its profile takes; none where they give none (195, 196).
    pub(crate) fn own_keys(&self) -> BTreeMap<String, Value> {
        self.resolved
            .own
            .as_ref()
            .map(|own| own.keys.clone())
            .unwrap_or_default()
    }
}

/// Why `agent` refuses `stage`'s own settings for `program` as `own`
/// resolved them, each key named with its layer and its file where it
/// came from one, in the profile's words; none where it takes them or
/// they are empty (195, 196; design 13).
pub(crate) fn own_refused(
    agent: &dyn Agent,
    stage: &str,
    program: &str,
    own: &Own,
) -> Option<String> {
    if own.keys.is_empty() {
        return None;
    }
    let words = agent.own(&own.keys).err()?;
    let keys = own
        .keys
        .keys()
        .map(|key| {
            let given = own.from.get(&format!("programs.{program}.settings.{key}"));
            let layer = given.map_or(Layer::Default, |given| given.layer);
            let file = given
                .and_then(|given| given.file.as_ref())
                .map_or(String::new(), |file| format!(" by the file {file}"));
            format!("`{key}` from the layer `{layer}`{file}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "the stage `{stage}`'s own settings for `{program}`, {keys}, are refused: {words}"
    ))
}

/// The program's name, the last component of the program its settings
/// name, by which its agent profile is bound and its entry of a mode's
/// table of programs taken (105, 169).
pub(crate) fn program_name(program: &str) -> &str {
    std::path::Path::new(program)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(program)
}

/// A deliverable as a refusal names it (24).
fn deliverable_words(deliverable: &Deliverable) -> String {
    match deliverable {
        Deliverable::Commits => "commits".into(),
        Deliverable::ChangeWritten => "a change written".into(),
        Deliverable::TasksTicked => "a change's tasks ticked".into(),
        Deliverable::ChangeArchived => "a change archived".into(),
        Deliverable::Report { path } => format!("a report at {path}"),
        Deliverable::CommittedReport { path } => format!("a committed report at {path}"),
        Deliverable::Unreadable { spelled } => spelled.clone(),
    }
}

/// The value `agent` passes for `stage`'s mode as `resolved` gives it,
/// none where no layer gives one, so nothing is mapped; or why the stage
/// cannot run in it, naming the mode as written, the program, the stage
/// and the layer: a table of programs naming no entry for its program,
/// which no later layer's mode stands in for, `plan` on a stage that is
/// no conversation, or a mode its profile refuses (4, 32, 109, 169).
fn mode_for(
    resolved: &Resolved,
    agent: &dyn Agent,
    stage: &Stage,
) -> Result<Option<String>, String> {
    let Some(mode) = &resolved.settings.mode else {
        return Ok(None);
    };
    let written = mode.written();
    let layer = resolved.layer_of("mode");
    let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
    let name = &stage.name;
    let entry = mode.entry(program).ok_or_else(|| {
        format!(
            "the stage `{name}`'s mode `{written}`, from the layer `{layer}`, names no entry for `{program}`"
        )
    })?;
    let plan = ModeEntry::Flume(Mode::Plan);
    let planning = entry == plan
        || matches!(&entry, ModeEntry::Own(value) if agent.mode(&plan).as_ref() == Ok(value));
    if planning && !stage.conversation {
        let delivers = stage
            .deliverables
            .iter()
            .map(deliverable_words)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "the stage `{name}` delivers {delivers}, which an agent in plan mode cannot write, and its mode `{written}`, from the layer `{layer}`, is plan"
        ));
    }
    agent.mode(&entry).map(Some).map_err(|words| {
        format!(
            "the stage `{name}`'s mode `{written}`, from the layer `{layer}`, cannot run on `{program}`: {words}"
        )
    })
}

/// The agent's entry of `stage`'s model as `resolved` gives it, which
/// `agent` took, none where no layer gives one, so nothing is asked; or
/// why the stage cannot run on it, naming the model as written, the
/// program, the stage and the layer: a table of programs naming no entry
/// for its program, which no later layer's model stands in for, or an
/// entry its profile refuses, with the profile's words (4, 32, 172).
fn model_taken(
    resolved: &Resolved,
    agent: &dyn Agent,
    stage: &str,
) -> Result<Option<String>, String> {
    let Some(model) = &resolved.settings.model else {
        return Ok(None);
    };
    let written = model.written();
    let layer = resolved.layer_of("model");
    let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
    let entry = model.entry(program).ok_or_else(|| {
        format!(
            "the stage `{stage}`'s model `{written}`, from the layer `{layer}`, names no entry for `{program}`"
        )
    })?;
    agent.model(entry).map(|()| Some(entry.to_string())).map_err(|words| {
        format!(
            "the stage `{stage}`'s model `{written}`, from the layer `{layer}`, cannot run on `{program}`: {words}"
        )
    })
}

/// The agent's entry of `stage`'s effort as `resolved` gives it, which
/// `agent` took, asked only where a layer gives one; or why the stage
/// cannot run at it, naming the effort as written, the program, the
/// stage and the layer: a table of programs naming no entry for its
/// program, which no later layer's effort stands in for, or an entry its
/// profile refuses, with the profile's words (4, 32, 197).
fn effort_taken(
    resolved: &Resolved,
    agent: &dyn Agent,
    stage: &str,
) -> Result<Option<String>, String> {
    let Some(effort) = &resolved.settings.effort else {
        return Ok(None);
    };
    let written = effort.written();
    let layer = resolved.layer_of("effort");
    let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
    let entry = effort.entry(program).ok_or_else(|| {
        format!(
            "the stage `{stage}`'s effort `{written}`, from the layer `{layer}`, names no entry for `{program}`"
        )
    })?;
    agent.effort(entry).map(|()| Some(entry.to_string())).map_err(|words| {
        format!(
            "the stage `{stage}`'s effort `{written}`, from the layer `{layer}`, cannot run on `{program}`: {words}"
        )
    })
}

/// Whether `terminal` starts the kind of `resolution`'s agent's program
/// under the login environment with `env` over it, asked only where its
/// profile names a kind; or why not, naming the stage and the program,
/// with the terminal's words (4, 27, 32, C.5).
pub(crate) fn kind_started(
    terminal: &dyn Terminal,
    resolution: &Resolution,
    stage: &str,
    env: &BTreeMap<String, String>,
) -> Result<(), String> {
    let Some(kind) = resolution.agent.kind() else {
        return Ok(());
    };
    let program = program_name(
        resolution
            .resolved
            .settings
            .program
            .as_deref()
            .unwrap_or_default(),
    );
    terminal.starts(kind, env).map_err(|words| {
        format!("the stage `{stage}`'s program `{program}` cannot start in its terminal: {words}")
    })
}

/// Settings with `beside` and `session` taken away, for a layer that does
/// not say where in the terminal an agent runs (107, 32).
fn placeless(mut settings: Settings) -> Settings {
    settings.beside = None;
    sessionless(settings)
}

/// Settings with `session` taken away, for every layer but the launch's:
/// a kind names nothing a host owns, and a host's agents table names no
/// session (32, 107, 158).
fn sessionless(mut settings: Settings) -> Settings {
    settings.session = None;
    settings
}

/// `value` with a leading `~` read as the home and each `$VAR` or
/// `${VAR}` as `login` gives it, or the first variable `login` lacks
/// (29, C.7).
pub(crate) fn expand(
    value: &str,
    login: impl Fn(&str) -> Option<String>,
) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = value;
    if rest == "~" || rest.starts_with("~/") {
        out.push_str(&login("HOME").ok_or("HOME")?);
        rest = &rest[1..];
    }
    let named = |c: char| c.is_ascii_alphanumeric() || c == '_';
    while let Some(at) = rest.find('$') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let (name, next) = match after.strip_prefix('{') {
            Some(braced) => match braced.find('}') {
                Some(close) => (&braced[..close], &braced[close + 1..]),
                None => ("", after),
            },
            None => {
                let end = after.find(|c: char| !named(c)).unwrap_or(after.len());
                (&after[..end], &after[end..])
            }
        };
        if name.is_empty()
            || name.starts_with(|c: char| c.is_ascii_digit())
            || !name.chars().all(named)
        {
            out.push('$');
            rest = after;
            continue;
        }
        out.push_str(&login(name).ok_or_else(|| name.to_string())?);
        rest = next;
    }
    out.push_str(rest);
    Ok(out)
}

impl Engine {
    /// The eight layers of `stage`'s settings in `place`, most specific
    /// first, each its tables most specific first (design 3). The host's
    /// configuration is read here, at each start, and nowhere else, its
    /// own settings checked as they stand, or the refusal (32, 195, C.7).
    fn layers(&self, place: &PlaceRecord, stage: &Stage) -> Result<Vec<Tables>, String> {
        let host = self.host.settings(
            &place.repository_name(),
            &place.kind.name,
            &place.stream.name,
        );
        let name = &stage.name;
        let own_of =
            |settings: &KindSettings| settings.stages.get(name).cloned().unwrap_or_default();
        let checked = |settings: &Settings, table: Table| -> Result<Settings, String> {
            self.own_checked(settings, &table).map(placeless)
        };
        let for_kind = |settings: &KindSettings, layer: Layer| -> Result<Vec<Settings>, String> {
            Ok(vec![
                checked(&own_of(settings), Table::stage(layer, name))?,
                checked(&settings.every_stage, Table::every_stage(layer))?,
            ])
        };
        let mut host_repository = for_kind(&host.repository, Layer::HostRepository)?;
        host_repository.push(checked(
            &host.every_kind,
            Table::new(Layer::HostRepository, "for every kind"),
        )?);
        let stream = Table::new(
            Layer::HostStream,
            format!("for the stream `{}`", place.stream.name),
        );
        let repository = &place.repository_settings;
        let declared = &place.kind.agents;
        Ok(vec![
            (
                Layer::Launch,
                vec![
                    place.settings.get(name).cloned().unwrap_or_default(),
                    place.every_stage.clone(),
                ],
            ),
            (Layer::HostRepository, host_repository),
            (Layer::HostKind, for_kind(&host.kind, Layer::HostKind)?),
            (Layer::HostStream, vec![checked(&host.stream, stream)?]),
            (
                Layer::Host,
                vec![checked(
                    &host.every_agent,
                    Table::new(Layer::Host, "for every agent"),
                )?],
            ),
            (
                Layer::Repository,
                vec![
                    placeless(own_of(repository)),
                    placeless(repository.every_stage.clone()),
                    placeless(place.repository_agents.clone()),
                ],
            ),
            (
                Layer::Kind,
                vec![
                    sessionless(stage.agents.first().cloned().unwrap_or_default()),
                    sessionless(own_of(declared)),
                    sessionless(declared.every_stage.clone()),
                ],
            ),
            (
                Layer::Default,
                vec![self.defaults.settings.clone(), flumes_own_times()],
            ),
        ])
    }

    /// `stage`'s settings in `place` as each layer wrote them, folded
    /// from the most general layer to the most specific, keeping the
    /// layer of every setting and every variable, and its program's own
    /// settings key by key (32, 195). `account` is the program the
    /// account chosen names, none where none is chosen: where no layer
    /// above flume's default names a program, it stands in the default's
    /// place, at the layer `account`, and the own settings are read for
    /// it; where one does, it is ignored (32 item 8, 173; design 1).
    pub(crate) fn resolve(
        &self,
        place: &PlaceRecord,
        stage: &Stage,
        account: Option<&str>,
    ) -> Result<Resolved, String> {
        let mut resolved = Resolved {
            layers: self.layers(place, stage)?,
            ..Resolved::default()
        };
        for (layer, tables) in resolved.layers.iter().rev() {
            let layer = *layer;
            let given = tables
                .iter()
                .rev()
                .fold(Settings::default(), |under, table| under.under(table));
            let whole = [
                ("program", given.program.clone()),
                ("model", given.model.as_ref().map(PerProgram::written)),
                ("effort", given.effort.as_ref().map(PerProgram::written)),
                ("instructions", given.instructions.clone()),
                ("keep_open", given.keep_open.clone()),
                ("hibernate_after", given.hibernate_after.clone()),
            ];
            for (key, value) in whole {
                if let Some(value) = value {
                    resolved.from.insert(
                        key.into(),
                        Given {
                            layer,
                            value,
                            file: None,
                        },
                    );
                }
            }
            if let Some(mode) = &given.mode {
                resolved.from.insert(
                    "mode".into(),
                    Given {
                        layer,
                        value: mode.written(),
                        file: None,
                    },
                );
            }
            resolved.settings = resolved.settings.under(&given);
        }
        if let Some(program) = account.filter(|_| !resolved.names_program()) {
            resolved.settings.program = Some(program.to_string());
            resolved.from.insert(
                "program".into(),
                Given {
                    layer: Layer::Account,
                    value: program.to_string(),
                    file: None,
                },
            );
        }
        let program = resolved.settings.program.clone().unwrap_or_default();
        resolved.own = resolved.own_for(&program);
        resolved.settings.programs = BTreeMap::new();
        if let Some(own) = &resolved.own {
            resolved.from.extend(own.from.clone());
            if !own.keys.is_empty() {
                resolved.settings.programs.insert(
                    program_name(&program).to_string(),
                    ProgramSettings {
                        file: None,
                        settings: own.keys.clone(),
                    },
                );
            }
        }
        Ok(resolved)
    }

    /// The agent profile `resolved`'s program names, or why there is
    /// none, naming the program, the stage and its layer, or that its
    /// account names it (4, 32, 105, 173).
    pub(crate) fn agent_for(
        &self,
        resolved: &Resolved,
        stage: &str,
    ) -> Result<Arc<dyn Agent>, String> {
        let layer = resolved.layer_of("program");
        let Some(program) = &resolved.settings.program else {
            return Err(format!("no agent program is named for the stage {stage}"));
        };
        self.agent(program).ok_or_else(|| match layer {
            Layer::Account => format!(
                "no agent profile runs the program {program} its account names for the stage {stage}"
            ),
            _ => format!(
                "no agent profile runs the program {program} the stage {stage} names, from the layer {layer}"
            ),
        })
    }

    /// `account`'s environment with each value's `~` and variables read
    /// against the host user's login environment, or the variable it
    /// lacks, naming it and the account (27, 29, 144).
    pub(crate) fn account_env(
        &self,
        account: &Account,
        stage: &str,
    ) -> Result<BTreeMap<String, String>, String> {
        account
            .env
            .iter()
            .map(|(name, value)| {
                expand(value, |variable| self.host.login(variable))
                    .map(|read| (name.clone(), read))
                    .map_err(|variable| {
                        format!(
                            "the stage {stage}'s account {}'s env.{name} names ${variable}, which the host user's login environment lacks",
                            account.name
                        )
                    })
            })
            .collect()
    }

    /// The stage's agent at a start: its settings resolved, with the
    /// program `account` names where no layer above flume's default names
    /// one, the agent profile that runs its program, the value it passes
    /// for its mode, its effort, which the profile takes, and its
    /// program's own settings, which the profile takes; or why the stage
    /// is refused (27, 32, 169, 173, 195, 197).
    pub(crate) fn resolution(
        &self,
        place: &PlaceRecord,
        stage: &Stage,
        account: Option<&str>,
    ) -> Result<Resolution, String> {
        self.checked(self.resolve(place, stage, account)?, stage)
    }

    /// `stage`'s agent with `resolved` as its settings: the agent profile
    /// that runs its program, the value it passes for its mode, its model
    /// and its effort, which the profile takes, and its program's own
    /// settings, which the profile takes; or why the stage is refused (27,
    /// 32, 169, 172, 195, 197).
    pub(crate) fn checked(&self, resolved: Resolved, stage: &Stage) -> Result<Resolution, String> {
        let agent = self.agent_for(&resolved, &stage.name)?;
        let model = model_taken(&resolved, agent.as_ref(), &stage.name)?;
        let mode = mode_for(&resolved, agent.as_ref(), stage)?;
        let effort = effort_taken(&resolved, agent.as_ref(), &stage.name)?;
        if let Some(own) = &resolved.own {
            let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
            if let Some(refused) = own_refused(agent.as_ref(), &stage.name, program, own) {
                return Err(refused);
            }
        }
        Ok(Resolution {
            resolved,
            agent,
            mode,
            model,
            effort,
        })
    }
}

/// Each program a stage whose layers name none may run on, in the order
/// given, with its resolution, or the words that refuse it (145, 173;
/// design 2).
pub(crate) type RunsOn = Vec<(String, Result<Resolution, String>)>;

impl Engine {
    /// For each distinct program of `programs`, in order, `stage`'s agent
    /// in `place` resolved with that program as its account's, or the
    /// words that refuse it: a program no profile binds, or whose profile
    /// maps not the stage's mode or takes not its model, its effort or its
    /// own settings, refused as a start refuses it, so one reading serves
    /// the choice, the wait, the refusal and the start (105, 145, 169, 172,
    /// 173, 196, 197; design 2).
    pub(crate) fn runs_on<'a>(
        &self,
        place: &PlaceRecord,
        stage: &Stage,
        programs: impl IntoIterator<Item = &'a str>,
    ) -> RunsOn {
        let mut read: RunsOn = Vec::new();
        for program in programs {
            if read.iter().any(|(seen, _)| seen == program) {
                continue;
            }
            let resolution = self.resolution(place, stage, Some(program));
            read.push((program.to_string(), resolution));
        }
        read
    }
}

/// The programs of `runs` that cannot run the stage, each with the words
/// that refuse it, in order (145, 173).
pub(crate) fn refused(runs: &RunsOn) -> Vec<(String, String)> {
    runs.iter()
        .filter_map(|(program, resolution)| {
            resolution
                .as_ref()
                .err()
                .map(|words| (program.clone(), words.clone()))
        })
        .collect()
}

/// Why the stage `stage`, whose layers name no program, cannot start on
/// the pool `pool`, none of whose accounts' programs can run it: the
/// words a launch and a start refuse it in, naming the stage, the pool
/// and each program with the words that refuse it (4, 32, 145, 173;
/// design 2).
pub(crate) fn cannot_run(stage: &str, pool: &str, refused: &[(String, String)]) -> String {
    let why = if refused.is_empty() {
        "it holds no account".to_string()
    } else {
        refused
            .iter()
            .map(|(program, words)| format!("`{program}` cannot, as {words}"))
            .collect::<Vec<_>>()
            .join("; ")
    };
    format!(
        "the stage `{stage}` names no program, and no account of the pool `{pool}` can run it: {why}; so no room it gives could start it (145, 173)"
    )
}

/// flume's own table of the default layer: its one hour kept open for an
/// answer and its one hour a conversation may idle before it hibernates,
/// each written as a layer writes it, `1h`, so status gives them as it
/// gives every setting (32, 56, 198, 212).
fn flumes_own_times() -> Settings {
    Settings {
        keep_open: Some(format!("{}h", KEEP_OPEN / (60 * 60))),
        hibernate_after: Some(format!("{}h", HIBERNATE_AFTER / (60 * 60))),
        ..Settings::default()
    }
}

/// Refuses settings for a stage the kind does not hold, naming the stage
/// and the layer that gave them; the host's are ignored, never refused
/// (4, 32, C.2).
pub(crate) fn no_stage_lacking(
    kind: &Kind,
    launched: &BTreeMap<String, Settings>,
    repository: &KindSettings,
) -> Result<(), String> {
    let named = [
        (Layer::Launch, launched.keys().collect::<Vec<_>>()),
        (Layer::Repository, repository.stages.keys().collect()),
        (Layer::Kind, kind.agents.stages.keys().collect()),
    ];
    for (layer, stages) in named {
        if let Some(stage) = stages
            .into_iter()
            .find(|stage| !kind.stages.iter().any(|held| &&held.name == stage))
        {
            return Err(format!(
                "settings from the layer {layer} name the stage {stage}, which the kind {}@{} does not hold",
                kind.name, kind.version
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Defaults;
    use flume_fakes::*;
    use flume_seams::{Binding, Identity};
    use serde_json::json;

    /// An engine over fresh fakes whose default program is `agent`, its
    /// resolver binding a fake agent to `agent` and another to `other`,
    /// built as `engine` in `deliveries.rs`'s tests builds one (32).
    fn engine() -> (Engine, Arc<FakeAgent>, Arc<FakeAgent>) {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        let resolver = FakeResolver::new(Arc::new(FakeKinds::default()));
        let agent = Arc::new(FakeAgent::new(terminal.clone()));
        let other = Arc::new(FakeAgent::new(terminal.clone()));
        resolver.bind_agent("agent", agent.clone());
        resolver.bind_agent("other", other.clone());
        let engine = Engine::new(
            Arc::new(resolver),
            terminal,
            Arc::new(FakeStore::default()),
            Arc::new(FakeHistory::new(FakeGit::new())),
            host,
            Defaults {
                settings: Settings {
                    program: Some("agent".into()),
                    ..Settings::default()
                },
                send_backs: 2,
            },
        );
        (engine, agent, other)
    }

    /// The stage `review`, delivering commits, its declaration's agent
    /// `agents`.
    fn stage(agents: Settings) -> Stage {
        let mut stage: Stage = serde_json::from_value(json!({
            "name": "review",
            "prompt": "Review the work.",
            "conversation": false,
            "deliverables": ["commits"],
            "agents": [],
            "waits": false,
        }))
        .expect("a stage");
        stage.agents = vec![agents];
        stage
    }

    /// A place of `fix`, as `tests::record` in `lib.rs` makes one, whose
    /// kind gives own settings for both `agent` and `other` (195).
    fn place() -> PlaceRecord {
        let mut place = crate::tests::record(Binding::default());
        place.kind.agents.every_stage.programs = [("agent", "a"), ("other", "o")]
            .into_iter()
            .map(|(program, key)| {
                (
                    program.to_string(),
                    ProgramSettings {
                        file: None,
                        settings: BTreeMap::from([(key.to_string(), json!(true))]),
                    },
                )
            })
            .collect();
        place
    }

    fn identity() -> Identity {
        Identity {
            repository: "/r".into(),
            place: "fix--one".into(),
            stage: "review".into(),
            start: 1,
            resume: 0,
        }
    }

    #[test]
    fn an_accounts_program_stands_for_the_defaults() {
        // Only flume's default names the program, so an account's stands
        // in its place, at the layer `account`, with its own settings;
        // with no account, the default's (32 item 8, 173).
        let (engine, _, other) = engine();
        let (place, stage) = (place(), stage(Settings::default()));
        let alone = engine.resolve(&place, &stage, None).expect("resolved");
        assert_eq!(alone.settings.program.as_deref(), Some("agent"));
        assert_eq!(alone.from["program"].layer, Layer::Default);
        assert!(!alone.names_program());
        let own = alone.own.expect("agent's own settings");
        assert_eq!(own.keys, BTreeMap::from([("a".to_string(), json!(true))]));
        let account = engine
            .resolve(&place, &stage, Some("other"))
            .expect("resolved");
        assert_eq!(account.settings.program.as_deref(), Some("other"));
        assert_eq!(
            account.from["program"],
            Given {
                layer: Layer::Account,
                value: "other".into(),
                file: None,
            }
        );
        assert!(!account.names_program());
        let own = account.own.as_ref().expect("other's own settings");
        assert_eq!(own.keys, BTreeMap::from([("o".to_string(), json!(true))]));
        assert!(account.from.contains_key("programs.other.settings.o"));
        assert!(!account.from.contains_key("programs.agent.settings.a"));
        other.take_own();
        other.stop_short_words("other's way");
        let resolution = engine
            .resolution(&place, &stage, Some("other"))
            .unwrap_or_else(|words| panic!("{words}"));
        assert_eq!(resolution.agent.stop_short(&identity()), "other's way");
        assert_eq!(resolution.resolved, account);
    }

    #[test]
    fn the_programs_that_can_run_a_stage() {
        // `mode = { agent = "edits" }` names no entry for `other`, so
        // `agent` alone can run the stage, and a program no profile binds
        // is refused naming it; each program is read once, in order (105,
        // 145, 169, 173).
        let (engine, agent, _) = engine();
        agent.map_mode(Mode::Edits, "agent-edits");
        let place = crate::tests::record(Binding::default());
        let stage = stage(Settings {
            mode: Some(flume_seams::ModeSetting::Programs(BTreeMap::from([(
                "agent".to_string(),
                "edits".to_string(),
            )]))),
            ..Settings::default()
        });
        let runs = engine.runs_on(&place, &stage, ["other", "agent", "nothing-here", "other"]);
        let programs: Vec<&str> = runs.iter().map(|(program, _)| program.as_str()).collect();
        assert_eq!(programs, ["other", "agent", "nothing-here"]);
        let able = runs[1].1.as_ref().unwrap_or_else(|words| panic!("{words}"));
        assert_eq!(able.mode.as_deref(), Some("agent-edits"));
        assert_eq!(able.resolved.from["program"].layer, Layer::Account);
        let refused = refused(&runs);
        assert_eq!(
            refused,
            [
                (
                    "other".to_string(),
                    "the stage `review`'s mode `{ agent = \"edits\" }`, from the layer `kind`, names no entry for `other`".to_string()
                ),
                (
                    "nothing-here".to_string(),
                    "no agent profile runs the program nothing-here its account names for the stage review".to_string()
                ),
            ]
        );
        let words = cannot_run("review", "work", &refused);
        assert!(words.starts_with("the stage `review` names no program, and no account of the pool `work` can run it: `other` cannot, as the stage `review`'s mode"), "{words}");
        assert!(words.contains("names no entry for `other`; `nothing-here` cannot, as no agent profile runs the program nothing-here"), "{words}");
        assert!(words.ends_with("(145, 173)"), "{words}");
    }

    #[test]
    fn a_layers_program_stands_over_the_accounts() {
        // The kind names `agent`, so the account's `other` is ignored
        // and the own settings are `agent`'s (32 item 8, 173).
        let (engine, _, _) = engine();
        let place = place();
        let stage = stage(Settings {
            program: Some("agent".into()),
            ..Settings::default()
        });
        let resolved = engine
            .resolve(&place, &stage, Some("other"))
            .expect("resolved");
        assert_eq!(resolved.settings.program.as_deref(), Some("agent"));
        assert_eq!(resolved.from["program"].layer, Layer::Kind);
        assert!(resolved.names_program());
        let own = resolved.own.expect("agent's own settings");
        assert_eq!(own.keys, BTreeMap::from([("a".to_string(), json!(true))]));
    }

    fn login(name: &str) -> Option<String> {
        match name {
            "HOME" => Some("/home/user".into()),
            "ACCOUNT" => Some("work".into()),
            _ => None,
        }
    }

    #[test]
    fn the_home_and_the_variables_are_read() {
        assert_eq!(
            expand("~/.agent-work", login),
            Ok("/home/user/.agent-work".into())
        );
        assert_eq!(expand("~", login), Ok("/home/user".into()));
        assert_eq!(expand("a~b", login), Ok("a~b".into()));
        assert_eq!(expand("~other", login), Ok("~other".into()));
        assert_eq!(expand("$ACCOUNT-x", login), Ok("work-x".into()));
        assert_eq!(expand("${ACCOUNT}x", login), Ok("workx".into()));
        assert_eq!(expand("~/$ACCOUNT", login), Ok("/home/user/work".into()));
    }

    #[test]
    fn a_dollar_naming_nothing_is_kept() {
        assert_eq!(expand("cost $5", login), Ok("cost $5".into()));
        assert_eq!(expand("a$", login), Ok("a$".into()));
        assert_eq!(expand("${unclosed", login), Ok("${unclosed".into()));
    }

    #[test]
    fn a_variable_the_login_environment_lacks_is_named() {
        assert_eq!(expand("$NOT_SET", login), Err("NOT_SET".into()));
        assert_eq!(expand("${NOT_SET}/x", login), Err("NOT_SET".into()));
        assert_eq!(expand("~", |_| None), Err("HOME".into()));
    }
}
