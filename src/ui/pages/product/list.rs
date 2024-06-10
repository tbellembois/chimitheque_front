use crate::ui::app::App;
use egui::Ui;
use rust_i18n::t;

pub fn update(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut Ui) {
    ui.vertical(|ui| {
        if let Some((products, count)) = &app.products {
            for product in products {
                if let Some(cas_number) = &product.cas_number {
                    ui.label(cas_number.casnumber_label.clone());
                }
            }
        }
    });
}
