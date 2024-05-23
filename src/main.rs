#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
mod api;
mod error;
mod ui;

use eframe::egui;
use log::info;
use ui::app::App;

// Init translations for current crate.
rust_i18n::i18n!("locales", fallback = "en-GB");

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    // Set default locale.
    rust_i18n::set_locale("fr-FR");

    // Set window options.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    info!("Creating app.");

    // Create GUI.
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support.
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(App::new(cc))
        }),
    )
}
