use egui::Color32;
use egui::{CornerRadius, vec2};

#[derive(Clone, Copy)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

pub struct ButtonStyle {
    pub min_size: egui::Vec2,
    pub padding: egui::Vec2,
    pub radius: CornerRadius,
}

pub fn style_for(size: ButtonSize) -> ButtonStyle {
    match size {
        ButtonSize::Sm => ButtonStyle {
            min_size: vec2(80.0, 28.0),
            padding: vec2(8.0, 4.0),
            radius: CornerRadius::same(4),
        },
        ButtonSize::Md => ButtonStyle {
            min_size: vec2(110.0, 36.0),
            padding: vec2(12.0, 8.0),
            radius: CornerRadius::same(6),
        },
        ButtonSize::Lg => ButtonStyle {
            min_size: vec2(140.0, 44.0),
            padding: vec2(16.0, 10.0),
            radius: CornerRadius::same(8),
        },
    }
}

pub fn apply_variant(ui: &mut egui::Ui, variant: ButtonVariant) {
    let visuals = &mut ui.style_mut().visuals.widgets;

    match variant {
        ButtonVariant::Primary => {
            visuals.inactive.bg_fill = Color32::from_rgb(70, 120, 255);
            visuals.hovered.bg_fill = Color32::from_rgb(90, 140, 255);
        }
        ButtonVariant::Secondary => {
            visuals.inactive.bg_fill = Color32::from_gray(60);
            visuals.hovered.bg_fill = Color32::from_gray(80);
        }
        ButtonVariant::Danger => {
            visuals.inactive.bg_fill = Color32::from_rgb(200, 60, 60);
            visuals.hovered.bg_fill = Color32::from_rgb(220, 80, 80);
        }
    }
}

pub fn mybutton(
    ui: &mut egui::Ui,
    label: &str,
    size: ButtonSize,
    variant: ButtonVariant,
) -> egui::Response {
    let style_cfg = style_for(size);

    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding = style_cfg.padding;

        let visuals = &mut ui.style_mut().visuals.widgets;

        visuals.inactive.corner_radius = style_cfg.radius;
        visuals.hovered.corner_radius = style_cfg.radius;
        visuals.active.corner_radius = style_cfg.radius;

        apply_variant(ui, variant);

        ui.add_sized(style_cfg.min_size, egui::Button::new(label))
    })
    .inner
}
