use chimitheque_utils::{
    casnumber::is_cas_number,
    string::{Transform::Uppercase, clean_required_text},
};

pub fn validate(s: &str) -> Result<String, String> {
    // An empty input is rejected by is_cas_number.
    let cleaned = clean_required_text(s, Uppercase).unwrap_or_default();
    let mayerr_is_cas_number = is_cas_number(cleaned.as_str());

    match mayerr_is_cas_number {
        Ok(()) => Ok(cleaned),
        Err(e) => Err(format!("Invalid CAS number: {e}")),
    }
}
