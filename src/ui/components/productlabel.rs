use chimitheque_types::casnumber::CasNumber;
use chimitheque_types::empiricalformula::EmpiricalFormula;
use chimitheque_types::product::Product;
use egui::{FontFamily, FontId, RichText};
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

use crate::ui::app::App;
use crate::ui::widgets::buttonwithicon::button_with_icon;
use crate::ui::widgets::buttonwithiconandtext::button_with_icon_and_text;
use crate::ui::widgets::size::Size;

pub fn render_product_label(
    app: &mut App,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    product: Product,
) {
    let window_available_width = app.state.window_available_rect.width();

    // let product_label_outer_margin = if window_available_width > 1024.0 {
    //     egui::Margin::symmetric(40, 5)
    // } else {
    //     egui::Margin::symmetric(5, 5)
    // };

    let label_width =
        window_available_width - (2.0 * app.visual.product_label_outer_margin.leftf());
    let available_space_for_cols =
        label_width - (2.0 * app.visual.product_label_inner_margin.leftf());
    let available_space_for_dyn_cols = available_space_for_cols
        - app.visual.product_label_plus_width
        - app.visual.product_label_action_width;
    let available_space_for_dyn_cols_in_percent = available_space_for_dyn_cols / 100.0;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .inner_margin(app.visual.product_label_inner_margin)
        .outer_margin(app.visual.product_label_outer_margin)
        .corner_radius(app.visual.app_corner_radius)
        .stroke(app.visual.normal_stroke);

    let product_card_shown = app
        .product_cards_shown
        .contains(&product.product_id.unwrap_or_default());

    let product_card_actions_shown = app
        .product_cards_actions_shown
        .contains(&product.product_id.unwrap_or_default());

    let hint_color = ui.visuals().weak_text_color.unwrap_or_else(|| {
        let text_color = ui.visuals().text_color();
        text_color.gamma_multiply(ui.visuals().weak_text_alpha)
    });

    custom_group_frame.show(ui, |ui| {
        TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(app.visual.product_label_plus_width))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 60.0,
            ))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 20.0,
            ))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 20.0,
            ))
            .column(Column::exact(app.visual.product_label_action_width))
            .id_salt(format!("{}_product_label", product.name.name_label))
            .body(|mut body| {
                body.row(50.0, |mut row| {
                    row.col(|ui| {
                        if product_card_shown {
                            if button_with_icon(ui, egui_phosphor::regular::MINUS).clicked() {
                                app.product_cards_shown
                                    .retain(|id| *id != product.product_id.unwrap_or_default());
                            }
                        } else if button_with_icon(ui, egui_phosphor::regular::PLUS).clicked() {
                            app.product_cards_shown
                                .push(product.product_id.unwrap_or_default());
                        }
                    });

                    row.col(|ui| {
                        ui.label(product.name.clone().name_label);

                        ui.horizontal(|ui| {
                            if let Some(product_sl) = product.product_sl {
                                ui.label(RichText::new(egui_phosphor::fill::DRESSER).font(
                                    FontId {
                                        family: FontFamily::Name("phosphor".into()),
                                        size: 20.0,
                                    },
                                ));
                                ui.label(product_sl);
                            }

                            if let Some(product_sc) = product.product_sc {
                                ui.label(
                                    RichText::new(format!(
                                        "[ {}: {product_sc} ]",
                                        t!("product_label_storages")
                                    ))
                                    .color(hint_color),
                                );
                            }
                        });
                    });

                    row.col(|ui| {
                        ui.label(
                            product
                                .empirical_formula
                                .clone()
                                .unwrap_or(EmpiricalFormula {
                                    empirical_formula_label: String::new(),
                                    ..Default::default()
                                })
                                .empirical_formula_label,
                        );
                    });

                    row.col(|ui| {
                        ui.label(
                            product
                                .cas_number
                                .clone()
                                .unwrap_or(CasNumber {
                                    cas_number_label: String::new(),
                                    ..Default::default()
                                })
                                .cas_number_label,
                        );

                        // Show CAS number CMR category.
                        if let Some(cas_number) = product.cas_number.clone()
                            && let Some(cas_number_cmr) = cas_number.cas_number_cmr
                        {
                            ui.label(cas_number_cmr);
                        }

                        // Show inflammable symbol.
                        ui.horizontal(|ui| {
                            if let Some(hs_cmr) = product.product_hs_cmr {
                                ui.label(hs_cmr);
                            }

                            if let Some(symbols) = product.symbols {
                                for symbol in symbols {
                                    if symbol.symbol_label == "GHS02" {
                                        ui.add(egui::Image::new("file://assets/GHS02.svg"));
                                    }
                                }
                            }
                        });
                    });

                    row.col(|ui| {
                        if product_card_actions_shown {
                            if button_with_icon(ui, egui_phosphor::regular::DOTS_THREE_CIRCLE)
                                .clicked()
                            {
                                app.product_cards_actions_shown
                                    .retain(|id| *id != product.product_id.unwrap_or_default());
                            }
                        } else if button_with_icon(
                            ui,
                            egui_phosphor::regular::DOTS_THREE_CIRCLE_VERTICAL,
                        )
                        .clicked()
                        {
                            app.product_cards_actions_shown
                                .push(product.product_id.unwrap_or_default());
                        }
                    });
                });
            });

        if product_card_actions_shown {
            ui.add_space(20.0);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Storages,
                    None,
                    &ehttp::Method::GET,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_label_action_storages").to_string(),
                    egui_phosphor::fill::PACKAGE,
                    &Size::Small,
                )
                .clicked()
                {
                    todo!()
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Storages,
                    None,
                    &ehttp::Method::POST,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_label_action_store").to_string(),
                    egui_phosphor::fill::BOX_ARROW_DOWN,
                    &Size::Small,
                )
                .clicked()
                {
                    todo!()
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Products,
                    None,
                    &ehttp::Method::POST,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_label_action_edit").to_string(),
                    egui_phosphor::fill::PENCIL,
                    &Size::Small,
                )
                .clicked()
                {
                    todo!()
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Products,
                    None,
                    &ehttp::Method::POST,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_label_action_bookmark").to_string(),
                    egui_phosphor::fill::BOOKMARK,
                    &Size::Small,
                )
                .clicked()
                {
                    todo!()
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Storages,
                    None,
                    &ehttp::Method::GET,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_label_action_stock").to_string(),
                    egui_phosphor::fill::STACK,
                    &Size::Small,
                )
                .clicked()
                {
                    todo!()
                }
            });
        }

        if product_card_shown {
            ui.add_space(20.0);

            ui.vertical(|ui| {
                ui.style_mut().spacing.item_spacing.y = 15.0;

                ui.horizontal(|ui| {
                    ui.label(RichText::new(t!("product_card_product_id")).italics());
                    ui.label(product.product_id.unwrap_or_default().to_string());
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t!("product_card_name")).italics());
                    ui.label(product.name.name_label);
                });
                if let Some(synonyms) = product.synonyms {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(t!("product_card_synonyms")).italics());
                        for synonym in synonyms {
                            ui.label(synonym.name_label);
                            ui.add_space(10.0);
                        }
                    });
                }
                if let Some(specificity) = product.product_specificity {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_product_specificity")).italics());
                        ui.label(specificity);
                    });
                }
                if let Some(empirical_formula) = product.empirical_formula {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_empirical_formula")).italics());
                        ui.label(empirical_formula.empirical_formula_label);
                    });
                }
                if let Some(linear_formula) = product.linear_formula {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_linear_formula")).italics());
                        ui.label(linear_formula.linear_formula_label);
                    });
                }
                if let Some(cas_number) = product.cas_number {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_cas_number")).italics());
                        ui.label(cas_number.cas_number_label);
                    });
                }
                if let Some(ce_number) = product.ce_number {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_ce_number")).italics());
                        ui.label(ce_number.ce_number_label);
                    });
                }
            });
        }
    });
}
