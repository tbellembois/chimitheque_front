use wasm_bindgen::JsValue;
use web_sys::window;

pub fn get_token() -> Option<String> {
    let window = window().expect("should have a window in this context");
    let keycloak = window.get("keycloak").expect("keycloak not initialized");

    // Access the token property
    let token: JsValue = js_sys::Reflect::get(&keycloak, &JsValue::from_str("token"))
        .expect("keycloak token not found");

    // Convert to String
    token.as_string()
}
