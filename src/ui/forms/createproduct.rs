use chimitheque_utils::inchi::is_inchi;
use egui::{Response, TextBuffer};
use egui_select2::select2::{Configuration, EguiSelect2};
use rust_i18n::t;
use std::sync::Arc;

use crate::{
    types::ProductType,
    ui::{
        state::ApplicationState,
        visual::ApplicationVisual,
        widgets::{
            buttonwithiconandtext::button_with_icon_and_text, icon::icon,
            invalidinputlabel::invalid_input_label, size::Size,
        },
    },
};

const CREATE_PRODUCT_FORM_DESIRED_WIDTH: f32 = 925.0;
const CREATE_PRODUCT_FORM_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 20);
const CREATE_PRODUCT_FORM_CORNER_RADIUS: f32 = 8.0;
const WIDGET_HORIZONTAL_SPACING: f32 = 10.0;
const WIDGET_VERTICAL_SPACING: f32 = 20.0;

#[derive(Default)]
pub struct CreateProductForm {
    pub create_product_type: ProductType,
    pub create_product_tag_widget: EguiSelect2,
    pub create_product_category_widget: EguiSelect2,
    pub create_product_name_widget: EguiSelect2,
    pub create_product_synonym_widget: EguiSelect2,
    pub create_product_empirical_formula_widget: EguiSelect2,
    pub create_product_linear_formula_widget: EguiSelect2,
    pub create_product_cas_number_widget: EguiSelect2,
    pub create_product_ce_number_widget: EguiSelect2,
    pub create_product_specificity: String,
    pub create_product_inchi: String,
    pub create_product_inchikey: String,
    pub create_product_smiles: String,
    pub create_product_molecular_weight: f64,
    pub create_product_unit_molecular_weight_widget: EguiSelect2,
    pub create_product_unit_temperature_widget: EguiSelect2,
    pub create_product_3d_formula: String,
    pub create_product_molecule_picture: Vec<u8>,
    // // Works on both native and WASM
    // let file = rfd::AsyncFileDialog::new()
    //     .add_filter("text", &["txt", "rs"])
    //     .pick_file()
    //     .await;

    // if let Some(file) = file {
    //     // On WASM, you must read the content into bytes
    //     // On native, you can also access file.path()
    //     let data: Vec<u8> = file.read().await;
    // }
    pub create_product_msds_link: String,
    pub create_product_producer_sheet: String,
    pub create_product_physical_state_widget: EguiSelect2,
    pub create_product_class_of_compound_widget: EguiSelect2,
    pub create_product_signal_word_widget: EguiSelect2,
    pub create_product_symbol_widget: EguiSelect2,
    pub create_product_hazard_statement_widget: EguiSelect2,
    pub create_product_precautionary_statement_widget: EguiSelect2,
    pub create_product_disposal_comment: String,
    pub create_product_remark: String,
    pub create_product_restricted: bool,
    pub create_product_radioactive: bool,

    pub inchi_is_valid: bool,
    pub inchikey_is_valid: bool,
    pub smiles_is_valid: bool,
}

