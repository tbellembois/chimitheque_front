use crate::ui::widgets::size::Size;
use egui::{CursorIcon, FontFamily, FontId, RichText, Sense};

const CORNER_RADIUS: f32 = 5.0;

pub fn button_with_icon_and_text(
    ui: &mut egui::Ui,
    text: String,
    icon: &str,
    size: &Size,
) -> egui::Response {
    let widgets = &ui.visuals().widgets;

    let normal_bg = widgets.inactive.bg_fill;
    let normal_stroke = widgets.noninteractive.bg_stroke;
    let hovered_stroke = widgets.hovered.fg_stroke;

    let frame = egui::Frame::new()
        .fill(normal_bg)
        .corner_radius(CORNER_RADIUS)
        .inner_margin(size.inner_margin())
        .outer_margin(0.0)
        .stroke(normal_stroke);

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
                }));

                ui.label(RichText::new(text));
            });
    });

    // 2. Allocate the entire frame area as a single interactive element
    let response = ui.interact(inner.response.rect, inner.response.id, Sense::click());

    // 3. Apply hover effects and cursor changes directly via the response
    if response.hovered() {
        ui.painter().rect_stroke(
            response.rect,
            CORNER_RADIUS,
            hovered_stroke,
            egui::StrokeKind::Outside,
        );

        // This will change the cursor seamlessly for the entire button area
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }

    response
}
