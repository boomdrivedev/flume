//! An agents table's `model` and `effort`, read alike from a declaration
//! of kinds, from the host's configuration and from a role's entry, each
//! TOML (172, 197, C.2, C.7; design 3): a string, its program's own name
//! or word, or a table of programs whose entries are strings, each kept
//! as written, since only the agent's profile can say whether a value is
//! one its program takes.

use flume_seams::PerProgram;
use std::collections::BTreeMap;
use toml::Value;

/// The model or the effort `value` gives, or words saying why it gives
/// none, which the caller puts after the key it read.
pub fn read(value: &Value) -> Result<PerProgram, String> {
    match value {
        Value::String(one) => Ok(PerProgram::One(one.clone())),
        Value::Table(table) => {
            let mut programs = BTreeMap::new();
            for (program, entry) in table {
                let Value::String(entry) = entry else {
                    return Err(format!("the entry for `{program}` must be a string"));
                };
                programs.insert(program.clone(), entry.clone());
            }
            PerProgram::from_programs(programs)
        }
        _ => Err("must be a string or a table of programs".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(text: &str) -> Result<PerProgram, String> {
        let table: toml::Table = format!("model = {text}").parse().unwrap();
        read(&table["model"])
    }

    #[test]
    fn a_string_or_a_table_of_programs() {
        assert_eq!(model("\"opus\""), Ok(PerProgram::One("opus".into())));
        assert_eq!(
            model("{ claude = \"opus\", codex = \"gpt-5.5\" }"),
            Ok(PerProgram::Programs(BTreeMap::from([
                ("claude".to_string(), "opus".to_string()),
                ("codex".to_string(), "gpt-5.5".to_string()),
            ])))
        );
    }

    #[test]
    fn a_value_that_is_neither_is_refused_naming_why() {
        for (text, named) in [
            ("3", "a string or a table of programs"),
            ("[\"opus\"]", "a string or a table of programs"),
            ("{}", "names no program"),
            ("{ claude = 3 }", "`claude`"),
        ] {
            let refused = model(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
    }
}
