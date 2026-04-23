/// Parse a first-line slash command of the form `/name <rest>`.
/// Returns `(name, rest_after_name)` if the line begins with `/` and contains
/// a non-empty name; otherwise returns `None`.
pub fn parse_slash_name(line: &str) -> Option<(&str, &str)> {
    let stripped = line.strip_prefix('/')?;
    let mut name_end = stripped.len();
    for (idx, ch) in stripped.char_indices() {
        if ch.is_whitespace() {
            name_end = idx;
            break;
        }
    }
    let name = &stripped[..name_end];
    if name.is_empty() {
        return None;
    }
    let rest = stripped[name_end..].trim_start();
    Some((name, rest))
}

#[cfg(test)]
mod tests {
    use super::parse_slash_name;

    #[test]
    fn parses_slash_command_name_and_args() {
        assert_eq!(
            parse_slash_name("/review-branch feature"),
            Some(("review-branch", "feature"))
        );
    }

    #[test]
    fn returns_none_for_non_command_input() {
        assert_eq!(parse_slash_name("review-branch feature"), None);
        assert_eq!(parse_slash_name("/"), None);
    }
}
