//! A stage's start (27 to 33): its settings resolved from their eight
//! layers, or the stage refused naming the setting (32), a fresh identity
//! and secret, its prompt filled or the stage refused naming the part,
//! and one agent opened through the agent seam's command in its stream's
//! terminal, never while another of the place's is live, and only under
//! the host's bound (70, 190). Each file the command lists is written
//! into the place's folder before the terminal opens the agent, and a
//! start whose command lists one outside it is not opened (196).

use crate::accounts::{Choice, Programs};
use crate::live::LiveRead;
use crate::open_sessions::told;
use crate::prompt::{
    after_roles, asks, group_text, may_name, notes_path, render, way_to_stop_short,
};
use crate::room::Admit;
use crate::settings::{cannot_run, kind_started, program_name, Resolution, RunsOn};
use crate::start_record::start_record;
use crate::{repository_id, Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// A start written as live, its agent still to open.
pub(crate) struct Claim {
    identity: Identity,
    secret: Secret,
    text: String,
    place: PlaceRef,
    /// The settings the agent starts with.
    settings: Settings,
    /// The environment its account gives, each value read against the
    /// login environment (29, 144).
    env: BTreeMap<String, String>,
    agent: Arc<dyn Agent>,
    /// The session name the agent carries: a role's run's address (132).
    session: Option<String>,
    /// The terminal it opens in, its stream's as the host's
    /// configuration stands now (190).
    terminal: Arc<dyn Terminal>,
    /// The terminal's session the agent opens in, none where it opens
    /// beside the pane its settings name (107, 143).
    opens_in: Option<SessionHeld>,
    /// The account the start runs under (144, 145).
    account: Option<AccountHeld>,
    /// The value its agent profile passes for its mode, none where no
    /// layer gives one (162, 169).
    mode: Option<String>,
    /// The session it resumes, carrying the conversation of the stage's
    /// start whose agent stopped at its account's usage limit, or whose
    /// conversation was reported lost (49, 146, 179).
    resume: Option<String>,
    /// Its program's own settings as the layers resolved them, which its
    /// profile takes, none where they give none (195, 196).
    own: BTreeMap<String, serde_json::Value>,
}

impl Claim {
    /// The account the start runs under, none where it has no pool.
    pub(crate) fn account(&self) -> Option<&AccountHeld> {
        self.account.as_ref()
    }
}

/// What a start came to.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Start {
    Opened(Identity),
    /// The terminal read the start held at a question its program asks
    /// before it starts, which flume does not answer: the start waits on
    /// a person with the question's words (34, 35).
    Held(Identity),
    /// The terminal read the start's program not come up: never begun at
    /// once (35).
    NeverBegun(Identity),
    /// The host has no room, or stages waiting rank ahead of it (70).
    WaitingForRoom,
    /// The prompt asks for a part with nothing to fill it (27).
    Refused(PromptPart),
    /// Another agent of the place's is live (30).
    AgentLive,
    /// The terminal refused to open the agent.
    NotOpened(String),
    /// The stage's settings name a program no agent profile answers for,
    /// or a variable the host user's login environment lacks (27, 32,
    /// 105).
    Unresolved(String),
    NoPlace,
}

/// Why flume starts a stage afresh on its own, with words of its own in
/// its prompt where it asks for `{{words}}`, counted toward no bound of
/// the kind's and waiting on no caller (112).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StartReason {
    /// The stage's earlier agent stopped at its account's usage limit
    /// (146).
    Spent,
    /// The stage's earlier agent began without the settings its start
    /// recorded (161).
    Unsettled,
    /// The stage's earlier agent stopped at its account's usage limit, and
    /// this start resumes its conversation: its prompt is these words and
    /// the way to stop short alone (28, 146, 179).
    Carried,
    /// The stage, a conversation, was reported lost and its caller
    /// retried it, and this start resumes its conversation: its prompt is
    /// these words, the retry's words and the way to stop short (28, 49,
    /// 179).
    Resumed,
    /// The stage, a conversation, was reported lost and its caller
    /// retried it, and its conversation could not be carried: these words
    /// stand before the retry's in the stage's prompt (27, 49, 179).
    Lost,
    /// The stage's agent hibernated and a resume could not carry its
    /// conversation: these words stand before the resume's in the stage's
    /// prompt (27, 213, S127).
    Hibernated,
    /// The stage's agent hibernated and its caller resumed it with no
    /// words: these are the resume's words, before the way to stop short
    /// (28, 213).
    Woken,
}

impl StartReason {
    /// flume's words for the fresh start.
    pub(crate) fn words(self) -> String {
        match self {
            StartReason::Spent => "The stage's earlier agent stopped at its account's usage limit, and this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Unsettled => "The stage's earlier agent began without the settings its start recorded and was ended, and this agent starts the stage afresh. Anything it committed is on the place's branch and anything it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Carried => "Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.".into(),
            StartReason::Resumed => "Your session was lost and goes on here, with its conversation as it stood. Anything you had running then has stopped: read the place again before you carry on.".into(),
            StartReason::Lost => "The stage's earlier agent was lost, and its conversation could not be carried, so this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Hibernated => "The stage's earlier agent hibernated, and its conversation could not be carried, so this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Woken => "Your session hibernated and resumes here, with its conversation as it stood. Anything you had running in it then has stopped: read the place again before you carry on.".into(),
        }
    }
}

impl StartReason {
    /// flume's words for the fresh start after `start`, then the words
    /// `start` was given and each answer it took in its session, so none
    /// is lost with its agent (42, 146, 161).
    pub(crate) fn after(self, held: &HeldState, start: &StartHeld) -> String {
        then_words(self.words(), told(held, start).as_deref())
    }
}

