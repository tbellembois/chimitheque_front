use chimitheque_utils::string::{Transform::Uppercase, clean};
use egui_typed_input::ValText;

pub fn validate(s: &str) -> Result<String, String> {
    let cleaned = clean(s, Uppercase);

    if cleaned.is_empty() {
        Err("Input cannot be empty".to_string())
    } else {
        Ok(cleaned)
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
