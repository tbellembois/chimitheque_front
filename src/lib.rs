#![warn(clippy::all, clippy::pedantic)]
#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]
#![allow(
    clippy::missing_errors_doc,
    // clippy::missing_panics_doc,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]
mod api;
pub mod atomic;
pub mod download;
mod error;
mod keycloak;
pub mod logger;
mod types;
pub mod ui;
pub mod utils;

// Init translations for current crate.
rust_i18n::i18n!("locales", fallback = "en-GB");
