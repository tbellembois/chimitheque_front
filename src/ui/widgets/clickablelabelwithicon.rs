use crate::ui::widgets::size::Size;
use egui::{CursorIcon, FontFamily, FontId, RichText, Sense};

pub fn clickable_label_with_icon(ui: &mut egui::Ui, icon: &str, size: &Size) -> egui::Response {
    let frame = egui::Frame::new().inner_margin(size.inner_margin());

    // 1. Draw the frame layout content
    let inner = frame.show(ui, |ui| {
        egui::Frame::group(ui.style())
            .stroke(egui::Stroke::NONE)
            .inner_margin(0.0)
            .inner_margin(0.0)
            .show(ui, |ui| {
                ui.label(RichText::new(icon).font(FontId {
                    size: size.icon_size(),
                    family: FontFamily::Name("phosphor".into()),
                }))
                .on_hover_cursor(CursorIcon::PointingHand);
            });
    });

    // 2. Allocate the entire frame area as a single interactive element
    let response = ui.interact(inner.response.rect, inner.response.id, Sense::click());

    // 3. Apply hover effects and cursor changes directly via the response
    if response.hovered() {
        //         let rect = response.rect;
        //
        //         ui.painter().line_segment(
        //             [
        //                 Pos2::new(rect.left(), rect.bottom()),
        //                 Pos2::new(rect.right(), rect.bottom()),
        //             ],
        //             Stroke::new(2.0, hovered_stroke.color),
        //         );

        // This will change the cursor seamlessly for the entire button area
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }

    response
}