/// `first`, then `words` where there are any, joined by a blank line (27,
/// 28, 49).
fn then_words(first: String, words: Option<&str>) -> String {
    match words {
        Some(words) => format!("{first}\n\n{words}"),
        None => first,
    }
}

/// Why a start after a spent agent carries no conversation where its
/// agent profile offers no carry (171, 179).
pub(crate) const CARRIES_NONE: &str = "its agent profile carries no conversation";

/// Whether the start after `after`, the stage's start whose agent stopped
/// at its account's usage limit or whose conversation was reported lost,
/// carries that agent's conversation, as `carry`, its profile's answer
/// for the account chosen, says; or why not: the profile's words, or the
/// wait's where it may not carry, or that no account was chosen to carry
/// it to (49, 146, 179).
fn conversation(after: &CarryFrom, carry: Option<&Carry>) -> Conversation {
    let why = match carry {
        Some(Carry::Yes { .. }) => None,
        Some(Carry::No { why }) => Some(why.clone()),
        Some(Carry::NotOffered) => Some(CARRIES_NONE.to_string()),
        None => Some(
            after
                .why
                .clone()
                .filter(|_| !after.carry)
                .unwrap_or_else(|| "its start runs under no account".to_string()),
        ),
    };
    Conversation {
        from: after.start.start,
        cause: after.cause,
        carried: why.is_none(),
        why,
    }
}

/// A stage's settings as a start reads them before its account is
/// chosen: resolved as they stand, a layer above flume's default naming
/// its program or its stream having no pool; or, where neither holds, for
/// each program of its pool's accounts (32 item 8, 173; design 6).
enum Resolving {
    Named(Box<Resolution>),
    Pool(RunsOn),
}

/// Why a start does not carry the conversation of a start that ran
/// `earlier`: the account chosen, `account`, runs `program`, and no
/// profile reads another program's session as its own (146, 173, 179,
/// 213; design 6).
fn another_program(account: &str, program: &str, earlier: &str) -> String {
    format!(
        "the account chosen, {account}, runs {program}, another program than its conversation's {earlier}"
    )
}

/// The count a start of `stage` carries, its own point `own` where it
/// begins one: a stage with no count, never started, delivered since, sent
/// back to by a fix round or held before stages kept their count, begins
/// its count at this start, which carries none; a repetition given a group
/// carries the count of the stage's last start where that start was given
/// the same group, by number and title, and counts from itself otherwise;
/// every other start, a repetition given no group among them, carries the
/// stage's count. Nothing asks why the stage starts (25, 140).
fn carried_count(
    stage: &mut StageHeld,
    group: Option<&GroupHeld>,
    own: Counted,
) -> Option<Counted> {
    let Some(counts) = stage.counts.clone() else {
        stage.counts = Some(own);
        return None;
    };
    match group {
        Some(group) => stage
            .start
            .as_ref()
            .filter(|last| {
                last.group
                    .as_ref()
                    .is_some_and(|given| same_group(given, group))
            })
            .map(StartHeld::counted),
        None => Some(counts),
    }
}

/// Whether two repetitions were given the same group, by its number and
/// its title, as a group is read at the head (140).
fn same_group(one: &GroupHeld, other: &GroupHeld) -> bool {
    one.number == other.number && one.title == other.title
}

/// What readiness came to, as a prompt's `{{unready}}` gives it (12).
fn unready(readiness: &Readiness) -> String {
    match readiness.failed() {
        Some(step) => format!(
            "The place's readiness failed at {}: {}",
            step.step, step.words
        ),
        None if readiness.steps.is_empty() => "The repository declares no readiness.".into(),
        None => "The place's readiness passed.".into(),
    }
}

impl Engine {
    /// A report's path: under the place's own folder for `{{state}}`, or
    /// inside the place (24, 123).
    pub(crate) fn report_path(&self, place: &PlaceRecord, path: &str) -> String {
        match path.strip_prefix("{{state}}") {
            Some(rest) => format!("{}{rest}", self.store.folder(&place.id)),
            None => format!("{}/{path}", place.location),
        }
    }

    /// A repository's id: of its common git directory, or of its
    /// location where git cannot say (33, 147, C.4).
    pub(crate) fn repository_id_of(&self, repository: &str) -> String {
        let common = self
            .history
            .common_dir(repository)
            .unwrap_or_else(|| repository.to_string());
        repository_id(&common)
    }

    /// The identity of the stage's next start, its repository named by
    /// its id (33, C.4).
    pub(crate) fn next_identity(&self, held: &HeldState, index: usize) -> Identity {
        Identity {
            repository: self.repository_id_of(&held.place.repository),
            place: held.place.short(),
            stage: held.place.kind.stages[index].name.clone(),
            start: held.stages[index].starts + 1,
            resume: 0,
        }
    }

    /// The roles a stop-short of the place may name, read from the host's
    /// sources as they stand now: every role they declare with words of
    /// what it answers, but the place's own where it is a role's run, its
    /// stream's first (28, 41, 132, 193).
    pub(crate) fn may_name(&self, held: &HeldState) -> Vec<AnsweringRole> {
        may_name(
            self.host.answering(),
            &held.place.stream.name,
            held.role.as_deref(),
        )
    }

    /// The kinds a role's place's entry names its projects' kinds by,
    /// each with its words, as the host's sources' roles give them now,
    /// as `may_name` reads the roles; none for a place of no role's, or
    /// whose address no source gives a run (28, 193).
    pub(crate) fn kinds_named(&self, held: &HeldState) -> BTreeMap<String, String> {
        let Some(address) = held.role.as_deref() else {
            return BTreeMap::new();
        };
        match self.host.roles().declared(address) {
            Ok(RoleEntry::Run(run)) => run.kinds,
            _ => BTreeMap::new(),
        }
    }

