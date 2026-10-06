use crate::ui::widgets::size::Size;
use egui::{FontFamily, FontId, RichText};

pub fn invalid_input_label(ui: &mut egui::Ui, text: String) -> egui::Response {
    let frame = egui::Frame::new();

    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(egui_phosphor::fill::WARNING_CIRCLE).font(FontId {
                    size: Size::Medium.icon_size(),
                    family: FontFamily::Name("phosphor".into()),
                }),
            );

            ui.label(RichText::new(text));
        });
    });

    inner.response
}
