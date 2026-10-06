use chimitheque_utils::{
    inchi::is_inchi,
    string::{Transform, clean},
};

pub fn validate(s: &str) -> Result<String, String> {
    let cleaned = clean(s, Transform::None);
    let is_inchi = is_inchi(cleaned.as_str());

    if is_inchi {
        Ok(cleaned)
    } else {
        Err(format!("Invalid inchi: {s}"))
    }
}
