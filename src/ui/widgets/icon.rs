use crate::ui::widgets::size::Size;
use egui::RichText;

pub fn icon(ui: &mut egui::Ui, icon: &str, _size: &Size) -> egui::Response {
    let frame = egui::Frame::new();

    let inner = frame.show(ui, |ui| {
        ui.label(RichText::new(icon));
    });

    inner.response
}
