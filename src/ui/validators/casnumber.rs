use chimitheque_utils::{
    casnumber::is_cas_number,
    string::{Transform::Uppercase, clean},
};
use egui_typed_input::ValText;

pub fn validate(s: &str) -> Result<String, String> {
    let cleaned = clean(s, Uppercase);
    let mayerr_is_cas_number = is_cas_number(cleaned.as_str());

    match mayerr_is_cas_number {
        Ok(_) => Ok(cleaned),
        Err(e) => Err(format!("Invalid CAS number: {e}")),
    }
}

pub fn name_validator() -> ValText<String, String> {
    ValText::new(
        // parser
        validate,
        // input validator
        |_current_text, input, _index| input.chars().all(|c| c.is_ascii_alphabetic()),
    )
}
