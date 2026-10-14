use egui::{CursorIcon, Sense, TextureHandle};

const CORNER_RADIUS: f32 = 5.0;
const INNER_MARGIN: egui::Margin = egui::Margin::symmetric(10, 10);

pub fn button_with_image(ui: &mut egui::Ui, image: &TextureHandle) -> egui::Response {
    //let visuals = ui.visuals();
    let widgets = &ui.visuals().widgets;

    //let hovered_bg = widgets.hovered.bg_fill;
    let normal_bg = widgets.inactive.bg_fill;
    let normal_stroke = widgets.noninteractive.bg_stroke;
    let hovered_stroke = widgets.hovered.fg_stroke;

    // 1. Draw the frame layout content
    let frame = egui::Frame::new()
        .fill(normal_bg)
        .corner_radius(CORNER_RADIUS)
        .inner_margin(INNER_MARGIN)
        .outer_margin(0.0)
        .stroke(normal_stroke);

    let inner = frame.show(ui, |ui| {
        ui.image(image).on_hover_cursor(CursorIcon::PointingHand);
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