impl CreateProductForm {
    /// Apply theme-dependent visual settings to all select2 widgets.
    /// Must be called once after the egui visuals are available (i.e. from the first `ui()` frame).
    pub fn apply_visual(&mut self, visual: &ApplicationVisual) {
        let layout = visual.select2_border_when_selected.clone();
        self.create_product_tag_widget.selected_layout = layout.clone();
        self.create_product_category_widget.selected_layout = layout.clone();
        self.create_product_name_widget.selected_layout = layout.clone();
        self.create_product_synonym_widget.selected_layout = layout.clone();
        self.create_product_empirical_formula_widget.selected_layout = layout.clone();
        self.create_product_linear_formula_widget.selected_layout = layout.clone();
        self.create_product_cas_number_widget.selected_layout = layout.clone();
        self.create_product_ce_number_widget.selected_layout = layout.clone();
        self.create_product_unit_molecular_weight_widget
            .selected_layout = layout.clone();
        self.create_product_unit_temperature_widget.selected_layout = layout.clone();
        self.create_product_physical_state_widget.selected_layout = layout.clone();
        self.create_product_class_of_compound_widget.selected_layout = layout.clone();
        self.create_product_signal_word_widget.selected_layout = layout.clone();
        self.create_product_symbol_widget.selected_layout = layout.clone();
        self.create_product_hazard_statement_widget.selected_layout = layout.clone();
        self.create_product_precautionary_statement_widget
            .selected_layout = layout;
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

        let create_product_form = Self {
            create_product_tag_widget: EguiSelect2::new(
                Arc::new(crate::api::tag::load_suggestions),
                t!("select2_hint_tag").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_category_widget: EguiSelect2::new(
                Arc::new(crate::api::category::load_suggestions),
                t!("select2_hint_category").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_name_widget: EguiSelect2::new(
                Arc::new(crate::api::name::load_suggestions),
                t!("select2_hint_name").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_synonym_widget: EguiSelect2::new(
                Arc::new(crate::api::name::load_suggestions),
                t!("select2_hint_synonym").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_empirical_formula_widget: EguiSelect2::new(
                Arc::new(crate::api::empiricalformula::load_suggestions),
                t!("select2_hint_empirical_formula").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_linear_formula_widget: EguiSelect2::new(
                Arc::new(crate::api::linearformula::load_suggestions),
                t!("select2_hint_linear_formula").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_cas_number_widget: EguiSelect2::new(
                Arc::new(crate::api::casnumber::load_suggestions),
                t!("select2_hint_cas_number").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_ce_number_widget: EguiSelect2::new(
                Arc::new(crate::api::cenumber::load_suggestions),
                t!("select2_hint_ce_number").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_unit_molecular_weight_widget: EguiSelect2::new(
                Arc::new(crate::api::unitmolecularweight::load_suggestions),
                t!("select2_hint_unit_molecular_weight").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_unit_temperature_widget: EguiSelect2::new(
                Arc::new(crate::api::unittemperature::load_suggestions),
                t!("select2_hint_unit_temperature").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_physical_state_widget: EguiSelect2::new(
                Arc::new(crate::api::physicalstate::load_suggestions),
                t!("select2_hint_physical_state").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_class_of_compound_widget: EguiSelect2::new(
                Arc::new(crate::api::classofcompound::load_suggestions),
                t!("select2_hint_class_of_compound").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: false,
                    ..Default::default()
                }),
                None,
            ),
            create_product_signal_word_widget: EguiSelect2::new(
                Arc::new(crate::api::signalword::load_suggestions),
                t!("select2_hint_signal_word").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: false,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_symbol_widget: EguiSelect2::new(
                Arc::new(crate::api::symbol::load_suggestions),
                t!("select2_hint_symbol").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_hazard_statement_widget: EguiSelect2::new(
                Arc::new(crate::api::hazardstatement::load_suggestions),
                t!("select2_hint_hazard_statement").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),
            create_product_precautionary_statement_widget: EguiSelect2::new(
                Arc::new(crate::api::precautionarystatement::load_suggestions),
                t!("select2_hint_precautionary_statement").as_str(),
                Some(translations.clone()),
                None,
                None,
                Some(Configuration {
                    multiple: true,
                    read_only: true,
                    ..Default::default()
                }),
                None,
            ),

            // Initialize all remaining fields
            create_product_type: ProductType::default(),
            create_product_msds_link: String::default(),
            create_product_producer_sheet: String::default(),
            create_product_disposal_comment: String::default(),
            create_product_remark: String::default(),
            create_product_specificity: String::default(),
            create_product_inchi: String::default(),
            create_product_inchikey: String::default(),
            create_product_smiles: String::default(),
            create_product_molecular_weight: 0.0,
            create_product_restricted: false,
            create_product_radioactive: false,
            create_product_3d_formula: String::default(),
            create_product_molecule_picture: Vec::default(),

            inchi_is_valid: true,
            inchikey_is_valid: true,
            smiles_is_valid: true,
        };

        create_product_form
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        application_state: &mut ApplicationState,
        application_visual: &ApplicationVisual,
    ) -> Response {
        let top_panel_height = application_state.top_panel_rect.height();

        let window_available_rect = application_state.window_available_rect;
        let window_available_width = window_available_rect.width();
        let create_product_form_outer_x_margin =
            (window_available_width - CREATE_PRODUCT_FORM_DESIRED_WIDTH) / 2.0;

        let widgets = &ui.visuals().widgets;
        let normal_stroke = widgets.noninteractive.bg_stroke;

        // Calculate search form size and position (ie. rect).
        let create_product_form_top_left = window_available_rect.left_top()
            + egui::vec2(create_product_form_outer_x_margin, top_panel_height);
        let create_product_form_bottom_right = window_available_rect.right_bottom()
            - egui::vec2(create_product_form_outer_x_margin, 0.0);
        let create_product_form_rec = egui::Rect::from_two_pos(
            create_product_form_top_left,
            create_product_form_bottom_right,
        );

        application_state.search_rect = create_product_form_rec;

        let response = ui.vertical(|ui| {
            ui.scope_builder(
                egui::UiBuilder::new().max_rect(create_product_form_rec),
                |ui| {
                    // FIXME
                    ui.add_space(20.0);

                    // egui's ui.group does not support margins, so we use a custom frame instead.
                    let custom_group_frame = egui::Frame::new()
                        .fill(application_visual.faint_bg_color)
                        .inner_margin(CREATE_PRODUCT_FORM_INNER_MARGIN)
                        .corner_radius(CREATE_PRODUCT_FORM_CORNER_RADIUS)
                        .stroke(normal_stroke);

                    custom_group_frame.show(ui, |ui| {
                        egui::Grid::new("create_product_form_product_type_grid")
                            .min_col_width(
                                (CREATE_PRODUCT_FORM_DESIRED_WIDTH
                                    - (2.0 * CREATE_PRODUCT_FORM_INNER_MARGIN.leftf())
                                    - (4.0 * WIDGET_HORIZONTAL_SPACING))
                                    / 3.0,
                            )
                            .num_columns(3)
                            .spacing([WIDGET_HORIZONTAL_SPACING, WIDGET_VERTICAL_SPACING])
                            .striped(false)
                            .show(ui, |ui| {
                                ui.radio_value(
                                    &mut self.create_product_type,
                                    ProductType::Chemical,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::ATOM,
                                        t!("create_product_form_product_type_chemical")
                                    ),
                                );
                                ui.radio_value(
                                    &mut self.create_product_type,
                                    ProductType::Biological,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::DNA,
                                        t!("create_product_form_product_type_biological")
                                    ),
                                );
                                ui.radio_value(
                                    &mut self.create_product_type,
                                    ProductType::Consumable,
                                    format!(
                                        "{} {}",
                                        egui_phosphor::fill::PACKAGE,
                                        t!("create_product_form_product_type_consumable")
                                    ),
                                );

                                ui.end_row();
                            });

                        ui.add_space(WIDGET_VERTICAL_SPACING);

                        egui::Grid::new("create_product_form_grid")
                            .min_col_width(
                                (CREATE_PRODUCT_FORM_DESIRED_WIDTH
                                    - (2.0 * CREATE_PRODUCT_FORM_INNER_MARGIN.leftf())
                                    - (3.0 * WIDGET_HORIZONTAL_SPACING))
                                    / 2.0,
                            )
                            .num_columns(2)
                            .spacing([WIDGET_HORIZONTAL_SPACING, WIDGET_VERTICAL_SPACING])
                            .striped(false)
                            .show(ui, |ui| {
                                self.create_product_tag_widget.ui(ui);
                                self.create_product_category_widget.ui(ui);
                                ui.end_row();

                                // ui.horizontal() destroys the layout with select2
                                egui::Frame::group(ui.style())
                                    .stroke(egui::Stroke::NONE)
                                    .show(ui, |ui| {
                                        icon(ui, egui_phosphor::regular::ASTERISK, &Size::Small);
                                        self.create_product_name_widget.ui(ui);
                                    });

                                self.create_product_synonym_widget.ui(ui);
                                ui.end_row();

                                self.create_product_empirical_formula_widget.ui(ui);
                                self.create_product_linear_formula_widget.ui(ui);
                                ui.end_row();

                                self.create_product_cas_number_widget.ui(ui);
                                self.create_product_ce_number_widget.ui(ui);
                                ui.end_row();

                                ui.add(
                                    egui::TextEdit::singleline(
                                        &mut self.create_product_specificity,
                                    )
                                    .hint_text(t!("create_product_form_specificity")),
                                );
                                ui.end_row();

                                // inchi input and validation.
                                egui::Frame::group(ui.style())
                                    .stroke(egui::Stroke::NONE)
                                    .show(ui, |ui| {
                                        let response = ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.create_product_inchi,
                                            )
                                            .hint_text(t!("create_product_form_inchi")),
                                        );
                                        if response.changed() {
                                            self.inchi_is_valid =
                                                is_inchi(self.create_product_inchi.as_str());
                                        }
                                        if !self.inchi_is_valid {
                                            invalid_input_label(
                                                ui,
                                                t!("create_product_form_inchi_invalid").to_string(),
                                            );
                                        }
                                    });

                                // inchi key input and validation.
                                egui::Frame::group(ui.style())
                                    .stroke(egui::Stroke::NONE)
                                    .show(ui, |ui| {
                                        let response = ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.create_product_inchi,
                                            )
                                            .hint_text(t!("create_product_form_inchikey")),
                                        );
                                        if response.changed() {
                                            self.inchikey_is_valid =
                                                is_inchi(self.create_product_inchikey.as_str());
                                        }
                                        if !self.inchikey_is_valid {
                                            invalid_input_label(
                                                ui,
                                                t!("create_product_form_inchikey_invalid")
                                                    .to_string(),
                                            );
                                        }
                                    });
                                ui.end_row();

                                // smiles input and validation.
                                egui::Frame::group(ui.style())
                                    .stroke(egui::Stroke::NONE)
                                    .show(ui, |ui| {
                                        let response = ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.create_product_smiles,
                                            )
                                            .hint_text(t!("create_product_form_smiles")),
                                        );
                                        if response.changed() {
                                            self.smiles_is_valid =
                                                is_inchi(self.create_product_smiles.as_str());
                                        }
                                        if !self.smiles_is_valid {
                                            invalid_input_label(
                                                ui,
                                                t!("create_product_form_smiles_invalid")
                                                    .to_string(),
                                            );
                                        }
                                    });
                                ui.end_row();

                                egui::Frame::group(ui.style())
                                    .stroke(egui::Stroke::NONE)
                                    .show(ui, |ui| {
                                        ui.label(t!("create_product_form_molecular_weight"));
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.create_product_molecular_weight,
                                            )
                                            .speed(0.1),
                                        );
                                        self.create_product_unit_molecular_weight_widget.ui(ui);
                                    });

                                // app.search_cas_number_widget.ui(ui);
                                // app.search_empirical_formula_widget.ui(ui);
                                // app.search_producer_ref_widget.ui(ui);
                                // ui.end_row();
                            });

                        ui.add_space(WIDGET_VERTICAL_SPACING);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if button_with_icon_and_text(
                                ui,
                                t!("create_product_form_action_cancel").to_string(),
                                egui_phosphor::fill::MAGIC_WAND,
                                &Size::Medium,
                            )
                            .clicked()
                            {}

                            if button_with_icon_and_text(
                                ui,
                                t!("create_product_form_action_create").to_string(),
                                egui_phosphor::fill::ERASER,
                                &Size::Medium,
                            )
                            .clicked()
                            {}
                        });
                    });
                },
            );
        });

        response.response
    }
}
