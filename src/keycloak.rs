use wasm_bindgen::JsValue;
use web_sys::window;

pub fn get_token() -> Option<String> {
    let window = window()?;
    let keycloak = window.get("keycloak")?;

    // Access the token property.
    let token: JsValue =
        js_sys::Reflect::get(&keycloak, &JsValue::from_str("token")).unwrap_or_default();

    token.as_string()
}

pub fn get_base_url() -> String {
    let window = match window() {
        Some(w) => w,
        None => return "https://localhost:8443".to_string(),
    };

    let value = js_sys::Reflect::get(&window, &JsValue::from_str("chimitheque_base_url"))
        .unwrap_or_default();

    value
        .as_string()
        .unwrap_or_else(|| "https://localhost:8443".to_string())
}
