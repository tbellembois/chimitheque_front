use crate::ui::widgets::size::Size;
use egui::{FontFamily, FontId, Pos2, RichText, Sense, Stroke};

pub fn clickable_label_with_icon_and_text(
    ui: &mut egui::Ui,
    text: &str,
    icon: &str,
    size: &Size,
) -> egui::Response {
    let widgets = &ui.visuals().widgets;
    let hovered_stroke = widgets.hovered.fg_stroke;

    let frame = egui::Frame::new().inner_margin(size.inner_margin());

    let inner = frame.show(ui, |ui| {
        egui::Frame::group(ui.style())
            .stroke(egui::Stroke::NONE)
            .inner_margin(0.0)
            .inner_margin(0.0)
            .show(ui, |ui| {
                ui.label(RichText::new(icon).font(FontId {
                    size: size.icon_size(),
                    family: FontFamily::Name("phosphor".into()),
                }));

                ui.label(RichText::new(text));
            });
    });

    let response = inner
        .response
        .interact(Sense::click())
        .interact(Sense::hover());

    if response.hovered() {
        let rect = response.rect;

        ui.painter().line_segment(
            [
                Pos2::new(rect.left(), rect.bottom()),
                Pos2::new(rect.right(), rect.bottom()),
            ],
            Stroke::new(2.0, hovered_stroke.color),
        );
    }

    response
}
