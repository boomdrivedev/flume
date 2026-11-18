//! The glob a stage's rule `every-file` matches its place's files by
//! (175, C.2): `*` any run of characters within one part of a path, `**`
//! any run of parts, none among them, and `?` one character of a part;
//! every other character is itself. A path's parts are split by `/`.

/// Whether `path` matches `pattern` whole (175, C.2).
pub(crate) fn matches(pattern: &str, path: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let path: Vec<char> = path.chars().collect();
    matches_at(&pattern, &path)
}

fn matches_at(pattern: &[char], path: &[char]) -> bool {
    match pattern {
        [] => path.is_empty(),
        ['*', '*', '/', rest @ ..] => {
            // No part at all, else one whole part and on.
            matches_at(rest, path)
                || (0..path.len())
                    .filter(|at| path[*at] == '/')
                    .any(|at| matches_at(rest, &path[at + 1..]))
        }
        ['*', '*', rest @ ..] => (0..=path.len()).any(|at| matches_at(rest, &path[at..])),
        ['*', rest @ ..] => {
            let part = path.iter().position(|c| *c == '/').unwrap_or(path.len());
            (0..=part).any(|at| matches_at(rest, &path[at..]))
        }
        ['?', rest @ ..] => {
            matches!(path.first(), Some(c) if *c != '/') && matches_at(rest, &path[1..])
        }
        [c, rest @ ..] => path.first() == Some(c) && matches_at(rest, &path[1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_glob_matches_within_and_across_parts() {
        // `*` within one part.
        assert!(matches("personas/*.md", "personas/a.md"));
        assert!(matches("personas/*.md", "personas/.md"));
        assert!(!matches("personas/*.md", "personas/deep/a.md"));
        assert!(!matches("personas/*.md", "notes/c.md"));
        assert!(!matches("personas/*.md", "personas/a.mdx"));
        // `**` across parts, none among them.
        assert!(matches("personas/**/*.md", "personas/a.md"));
        assert!(matches("personas/**/*.md", "personas/deep/er/a.md"));
        assert!(matches("**/*.md", "a.md"));
        assert!(matches("**/*.md", "notes/c.md"));
        assert!(matches("personas/**", "personas/deep/a.md"));
        assert!(!matches("personas/**/*.md", "notes/a.md"));
        // `?` one character of a part.
        assert!(matches("personas/?.md", "personas/a.md"));
        assert!(!matches("personas/?.md", "personas/ab.md"));
        assert!(!matches("a?b", "a/b"));
        // Every other character is itself, the path matched whole.
        assert!(matches("README.md", "README.md"));
        assert!(!matches("README.md", "docs/README.md"));
        assert!(!matches("personas", "personas/a.md"));
    }
}
