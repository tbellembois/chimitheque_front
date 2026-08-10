use crate::ui::widgets::size::Size;
use egui::{FontFamily, FontId, RichText};

pub fn icon(ui: &mut egui::Ui, icon: &str, size: &Size) -> egui::Response {
    let frame = egui::Frame::new();

    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(icon).font(FontId {
                size: size.icon_size(),
                family: FontFamily::Name("phosphor".into()),
            }));
        });
    });

    inner.response
}
