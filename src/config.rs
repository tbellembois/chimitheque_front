use crate::keycloak::get_base_url;
use std::sync::LazyLock;
use std::sync::OnceLock;

/// The backend base URL, read once from `window.chimitheque_base_url` at startup.
/// Falls back to `"https://localhost:8443"` if the JS global is not set.
pub static BASE_URL: LazyLock<String> = LazyLock::new(get_base_url);

pub static HOVERED_BG_COLOR: OnceLock<egui::Color32> = OnceLock::new();
pub static SELECTED_BG_COLOR: OnceLock<egui::Color32> = OnceLock::new();
pub static NORMAL_BG_COLOR: OnceLock<egui::Color32> = OnceLock::new();
pub static FAINT_BG_COLOR: OnceLock<egui::Color32> = OnceLock::new();
pub static WEAK_TEXT_COLOR: OnceLock<egui::Color32> = OnceLock::new();

pub static NORMAL_STROKE: OnceLock<egui::Stroke> = OnceLock::new();
pub static HOVERED_STROKE: OnceLock<egui::Stroke> = OnceLock::new();
pub static SELECTED_STROKE: OnceLock<egui::Stroke> = OnceLock::new();

pub static INFO_COLOR: OnceLock<egui::Color32> = OnceLock::new();
pub static ERROR_COLOR: OnceLock<egui::Color32> = OnceLock::new();

pub static APP_TOP_MARGIN: OnceLock<i8> = OnceLock::new();
pub static APP_BOTTOM_MARGIN: OnceLock<i8> = OnceLock::new();
pub static APP_LEFT_MARGIN: OnceLock<i8> = OnceLock::new();
pub static APP_RIGHT_MARGIN: OnceLock<i8> = OnceLock::new();
pub static HEADER_BOTTOM_MARGIN: OnceLock<i8> = OnceLock::new();

pub static APP_FONT_SIZE: OnceLock<f32> = OnceLock::new();

pub static APP_CORNER_RADIUS: OnceLock<egui::CornerRadius> = OnceLock::new();

pub static PRODUCT_LABEL_INNER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static PRODUCT_LABEL_OUTER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static PRODUCT_LABEL_PLUS_WIDTH: OnceLock<f32> = OnceLock::new();
pub static PRODUCT_LABEL_ACTION_WIDTH: OnceLock<f32> = OnceLock::new();

pub static STORAGE_LABEL_INNER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static STORAGE_LABEL_OUTER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static STORAGE_LABEL_PLUS_WIDTH: OnceLock<f32> = OnceLock::new();
pub static STORAGE_LABEL_ACTION_WIDTH: OnceLock<f32> = OnceLock::new();

pub static SEARCH_FORM_WIDTH: OnceLock<f32> = OnceLock::new();
pub static SEARCH_FORM_INNER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static SEARCH_FORM_WIDGET_HORIZONTAL_SPACING: OnceLock<f32> = OnceLock::new();
pub static SEARCH_FORM_WIDGET_VERTICAL_SPACING: OnceLock<f32> = OnceLock::new();

pub static CREATE_PRODUCT_FORM_DESIRED_WIDTH: OnceLock<f32> = OnceLock::new();
pub static CREATE_PRODUCT_FORM_INNER_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static CREATE_PRODUCT_FORM_CORNER_RADIUS: OnceLock<f32> = OnceLock::new();
pub static CREATE_PRODUCT_FORM_WIDGET_HORIZONTAL_SPACING: OnceLock<f32> = OnceLock::new();
pub static CREATE_PRODUCT_FORM_WIDGET_VERTICAL_SPACING: OnceLock<f32> = OnceLock::new();

pub static SELECT2_BORDER_WHEN_SELECTED_STROKE: OnceLock<egui::Stroke> = OnceLock::new();
pub static SELECT2_BORDER_WHEN_SELECTED_MARGIN: OnceLock<egui::Margin> = OnceLock::new();
pub static SELECT2_BORDER_WHEN_SELECTED_CORNER_RADIUS: OnceLock<f32> = OnceLock::new();

pub static INPUT_FILLED_STROKE: OnceLock<egui::Stroke> = OnceLock::new();
