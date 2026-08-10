use egui::{Color32, CornerRadius, Margin, Stroke};
use egui_select2::select2::SelectedLayout;

/// Application visual.
#[derive(Debug)]
pub struct ApplicationVisual {
    pub is_init: bool,

    pub hovered_bg_color: Color32,
    pub selected_bg_color: Color32,

    pub normal_bg_color: Color32,
    pub faint_bg_color: Color32,

    pub normal_stroke: Stroke,
    pub hovered_stroke: Stroke,
    pub selected_stroke: Stroke,

    pub info_color: Color32,
    pub error_color: Color32,

    pub app_top_margin: i8,
    pub app_bottom_margin: i8,
    pub app_left_margin: i8,
    pub app_right_margin: i8,

    pub app_font_size: f32,

    pub app_corner_radius: CornerRadius,

    pub product_label_inner_margin: Margin,
    pub product_label_outer_margin: Margin,
    pub product_label_plus_width: f32,
    pub product_label_action_width: f32,

    pub storage_label_inner_margin: Margin,
    pub storage_label_outer_margin: Margin,
    pub storage_label_plus_width: f32,
    pub storage_label_action_width: f32,

    pub search_form_width: f32,
    pub search_form_inner_margin: Margin,
    pub search_form_widget_horizontal_spacing: f32,
    pub search_form_widget_vertical_spacing: f32,

    pub select2_border_when_selected_stroke: Stroke,
    pub select2_border_when_selected_margin: Margin,
    pub select2_border_when_selected_corner_radius: f32,
    pub select2_border_when_selected: SelectedLayout,

    pub input_filled_stroke: Stroke,
}

impl Default for ApplicationVisual {
    fn default() -> ApplicationVisual {
        Self {
            is_init: false,
            normal_bg_color: Color32::WHITE,
            faint_bg_color: Color32::WHITE,
            hovered_bg_color: Color32::WHITE,
            selected_bg_color: Color32::WHITE,
            normal_stroke: Stroke::default(),
            hovered_stroke: Stroke::default(),
            selected_stroke: Stroke::default(),
            info_color: Color32::WHITE,
            error_color: Color32::WHITE,
            app_top_margin: 0,
            app_bottom_margin: 0,
            app_left_margin: 0,
            app_right_margin: 0,
            app_font_size: 16.0,
            app_corner_radius: CornerRadius::default(),
            product_label_inner_margin: Margin::default(),
            product_label_outer_margin: Margin::default(),
            product_label_plus_width: 0.0,
            product_label_action_width: 0.0,
            storage_label_inner_margin: Margin::default(),
            storage_label_outer_margin: Margin::default(),
            storage_label_plus_width: 0.0,
            storage_label_action_width: 0.0,
            search_form_width: 0.0,
            search_form_inner_margin: Margin::default(),
            search_form_widget_horizontal_spacing: 0.0,
            search_form_widget_vertical_spacing: 0.0,
            input_filled_stroke: Stroke::default(),
            select2_border_when_selected_stroke: Stroke::default(),
            select2_border_when_selected_margin: Margin::default(),
            select2_border_when_selected_corner_radius: 0.0,
            select2_border_when_selected: SelectedLayout::default(),
        }
    }
}
