//! A start's environment compared with its record, as an agent's word
//! that it started compares it (161; core.10, core.30): each variable the
//! start's record names that the environment lacks, and each it holds
//! whose digest keyed by the start's secret differs from the record's, by
//! name alone, never a value. Claude Code's started word and flume's
//! launcher each compare through it, and each compares its program, and
//! Claude Code's its session, beside it.

use flume_core::variable_digest;
use flume_seams::{Secret, StartRecord, Unsettled};

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
