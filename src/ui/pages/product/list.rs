use crate::ui::app::App;
use egui::Ui;
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.vertical(|ui| match app.products.lock().unwrap().as_ref() {
        Some((products, _)) => {
            for product in products {
                ui.label(format!("- {}", product.name.name_label));
            }
        }
        None => {}
    });
}