    /// The stage's prompt filled for `identity`'s start, or the first
    /// part it asks for with nothing to fill it (27). The way to stop
    /// short is in every prompt (28), as `agent`, the profile of the
    /// stage's program, gives it, saying it is the agent's one way to ask;
    /// and a first stage started in a place that could not be made ready
    /// carries the step and its words (12). A repeating stage fills
    /// `{{group}}` with `group`, the start's, which none fills where the
    /// change could not be read, and `{{notes}}` with its notes' path; a
    /// stage that does not repeat fills neither (27, 141). The roles in
    /// `listed`, which a stop-short may name, come last, after every part
    /// the stage's repetitions share, and after them the kinds `kinds` a
    /// role's entry names (28, 41, 193; core.31). A role's run's
    /// start's `batch`, every delivery pending at its address, comes after
    /// the kind's template and before the way to stop short (214; design
    /// 9).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn fill(
        &self,
        held: &HeldState,
        index: usize,
        identity: &Identity,
        words: Option<&str>,
        agent: Option<&dyn Agent>,
        group: Option<Option<&GroupHeld>>,
        listed: &[AnsweringRole],
        kinds: &BTreeMap<String, String>,
        batch: Option<&str>,
    ) -> Result<String, PromptPart> {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let change = self.bound(place).ok().and_then(|bound| bound.change);
        let stop_short = agent.map(|agent| way_to_stop_short(agent, identity));
        let fill = |part: PromptPart| -> Option<String> {
            match part {
                PromptPart::Brief => place.brief.clone(),
                PromptPart::Name => Some(place.name.clone()),
                PromptPart::Kind => Some(place.kind.name.clone()),
                PromptPart::Stage => Some(stage.name.clone()),
                PromptPart::Branch => Some(place.branch.clone()),
                PromptPart::Base => Some(place.base.clone()),
                PromptPart::Place => Some(place.location.clone()),
                PromptPart::Change => change.as_ref().map(|change| change.path(&place.name)),
                PromptPart::Report => match stage.deliverables.first() {
                    Some(Deliverable::Report { path }) => Some(self.report_path(place, path)),
                    Some(Deliverable::CommittedReport { path }) => Some(path.clone()),
                    _ => None,
                },
                PromptPart::Words => words.map(str::to_string),
                PromptPart::Unready => Some(unready(&held.readiness)),
                PromptPart::Needs => stop_short.clone(),
                PromptPart::Made => (!stage.makes.is_empty()).then(|| self.made_path(&place.id)),
                PromptPart::Group => group.filter(|_| stage.repeats).map(group_text),
                PromptPart::Notes => stage
                    .repeats
                    .then(|| notes_path(&self.store.folder(&place.id), &stage.name)),
            }
        };
        let mut text = render(&stage.prompt, fill)?;
        if let Some(batch) = batch {
            text.push_str("\n\n");
            text.push_str(batch);
        }
        if !asks(&stage.prompt, PromptPart::Needs) {
            text.push_str("\n\n");
            text.push_str(&stop_short.clone().ok_or(PromptPart::Needs)?);
        }
        if index == 0
            && held.readiness.failed().is_some()
            && !asks(&stage.prompt, PromptPart::Unready)
        {
            text.push_str("\n\n");
            text.push_str(&unready(&held.readiness));
        }
        if let Some(after) =
            agent.and_then(|agent| after_roles(listed, kinds, &place.stream.name, agent))
        {
            text.push_str("\n\n");
            text.push_str(&after);
        }
        Ok(text)
    }

    /// The task group a start of the stage at `index` takes at `head`,
    /// for a repeating stage: the first of the change's groups holding a
    /// task unticked there, or none where every task is ticked or the
    /// change holds none (140). None for a stage that does not repeat,
    /// and for a change that cannot be read at `head`, which gives a
    /// repetition nothing to work from.
    pub(crate) fn group_at(
        &self,
        held: &HeldState,
        index: usize,
        head: &Revision,
    ) -> Option<Option<GroupHeld>> {
        let place = &held.place;
        if !place.kind.stages[index].repeats {
            return None;
        }
        let change = self.bound(place).ok()?.change?;
        let ChangeRead::Read { facts } = change.read_change(&place.repository, head, &place.name)
        else {
            return None;
        };
        Some(facts.first_unticked_group().map(|at| {
            let group = &facts.groups[at];
            GroupHeld {
                index: at,
                number: group.number.clone(),
                title: group.title.clone(),
                tasks: facts.tasks_of(at).map(|task| task.text.clone()).collect(),
            }
        }))
    }

    /// Starts the stage at `index` with one fresh agent and `words` in
    /// its prompt, admitted under the host's room lock: the start is
    /// written to held state as live while there is room and no stage
    /// waiting ranks ahead of it, so two acts at once open one agent and
    /// two settles at once never start past the bound (30, 70, 75), and
    /// the agent is opened once the lock is released; otherwise the stage
    /// waits for room with its words.
    pub(crate) fn start_stage(&self, id: &PlaceId, index: usize, words: Option<String>) -> Start {
        let claimed = {
            let _room = self.host.room();
            let queue = self.queue();
            self.claim(id, index, words, Admit::Queue(&queue))
        };
        match claimed {
            Ok(claim) => self.open(id, claim),
            Err(start) => start,
        }
    }

    /// Claims the stage's start in held state: refused naming the part
    /// its prompt cannot fill, left waiting for room where `admit` says
    /// so, its wait to carry a conversation kept, or written as live;
    /// called under the room lock. A start after its stage's agent stopped
    /// at its account's usage limit, or after its caller's retry of a
    /// conversation reported lost, resumes that agent's conversation where
    /// the stage's profile answers that the account chosen reaches it, its
    /// prompt flume's words, a lost one's retry's words, and the way to
    /// stop short, and its record naming the session; and starts afresh
    /// otherwise, saying why. Such a start, and a lost conversation's
    /// fresh start, waits for room while the profile's live reads a
    /// process running the conversation, or cannot read whether one does,
    /// or while the agent the terminal brought back in the lost start's
    /// pane stands (28, 30, 49, 146, 179, 207). A resume of a hibernated
    /// start is read as a lost conversation's retry is: the account its
    /// agent ran under first while it is usable, the profile asked, the
    /// wait held while a process runs the conversation; where it carries,
    /// the start is that start's resume, its identity the start's number
    /// with the next resume's, the stage's starts kept, and what the start
    /// was given and took over taken over with the resume appended, its
    /// prompt the resume's words then the way to stop short; where it does
    /// not, the next start, the stage's prompt with flume's words before
    /// the resume's (28, 30, 49, 70, 145, 179, 207, 213; core.10,
    /// core.32).
    pub(crate) fn claim(
        &self,
        id: &PlaceId,
        index: usize,
        words: Option<String>,
        admit: Admit<'_>,
    ) -> Result<Claim, Start> {
        let Some(read) = self.store.read(id) else {
            return Err(Start::NoPlace);
        };
        let place = &read.state.place;
        let queue = admit.queue();
        // A stream whose labels are at fault, or whose multiplexer names a
        // terminal profile the resolver does not carry, starts no stage:
        // its words refuse the start before its settings are resolved (4,
        // 181, 182, 190). Where the settings name no pane and no session,
        // the stream's word for the stage's kind or the stage, read once
        // they are resolved, refuses it where that word is at fault (4,
        // 107, 199). A stage whose settings name no program above flume's
        // default, on a pool, is read for each program of its pool's
        // accounts, the session it opens in read from its settings before
        // the program, which name none; every other is resolved as it
        // stands (32 item 8, 144, 145, 173, 199; design 6).
        let stage = &place.kind.stages[index];
        let pool_room = queue
            .pool_of(&place.stream.name)
            .and_then(|pool| queue.pools.get(pool));
        let reading = match (
            queue.stream(&place.stream.name).fault,
            self.stream_terminal(&place.stream.name),
        ) {
            (Some(fault), _) | (None, Err(fault)) => Err(fault),
            (None, Ok(terminal)) => self.resolve(place, stage, None).and_then(|resolved| {
                let opens_in = queue.session_of(
                    &place.stream.name,
                    &place.kind.name,
                    &stage.name,
                    &resolved.settings,
                )?;
                let reading = match pool_room {
                    Some(room) if !resolved.names_program() => Resolving::Pool(
                        self.runs_on(
                            place,
                            stage,
                            room.accounts
                                .iter()
                                .map(|room| room.account.program.as_str()),
                        ),
                    ),
                    _ => Resolving::Named(Box::new(self.checked(resolved, stage)?)),
                };
                Ok((reading, terminal, opens_in))
            }),
        };
        let now = self.host.now();
        let affinity = queue
            .pool_of(&place.stream.name)
            .and_then(|_| self.affinity(&place.intent))
            .map(|(_, affinity)| affinity);
        let programs = match &reading {
            Ok((Resolving::Pool(runs), _, _)) => Programs::of_pool(runs),
            Ok((Resolving::Named(resolution), _, _)) => {
                Programs::named(resolution.resolved.settings.program.as_deref())
            }
            Err(_) => Programs::default(),
        };
        let stage_name = &stage.name;
        // A stage waiting after its agent stopped at its account's usage
        // limit, or after its caller's retry of a conversation reported
        // lost: its profile is asked, under the room lock once an account
        // is chosen, whether that account reaches the agent's conversation
        // (49, 145, 146, 179).
        let after = read.state.stages[index]
            .room
            .as_ref()
            .and_then(|room| room.carry.clone());
        let lost = after
            .as_ref()
            .is_some_and(|after| after.cause == CarryCause::Lost);
        // A resume of a hibernated start, read as a lost conversation's
        // retry is (213; core.10).
        let hibernated = after
            .as_ref()
            .is_some_and(|after| after.cause == CarryCause::Hibernated);
        // The lost or hibernated start, while it is the stage's last: the
        // account it ran under holds its conversation (145, 179, 213).
        let lost_start = read.state.stages[index]
            .start
            .as_ref()
            .filter(|start| {
                (lost || hibernated)
                    && after
                        .as_ref()
                        .is_some_and(|after| after.start == start.identity)
            })
            .cloned();
        let (chosen, intent_moved) = match &reading {
            Ok(_) => self.choose_account(
                place,
                stage_name,
                &programs,
                affinity.as_ref(),
                lost_start.as_ref().and_then(|start| start.account.as_ref()),
                queue,
                now,
            ),
            Err(_) => (Choice::NoPool, None),
        };
        // The resolution of the program the account chosen runs, or, while
        // the stage waits for an account, of the first program of its
        // pool's that can run it, so its prompt is filled as a start would
        // fill it (173; design 6).
        let resolution = reading.and_then(|(reading, terminal, opens_in)| {
            let resolution = match reading {
                Resolving::Named(resolution) => Ok(*resolution),
                Resolving::Pool(runs) => {
                    let chosen = match &chosen {
                        Choice::Chose { account, .. } => Some(account.program.as_str()),
                        _ => None,
                    };
                    let pool = queue.pool_of(&place.stream.name).unwrap_or_default();
                    let refused = cannot_run(stage_name, pool, &programs.refused);
                    runs.into_iter()
                        .filter(|(program, _)| chosen.is_none_or(|chosen| chosen == program))
                        .find_map(|(_, resolution)| resolution.ok())
                        .ok_or(refused)
                }
            };
            resolution.map(|resolution| (resolution, terminal, opens_in))
        });
        let program = resolution
            .as_ref()
            .ok()
            .and_then(|(resolution, _, _)| resolution.resolved.settings.program.clone());
        let env = match &chosen {
            Choice::Chose { account, .. } => self.account_env(account, stage_name),
            _ => Ok(BTreeMap::new()),
        };
        let resolution = match (&chosen, env.as_ref()) {
            (Choice::Undeclared(reason) | Choice::NoAccount(reason), _) | (_, Err(reason)) => {
                Err(reason.clone())
            }
            _ => resolution,
        };
        let env = env.unwrap_or_default();
        // The terminal the start records asked whether it starts the
        // agent's program's kind under the chosen account's environment,
        // so it reads the home the agent will hold; a refusal leaves the
        // stage refused in its words until a retry (27, 32, C.5).
        let resolution = resolution.and_then(|(resolution, terminal, opens_in)| {
            kind_started(terminal.0.as_ref(), &resolution, stage_name, &env)?;
            Ok((resolution, terminal, opens_in))
        });
        // The program of the start whose conversation the start would
        // carry: its profile alone reads that conversation, so a start on
        // an account of another program carries it not, asking no profile
        // (146, 173, 179, 213; design 6).
        let earlier = after.as_ref().and_then(|after| {
            read.state
                .index_of(&after.start)
                .and_then(|at| read.state.stages[at].start.as_ref())
                .filter(|start| start.identity == after.start)
                .map(|start| start.program.clone())
        });
        let elsewhere = match (&chosen, &earlier, &program) {
            (Choice::Chose { account, .. }, Some(earlier), Some(program))
                if program_name(earlier) != program_name(program) =>
            {
                Some(another_program(&account.name, program, earlier))
            }
            _ => None,
        };
        // A lost conversation's start under no pool asks the profile of
        // the login environment's, an empty one; a spent agent always ran
        // under an account (146, 179).
        let carry = match (&after, &chosen, &resolution) {
            (Some(after), Choice::Chose { .. }, Ok(_)) if after.carry && elsewhere.is_some() => {
                elsewhere.clone().map(|why| Carry::No { why })
            }
            (Some(after), Choice::Chose { .. }, Ok((resolution, _, _))) if after.carry => Some(
                resolution
                    .agent
                    .carry(&self.store.folder(id), &after.start, &env),
            ),
            (Some(after), Choice::NoPool, Ok((resolution, _, _)))
                if after.carry && (lost || hibernated) =>
            {
                Some(
                    resolution
                        .agent
                        .carry(&self.store.folder(id), &after.start, &BTreeMap::new()),
                )
            }
            _ => None,
        };
        let conversation = after
            .as_ref()
            .map(|after| conversation(after, carry.as_ref()));
        let resume = match carry {
            Some(Carry::Yes { session }) => Some(session),
            _ => None,
        };
        // A start that carries a conversation, and a lost conversation's
        // fresh start, opens only while no process runs that conversation:
        // live is asked of every home the host declares for the program,
        // and the stage waits while one runs it or live cannot read whether
        // one does, flume adopting, ending and sending nothing to it (30,
        // 49, 207; core.10).
        let ask = matches!(
            (&after, &chosen, &resolution),
            (Some(_), Choice::Chose { .. } | Choice::NoPool, Ok(_))
        ) && (resume.is_some() || lost || hibernated);
        let (session, standing) = match &after {
            Some(after) => self.session_holds(
                id,
                after,
                lost_start.as_ref(),
                earlier
                    .as_deref()
                    .or(program.as_deref())
                    .unwrap_or_default(),
                queue,
                ask,
            ),
            None => (None, None),
        };
        let lost_identity = lost_start.as_ref().map(|start| start.identity.clone());
        let fresh = if lost {
            Some(then_words(StartReason::Lost.words(), words.as_deref()))
        } else if hibernated {
            Some(then_words(
                StartReason::Hibernated.words(),
                words.as_deref(),
            ))
        } else {
            words.clone()
        };
        // The hibernated start a carried resume resumes: its identity's
        // next resume, and what it was given and took over (213).
        let resumes = lost_start
            .as_ref()
            .filter(|_| hibernated && resume.is_some())
            .cloned();
        let resumed_by = after
            .as_ref()
            .and_then(|after| after.resumed_by)
            .unwrap_or(ResumedBy::Caller);
        let made_from = self.store.made(id).len();
        let label = self.label_of(place);
        // The roles a stop-short may name, read once for the start before
        // its claim, so a claim written again reads no file again (28, 41,
        // 193).
        let listed = self.may_name(&read.state);
        let kinds = self.kinds_named(&read.state);
        // A role's run's start whose prompt is filled carries its
        // address's batch, claimed under the start's identity before the
        // start is written, and let go of where the start is not made
        // with it (214; design 9).
        // A resume by a delivery claims the batch under the resume's own
        // identity, its prompt the batch's words (213, 214; design 7).
        let batched = match (&read.state.role, &resume, &resolution, &resumes) {
            (Some(address), None, Ok(_), _) => {
                self.claim_for_start(address, &self.next_identity(&read.state, index))
            }
            (Some(address), Some(_), Ok(_), Some(from)) if resumed_by == ResumedBy::Delivery => {
                self.claim_for_start(
                    address,
                    &Identity {
                        resume: from.identity.resume + 1,
                        ..from.identity.clone()
                    },
                )
            }
            _ => None,
        };
        let claimed = self.update(id, |held| {
            if let Admit::Waiting(_) = admit {
                if held.phase != Phase::Live
                    || held.stages[index].state != StageState::WaitingForRoom
                {
                    return Step::Keep(Err(Start::NoPlace));
                }
            }
            if let Some(lost) = lost_identity.as_ref().filter(|_| standing == Some(false)) {
                if let Some(start) = held
                    .index_of(lost)
                    .and_then(|at| held.stages[at].start.as_mut())
                {
                    start.live = false;
                }
            }
            match (&session, admit) {
                (Some(session), Admit::Waiting(_)) => {
                    let room = held.stages[index]
                        .room
                        .get_or_insert_with(RoomWait::default);
                    session.hold(room);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
                (Some(session), Admit::Queue(queue)) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    let mut room = RoomWait {
                        order: queue.next_order(),
                        words: words.clone(),
                        carry: after.clone(),
                        ..RoomWait::default()
                    };
                    session.hold(&mut room);
                    stage.room = Some(room);
                    held.current = Some(index);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
                (None, _) => {
                    // Read no longer running: the wait holds no process.
                    if let Some(room) = held.stages[index].room.as_mut() {
                        LiveRead::default().hold(room);
                    }
                }
            }
            if held
                .stages
                .iter()
                .any(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            {
                return Step::Write(Err(Start::AgentLive));
            }
            let (resolution, (terminal, named), opens_in) = match &resolution {
                Ok((resolution, terminal, opens_in)) => (resolution, terminal, opens_in),
                Err(reason) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Refused;
                    stage.refused_part = None;
                    stage.refused_setting = Some(reason.clone());
                    stage.room = None;
                    held.current = Some(index);
                    return Step::Write(Err(Start::Unresolved(reason.clone())));
                }
            };
            let head = self
                .history
                .head(&held.place.repository, &held.place.branch)
                .unwrap_or_else(|| held.place.base_revision.clone());
            let group = self.group_at(held, index, &head);
            let identity = match &resumes {
                Some(from) => Identity {
                    resume: from.identity.resume + 1,
                    ..from.identity.clone()
                },
                None => self.next_identity(held, index),
            };
            let agent = resolution.agent.as_ref();
            // A start that carries its conversation is sent flume's words,
            // a lost conversation's retry's words after them, and the way
            // to stop short, nothing of the stage's prompt filled; a lost
            // conversation's fresh start is given flume's words before the
            // retry's as its words (27, 28, 49, 146). A lost carried
            // start keeps the retry's words, so a fresh start after it
            // that did not begin is given them again (S79).
            let stream = &held.place.stream.name;
            let way = |first: String| {
                let way = format!("{first}\n\n{}", way_to_stop_short(agent, &identity));
                match after_roles(&listed, &kinds, stream, agent) {
                    Some(after) => format!("{way}\n\n{after}"),
                    None => way,
                }
            };
            let (text, started_with) = match &resume {
                Some(_) if lost => {
                    let resumed = then_words(StartReason::Resumed.words(), words.as_deref());
                    (way(resumed), words.clone())
                }
                // A resume of a hibernated start is sent the resume's words,
                // a resume by a delivery its batch's, else flume's for a
                // bare resume, and the way to stop short, and keeps the
                // words its start was given (28, 213, 214).
                Some(_) if hibernated => {
                    let resumed = batched
                        .as_ref()
                        .filter(|batched| batched.identity == identity)
                        .map(|batched| batched.words.clone())
                        .or_else(|| words.clone())
                        .unwrap_or_else(|| StartReason::Woken.words());
                    let given = resumes.as_ref().and_then(|from| from.words.clone());
                    (way(resumed), given)
                }
                Some(_) => {
                    let words = StartReason::Carried.words();
                    (way(words.clone()), Some(words))
                }
                None => match self.fill(
                    held,
                    index,
                    &identity,
                    fresh.as_deref(),
                    Some(agent),
                    group.as_ref().map(Option::as_ref),
                    &listed,
                    &kinds,
                    batched
                        .as_ref()
                        .filter(|batched| batched.identity == identity)
                        .map(|batched| batched.words.as_str()),
                ) {
                    Err(part) => {
                        let stage = &mut held.stages[index];
                        stage.state = StageState::Refused;
                        stage.refused_part = Some(part.name().into());
                        stage.refused_setting = None;
                        stage.room = None;
                        return Step::Write(Err(Start::Refused(part)));
                    }
                    Ok(text) => (text, fresh.clone()),
                },
            };
            if let (Admit::Waiting(_), Choice::Wait) = (admit, &chosen) {
                return Step::Write(Err(Start::WaitingForRoom));
            }
            if let Admit::Queue(queue) = admit {
                let program = resolution.resolved.settings.program.as_deref();
                if matches!(chosen, Choice::Wait)
                    || queue.must_wait(
                        id,
                        &held.place.stream.name,
                        label.as_ref().map(|label| label.name.as_str()),
                        program,
                        held.priority,
                    )
                {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    stage.room = Some(RoomWait {
                        order: queue.next_order(),
                        words: words.clone(),
                        carry: after.clone(),
                        ..RoomWait::default()
                    });
                    held.current = Some(index);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
            }
            let account = match &chosen {
                Choice::Chose { held, .. } => Some(held.clone()),
                _ => None,
            };
            let carried = conversation
                .as_ref()
                .map(|conversation| conversation.carried);
            if let Some(moved) =
                self.stage_moved(held, index, account.as_ref(), carried, queue, now)
            {
                held.moves.push(moved);
            }
            if let Some(moved) = &intent_moved {
                held.moves.push(moved.clone());
            }
            let mut from = resolution.resolved.from.clone();
            if let Choice::Chose { account, .. } = &chosen {
                for (name, value) in &account.env {
                    from.insert(
                        format!("env.{name}"),
                        Given {
                            layer: Layer::Account,
                            value: value.clone(),
                            file: None,
                        },
                    );
                }
            }
            let secret = self.host.secret();
            let program = resolution
                .resolved
                .settings
                .program
                .clone()
                .unwrap_or_default();
            let withheld = self
                .agent(&program)
                .map(|agent| agent.withheld())
                .unwrap_or_default();
            let start_record = StartRecord {
                mode: resolution.mode.clone(),
                session: resume.clone(),
                ..start_record(
                    &program,
                    account.as_ref(),
                    &env,
                    &withheld,
                    &identity,
                    &held.place.location,
                    &secret,
                )
            };
            // A resume takes over its start's number, its head and lines,
            // its group and its answers, and appends itself to its resumes
            // (48, 129, 140, 213).
            let (n, start_head, made_from, answered, group, resumed) = match &resumes {
                Some(from) => {
                    let mut resumed = from.resumes.clone();
                    // A bare resume's words are flume's own, and none are
                    // kept of it, so no fresh start after it repeats them
                    // (213).
                    resumed.push(ResumeHeld {
                        at: now,
                        by: resumed_by,
                        identity: identity.clone(),
                        words: words.clone().filter(|_| resumed_by != ResumedBy::Caller),
                        conversation: conversation.clone().unwrap_or(Conversation {
                            from: from.n,
                            cause: CarryCause::Hibernated,
                            carried: true,
                            why: None,
                        }),
                    });
                    (
                        from.n,
                        from.start_head.clone(),
                        from.made_from,
                        from.answered,
                        from.group.clone(),
                        resumed,
                    )
                }
                None => (
                    identity.start,
                    head.clone(),
                    made_from,
                    0,
                    group.clone().flatten(),
                    vec![],
                ),
            };
            let stage = &mut held.stages[index];
            // A resume goes on with its start, so its deliverable counts
            // from where that start's counted (25, 129, 213).
            let counts = match &resumes {
                Some(from) => Some(from.counted()),
                None => carried_count(
                    stage,
                    group.as_ref(),
                    Counted {
                        start: identity.start,
                        at: now,
                        head: head.clone(),
                        made_from,
                    },
                ),
            };
            stage.state = StageState::Starting;
            stage.starts = stage.starts.max(n);
            stage.refused_part = None;
            stage.refused_setting = None;
            stage.room = None;
            stage.end = None;
            stage.start = Some(StartHeld {
                n,
                started_at: now,
                start_head,
                made_from,
                words: started_with,
                identity: identity.clone(),
                secret: secret.clone(),
                program,
                settings: resolution.resolved.settings.clone(),
                // Names only: the values reach the agent's command and
                // nothing kept (32, 56).
                from: flume_seams::unvalued_from(&from),
                account: account.clone(),
                start_record,
                record: None,
                prompt: Prompt {
                    text: text.clone(),
                    ..Prompt::default()
                },
                live: true,
                begun_at: None,
                first_quiet: None,
                last_head: head,
                last_head_move: now,
                past_limit: false,
                printed: None,
                group,
                may_name: listed.iter().map(|role| role.address.clone()).collect(),
                opening: Some(self.host.command()),
                dialog: None,
                conversation: conversation.clone(),
                session: opens_in.clone(),
                label: label.clone(),
                terminal: Some(named.clone()),
                own: resolution.resolved.own.as_ref().map(|own| OwnHeld {
                    files: own.files.clone(),
                    passed: None,
                }),
                answered,
                hibernated: None,
                resumes: resumed,
                handing: None,
                messages: vec![],
                counts,
                span: None,
                asking_since: None,
                waiting: None,
            });
            if !held
                .terminals
                .iter()
                .any(|opened| opened.profile == named.profile)
            {
                held.terminals.push(named.clone());
            }
            held.current = Some(index);
            Step::Write(Ok(Claim {
                identity,
                secret,
                text,
                place: held.place.place_ref(),
                settings: resolution.command_settings(),
                env: env.clone(),
                agent: resolution.agent.clone(),
                session: held.role.clone(),
                terminal: terminal.clone(),
                opens_in: opens_in.clone(),
                account,
                mode: resolution.mode.clone(),
                resume: resume.clone(),
                own: resolution.own_keys(),
            }))
        });
        let claimed = claimed.unwrap_or(Err(Start::NoPlace));
        if let Some(batched) = &batched {
            if !matches!(&claimed, Ok(claim) if claim.identity == batched.identity) {
                self.release_start(&batched.identity);
            }
        }
        if let (Ok(claim), Some(place), Some(program)) = (&claimed, self.store.read(id), &program) {
            if let Some(account) = &claim.account {
                self.affinity_after(&place.state.place, program, account, intent_moved.as_ref());
            }
        }
        claimed
    }

    /// Opens the agent of a claimed start, out of the room lock, a
    /// standing role's checkouts first moved to their bases' tips (193,
    /// 213); an open the terminal refuses ends the start, freeing its room
    /// (70). Once
    /// the open answers, the start is no longer opening, and an agent that
    /// came up is sent its prompt at once where it said it started and
    /// waits, its receipt read by the settles that follow, so the command
    /// that opened it, a launch, a retry or a settle, moves it on with no
    /// wait on the agent (6, 37, 38, 46). A start held at its program's
    /// question stays starting, the question's words kept with it, and
    /// waits on a person (34, 35).
    pub(crate) fn open(&self, id: &PlaceId, claim: Claim) -> Start {
        let Claim {
            identity,
            secret,
            text,
            place,
            settings,
            env,
            agent,
            session,
            terminal,
            opens_in,
            account: _,
            mode,
            resume,
            own,
        } = claim;
        self.move_checkouts(id);
        let folder = self.store.folder(id);
        let mut command = agent.command(
            &folder,
            &settings,
            mode.as_deref(),
            &own,
            resume.as_deref(),
            &env,
            &place,
            &identity,
            &secret,
            &text,
        );
        self.give_telemetry(id, &identity, &mut command.env);
        if let Err(failure) = self.write_files(id, &folder, &command) {
            self.with_start(id, &identity, |start, _| start.opening = None);
            self.not_opened(
                id,
                &identity,
                format!("{identity} was not opened: {failure}"),
            );
            return Start::NotOpened(failure);
        }
        let passed = passed(&command);
        let at = match (&settings.beside, opens_in) {
            (Some(pane), _) => OpenIn::Beside { pane: pane.clone() },
            (
                None,
                Some(SessionHeld {
                    name: Some(name), ..
                }),
            ) => OpenIn::Session { name },
            (None, _) => OpenIn::Own,
        };
        let opened = terminal.open(
            &place,
            &folder,
            &command,
            &identity,
            &at,
            session.as_deref(),
        );
        self.with_start(id, &identity, |start, _| {
            start.opening = None;
            if let Some(own) = start.own.as_mut() {
                own.passed.clone_from(&passed);
            }
            if let Opened::Opened { carried, printed } = &opened {
                start.prompt.carried = *carried;
                start.printed.clone_from(printed);
            }
        });
        match opened {
            Opened::Opened { .. } => {
                self.send_prompt(id);
                Start::Opened(identity)
            }
            Opened::Held { screen } => {
                let since = self.host.now();
                self.with_start(id, &identity, |start, _| {
                    start.dialog = Some(Dialog {
                        words: screen.clone(),
                        since,
                    });
                });
                Start::Held(identity)
            }
            Opened::NotComeUp { screen } => {
                self.never_begun_with(id, &identity, screen);
                Start::NeverBegun(identity)
            }
            Opened::Refused { reason } => {
                let failure = format!("the terminal did not open {identity}: {reason}");
                self.not_opened(id, &identity, failure);
                Start::NotOpened(reason)
            }
        }
    }

    /// The start under `identity` ended never begun, its room freed, and
    /// `failure` kept with the place (35, 70).
    fn not_opened(&self, id: &PlaceId, identity: &Identity, failure: String) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                stage.state = StageState::NeverBegun;
                if let Some(start) = stage.start.as_mut() {
                    start.live = false;
                }
                held.failures.push(failure.clone());
            }
            Step::Write(())
        });
    }

    /// Writes each file `command` lists into the place's own `folder`
    /// through the store, written beside and renamed over, readable by
    /// the host's user alone; or, writing nothing, why not: a file
    /// outside the folder, named, or the store's words (196; core.16,
    /// core.17).
    fn write_files(
        &self,
        id: &PlaceId,
        folder: &str,
        command: &AgentCommand,
    ) -> Result<(), String> {
        if let Some(outside) = command.files.keys().find(|path| !under(folder, path)) {
            return Err(format!(
                "its command lists the file {outside}, which lies outside the place's folder {folder}"
            ));
        }
        for (path, text) in &command.files {
            self.store.write_file(id, path, text)?;
        }
        Ok(())
    }

    /// Changes the start under `identity`, if it is still the stage's,
    /// and writes `end` as its stage's end in the same write, so the end
    /// is recorded whole with what was read of it (43).
    pub(crate) fn end_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
        end: Option<StageEnd>,
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
                if let Some(end) = &end {
                    stage.end = Some(end.clone());
                }
            }
            Step::Write(())
        });
    }

    /// Changes the start under `identity`, if it is still the stage's.
    pub(crate) fn with_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
            }
            Step::Write(())
        });
    }
}

