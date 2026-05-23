use crate::{
    ui::{
        app::{App, StoreLocationsOrder, StoreLocationsOrderBy},
        state::Action,
        widgets::{buttonwithiconandtext::button_with_icon_and_text, size::Size},
    },
    utils::html_color_to_egui,
};
use egui::{FontFamily, FontId, RichText};
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let hint_color = ui.visuals().weak_text_color.unwrap_or_else(|| {
        let text_color = ui.visuals().text_color();
        text_color.gamma_multiply(ui.visuals().weak_text_alpha)
    });

    ui.vertical(|ui| {
        if let Ok(maybe_store_locations_and_count) = app.get_store_locations_and_count()
            && let Some((store_locations, count)) = maybe_store_locations_and_count
        {
            const MENU_HEIGHT: f32 = 140.0; // TODO: We could make this dynamic based on the menu's actual height.
            const SEARCH_FORM_SIDE_MARGIN: f32 = 400.0;
            const SEARCH_FORM_HEIGHT: f32 = 10.0; // Random value, only used space will be allocated.

            // Calculate search form size and position (ie. rect).
            let search_form_top_left =
                app.state.window_rect.left_top() + egui::vec2(SEARCH_FORM_SIDE_MARGIN, MENU_HEIGHT);
            let search_form_bottom_right = app.state.window_rect.right_bottom()
                - egui::vec2(SEARCH_FORM_SIDE_MARGIN, SEARCH_FORM_HEIGHT);
            let search_form_rec =
                egui::Rect::from_two_pos(search_form_top_left, search_form_bottom_right);

            ui.scope_builder(egui::UiBuilder::new().max_rect(search_form_rec), |ui| {
                ui.horizontal(|ui| {
                    ui.label(t!("total", total = count));

                    ui.add_space(20.0);

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut app.search_store_location)
                            .hint_text(t!("storelocation_search_hint")),
                    );
                    if button_with_icon_and_text(
                        ui,
                        t!("search_form_action_reset_filter").to_string(),
                        egui_phosphor::fill::ERASER,
                        &Size::Small,
                    )
                    .clicked()
                    {
                        app.search_store_location = String::new();
                        app.state.action = Action::GetStorelocations;
                    }

                    let ctx = ui.ctx();
                    let now = ctx.input(|i| i.time);

                    // Detect changes.
                    if response.changed() {
                        app.search_store_location_last_edit = now;
                        app.search_store_location_action_triggered = false;
                    }

                    // Debounce logic.
                    if !app.search_store_location_action_triggered
                        && (now - app.search_store_location_last_edit) >= 0.5
                    {
                        app.search_store_location_action_triggered = true;

                        app.state.action = Action::GetStorelocations;
                    }

                    ctx.request_repaint();
                });

                ui.add_space(20.0);

                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::exact(100.0))
                    .column(Column::exact(100.0))
                    .column(Column::auto())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(60.0, |mut header| {
                        header.col(|ui| {
                            if ui.link(t!("storelocation_parent")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Parent;
                                app.store_locations_order = match app.store_locations_order {
                                    StoreLocationsOrder::Asc => StoreLocationsOrder::Desc,
                                    StoreLocationsOrder::Desc => StoreLocationsOrder::Asc,
                                };

                                app.state.action = Action::GetStorelocations;
                            }
                        });
                        header.col(|ui| {
                            if ui.link(t!("storelocation_name")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Name;
                                app.store_locations_order = match app.store_locations_order {
                                    StoreLocationsOrder::Asc => StoreLocationsOrder::Desc,
                                    StoreLocationsOrder::Desc => StoreLocationsOrder::Asc,
                                };

                                app.state.action = Action::GetStorelocations;
                            }
                        });
                        header.col(|ui| {
                            if ui.link(t!("storelocation_entity")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Entity;
                                app.store_locations_order = match app.store_locations_order {
                                    StoreLocationsOrder::Asc => StoreLocationsOrder::Desc,
                                    StoreLocationsOrder::Desc => StoreLocationsOrder::Asc,
                                };

                                app.state.action = Action::GetStorelocations;
                            }
                        });

                        header.col(|_ui| {
                            // ui.strong(t!("storelocation_color"));
                        });
                        header.col(|ui| {
                            ui.strong(t!("storelocation_canstore"));
                        });
                    })
                    .body(|mut body| {
                        for store_location in store_locations {
                            body.row(80.0, |mut row| {
                                row.col(|ui| {
                                    if let Some(parent) = &store_location.store_location {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(parent.store_location_name.clone());
                                        });
                                    }
                                });

                                row.col(|ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(store_location.store_location_name.clone());

                                        if let Some(nb_storages) =
                                            store_location.store_location_nb_storages
                                            && nb_storages > 0
                                        {
                                            ui.label(
                                                RichText::new(format!(
                                                    "[ {}: {nb_storages} ]",
                                                    t!("storelocation_storages")
                                                ))
                                                .color(hint_color),
                                            );
                                        }

                                        ui.add_space(5.0);

                                        if let Some(nb_children) =
                                            store_location.store_location_nb_children
                                            && nb_children > 0
                                        {
                                            ui.label(
                                                RichText::new(format!(
                                                    "[ {}: {nb_children} ]",
                                                    t!("storelocation_children")
                                                ))
                                                .color(hint_color),
                                            );
                                        }
                                    });
                                });

                                row.col(|ui| {
                                    if let Some(entity) = &store_location.entity {
                                        ui.label(entity.entity_name.clone());
                                    }
                                });

                                row.col(|ui| {
                                    if let Some(color) = &store_location.store_location_color {
                                        let color = html_color_to_egui(color).unwrap_or_default();
                                        ui.label(
                                            RichText::new(egui_phosphor::fill::PAINT_BUCKET)
                                                .color(color),
                                        );
                                    }
                                });

                                row.col(|ui| {
                                    if store_location.store_location_can_store {
                                        ui.label(RichText::new(egui_phosphor::fill::CHECK).font(
                                            FontId {
                                                family: FontFamily::Name("phosphor".into()),
                                                size: 20.0,
                                            },
                                        ));
                                    } else {
                                        ui.label(RichText::new(egui_phosphor::fill::X).font(
                                            FontId {
                                                family: FontFamily::Name("phosphor".into()),
                                                size: 20.0,
                                            },
                                        ));
                                    }
                                });

                                row.col(|ui| {
                                    ui.label("todo");
                                });
                            });
                        }
                    });
            });
        }
    });
}
