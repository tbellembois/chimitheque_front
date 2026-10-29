use chimitheque_types::product::Product;
use egui::{RichText, Vec2};
use egui_extras::{Column, TableBuilder};
use egui_phosphor_icons::icons;
use elegance::{Badge, BadgeTone, Button, Callout, CalloutTone, Card};
use rust_i18n::t;

use crate::config::{
    APP_CORNER_RADIUS, NORMAL_STROKE, PRODUCT_LABEL_ACTION_WIDTH, PRODUCT_LABEL_INNER_MARGIN,
    PRODUCT_LABEL_OUTER_MARGIN, PRODUCT_LABEL_PLUS_WIDTH, WEAK_TEXT_COLOR,
};
use crate::ui::app::App;
use crate::utils::first_char;

pub fn render_product_label(
    app: &mut App,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    product: Product,
) {
    let product_label_outer_margin = *PRODUCT_LABEL_OUTER_MARGIN.get().unwrap();
    let product_label_inner_margin = *PRODUCT_LABEL_INNER_MARGIN.get().unwrap();
    let product_label_plus_width = *PRODUCT_LABEL_PLUS_WIDTH.get().unwrap();
    let product_label_action_width = *PRODUCT_LABEL_ACTION_WIDTH.get().unwrap();
    let app_corner_radius = *APP_CORNER_RADIUS.get().unwrap();
    let normal_stroke = *NORMAL_STROKE.get().unwrap();
    let weak_text_color = *WEAK_TEXT_COLOR.get().unwrap();

    let label_width = app.state.central_panel_rect.width()
        - (2.0 * product_label_outer_margin.leftf())
        - (2.0 * product_label_inner_margin.leftf());
    let available_space_for_cols = label_width - (2.0 * product_label_inner_margin.leftf());
    let available_space_for_dyn_cols_in_percent =
        (available_space_for_cols - product_label_plus_width - product_label_action_width) / 100.0;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .outer_margin(product_label_outer_margin)
        .inner_margin(product_label_inner_margin)
        .corner_radius(app_corner_radius)
        .stroke(normal_stroke);

    let product_card_shown = app
        .product_state
        .product_cards_shown
        .contains(&product.product_id.unwrap_or_default());

    let product_card_actions_shown = app
        .product_state
        .product_cards_actions_shown
        .contains(&product.product_id.unwrap_or_default());

    Card::new().show(ui, |ui| {
        ui.vertical(|ui| {
            ui.add_sized(
                [ui.available_width(), 0.0],
                egui::Label::new(product.name.clone().name_label)
                    .wrap_mode(egui::TextWrapMode::Truncate),
            );

            egui::Grid::new(format!(
                "product_label_grid_{}",
                product.product_id.unwrap_or_default()
            ))
            .num_columns(3)
            .striped(false)
            .spacing(Vec2 { x: 40.0, y: 0.0 })
            .show(ui, |ui| {
                // Button to toggle the product card shown state.
                if product_card_shown {
                    if ui
                        .add(Button::icon(first_char(&icons::MINUS), String::new()).outline())
                        .clicked()
                    {
                        app.product_state
                            .product_cards_shown
                            .retain(|id| *id != product.product_id.unwrap_or_default());
                    }
                } else if ui
                    .add(Button::icon(first_char(&icons::PLUS), String::new()).outline())
                    .clicked()
                {
                    app.product_state
                        .product_cards_shown
                        .push(product.product_id.unwrap_or_default());
                }

                // Product empirical formula label.
                if let Some(empirical_formula) = product.empirical_formula {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(t!("product_card_empirical_formula")).italics(),
                        );
                        ui.label(empirical_formula.empirical_formula_label);
                    });
                } else {
                    ui.label("");
                }

                // Product cas number label.
                if let Some(cas_number) = product.cas_number {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(t!("product_card_cas_number")).italics());
                        ui.label(cas_number.cas_number_label);
                    });
                } else {
                    ui.label("");
                }

                ui.end_row();

                ui.label("");

                if let Some(product_sc) = product.product_sc {
                    if product_sc > 0 {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(t!("product_label_storages")).italics());
                            ui.label(product_sc.to_string());
                        });
                    } else {
                        ui.label("");
                    }
                } else {
                    ui.label("");
                }

                if let Some(product_sl) = product.product_sl
                    && !product_sl.is_empty()
                {
                    ui.add(Badge::new(product_sl, BadgeTone::Neutral));
                } else {
                    ui.label("");
                }

                ui.end_row();
            });

            //             ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            //                 if app.has_permission(
            //                     &chimitheque_types::permission::PermissionItem::Storages,
            //                     None,
            //                     &ehttp::Method::GET,
            //                     &app.permissions.clone(),
            //                 ) && ui
            //                     .add(
            //                         Button::icon_text(
            //                             icons::PACKAGE
            //                                 .fill()
            //                                 .text()
            //                                 .chars()
            //                                 .next()
            //                                 .unwrap_or_default(),
            //                             t!("product_label_action_storages").to_string(),
            //                         )
            //                         .outline(),
            //                     )
            //                     .clicked()
            //                 {
            //                     todo!()
            //                 }
            //
            //                 if app.has_permission(
            //                     &chimitheque_types::permission::PermissionItem::Storages,
            //                     None,
            //                     &ehttp::Method::POST,
            //                     &app.permissions.clone(),
            //                 ) && ui
            //                     .add(
            //                         Button::icon_text(
            //                             icons::BOX_ARROW_DOWN
            //                                 .fill()
            //                                 .text()
            //                                 .chars()
            //                                 .next()
            //                                 .unwrap_or_default(),
            //                             t!("product_label_action_store").to_string(),
            //                         )
            //                         .outline(),
            //                     )
            //                     .clicked()
            //                 {
            //                     todo!()
            //                 }
            //
            //                 if app.has_permission(
            //                     &chimitheque_types::permission::PermissionItem::Products,
            //                     None,
            //                     &ehttp::Method::POST,
            //                     &app.permissions.clone(),
            //                 ) && ui
            //                     .add(
            //                         Button::icon_text(
            //                             icons::PENCIL
            //                                 .fill()
            //                                 .text()
            //                                 .chars()
            //                                 .next()
            //                                 .unwrap_or_default(),
            //                             t!("product_label_action_edit").to_string(),
            //                         )
            //                         .outline(),
            //                     )
            //                     .clicked()
            //                 {
            //                     todo!()
            //                 }
            //
            //                 if app.has_permission(
            //                     &chimitheque_types::permission::PermissionItem::Products,
            //                     None,
            //                     &ehttp::Method::POST,
            //                     &app.permissions.clone(),
            //                 ) && ui
            //                     .add(
            //                         Button::icon_text(
            //                             icons::BOOKMARK
            //                                 .fill()
            //                                 .text()
            //                                 .chars()
            //                                 .next()
            //                                 .unwrap_or_default(),
            //                             t!("product_label_action_bookmark").to_string(),
            //                         )
            //                         .outline(),
            //                     )
            //                     .clicked()
            //                 {
            //                     todo!()
            //                 }
            //
            //                 if app.has_permission(
            //                     &chimitheque_types::permission::PermissionItem::Storages,
            //                     None,
            //                     &ehttp::Method::GET,
            //                     &app.permissions.clone(),
            //                 ) && ui
            //                     .add(
            //                         Button::icon_text(
            //                             icons::STACK
            //                                 .fill()
            //                                 .text()
            //                                 .chars()
            //                                 .next()
            //                                 .unwrap_or_default(),
            //                             t!("product_label_action_stock").to_string(),
            //                         )
            //                         .outline(),
            //                     )
            //                     .clicked()
            //                 {
            //                     todo!()
            //                 }
            //             });

            if product_card_shown {
                // ui.style_mut().spacing.item_spacing.y = 15.0;

                ui.horizontal(|ui| {
                    ui.label(RichText::new(t!("product_card_product_id")).italics());
                    ui.label(product.product_id.unwrap_or_default().to_string());
                });
                // ui.horizontal(|ui| {
                //     ui.label(RichText::new(t!("product_card_name")).italics());
                //     ui.label(product.name.name_label);
                // });
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
                    if !specificity.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(t!("product_card_product_specificity")).italics(),
                            );
                            ui.label(specificity);
                        });
                    }
                }
                // if let Some(empirical_formula) = product.empirical_formula {
                //     ui.horizontal(|ui| {
                //         ui.label(RichText::new(t!("product_card_empirical_formula")).italics());
                //         ui.label(empirical_formula.empirical_formula_label);
                //     });
                // }
                if let Some(linear_formula) = product.linear_formula {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_linear_formula")).italics());
                        ui.label(linear_formula.linear_formula_label);
                    });
                }
                // if let Some(cas_number) = product.cas_number {
                //     ui.horizontal(|ui| {
                //         ui.label(RichText::new(t!("product_card_cas_number")).italics());
                //         ui.label(cas_number.cas_number_label);
                //     });
                // }
                if let Some(ce_number) = product.ce_number {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(t!("product_card_ce_number")).italics());
                        ui.label(ce_number.ce_number_label);
                    });
                }
            }
        });
    });

    //     custom_group_frame.show(ui, |ui| {
    //         TableBuilder::new(ui)
    //             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
    //             .column(Column::auto())
    //             .column(Column::remainder())
    //             .column(Column::auto())
    //             .column(Column::auto())
    //             .column(Column::auto())
    //             .id_salt(format!(
    //                 "{}_{}_product_label",
    //                 product.product_id.unwrap_or_default(),
    //                 product.name.name_label
    //             ))
    //             .body(|mut body| {
    //                 body.row(50.0, |mut row| {
    //                     row.col(|ui| {
    //                         if product_card_shown {
    //                             if ui
    //                                 .add(
    //                                     Button::icon(
    //                                         icons::MINUS
    //                                             .fill()
    //                                             .text()
    //                                             .chars()
    //                                             .next()
    //                                             .unwrap_or_default(),
    //                                         "".to_string(),
    //                                     )
    //                                     .outline(),
    //                                 )
    //                                 .clicked()
    //                             {
    //                                 app.product_state
    //                                     .product_cards_shown
    //                                     .retain(|id| *id != product.product_id.unwrap_or_default());
    //                             }
    //                         } else if ui
    //                             .add(
    //                                 Button::icon(
    //                                     icons::PLUS.fill().text().chars().next().unwrap_or_default(),
    //                                     "".to_string(),
    //                                 )
    //                                 .outline(),
    //                             )
    //                             .clicked()
    //                         {
    //                             app.product_state
    //                                 .product_cards_shown
    //                                 .push(product.product_id.unwrap_or_default());
    //                         }
    //                     });
    //
    //                     row.col(|ui| {
    //                         ui.label(product.name.clone().name_label);
    //
    //                         if let Some(product_sl) = product.product_sl {
    //                             ui.label(icons::DRESSER.regular());
    //                             ui.label(product_sl);
    //                         }
    //
    //                         if let Some(product_sc) = product.product_sc {
    //                             ui.label(
    //                                 RichText::new(format!(
    //                                     "[ {}: {product_sc} ]",
    //                                     t!("product_label_storages")
    //                                 ))
    //                                 .color(weak_text_color),
    //                             );
    //                         }
    //                     });
    //
    //                     row.col(|ui| {
    //                         ui.label(
    //                             product
    //                                 .empirical_formula
    //                                 .clone()
    //                                 .unwrap_or(EmpiricalFormula {
    //                                     empirical_formula_label: String::new(),
    //                                     ..Default::default()
    //                                 })
    //                                 .empirical_formula_label,
    //                         );
    //                     });
    //
    //                     row.col(|ui| {
    //                         ui.label(
    //                             product
    //                                 .cas_number
    //                                 .clone()
    //                                 .unwrap_or(CasNumber {
    //                                     cas_number_label: String::new(),
    //                                     ..Default::default()
    //                                 })
    //                                 .cas_number_label,
    //                         );
    //
    //                         // Show CAS number CMR category.
    //                         if let Some(cas_number) = product.cas_number.clone()
    //                             && let Some(cas_number_cmr) = cas_number.cas_number_cmr
    //                         {
    //                             ui.label(cas_number_cmr);
    //                         }
    //
    //                         // Show inflammable symbol.
    //                         if let Some(hs_cmr) = product.product_hs_cmr {
    //                             ui.label(hs_cmr);
    //                         }
    //
    //                         if let Some(symbols) = product.symbols {
    //                             for symbol in symbols {
    //                                 if symbol.symbol_label == "GHS02" {
    //                                     ui.add(egui::Image::new("file://assets/GHS02.svg"));
    //                                 }
    //                             }
    //                         }
    //                     });
    //
    //                     row.col(|ui| {
    //                         if product_card_actions_shown {
    //                             if ui
    //                                 .add(
    //                                     Button::icon(
    //                                         icons::DOTS_THREE
    //                                             .fill()
    //                                             .text()
    //                                             .chars()
    //                                             .next()
    //                                             .unwrap_or_default(),
    //                                         "".to_string(),
    //                                     )
    //                                     .outline(),
    //                                 )
    //                                 .clicked()
    //                             {
    //                                 app.product_state
    //                                     .product_cards_actions_shown
    //                                     .retain(|id| *id != product.product_id.unwrap_or_default());
    //                             }
    //                         } else if ui
    //                             .add(
    //                                 Button::icon(
    //                                     icons::DOTS_THREE
    //                                         .fill()
    //                                         .text()
    //                                         .chars()
    //                                         .next()
    //                                         .unwrap_or_default(),
    //                                     "".to_string(),
    //                                 )
    //                                 .outline(),
    //                             )
    //                             .clicked()
    //                         {
    //                             app.product_state
    //                                 .product_cards_actions_shown
    //                                 .push(product.product_id.unwrap_or_default());
    //                         }
    //                     });
    //                 });
    //             });
    //

    //     });
}
