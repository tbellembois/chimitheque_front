use crate::types::ProductType;
use crate::ui::widgets::size::Size;
use crate::ui::{
    app::App,
    state::{Action, Page},
    widgets::{
        buttonwithiconandtext::button_with_icon_and_text,
        clickablelabelwithiconandtext::clickable_label_with_icon_and_text,
    },
};
use egui::{Stroke, TextBuffer};
use rust_i18n::t;

pub fn render_search_form(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let top_panel_height = app.state.top_panel_rect.height();

    let window_available_rect = app.state.window_available_rect;
    let window_available_width = window_available_rect.width();
    let search_form_outer_x_margin = (window_available_width - app.visual.search_form_width) / 2.0;

    let widgets = &ui.visuals().widgets;
    let normal_stroke = widgets.noninteractive.bg_stroke;

    // Calculate search form size and position (ie. rect).
    let search_form_top_left =
        window_available_rect.left_top() + egui::vec2(search_form_outer_x_margin, top_panel_height);
    let search_form_bottom_right =
        window_available_rect.right_bottom() - egui::vec2(search_form_outer_x_margin, 0.0);
    let search_form_rec = egui::Rect::from_two_pos(search_form_top_left, search_form_bottom_right);

    app.state.search_rect = search_form_rec;

    // Initialize advanced search rect to the same size as the search form for the first frame draw.
    app.state.advanced_search_rect = search_form_rec;

    ui.vertical(|ui| {
        ui.scope_builder(egui::UiBuilder::new().max_rect(search_form_rec), |ui| {
            // FIXME
            ui.add_space(20.0);

            // egui's ui.group does not support margins, so we use a custom frame instead.
            let custom_group_frame = egui::Frame::new()
                .inner_margin(app.visual.search_form_inner_margin)
                .corner_radius(app.visual.app_corner_radius)
                .fill(app.visual.faint_bg_color)
                .stroke(normal_stroke);

            if app.search_form_expanded {
                custom_group_frame.show(ui, |ui| {
                    egui::Grid::new("search_form_product_type_grid")
                        .min_col_width(
                            (app.visual.search_form_width
                                - (2.0 * app.visual.search_form_inner_margin.leftf())
                                - (5.0 * app.visual.search_form_widget_horizontal_spacing))
                                / 4.0,
                        )
                        .max_col_width(
                            (app.visual.search_form_width
                                - (2.0 * app.visual.search_form_inner_margin.leftf())
                                - (5.0 * app.visual.search_form_widget_horizontal_spacing))
                                / 4.0,
                        )
                        .num_columns(4)
                        .spacing([
                            app.visual.search_form_widget_horizontal_spacing,
                            app.visual.search_form_widget_vertical_spacing,
                        ])
                        .striped(false)
                        .show(ui, |ui| {
                            ui.radio_value(
                                &mut app.search_product_type,
                                ProductType::Chemical,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::ATOM,
                                    t!("search_form_product_type_chemical")
                                ),
                            );
                            ui.radio_value(
                                &mut app.search_product_type,
                                ProductType::Biological,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::DNA,
                                    t!("search_form_product_type_biological")
                                ),
                            );
                            ui.radio_value(
                                &mut app.search_product_type,
                                ProductType::Consumable,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::PACKAGE,
                                    t!("search_form_product_type_consumable")
                                ),
                            );
                            ui.radio_value(
                                &mut app.search_product_type,
                                ProductType::All,
                                format!(
                                    "{} {}",
                                    egui_phosphor::fill::CIRCLE,
                                    t!("search_form_product_type_all")
                                ),
                            );

                            ui.end_row();
                        });

                    ui.add_space(app.visual.search_form_widget_vertical_spacing);

                    egui::Grid::new("search_form_grid")
                        .min_col_width(
                            (app.visual.search_form_width
                                - (2.0 * app.visual.search_form_inner_margin.leftf())
                                - (5.0 * app.visual.search_form_widget_horizontal_spacing))
                                / 3.0,
                        )
                        .max_col_width(
                            (app.visual.search_form_width
                                - (2.0 * app.visual.search_form_inner_margin.leftf())
                                - (5.0 * app.visual.search_form_widget_horizontal_spacing))
                                / 3.0,
                        )
                        .num_columns(3)
                        .spacing([
                            app.visual.search_form_widget_horizontal_spacing,
                            app.visual.search_form_widget_vertical_spacing,
                        ])
                        .striped(false)
                        .show(ui, |ui| {
                            // part of name
                            let stroke = if app.search_part_of_name.is_empty() {
                                Stroke::default()
                            } else {
                                app.visual.input_filled_stroke
                            };
                            let inner_margin = if app.search_part_of_name.is_empty() {
                                egui::Margin::symmetric(0, 0)
                            } else {
                                egui::Margin::symmetric(5, 5)
                            };
                            let frame = egui::Frame::new()
                                .stroke(stroke)
                                .inner_margin(inner_margin)
                                .corner_radius(app.visual.app_corner_radius);
                            frame.show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut app.search_part_of_name)
                                        .hint_text(t!("search_input_part_of_name")),
                                );
                            });

                            // name
                            app.search_name_widget.ui(ui);
                            // let stroke = if app.search_name_widget.selected.is_empty() {
                            //     Stroke::default()
                            // } else {
                            //     app.visual.input_filled_stroke
                            // };
                            // let inner_margin = if app.search_name_widget.selected.is_empty() {
                            //     egui::Margin::symmetric(0, 0)
                            // } else {
                            //     egui::Margin::symmetric(5, 5)
                            // };
                            // let frame = egui::Frame::new()
                            //     .stroke(stroke)
                            //     .inner_margin(inner_margin)
                            //     .corner_radius(app.visual.app_corner_radius);
                            // frame.show(ui, |ui| {
                            //     app.search_name_widget.ui(ui);
                            // });

                            // barecode
                            let stroke = if app.search_barecode.is_empty() {
                                Stroke::default()
                            } else {
                                app.visual.input_filled_stroke
                            };
                            let inner_margin = if app.search_barecode.is_empty() {
                                egui::Margin::symmetric(0, 0)
                            } else {
                                egui::Margin::symmetric(5, 5)
                            };
                            let frame = egui::Frame::new()
                                .stroke(stroke)
                                .inner_margin(inner_margin)
                                .corner_radius(app.visual.app_corner_radius);
                            frame.show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut app.search_barecode)
                                        .hint_text(t!("search_input_barecode")),
                                );
                            });
                            ui.end_row();

                            // cas number
                            let stroke = if app.search_cas_number_widget.selected.is_empty() {
                                Stroke::default()
                            } else {
                                app.visual.input_filled_stroke
                            };
                            let inner_margin = if app.search_cas_number_widget.selected.is_empty() {
                                egui::Margin::symmetric(0, 0)
                            } else {
                                egui::Margin::symmetric(5, 5)
                            };
                            let frame = egui::Frame::new()
                                .stroke(stroke)
                                .inner_margin(inner_margin)
                                .corner_radius(app.visual.app_corner_radius);
                            frame.show(ui, |ui| {
                                app.search_cas_number_widget.ui(ui);
                            });

                            // empirical formula
                            let stroke = if app.search_empirical_formula_widget.selected.is_empty()
                            {
                                Stroke::default()
                            } else {
                                app.visual.input_filled_stroke
                            };
                            let inner_margin =
                                if app.search_empirical_formula_widget.selected.is_empty() {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                            let frame = egui::Frame::new()
                                .stroke(stroke)
                                .inner_margin(inner_margin)
                                .corner_radius(app.visual.app_corner_radius);
                            frame.show(ui, |ui| {
                                app.search_empirical_formula_widget.ui(ui);
                            });

                            // producer ref
                            let stroke = if app.search_producer_ref_widget.selected.is_empty() {
                                Stroke::default()
                            } else {
                                app.visual.input_filled_stroke
                            };
                            let inner_margin = if app.search_producer_ref_widget.selected.is_empty()
                            {
                                egui::Margin::symmetric(0, 0)
                            } else {
                                egui::Margin::symmetric(5, 5)
                            };
                            let frame = egui::Frame::new()
                                .stroke(stroke)
                                .inner_margin(inner_margin)
                                .corner_radius(app.visual.app_corner_radius);
                            frame.show(ui, |ui| {
                                app.search_producer_ref_widget.ui(ui);
                            });
                            ui.end_row();
                        });

                    ui.add_space(app.visual.search_form_widget_vertical_spacing);

                    let collapse_state = ui.collapsing(t!("advanced_search"), |ui| {
                        ui.add_space(app.visual.search_form_widget_vertical_spacing);

                        let advanced_search_form_width: f32 =
                            app.state.advanced_search_rect.width();

                        egui::Grid::new("advanced_search_form_grid")
                            .min_col_width(
                                (advanced_search_form_width
                                    - (4.0 * app.visual.search_form_widget_horizontal_spacing))
                                    / 3.0,
                            )
                            .max_col_width(
                                (advanced_search_form_width
                                    - (4.0 * app.visual.search_form_widget_horizontal_spacing))
                                    / 3.0,
                            )
                            .num_columns(3)
                            .spacing([
                                app.visual.search_form_widget_horizontal_spacing,
                                app.visual.search_form_widget_vertical_spacing,
                            ])
                            .striped(false)
                            .show(ui, |ui| {
                                // entity
                                let stroke = if app.search_entity_widget.selected.is_empty() {
                                    Stroke::default()
                                } else {
                                    app.visual.input_filled_stroke
                                };
                                let inner_margin = if app.search_entity_widget.selected.is_empty() {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_entity_widget.ui(ui);
                                });

                                // store location
                                let stroke = if app.search_store_location_widget.selected.is_empty()
                                {
                                    Stroke::default()
                                } else {
                                    app.visual.input_filled_stroke
                                };
                                let inner_margin =
                                    if app.search_store_location_widget.selected.is_empty() {
                                        egui::Margin::symmetric(0, 0)
                                    } else {
                                        egui::Margin::symmetric(5, 5)
                                    };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_store_location_widget.ui(ui);
                                });

                                // signal word
                                let stroke = if app.search_signal_word_widget.selected.is_empty() {
                                    Stroke::default()
                                } else {
                                    app.visual.input_filled_stroke
                                };
                                let inner_margin =
                                    if app.search_signal_word_widget.selected.is_empty() {
                                        egui::Margin::symmetric(0, 0)
                                    } else {
                                        egui::Margin::symmetric(5, 5)
                                    };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_signal_word_widget.ui(ui);
                                });
                                ui.end_row();

                                // symbol
                                let stroke = if app.search_symbol_widget.selected.is_empty() {
                                    Stroke::default()
                                } else {
                                    app.visual.input_filled_stroke
                                };
                                let inner_margin = if app.search_symbol_widget.selected.is_empty() {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_symbol_widget.ui(ui);
                                });

                                // hazard statement
                                let stroke =
                                    if app.search_hazard_statement_widget.selected.is_empty() {
                                        Stroke::default()
                                    } else {
                                        app.visual.input_filled_stroke
                                    };
                                let inner_margin =
                                    if app.search_hazard_statement_widget.selected.is_empty() {
                                        egui::Margin::symmetric(0, 0)
                                    } else {
                                        egui::Margin::symmetric(5, 5)
                                    };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_hazard_statement_widget.ui(ui);
                                });

                                // precautionary statement
                                let stroke = if app
                                    .search_precautionary_statement_widget
                                    .selected
                                    .is_empty()
                                {
                                    Stroke::default()
                                } else {
                                    app.visual.input_filled_stroke
                                };
                                let inner_margin = if app
                                    .search_precautionary_statement_widget
                                    .selected
                                    .is_empty()
                                {
                                    egui::Margin::symmetric(0, 0)
                                } else {
                                    egui::Margin::symmetric(5, 5)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    app.search_precautionary_statement_widget.ui(ui);
                                });
                                ui.end_row();

                                // cmr
                                let stroke = if app.search_product_cmr {
                                    app.visual.input_filled_stroke
                                } else {
                                    Stroke::default()
                                };
                                let inner_margin = if app.search_product_cmr {
                                    egui::Margin::symmetric(5, 5)
                                } else {
                                    egui::Margin::symmetric(0, 0)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    ui.add(egui::Checkbox::new(
                                        &mut app.search_product_cmr,
                                        t!("search_form_product_cmr"),
                                    ));
                                });

                                // borrowed
                                let stroke = if app.search_product_borrowed {
                                    app.visual.input_filled_stroke
                                } else {
                                    Stroke::default()
                                };
                                let inner_margin = if app.search_product_borrowed {
                                    egui::Margin::symmetric(5, 5)
                                } else {
                                    egui::Margin::symmetric(0, 0)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    ui.add(egui::Checkbox::new(
                                        &mut app.search_product_borrowed,
                                        t!("search_form_product_borrowed"),
                                    ));
                                });

                                // to destroy
                                let stroke = if app.search_product_to_destroy {
                                    app.visual.input_filled_stroke
                                } else {
                                    Stroke::default()
                                };
                                let inner_margin = if app.search_product_to_destroy {
                                    egui::Margin::symmetric(5, 5)
                                } else {
                                    egui::Margin::symmetric(0, 0)
                                };
                                let frame = egui::Frame::new()
                                    .stroke(stroke)
                                    .inner_margin(inner_margin)
                                    .corner_radius(app.visual.app_corner_radius);
                                frame.show(ui, |ui| {
                                    ui.add(egui::Checkbox::new(
                                        &mut app.search_product_to_destroy,
                                        t!("search_form_product_to_destroy"),
                                    ));
                                });
                                ui.end_row();
                            });

                        app.state.advanced_search_rect = ui.min_rect();
                    });

                    if collapse_state.fully_open() {
                        ui.add_space(app.visual.search_form_widget_vertical_spacing);
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
                            app.search_form_expanded = false;
                            app.current_search_offset = 0;
                            app.state.action.push_back(Action::GetProducts(false));
                            app.state.active_page = Page::ProductList;
                        }

                        if button_with_icon_and_text(
                            ui,
                            t!("search_form_action_reset_filter").to_string(),
                            egui_phosphor::fill::ERASER,
                            &Size::Medium,
                        )
                        .clicked()
                        {
                            app.current_search_offset = 0;

                            app.search_product_type = ProductType::default();
                            app.search_part_of_name = String::new();
                            app.search_name_widget.clear_selected_items();
                            app.search_barecode = String::new();
                            app.search_cas_number_widget.clear_selected_items();
                            app.search_empirical_formula_widget.clear_selected_items();
                            app.search_producer_ref_widget.clear_selected_items();
                            app.search_entity_widget.clear_selected_items();
                            app.search_store_location_widget.clear_selected_items();
                            app.search_signal_word_widget.clear_selected_items();
                            app.search_symbol_widget.clear_selected_items();
                            app.search_hazard_statement_widget.clear_selected_items();
                            app.search_precautionary_statement_widget
                                .clear_selected_items();
                            app.search_product_borrowed = false;
                            app.search_product_to_destroy = false;
                            app.search_product_cmr = false;
                        }

                        if button_with_icon_and_text(
                            ui,
                            t!("search_form_shrink").to_string(),
                            egui_phosphor::fill::ARROWS_IN,
                            &Size::Medium,
                        )
                        .clicked()
                        {
                            app.search_form_expanded = false;
                        }
                    });
                });
            } else if clickable_label_with_icon_and_text(
                ui,
                t!("search_form_expand").as_str(),
                egui_phosphor::regular::ARROWS_OUT,
                &Size::Medium,
            )
            .clicked()
            {
                app.search_form_expanded = true;
            }
        });
    });
}
