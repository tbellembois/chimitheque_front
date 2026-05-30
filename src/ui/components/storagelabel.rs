use chimitheque_types::storage::Storage;
use egui::{RichText, TextBuffer};
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

use crate::ui::app::App;
use crate::ui::widgets::buttonwithicon::button_with_icon;

const STORAGE_LABEL_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 10);
const STORAGE_LABEL_PLUS_WIDTH: f32 = 50.0;
const STORAGE_LABEL_ACTIONS_WIDTH: f32 = 50.0;
const STORAGE_LABEL_CORNER_RADIUS: f32 = 8.0;

pub fn render_storage_label(
    app: &mut App,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    storage: Storage,
) {
    let widgets = &ui.visuals().widgets;
    let stroke = widgets.noninteractive.bg_stroke;

    let window_available_width = app.state.window_available_rect.width();

    let storage_label_outer_margin = if window_available_width > 1024.0 {
        egui::Margin::symmetric(40, 5)
    } else {
        egui::Margin::symmetric(5, 5)
    };

    let label_width = window_available_width - (2.0 * storage_label_outer_margin.leftf());
    let available_space_for_cols = label_width - (2.0 * STORAGE_LABEL_INNER_MARGIN.leftf());
    let available_space_for_dyn_cols =
        available_space_for_cols - STORAGE_LABEL_PLUS_WIDTH - STORAGE_LABEL_ACTIONS_WIDTH;
    let available_space_for_dyn_cols_in_percent = available_space_for_dyn_cols / 100.0;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .inner_margin(STORAGE_LABEL_INNER_MARGIN)
        .outer_margin(storage_label_outer_margin)
        .corner_radius(STORAGE_LABEL_CORNER_RADIUS)
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
            .column(Column::exact(STORAGE_LABEL_PLUS_WIDTH))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 40.0,
            ))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 40.0,
            ))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 10.0,
            ))
            .column(Column::exact(
                available_space_for_dyn_cols_in_percent * 10.0,
            ))
            .column(Column::exact(STORAGE_LABEL_ACTIONS_WIDTH))
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
                        ui.horizontal(|ui| {
                            ui.label(storage.product.name.name_label);

                            if storage.storage_archive {
                                ui.label(
                                    RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::fill::ARCHIVE,
                                        t!("storage_card_archive")
                                    ))
                                    .color(hint_color),
                                );
                            }
                        });

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
    });
}
