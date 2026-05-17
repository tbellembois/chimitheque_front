use crate::ui::widgets::size::Size;
use egui::{FontFamily, FontId, RichText, Sense};

const CORNER_RADIUS: f32 = 5.0;

pub fn button_with_icon_and_text(
    ui: &mut egui::Ui,
    text: String,
    icon: &str,
    size: &Size,
) -> egui::Response {
    // let visuals = ui.visuals();
    let widgets = &ui.visuals().widgets;

    // let hovered_bg = widgets.hovered.bg_fill;
    let normal_bg = widgets.inactive.bg_fill;
    let normal_stroke = widgets.noninteractive.bg_stroke;
    let hovered_stroke = widgets.hovered.fg_stroke;

    let frame = egui::Frame::new()
        .fill(normal_bg)
        .corner_radius(CORNER_RADIUS)
        .inner_margin(size.inner_margin())
        .stroke(normal_stroke);

    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(icon).font(FontId {
                size: size.icon_size(),
                family: FontFamily::Name("phosphor".into()),
            }));

            ui.label(RichText::new(text));
        });
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
