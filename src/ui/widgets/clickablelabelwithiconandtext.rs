use crate::ui::widgets::size::Size;
use egui::{CursorIcon, RichText, Sense};

pub fn clickable_label_with_icon_and_text(
    ui: &mut egui::Ui,
    text: &str,
    icon: &str,
    size: &Size,
) -> egui::Response {
    let widgets = &ui.visuals().widgets;
    let frame = egui::Frame::new().inner_margin(size.inner_margin());

    let hovered_stroke = widgets.hovered.fg_stroke;

    // 1. Draw the frame layout content
    let inner = frame.show(ui, |ui| {
        egui::Frame::group(ui.style())
            .stroke(egui::Stroke::NONE)
            .inner_margin(0.0)
            .outer_margin(0.0)
            .show(ui, |ui| {
                ui.label(RichText::new(icon))
                    .on_hover_cursor(CursorIcon::PointingHand);

                ui.label(RichText::new(text));
            });
    });

    // 2. Allocate the entire frame area as a single interactive element
    let response = ui.interact(inner.response.rect, inner.response.id, Sense::click());

    // 3. Apply hover effects and cursor changes directly via the response
    if response.hovered() {
        let rect = response.rect;

        // Draw a rounded border around the entire frame
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(5),
            hovered_stroke,
            egui::StrokeKind::Inside,
        );

        // This will change the cursor seamlessly for the entire button area
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }

    response
}
