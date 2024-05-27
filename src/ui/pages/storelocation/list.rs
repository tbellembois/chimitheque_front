use crate::ui::app::App;
use chimitheque_types::storelocation;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

pub fn update(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut Ui) {
    ui.vertical(|ui| {
        if let Some((storelocations, count)) = &app.storelocations {
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
                    for storelocation in storelocations.iter() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(storelocation.storelocation_name.clone());
                            });

                            row.col(|ui| {
                                if let Some(entity) = &storelocation.entity {
                                    ui.label(entity.entity_name.clone());
                                }
                            });

                            row.col(|ui| {
                                if let Some(color) = &storelocation.storelocation_color {
                                    ui.label(color.clone());
                                }
                            });

                            row.col(|ui| {
                                if storelocation.storelocation_canstore {
                                    ui.label("");
                                }
                            });

                            row.col(|ui| {
                                if let Some(parent) = &storelocation.storelocation {
                                    ui.label(parent.storelocation_name.clone());
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
