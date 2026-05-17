use crate::ui::app::App;
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    ui.vertical(|ui| {
        if let Ok(maybe_store_locations_and_count) = app.get_store_locations_and_count()
            && let Some((store_locations, count)) = maybe_store_locations_and_count
        {
            ui.label(t!("total", total = count));

            let available_height = ui.available_height();
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong(t!("storelocation_name"));
                    });
                    header.col(|ui| {
                        ui.strong(t!("storelocation_entity"));
                    });
                    header.col(|ui| {
                        ui.strong(t!("storelocation_color"));
                    });
                    header.col(|ui| {
                        ui.strong(t!("storelocation_canstore"));
                    });
                    header.col(|ui| {
                        ui.strong(t!("storelocation_parent"));
                    });
                })
                .body(|mut body| {
                    for store_location in store_locations {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(store_location.store_location_name.clone());
                            });

                            row.col(|ui| {
                                if let Some(entity) = &store_location.entity {
                                    ui.label(entity.entity_name.clone());
                                }
                            });

                            row.col(|ui| {
                                if let Some(color) = &store_location.store_location_color {
                                    ui.label(color.clone());
                                }
                            });

                            row.col(|ui| {
                                if store_location.store_location_can_store {
                                    ui.label("ok");
                                }
                            });

                            row.col(|ui| {
                                if let Some(parent) = &store_location.store_location {
                                    ui.label(parent.store_location_name.clone());
                                }
                            });

                            row.col(|ui| {
                                ui.label("todo");
                            });
                        });
                    }
                });
        }
    });
}
