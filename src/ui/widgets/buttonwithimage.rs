use egui::{Sense, TextureHandle};

const CORNER_RADIUS: f32 = 5.0;
const INNER_MARGIN: egui::Margin = egui::Margin::symmetric(10, 10);

pub fn button_with_image(ui: &mut egui::Ui, image: &TextureHandle) -> egui::Response {
    //let visuals = ui.visuals();
    let widgets = &ui.visuals().widgets;

    //let hovered_bg = widgets.hovered.bg_fill;
    let normal_bg = widgets.inactive.bg_fill;
    let normal_stroke = widgets.noninteractive.bg_stroke;
    let hovered_stroke = widgets.hovered.fg_stroke;

    let frame = egui::Frame::new()
        .fill(normal_bg)
        .corner_radius(CORNER_RADIUS)
        .inner_margin(INNER_MARGIN)
        .stroke(normal_stroke);

    let inner = frame.show(ui, |ui| {
        ui.image(image);
    });

    let response = inner.response.interact(Sense::click());

    if response.hovered() {
        ui.painter().rect_stroke(
            response.rect,
            CORNER_RADIUS,
            hovered_stroke,
            egui::StrokeKind::Outside,
        );
    }

    response
}
