use std::sync::Arc;

use egui::{Color32, Margin, Response, Stroke, TextBuffer};
use egui_select2::select2::EguiSelect2;
use rust_i18n::t;

use crate::{
    api::symbol::format_suggestion,
    config::{
        APP_CORNER_RADIUS, FAINT_BG_COLOR, INPUT_FILLED_STROKE, NORMAL_STROKE,
        SEARCH_FORM_INNER_MARGIN, SEARCH_FORM_WIDGET_HORIZONTAL_SPACING,
        SEARCH_FORM_WIDGET_VERTICAL_SPACING, SEARCH_FORM_WIDTH,
        SELECT2_BORDER_WHEN_SELECTED_CORNER_RADIUS, SELECT2_BORDER_WHEN_SELECTED_MARGIN,
        SELECT2_BORDER_WHEN_SELECTED_STROKE,
    },
    types::ProductType,
    ui::{
        state::{Action, ApplicationState, Page},
        widgets::{buttonwithiconandtext::button_with_icon_and_text, size::Size},
    },
};

#[derive(Default)]
pub struct SearchForm {
    pub is_expanded: bool,
    pub search_offset: usize,
    pub search_limit: usize,

    pub search_store_location_widget: EguiSelect2,
    pub search_name_widget: EguiSelect2,
    pub search_entity_widget: EguiSelect2,
    pub search_producer_ref_widget: EguiSelect2,
    pub search_signal_word_widget: EguiSelect2,
    pub search_category_widget: EguiSelect2,
    pub search_cas_number_widget: EguiSelect2,
    pub search_empirical_formula_widget: EguiSelect2,
    pub search_hazard_statement_widget: EguiSelect2,
    pub search_precautionary_statement_widget: EguiSelect2,
    pub search_symbol_widget: EguiSelect2,
    pub search_tag_widget: EguiSelect2,

    pub search_part_of_name: String,
    pub search_barecode: String,
    pub search_product_cmr: bool,
    pub search_product_to_destroy: bool,
    pub search_product_borrowed: bool,
    pub search_product_type: ProductType,
}

impl SearchForm {
    pub fn apply_layout(&mut self) {
        let layout = egui_select2::select2::SelectedLayout {
            show_border_when_selected: true,
            border_when_selected_stroke: *SELECT2_BORDER_WHEN_SELECTED_STROKE.get().unwrap_or(
                &Stroke {
                    width: 0.0,
                    color: Color32::TRANSPARENT,
                },
            ),
            border_when_selected_margin: *SELECT2_BORDER_WHEN_SELECTED_MARGIN.get().unwrap_or(
                &Margin {
                    top: 0,
                    right: 0,
                    bottom: 0,
                    left: 0,
                },
            ),
            border_when_selected_corner_radius: *SELECT2_BORDER_WHEN_SELECTED_CORNER_RADIUS
                .get()
                .unwrap_or(&0.0f32),
        };

        self.search_store_location_widget.selected_layout = layout.clone();
        self.search_name_widget.selected_layout = layout.clone();
        self.search_entity_widget.selected_layout = layout.clone();
        self.search_producer_ref_widget.selected_layout = layout.clone();
        self.search_signal_word_widget.selected_layout = layout.clone();
        self.search_category_widget.selected_layout = layout.clone();
        self.search_cas_number_widget.selected_layout = layout.clone();
        self.search_empirical_formula_widget.selected_layout = layout.clone();
        self.search_hazard_statement_widget.selected_layout = layout.clone();
        self.search_precautionary_statement_widget.selected_layout = layout.clone();
        self.search_symbol_widget.selected_layout = layout.clone();
        self.search_tag_widget.selected_layout = layout;
    }

