mod api;
mod error;
mod promises;
pub mod ui;

// Init translations for current crate.
rust_i18n::i18n!("locales", fallback = "en-GB");
