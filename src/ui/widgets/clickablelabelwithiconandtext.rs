use egui::{FontFamily, FontId, Pos2, RichText, Sense, Stroke};

const CORNER_RADIUS: f32 = 5.0;
const INNER_MARGIN: egui::Margin = egui::Margin::symmetric(10, 10);
const ICON_SIZE: f32 = 30.0;

pub fn clickable_label_with_icon_and_text(
    ui: &mut egui::Ui,
    text: String,
    icon: &str,
) -> egui::Response {
    let widgets = &ui.visuals().widgets;
    let hovered_stroke = widgets.hovered.fg_stroke;

    let frame = egui::Frame::new()
        .corner_radius(CORNER_RADIUS)
        .inner_margin(INNER_MARGIN);

    let inner = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(icon).font(FontId {
                size: ICON_SIZE,
                family: FontFamily::Name("phosphor".into()),
            }));

            ui.label(RichText::new(text.clone()));
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
