#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release

use chimitheque_front::ui::app::App;
use eframe::egui;

// Init translations for current crate.
rust_i18n::i18n!("locales", fallback = "en-GB");

fn main() -> Result<(), eframe::Error> {
    // Set default locale.
    rust_i18n::set_locale("fr-FR");

    // Set window options.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Create GUI.
    eframe::run_native(
        "Chimithèque",
        options,
        Box::new(|cc| {
            // This gives us image support.
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(App::new(cc)))
        }),
    )
}
