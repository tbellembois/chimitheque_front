use chimitheque_utils::string::{Transform::Uppercase, clean};

pub fn validate(s: &str) -> Result<String, String> {
    let cleaned = clean(s, Uppercase);

    if cleaned.is_empty() {
        Err("Input cannot be empty".to_string())
    } else {
        Ok(cleaned)
    }
}
