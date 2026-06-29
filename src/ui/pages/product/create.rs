use crate::types::ProductType;
use crate::ui::widgets::size::Size;
use crate::ui::{app::App, widgets::buttonwithiconandtext::button_with_icon_and_text};
use rust_i18n::t;

const CREATE_PRODUCT_FORM_DESIRED_WIDTH: f32 = 925.0;
const CREATE_PRODUCT_FORM_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 20);
const CREATE_PRODUCT_FORM_CORNER_RADIUS: f32 = 8.0;
const WIDGET_HORIZONTAL_SPACING: f32 = 10.0;
const WIDGET_VERTICAL_SPACING: f32 = 20.0;

pub fn render(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let top_panel_height = app.state.top_panel_rect.height();

    let window_available_rect = app.state.window_available_rect;
    let window_available_width = window_available_rect.width();
    let create_product_form_outer_x_margin =
        (window_available_width - CREATE_PRODUCT_FORM_DESIRED_WIDTH) / 2.0;

    let widgets = &ui.visuals().widgets;
    let normal_stroke = widgets.noninteractive.bg_stroke;

    // Calculate search form size and position (ie. rect).
    let create_product_form_top_left = window_available_rect.left_top()
        + egui::vec2(create_product_form_outer_x_margin, top_panel_height);
    let create_product_form_bottom_right =
        window_available_rect.right_bottom() - egui::vec2(create_product_form_outer_x_margin, 0.0);
    let create_product_form_rec = egui::Rect::from_two_pos(
        create_product_form_top_left,
        create_product_form_bottom_right,
    );

    app.state.search_rect = create_product_form_rec;

    ui.vertical(|ui| {
        ui.scope_builder(
            egui::UiBuilder::new().max_rect(create_product_form_rec),
            |ui| {
                // FIXME
                ui.add_space(20.0);

                // egui's ui.group does not support margins, so we use a custom frame instead.
                let custom_group_frame = egui::Frame::new()
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
                                &mut app.search_product_type,
                                ProductType::Chemical,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::ATOM,
                                    t!("create_product_form_product_type_chemical")
                                ),
                            );
                            ui.radio_value(
                                &mut app.search_product_type,
                                ProductType::Biological,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::DNA,
                                    t!("create_product_form_product_type_biological")
                                ),
                            );
                            ui.radio_value(
                                &mut app.search_product_type,
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
                            app.create_product_tag_widget.ui(ui);
                            app.create_product_category_widget.ui(ui);
                            ui.end_row();

                            app.create_product_name_widget.ui(ui);
                            app.create_product_synonym_widget.ui(ui);
                            ui.end_row();

                            app.create_product_empirical_formula_widget.ui(ui);
                            app.create_product_linear_formula_widget.ui(ui);
                            ui.end_row();

                            // ui.add(
                            //     egui::TextEdit::singleline(&mut app.search_part_of_name)
                            //         .hint_text(t!("search_input_part_of_name")),
                            // );
                            // app.search_name_widget.ui(ui);
                            // ui.add(
                            //     egui::TextEdit::singleline(&mut app.search_barecode)
                            //         .hint_text(t!("search_input_barecode")),
                            // );
                            // ui.end_row();

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
}
