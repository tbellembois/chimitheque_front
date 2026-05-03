use wasm_bindgen::JsValue;
use web_sys::window;

pub fn get_token() -> Option<String> {
    let window = window()?;
    let keycloak = window.get("keycloak")?;

    // Access the token property
    let token: JsValue = js_sys::Reflect::get(&keycloak, &JsValue::from_str("token"))
        .expect("keycloak token not found");

    token.as_string()
}
