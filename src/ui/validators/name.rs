use chimitheque_utils::string::{Transform::Uppercase, clean_required_text};

pub fn validate(s: &str) -> Result<String, String> {
    clean_required_text(s, Uppercase).ok_or_else(|| "Input cannot be empty".to_string())
}