    pub fn new() -> Self {
        let translations = egui_select2::select2::Translations {
            loading: t!("select2_loading").to_string(),
            no_results: t!("select2_no_results").to_string(),
            add: t!("select2_add").to_string(),
            new: t!("select2_new").to_string(),
            clear_all: t!("select2_clear_all").to_string(),
            load_more: t!("select2_load_more").to_string(),
            ..Default::default()
        };

        // We can not set selected_layout as when new is called statics (in config.rs) have not been initialized yet.
        SearchForm {
            search_store_location_widget: EguiSelect2::new(
                Arc::new(crate::api::storelocation::load_suggestions),
                t!("select2_hint_store_location").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_name_widget: EguiSelect2::new(
                Arc::new(crate::api::name::load_suggestions),
                t!("select2_hint_name").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_entity_widget: EguiSelect2::new(
                Arc::new(crate::api::entity::load_suggestions),
                t!("select2_hint_entity").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_producer_ref_widget: EguiSelect2::new(
                Arc::new(crate::api::producerref::load_suggestions),
                t!("select2_hint_producer_ref").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_signal_word_widget: EguiSelect2::new(
                Arc::new(crate::api::signalword::load_suggestions),
                t!("select2_hint_signal_word").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_category_widget: EguiSelect2::new(
                Arc::new(crate::api::category::load_suggestions),
                t!("select2_hint_category").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_cas_number_widget: EguiSelect2::new(
                Arc::new(crate::api::casnumber::load_suggestions),
                t!("select2_hint_cas_number").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_empirical_formula_widget: EguiSelect2::new(
                Arc::new(crate::api::empiricalformula::load_suggestions),
                t!("select2_hint_empirical_formula").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_hazard_statement_widget: EguiSelect2::new(
                Arc::new(crate::api::hazardstatement::load_suggestions),
                t!("select2_hint_hazard_statement").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_precautionary_statement_widget: EguiSelect2::new(
                Arc::new(crate::api::precautionarystatement::load_suggestions),
                t!("select2_hint_precautionary_statement").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),
            search_symbol_widget: EguiSelect2::new(
                Arc::new(crate::api::symbol::load_suggestions),
                t!("select2_hint_symbol").as_str(),
                Some(translations.clone()),
                Some(Box::new(format_suggestion)),
                None,
                None,
                None,
            ),
            search_tag_widget: EguiSelect2::new(
                Arc::new(crate::api::tag::load_suggestions),
                t!("select2_hint_tag").as_str(),
                Some(translations.clone()),
                None,
                None,
                None,
                None,
            ),

            is_expanded: false,
            search_offset: 0,
            search_limit: 20,
            search_part_of_name: Default::default(),
            search_barecode: Default::default(),
            search_product_cmr: Default::default(),
            search_product_to_destroy: Default::default(),
            search_product_borrowed: Default::default(),
            search_product_type: Default::default(),
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        application_state: &mut ApplicationState,
    ) -> Response {
        // Get config values from static OnceLock variables
        let search_form_width = *SEARCH_FORM_WIDTH.get().unwrap();
        let search_form_inner_margin = *SEARCH_FORM_INNER_MARGIN.get().unwrap();
        let search_form_widget_horizontal_spacing =
            *SEARCH_FORM_WIDGET_HORIZONTAL_SPACING.get().unwrap();
        let search_form_widget_vertical_spacing =
            *SEARCH_FORM_WIDGET_VERTICAL_SPACING.get().unwrap();
        let app_corner_radius = *APP_CORNER_RADIUS.get().unwrap();
        let faint_bg_color = *FAINT_BG_COLOR.get().unwrap();
        let normal_stroke = *NORMAL_STROKE.get().unwrap();
        let input_filled_stroke = *INPUT_FILLED_STROKE.get().unwrap();

        // Calculate search form outer x margin to center it horizontally.
        let search_form_outer_x_margin =
            (application_state.central_panel_rect.width() - (search_form_width)) / 2.0;

        // Calculate search form size and position (ie. rect).
        let search_form_top_left = application_state.central_panel_rect.left_bottom()
            + egui::vec2(search_form_outer_x_margin, 0.0);
        let search_form_bottom_right = application_state.central_panel_rect.right_top()
            - egui::vec2(search_form_outer_x_margin, 0.0);
        let search_form_rec =
            egui::Rect::from_two_pos(search_form_top_left, search_form_bottom_right);

        // Initialize advanced search rect to the same size as the search form for the first frame draw.
        application_state.search_rect = search_form_rec;
        application_state.advanced_search_rect = search_form_rec;

        let response = ui.vertical(|ui| {
            ui.scope_builder(egui::UiBuilder::new().max_rect(search_form_rec), |ui| {
                // egui's ui.group does not support margins, so we use a custom frame instead.
                let custom_group_frame = egui::Frame::new()
                    .inner_margin(search_form_inner_margin)
                    .corner_radius(app_corner_radius)
                    .fill(faint_bg_color)
                    .stroke(normal_stroke);

                if self.is_expanded {
                    custom_group_frame.show(ui, |ui| {
                        egui::Grid::new("search_form_product_type_grid")
                            .min_col_width(
                                (SEARCH_FORM_WIDTH.get().unwrap()
                                    - (2.0 * search_form_inner_margin.leftf())
                                    - (5.0 * search_form_widget_horizontal_spacing))
                                    / 4.0,
                            )
                            .max_col_width(
                                (SEARCH_FORM_WIDTH.get().unwrap()
                                    - (2.0 * search_form_inner_margin.leftf())
                                    - (5.0 * search_form_widget_horizontal_spacing))
                                    / 4.0,
                            )
                            .num_columns(4)
                            .spacing([
                                search_form_widget_horizontal_spacing,
                                search_form_widget_vertical_spacing,
                            ])
                            .striped(false)
                            .show(ui, |ui| {
                                ui.radio_value(
                                    &mut self.search_product_type,
                                    ProductType::Chemical,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::ATOM,
                                        t!("search_form_product_type_chemical")
                                    ),
                                );
                                ui.radio_value(
                                    &mut self.search_product_type,
                                    ProductType::Biological,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::DNA,
                                        t!("search_form_product_type_biological")
                                    ),
                                );
                                ui.radio_value(
                                    &mut self.search_product_type,
                                    ProductType::Consumable,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::PACKAGE,
                                        t!("search_form_product_type_consumable")
                                    ),
                                );
                                ui.radio_value(
                                    &mut self.search_product_type,
                                    ProductType::All,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::CIRCLE,
                                        t!("search_form_product_type_all")
                                    ),
                                );

                                ui.end_row();
                            });

                        ui.add_space(search_form_widget_vertical_spacing);

                        egui::Grid::new("search_form_grid")
                            .min_col_width(
                                (SEARCH_FORM_WIDTH.get().unwrap()
                                    - (2.0 * search_form_inner_margin.leftf())
                                    - (5.0 * search_form_widget_horizontal_spacing))
                                    / 3.0,
                            )
                            .max_col_width(
                                (SEARCH_FORM_WIDTH.get().unwrap()
                                    - (2.0 * search_form_inner_margin.leftf())
                                    - (5.0 * search_form_widget_horizontal_spacing))
                                    / 3.0,
                            )
                            .num_columns(3)
                            .spacing([
                                search_form_widget_horizontal_spacing,
                                search_form_widget_vertical_spacing,
                            ])
                            .striped(false)
                            .show(ui, |ui| {
                                // part of name
                                let stroke = if self.search_part_of_name.is_empty() {
                                    Stroke::default()
                                } else {
                                    input_filled_stroke
                                };
                                let inner_margin = if self.search_part_of_name.is_empty() {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app_corner_radius);
                                frame.show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.search_part_of_name)
                                            .hint_text(t!("search_input_part_of_name")),
                                    );
                                });

                                // name
                                self.search_name_widget.ui(ui);

                                // barecode
                                let stroke = if self.search_barecode.is_empty() {
                                    Stroke::default()
                                } else {
                                    input_filled_stroke
                                };
                                let inner_margin = if self.search_barecode.is_empty() {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app_corner_radius);
                                frame.show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.search_barecode)
                                            .hint_text(t!("search_input_barecode")),
                                    );
                                });

                                ui.end_row();

                                // cas number
                                self.search_cas_number_widget.ui(ui);

                                // empirical formula
                                self.search_empirical_formula_widget.ui(ui);

                                // producer ref
                                self.search_producer_ref_widget.ui(ui);

                                ui.end_row();
                            });

                        ui.add_space(search_form_widget_vertical_spacing);

                        let collapse_state = ui.collapsing(t!("advanced_search"), |ui| {
                            ui.add_space(search_form_widget_vertical_spacing);

                            let advanced_search_form_width: f32 =
                                application_state.advanced_search_rect.width();

                            egui::Grid::new("advanced_search_form_grid")
                                .min_col_width(
                                    (advanced_search_form_width
                                        - (4.0 * search_form_widget_horizontal_spacing))
                                        / 3.0,
                                )
                                .max_col_width(
                                    (advanced_search_form_width
                                        - (4.0 * search_form_widget_horizontal_spacing))
                                        / 3.0,
                                )
                                .num_columns(3)
                                .spacing([
                                    search_form_widget_horizontal_spacing,
                                    search_form_widget_vertical_spacing,
                                ])
                                .striped(false)
                                .show(ui, |ui| {
                                    // entity
                                    self.search_entity_widget.ui(ui);

                                    // store location
                                    self.search_store_location_widget.ui(ui);

                                    // signal word
                                    self.search_signal_word_widget.ui(ui);

                                    ui.end_row();

                                    // symbol
                                    self.search_symbol_widget.ui(ui);

                                    // hazard statement
                                    self.search_hazard_statement_widget.ui(ui);

                                    // precautionary statement
                                    self.search_precautionary_statement_widget.ui(ui);

                                    ui.end_row();

                                    // cmr
                                    let stroke = if self.search_product_cmr {
                                        input_filled_stroke
                                    } else {
                                        Stroke::default()
                                    };
                                    let inner_margin = if self.search_product_cmr {
                                        egui::Margin::symmetric(5, 5)
                                    } else {
                                        egui::Margin::symmetric(0, 0)
                                    };
                                    let frame = egui::Frame::new()
                                        .stroke(stroke)
                                        .inner_margin(inner_margin)
                                        .corner_radius(app_corner_radius);
                                    frame.show(ui, |ui| {
                                        ui.add(egui::Checkbox::new(
                                            &mut self.search_product_cmr,
                                            t!("search_form_product_cmr"),
                                        ));
                                    });

                                    // borrowed
                                    let stroke = if self.search_product_borrowed {
                                        input_filled_stroke
                                    } else {
                                        Stroke::default()
                                    };
                                    let inner_margin = if self.search_product_borrowed {
                                        egui::Margin::symmetric(5, 5)
                                    } else {
                                        egui::Margin::symmetric(0, 0)
                                    };
                                    let frame = egui::Frame::new()
                                        .stroke(stroke)
                                        .inner_margin(inner_margin)
                                        .corner_radius(app_corner_radius);
                                    frame.show(ui, |ui| {
                                        ui.add(egui::Checkbox::new(
                                            &mut self.search_product_borrowed,
                                            t!("search_form_product_borrowed"),
                                        ));
                                    });

                                    // to destroy
                                    let stroke = if self.search_product_to_destroy {
                                        input_filled_stroke
                                    } else {
                                        Stroke::default()
                                    };
                                    let inner_margin = if self.search_product_to_destroy {
                                        egui::Margin::symmetric(5, 5)
                                    } else {
                                        egui::Margin::symmetric(0, 0)
                                    };
                                    let frame = egui::Frame::new()
                                        .stroke(stroke)
                                        .inner_margin(inner_margin)
                                        .corner_radius(app_corner_radius);
                                    frame.show(ui, |ui| {
                                        ui.add(egui::Checkbox::new(
                                            &mut self.search_product_to_destroy,
                                            t!("search_form_product_to_destroy"),
                                        ));
                                    });
                                    ui.end_row();
                                });

                            application_state.advanced_search_rect = ui.min_rect();
                        });

                        if collapse_state.fully_open() {
                            ui.add_space(search_form_widget_vertical_spacing);
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if button_with_icon_and_text(
                                ui,
                                t!("search_form_action_search").to_string(),
                                egui_phosphor::fill::LIST_MAGNIFYING_GLASS,
                                &Size::Medium,
                            )
                            .clicked()
                            {
                                self.is_expanded = false;
                                self.search_offset = 0;
                                application_state
                                    .action
                                    .push_back(Action::GetProducts(false));
                                application_state.active_page = Page::ProductList;
                            }

                            if button_with_icon_and_text(
                                ui,
                                t!("search_form_action_reset_filter").to_string(),
                                egui_phosphor::fill::ERASER,
                                &Size::Medium,
                            )
                            .clicked()
                            {
                                self.search_offset = 0;

                                self.search_product_type = ProductType::default();
                                self.search_part_of_name = String::new();
                                self.search_name_widget.clear_selected_items();
                                self.search_barecode = String::new();
                                self.search_cas_number_widget.clear_selected_items();
                                self.search_empirical_formula_widget.clear_selected_items();
                                self.search_producer_ref_widget.clear_selected_items();
                                self.search_entity_widget.clear_selected_items();
                                self.search_store_location_widget.clear_selected_items();
                                self.search_signal_word_widget.clear_selected_items();
                                self.search_symbol_widget.clear_selected_items();
                                self.search_hazard_statement_widget.clear_selected_items();
                                self.search_precautionary_statement_widget
                                    .clear_selected_items();
                                self.search_product_borrowed = false;
                                self.search_product_to_destroy = false;
                                self.search_product_cmr = false;
                            }

                            if button_with_icon_and_text(
                                ui,
                                t!("search_form_shrink").to_string(),
                                egui_phosphor::fill::ARROWS_IN,
                                &Size::Medium,
                            )
                            .clicked()
                            {
                                self.is_expanded = false;
                            }
                        });
                    });
                }
                // } else if clickable_label_with_icon_and_text(
                //     ui,
                //     t!("search_form_expand").as_str(),
                //     egui_phosphor::regular::ARROWS_OUT,
                //     &Size::Medium,
                // )
                // .clicked()
                // {
                //     self.is_expanded = true;
                // }
            });
        });

        response.response
    }
}