/// Whether `path` is a file under `folder`, by its components, none of
/// them `..` (196).
fn under(folder: &str, path: &str) -> bool {
    let path = std::path::Path::new(path);
    path.starts_with(folder)
        && path != std::path::Path::new(folder)
        && !path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
}

/// How `command` passed its program's own settings: each of its
/// arguments naming a file it lists, with the argument before it; none
/// where it names none (195, 196; design 12).
fn passed(command: &AgentCommand) -> Option<String> {
    let words: Vec<String> = command
        .args
        .iter()
        .enumerate()
        .filter(|(_, arg)| command.files.contains_key(*arg))
        .map(
            |(at, file)| match at.checked_sub(1).map(|before| &command.args[before]) {
                Some(flag) => format!("{flag} {file}"),
                None => file.clone(),
            },
        )
        .collect();
    (!words.is_empty()).then(|| words.join(" "))
}

/// Finding a start in held state by its identity.
pub(crate) trait Starts {
    fn index_of(&self, identity: &Identity) -> Option<usize>;
    fn live_start(&self) -> Option<(usize, &StartHeld)>;
}

impl Starts for HeldState {
    fn index_of(&self, identity: &Identity) -> Option<usize> {
        self.stages.iter().position(|stage| {
            stage
                .start
                .as_ref()
                .is_some_and(|start| &start.identity == identity)
        })
    }

    fn live_start(&self) -> Option<(usize, &StartHeld)> {
        self.stages.iter().enumerate().find_map(|(index, stage)| {
            stage
                .start
                .as_ref()
                .filter(|start| start.live)
                .map(|start| (index, start))
        })
    }
}
