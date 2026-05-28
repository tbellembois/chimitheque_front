use chimitheque_types::casnumber::CasNumber;
use chimitheque_types::empiricalformula::EmpiricalFormula;
use chimitheque_types::storage::Storage;
use egui::{FontFamily, FontId, RichText, TextBuffer};
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

use crate::ui::app::App;
use crate::ui::widgets::buttonwithicon::button_with_icon;
use crate::ui::widgets::buttonwithiconandtext::button_with_icon_and_text;
use crate::ui::widgets::size::Size;

const PRODUCT_LABEL_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 10);
const PRODUCT_LABEL_PLUS_WIDTH: f32 = 50.0;
const PRODUCT_LABEL_ACTIONS_WIDTH: f32 = 50.0;
const PRODUCT_LABEL_CORNER_RADIUS: f32 = 8.0;

pub fn render_storage_label(
    app: &mut App,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    storage: Storage,
) {
    let widgets = &ui.visuals().widgets;
    let stroke = widgets.noninteractive.bg_stroke;

    let label_width = app.state.search_rect.width();
    let window_width = app.state.window_rect.width();
    let storage_label_outer_x_margin = if ((window_width - label_width) / 2.0) >= 127.0 {
        ((window_width - label_width) / 2.0) as i8
    } else {
        10
    };
    let storage_label_outer_y_margin = 5;

    let storage_label_outer_margin: egui::Margin =
        egui::Margin::symmetric(storage_label_outer_x_margin, storage_label_outer_y_margin);

    let cols_width_percent =
        (label_width - PRODUCT_LABEL_ACTIONS_WIDTH - PRODUCT_LABEL_PLUS_WIDTH) / 100.0;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .inner_margin(PRODUCT_LABEL_INNER_MARGIN)
        .outer_margin(storage_label_outer_margin)
        .corner_radius(PRODUCT_LABEL_CORNER_RADIUS)
        .stroke(egui::Stroke::new(1.0, stroke.color));

    let storage_card_shown = app
        .storage_cards_shown
        .contains(&storage.storage_id.unwrap_or_default());

    let storage_card_actions_shown = app
        .storage_cards_actions_shown
        .contains(&storage.storage_id.unwrap_or_default());

    let hint_color = ui.visuals().weak_text_color.unwrap_or_else(|| {
        let text_color = ui.visuals().text_color();
        text_color.gamma_multiply(ui.visuals().weak_text_alpha)
    });

    custom_group_frame.show(ui, |ui| {
        TableBuilder::new(ui)
            .column(Column::exact(PRODUCT_LABEL_PLUS_WIDTH))
            .column(Column::exact(cols_width_percent * 40.0))
            .column(Column::exact(cols_width_percent * 40.0))
            .column(Column::exact(cols_width_percent * 10.0))
            .column(Column::exact(cols_width_percent * 10.0))
            .column(Column::exact(PRODUCT_LABEL_ACTIONS_WIDTH))
            .id_salt(format!(
                "{}_storage_label",
                storage.storage_id.unwrap_or_default()
            ))
            .body(|mut body| {
                body.row(70.0, |mut row| {
                    row.col(|ui| {
                        if storage_card_shown {
                            if button_with_icon(ui, egui_phosphor::regular::MINUS).clicked() {
                                app.storage_cards_shown
                                    .retain(|id| *id != storage.storage_id.unwrap_or_default());
                            }
                        } else if button_with_icon(ui, egui_phosphor::regular::PLUS).clicked() {
                            app.storage_cards_shown
                                .push(storage.storage_id.unwrap_or_default());
                        }
                    });

                    row.col(|ui| {
                        ui.label(storage.product.name.name_label);

                        ui.horizontal(|ui| {
                            if let Some(storage_batch_number) = storage.storage_batch_number
                                && !storage_batch_number.is_empty()
                            {
                                ui.label(
                                    RichText::new(format!(
                                        "{}: ",
                                        t!("storage_label_batch_number")
                                    ))
                                    .italics(),
                                );
                                ui.label(RichText::new(storage_batch_number));
                            }
                        });

                        ui.horizontal(|ui| {
                            if let Some(storage_barecode) = storage.storage_barecode
                                && !storage_barecode.is_empty()
                            {
                                ui.label(
                                    RichText::new(format!("{}: ", t!("storage_label_barecode")))
                                        .italics(),
                                );
                                ui.label(RichText::new(storage_barecode));
                            }
                        });
                    });

                    row.col(|ui| {
                        ui.label(storage.store_location.store_location_name);
                    });

                    row.col(|ui| {
                        if let Some(quantity) = storage.storage_quantity {
                            ui.label(format!(
                                "{} {}",
                                quantity,
                                storage.unit_quantity.unwrap_or_default().unit_label
                            ));
                        }
                    });

                    row.col(|ui| {
                        ui.label(
                            storage
                                .storage_modification_date
                                .format(t!("date_format").as_str())
                                .to_string(),
                        );
                    });

                    row.col(|ui| {
                        if storage_card_actions_shown {
                            if button_with_icon(ui, egui_phosphor::regular::DOTS_THREE_CIRCLE)
                                .clicked()
                            {
                                app.storage_cards_actions_shown
                                    .retain(|id| *id != storage.storage_id.unwrap_or_default());
                            }
                        } else if button_with_icon(
                            ui,
                            egui_phosphor::regular::DOTS_THREE_CIRCLE_VERTICAL,
                        )
                        .clicked()
                        {
                            app.storage_cards_actions_shown
                                .push(storage.storage_id.unwrap_or_default());
                        }
                    });
                });
            });

        // if storage_card_actions_shown {
        //     ui.add_space(20.0);

        //     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        //         if app.has_permission(
        //             &chimitheque_types::permission::PermissionItem::Storages,
        //             None,
        //             &ehttp::Method::GET,
        //             &app.permissions.clone(),
        //         ) && button_with_icon_and_text(
        //             ui,
        //             t!("storage_label_action_storages").to_string(),
        //             egui_phosphor::fill::PACKAGE,
        //             &Size::Small,
        //         )
        //         .clicked()
        //         {
        //             todo!()
        //         }

        //         if app.has_permission(
        //             &chimitheque_types::permission::PermissionItem::Storages,
        //             None,
        //             &ehttp::Method::POST,
        //             &app.permissions.clone(),
        //         ) && button_with_icon_and_text(
        //             ui,
        //             t!("storage_label_action_store").to_string(),
        //             egui_phosphor::fill::BOX_ARROW_DOWN,
        //             &Size::Small,
        //         )
        //         .clicked()
        //         {
        //             todo!()
        //         }

        //         if app.has_permission(
        //             &chimitheque_types::permission::PermissionItem::Storages,
        //             None,
        //             &ehttp::Method::POST,
        //             &app.permissions.clone(),
        //         ) && button_with_icon_and_text(
        //             ui,
        //             t!("storage_label_action_edit").to_string(),
        //             egui_phosphor::fill::PENCIL,
        //             &Size::Small,
        //         )
        //         .clicked()
        //         {
        //             todo!()
        //         }

        //         if app.has_permission(
        //             &chimitheque_types::permission::PermissionItem::Storages,
        //             None,
        //             &ehttp::Method::POST,
        //             &app.permissions.clone(),
        //         ) && button_with_icon_and_text(
        //             ui,
        //             t!("storage_label_action_bookmark").to_string(),
        //             egui_phosphor::fill::BOOKMARK,
        //             &Size::Small,
        //         )
        //         .clicked()
        //         {
        //             todo!()
        //         }

        //         if app.has_permission(
        //             &chimitheque_types::permission::PermissionItem::Storages,
        //             None,
        //             &ehttp::Method::GET,
        //             &app.permissions.clone(),
        //         ) && button_with_icon_and_text(
        //             ui,
        //             t!("storage_label_action_stock").to_string(),
        //             egui_phosphor::fill::STACK,
        //             &Size::Small,
        //         )
        //         .clicked()
        //         {
        //             todo!()
        //         }
        //     });
        // }

        // if storage_card_shown {
        //     ui.add_space(20.0);

        //     ui.vertical(|ui| {
        //         ui.style_mut().spacing.item_spacing.y = 15.0;

        //         ui.horizontal(|ui| {
        //             ui.label(RichText::new(t!("storage_card_storage_id")).italics());
        //             ui.label(storage.storage_id.unwrap_or_default().to_string());
        //         });
        //         ui.horizontal(|ui| {
        //             ui.label(RichText::new(t!("storage_card_name")).italics());
        //             ui.label(storage.name.name_label);
        //         });
        //         if let Some(synonyms) = storage.synonyms {
        //             ui.horizontal_wrapped(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_synonyms")).italics());
        //                 for synonym in synonyms {
        //                     ui.label(synonym.name_label);
        //                     ui.add_space(10.0);
        //                 }
        //             });
        //         }
        //         if let Some(specificity) = storage.storage_specificity {
        //             ui.horizontal(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_storage_specificity")).italics());
        //                 ui.label(specificity);
        //             });
        //         }
        //         if let Some(empirical_formula) = storage.empirical_formula {
        //             ui.horizontal(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_empirical_formula")).italics());
        //                 ui.label(empirical_formula.empirical_formula_label);
        //             });
        //         }
        //         if let Some(linear_formula) = storage.linear_formula {
        //             ui.horizontal(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_linear_formula")).italics());
        //                 ui.label(linear_formula.linear_formula_label);
        //             });
        //         }
        //         if let Some(cas_number) = storage.cas_number {
        //             ui.horizontal(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_cas_number")).italics());
        //                 ui.label(cas_number.cas_number_label);
        //             });
        //         }
        //         if let Some(ce_number) = storage.ce_number {
        //             ui.horizontal(|ui| {
        //                 ui.label(RichText::new(t!("storage_card_ce_number")).italics());
        //                 ui.label(ce_number.ce_number_label);
        //             });
        //         }
        //     });

        //     // let card_available_width =
        //     //     app.state.window_rect.width() - PAGE_RIGHT_MARGIN - PAGE_LEFT_MARGIN;

        //     // let cols_width = (card_available_width / 6.0);

        //     // TableBuilder::new(ui)
        //     //     .column(Column::exact(cols_width))
        //     //     .column(Column::exact(cols_width))
        //     //     .column(Column::exact(cols_width))
        //     //     .column(Column::exact(cols_width))
        //     //     .column(Column::exact(cols_width))
        //     //     .column(Column::exact(cols_width))
        //     //     .id_salt(format!("{}_storage_card", storage.name.name_label))
        //     //     .body(|mut body| {
        //     //         body.row(50.0, |mut row| {
        //     //             row.col(|ui| {
        //     //                 ui.horizontal(|ui| {
        //     //                     ui.label(RichText::new(t!("storage_card_storage_id")).italics());
        //     //                     ui.label(storage.storage_id.unwrap_or_default().to_string());
        //     //                 });
        //     //             });
        //     //             row.col(|ui| {
        //     //                 ui.horizontal(|ui| {
        //     //                     ui.label(RichText::new(t!("storage_card_name")).italics());
        //     //                     ui.label(storage.name.name_label);
        //     //                 });
        //     //             });
        //     //             row.col(|ui| {
        //     //                 ui.horizontal_wrapped(|ui| {
        //     //                     if let Some(synonyms) = storage.synonyms {
        //     //                         ui.label(RichText::new(t!("storage_card_synonyms")).italics());
        //     //                         for synonym in synonyms {
        //     //                             ui.label(synonym.name_label);
        //     //                         }
        //     //                     }
        //     //                 });
        //     //             });

        //     //             row.col(|ui| {
        //     //                 ui.horizontal(|ui| {
        //     //                     if let Some(empirical_formula) = storage.empirical_formula {
        //     //                         ui.label(
        //     //                             RichText::new(t!("storage_card_empirical_formula"))
        //     //                                 .italics(),
        //     //                         );
        //     //                         ui.label(empirical_formula.empirical_formula_label);
        //     //                     }
        //     //                 });
        //     //             });
        //     //             row.col(|ui| {
        //     //                 ui.horizontal(|ui| {
        //     //                     if let Some(cas_number) = storage.cas_number {
        //     //                         ui.label(
        //     //                             RichText::new(t!("storage_card_cas_number")).italics(),
        //     //                         );
        //     //                         ui.label(cas_number.cas_number_label);
        //     //                     }
        //     //                 });
        //     //             });
        //     //             row.col(|ui| {
        //     //                 ui.horizontal(|ui| {
        //     //                     if let Some(ce_number) = storage.ce_number {
        //     //                         ui.label(RichText::new(t!("storage_card_ce_number")).italics());
        //     //                         ui.label(ce_number.ce_number_label);
        //     //                     }
        //     //                 });
        //     //             });
        //     //         });
        //     //     });
        // }
    });
}
