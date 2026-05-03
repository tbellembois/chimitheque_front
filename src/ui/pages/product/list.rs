use crate::ui::app::App;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.vertical(|ui| {
        if let Some((products, _)) = app.products.lock().unwrap().as_ref() {
            for product in products {
                ui.label(format!("- {}", product.name.name_label));
            }
        }
    });
}
