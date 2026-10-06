use crate::keycloak::get_base_url;
use std::sync::LazyLock;

/// The backend base URL, read once from `window.chimitheque_base_url` at startup.
/// Falls back to `"https://localhost:8443"` if the JS global is not set.
pub static BASE_URL: LazyLock<String> = LazyLock::new(get_base_url);
