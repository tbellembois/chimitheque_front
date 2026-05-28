use crate::{
    types::EntitiesOrder,
    ui::{
        app::App,
        state::Action,
        widgets::{buttonwithiconandtext::button_with_icon_and_text, size::Size},
    },
};
use egui::RichText;
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
                &chimitheque_types::permission::PermissionItem::Entities,
                None,
                &ehttp::Method::POST,
                &app.permissions.clone(),
            ) && button_with_icon_and_text(
                ui,
                t!("entity_create").to_string(),
                egui_phosphor::fill::MAGIC_WAND,
                &Size::Medium,
            )
            .clicked()
            {}
        },
    );

    ui.vertical(|ui| {
        if let Ok(maybe_entities_and_count) = app.get_entities_and_count()
            && let Some((entities, count)) = maybe_entities_and_count
        {
            let list_rec = app.state.search_rect;

            ui.scope_builder(egui::UiBuilder::new().max_rect(list_rec), |ui| {
                ui.horizontal(|ui| {
                    ui.label(t!("total", total = count));

                    ui.add_space(20.0);

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut app.search_entity)
                            .hint_text(t!("entity_search_hint")),
                    );
                    if button_with_icon_and_text(
                        ui,
                        t!("search_form_action_reset_filter").to_string(),
                        egui_phosphor::fill::ERASER,
                        &Size::Small,
                    )
                    .clicked()
                    {
                        app.search_entity = String::new();
                        app.state.action.push_back(Action::GetEntities);
                    }

                    let ctx = ui.ctx();
                    let now = ctx.input(|i| i.time);

                    // Detect changes.
                    if response.changed() {
                        app.search_entity_last_edit = now;
                        app.search_entity_action_triggered = false;
                    }

                    // Debounce logic.
                    if !app.search_entity_action_triggered
                        && (now - app.search_entity_last_edit) >= 0.5
                    {
                        app.search_entity_action_triggered = true;

                        app.state.action.push_back(Action::GetEntities);
                    }

                    ctx.request_repaint();
                });

                ui.add_space(20.0);

                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(list_rec.width() * 90.0 / 100.0))
                    .column(Column::exact(list_rec.width() * 10.0 / 100.0))
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                table
                    .header(60.0, |mut header| {
                        header.col(|ui| {
                            if ui.link(t!("entity_name")).clicked() {
                                app.entities_order = match app.entities_order {
                                    EntitiesOrder::Asc => EntitiesOrder::Desc,
                                    EntitiesOrder::Desc => EntitiesOrder::Asc,
                                };

                                app.state.action.push_back(Action::GetEntities);
                            }
                        });
                        // header.col(|ui| {
                        //     ui.label(t!("entity_managers"));
                        // });
                        header.col(|_ui| {
                            // actions
                        });
                    })
                    .body(|mut body| {
                        for entity in entities {
                            body.row(100.0, |mut row| {
                                row.col(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.label(entity.entity_name.clone());

                                            if let Some(description) = entity.entity_description
                                                && !description.is_empty()
                                            {
                                                ui.label(
                                                    egui::RichText::new(format!(
                                                        "( {} )",
                                                        description.clone()
                                                    ))
                                                    .italics(),
                                                );
                                            }

                                            if let Some(nb_people) = entity.entity_nb_people
                                                && nb_people > 0
                                            {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "[ {}: {nb_people} ]",
                                                        t!("entity_people")
                                                    ))
                                                    .color(hint_color),
                                                );
                                            }

                                            ui.add_space(5.0);

                                            if let Some(nb_store_locations) =
                                                entity.entity_nb_store_locations
                                                && nb_store_locations > 0
                                            {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "[ {}: {nb_store_locations} ]",
                                                        t!("entity_store_locations")
                                                    ))
                                                    .color(hint_color),
                                                );
                                            }
                                        });

                                        ui.add_space(10.0);

                                        if let Some(managers) = &entity.managers {
                                            ui.horizontal_wrapped(|ui| {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{}:",
                                                        t!("entity_managers")
                                                    ))
                                                    .underline(),
                                                );
                                                for manager in managers {
                                                    ui.label(
                                                        RichText::new(manager.clone().person_email)
                                                            .italics(),
                                                    );
                                                }
                                            });
                                        };
                                    });
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
