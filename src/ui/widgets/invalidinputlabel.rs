use crate::ui::widgets::size::Size;
use egui::{FontFamily, FontId, RichText};
use egui_phosphor_icons::icons;

pub fn invalid_input_label(ui: &mut egui::Ui, text: String) -> egui::Response {
    let frame = egui::Frame::new();

    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(icons::WARNING_CIRCLE.fill());

            ui.label(RichText::new(text));
        });
    });

    inner.response
}
