use chimitheque_utils::{
    inchi::is_inchi,
    string::{Transform, clean_required_text},
};

pub fn validate(s: &str) -> Result<String, String> {
    // An empty input is rejected by is_inchi.
    let cleaned = clean_required_text(s, Transform::None).unwrap_or_default();
    let is_inchi = is_inchi(cleaned.as_str());

    if is_inchi {
        Ok(cleaned)
    } else {
        Err(format!("Invalid inchi: {s}"))
    }
}
