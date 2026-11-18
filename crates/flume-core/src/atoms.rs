//! The names flume binds for a statechart kind: its one template, its one
//! rule, the evidence a kind may read, and the template's effect, its proof
//! and its end, each served or performed through one seam's operation or
//! git's history (175, 178, C.2). A kind naming any other is refused (4).

/// The stage template's text, `stage@1.toml`, read only through the kinds
/// seam (175, C.2; core.4).
pub const STAGE_TEMPLATE: &str = include_str!("stage@1.toml");

/// The one template flume binds, by the name a state's `machine` gives it
/// (175, 178).
pub const TEMPLATE: &str = "stage";

/// The stage template's one region, under each stage's path (175).
pub const TEMPLATE_REGION: &str = "run";

/// The stage template's one rule: a session for each file of the place's
/// branch at the settle's head its pattern matches (175, 178).
pub const EVERY_FILE: &str = "every-file";

/// When a region entered the state it holds, read from the held state
/// (178).
pub const REGION_ENTERED: &str = "region.entered";

/// The outcome of the place's newest submission, as flume confirmed it
/// (133, 178).
pub const PLACE_SUBMISSION: &str = "place.submission";

/// How many times flume submitted the place (178).
pub const PLACE_SUBMISSIONS: &str = "place.submissions";

/// The evidence a kind may read (178).
pub const KIND_EVIDENCE: [&str; 3] = [REGION_ENTERED, PLACE_SUBMISSION, PLACE_SUBMISSIONS];

/// The stage template's effect: its stage's sessions written into the
/// place's held state (103, 178).
pub const START_SESSIONS: &str = "start_sessions";

/// The evidence that proves [`START_SESSIONS`]: the stage's entry recorded
/// (178).
pub const STAGE_STARTED: &str = "stage.started";

/// Its stage's end, as the held state records its sessions' ends (178).
pub const STAGE_END: &str = "stage.end";

/// The stage template's evidence (178).
pub const TEMPLATE_EVIDENCE: [&str; 2] = [STAGE_STARTED, STAGE_END];

/// The ends `stage.end` reads, each a final state of the template (175).
pub const PASSED: &str = "passed";
/// A stage sent back (112, 175).
pub const SENT_BACK: &str = "sent-back";
/// A stage stopped (175).
pub const STOPPED: &str = "stopped";
