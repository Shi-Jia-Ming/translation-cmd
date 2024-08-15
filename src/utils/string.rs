pub(crate) struct StringUtils;

impl StringUtils {
    pub fn remove_quotes(s: &str) -> String {
        s.trim_matches(|c| c == '\"' || c == '\'').to_string()
    }
}