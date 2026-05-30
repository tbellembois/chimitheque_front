use crate::{
    types::{GenericOrder, StoreLocationsOrderBy},
    ui::{
        app::App,
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

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 32.0),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            if app.has_permission(
                &chimitheque_types::permission::PermissionItem::StoreLocations,
                None,
                &ehttp::Method::POST,
                &app.permissions.clone(),
            ) && button_with_icon_and_text(
                ui,
                t!("storelocation_create").to_string(),
                egui_phosphor::fill::MAGIC_WAND,
                &Size::Medium,
            )
            .clicked()
            {}
        },
    );

    ui.add_space(20.0);

    ui.vertical(|ui| {
        if let Ok(maybe_store_locations_and_count) = app.get_store_locations_and_count()
            && let Some((store_locations, count)) = maybe_store_locations_and_count
        {
            let list_rec = app.state.search_rect;

            ui.scope_builder(egui::UiBuilder::new().max_rect(list_rec), |ui| {
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
                        app.state.action.push_back(Action::GetStorelocations);
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

                        app.state.action.push_back(Action::GetStorelocations);
                    }

                    ctx.request_repaint();
                });

                ui.add_space(20.0);

                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(list_rec.width() * 25.0 / 100.0))
                    .column(Column::exact(list_rec.width() * 50.0 / 100.0))
                    .column(Column::exact(list_rec.width() * 20.0 / 100.0))
                    .column(Column::exact(list_rec.width() * 5.0 / 100.0))
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(60.0, |mut header| {
                        header.col(|ui| {
                            if ui.link(t!("storelocation_parent")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Parent;
                                app.store_locations_order = match app.store_locations_order {
                                    GenericOrder::Asc => GenericOrder::Desc,
                                    GenericOrder::Desc => GenericOrder::Asc,
                                };

                                app.state.action.push_back(Action::GetStorelocations);
                            }
                        });
                        header.col(|ui| {
                            if ui.link(t!("storelocation_name")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Name;
                                app.store_locations_order = match app.store_locations_order {
                                    GenericOrder::Asc => GenericOrder::Desc,
                                    GenericOrder::Desc => GenericOrder::Asc,
                                };

                                app.state.action.push_back(Action::GetStorelocations);
                            }
                        });
                        header.col(|ui| {
                            if ui.link(t!("storelocation_entity")).clicked() {
                                app.store_locations_order_by = StoreLocationsOrderBy::Entity;
                                app.store_locations_order = match app.store_locations_order {
                                    GenericOrder::Asc => GenericOrder::Desc,
                                    GenericOrder::Desc => GenericOrder::Asc,
                                };

                                app.state.action.push_back(Action::GetStorelocations);
                            }
                        });

                        header.col(|_ui| {
                            // actions
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
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(store_location.store_location_name.clone());

                                            if let Some(color) =
                                                &store_location.store_location_color
                                            {
                                                let color =
                                                    html_color_to_egui(color).unwrap_or_default();
                                                ui.label(
                                                    RichText::new(
                                                        egui_phosphor::fill::PAINT_BUCKET,
                                                    )
                                                    .color(color),
                                                );
                                            }
                                        });

                                        ui.horizontal(|ui| {
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

                                            if !store_location.store_location_can_store {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{}{}",
                                                        egui_phosphor::fill::X,
                                                        t!("storelocation_cannotstore"),
                                                    ))
                                                    .font(FontId {
                                                        family: FontFamily::Name("phosphor".into()),
                                                        size: 20.0,
                                                    }),
                                                );
                                            }
                                        });
                                    });
                                });

                                row.col(|ui| {
                                    if let Some(entity) = &store_location.entity {
                                        ui.label(entity.entity_name.clone());
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
