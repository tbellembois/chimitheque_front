use crate::{
    logger::{LOGS, LogMessage},
    ui::{
        app::App,
        components::searchform::render_search_form,
        pages::{entity, product, pubchem, storelocation},
        state::{Action, Page},
        widgets::{clickablelabelwithiconandtext::clickable_label_with_icon_and_text, size::Size},
    },
};
use egui::{Margin, RichText, TextBuffer};
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let visuals = ui.visuals();
    let bg_color = visuals.window_fill;

    let panel_response = egui::Panel::top("top_panel")
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    top: 20,
                    bottom: 10,
                    left: 50,
                    right: 50,
                })
                .fill(bg_color),
        )
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                // Info and error messages.
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        let logs = LOGS
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner);

                        if let Some(msg) = logs.back() {
                            match msg {
                                LogMessage::Info(text) => {
                                    ui.label(
                                        RichText::new(text)
                                            .color(egui::Color32::from_rgb(60, 180, 95)),
                                    );
                                }
                                LogMessage::Error(text) => {
                                    ui.label(
                                        RichText::new(text)
                                            .color(egui::Color32::from_rgb(220, 70, 70)),
                                    );
                                }
                                LogMessage::Debug(_) => (),
                            }
                        }
                    },
                );

                // Switch locale, theme and user info.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Switch locale.
                    if let Some(fr_locale_icon) = app.textures.get("flag_fr")
                        && ui
                            .add(egui::Button::image_and_text(fr_locale_icon, "Fr"))
                            .clicked()
                    {
                        rust_i18n::set_locale("fr-FR");
                    }
                    if let Some(en_locale_icon) = app.textures.get("flag_gb")
                        && ui
                            .add(egui::Button::image_and_text(en_locale_icon, "En"))
                            .clicked()
                    {
                        rust_i18n::set_locale("en-GB");
                    }

                    // Theme switch.
                    if ui
                        .checkbox(&mut app.state.darkmode, t!("darkmode"))
                        .changed()
                    {
                        if app.state.darkmode {
                            ui.ctx().set_visuals(egui::Visuals::dark());
                        } else {
                            ui.ctx().set_visuals(egui::Visuals::light());
                        }
                    }

                    // User info.
                    if let Ok(maybe_connected_user) = app.get_connected_user()
                        && let Some(connected_user) = maybe_connected_user
                    {
                        ui.label(egui::RichText::new(format!(
                            "{} {}",
                            egui_phosphor::regular::USER,
                            connected_user.person_email
                        )));
                    }
                });
            });

            // Render logo and menu.
            ui.horizontal(|ui| {
                // Logo.
                if app.state.darkmode {
                    if let Some(chimitheque_logo) = app.textures.get("chimitheque_logo_dark") {
                        ui.image(chimitheque_logo);
                    }
                } else if let Some(chimitheque_logo) = app.textures.get("chimitheque_logo_light") {
                    ui.image(chimitheque_logo);
                }

                ui.add_space(20.0);

                // Menu.
                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_bookmarks").as_str(),
                    egui_phosphor::regular::BOOKMARK,
                    &Size::Medium,
                )
                .clicked()
                {
                    //functionality
                }

                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_products").as_str(),
                    egui_phosphor::regular::TAG,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.search_form_expanded = false;
                    app.state.action = Action::GetProducts;
                    app.state.active_page = Page::ProductList;
                }

                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_pubchem").as_str(),
                    egui_phosphor::regular::LETTER_CIRCLE_P,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.active_page = Page::Pubchem;
                }

                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_storelocations").as_str(),
                    egui_phosphor::regular::DRESSER,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action = Action::GetStorelocations;
                    app.state.active_page = Page::StorelocationList;
                }

                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_entities").as_str(),
                    egui_phosphor::regular::WAREHOUSE,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action = Action::GetEntities;
                    app.state.active_page = Page::EntityList;
                }

                // egui::MenuBar::new().ui(ui, |ui| {
                //     ui.menu_button(
                //         egui::RichText::new(format!(
                //             "{} {}",
                //             egui_phosphor::fill::BOOKMARK,
                //             t!("menu_bookmarks")
                //         )),
                //         |ui| {
                //             if ui.button(t!("list")).clicked() {
                //                 //functionality
                //             }
                //         },
                //     );

                //     ui.menu_button(
                //         egui::RichText::new(format!(
                //             "{} {}",
                //             egui_phosphor::fill::TAG,
                //             t!("menu_products")
                //         )),
                //         |ui| {
                //             if ui.button(t!("list")).clicked() {
                //                 app.state.action = Action::GetProducts;
                //                 app.state.active_page = Page::ProductList;
                //             }
                //         },
                //     );

                //     ui.menu_button(
                //         egui::RichText::new(format!(
                //             "{} {}",
                //             egui_phosphor::fill::LETTER_CIRCLE_P,
                //             t!("menu_pubchem")
                //         )),
                //         |ui| {
                //             if ui.button(t!("search")).clicked() {
                //                 // app.state.action = Action::GetProducts;
                //                 app.state.active_page = Page::Pubchem;
                //             }
                //         },
                //     );

                //     ui.menu_button(
                //         egui::RichText::new(format!(
                //             "{} {}",
                //             egui_phosphor::fill::WAREHOUSE,
                //             t!("menu_storelocations")
                //         )),
                //         |ui| {
                //             if ui.button(t!("list")).clicked() {
                //                 app.state.action = Action::GetStorelocations;
                //                 app.state.active_page = Page::StorelocationList;
                //             }
                //         },
                //     );
                // });
            });
        });

    // Update top panel rect.
    app.state.top_panel_rect = panel_response.response.rect;

    //
    // Footer bar
    //
    egui::Panel::bottom("footer").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("© 2026 Chimithèque, released under the GPL-3.0 license.");
        });
    });

    //
    // Render active page.
    //

    egui::CentralPanel::default()
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    top: 10,
                    bottom: 10,
                    left: 50,
                    right: 50,
                })
                .fill(bg_color),
        )
        .show_inside(ui, |ui| match app.state.active_page {
            Page::ProductList => {
                render_search_form(app, ui, frame);

                ui.add_space(20.0);

                product::list::update(app, ui, frame);
            }
            Page::StorelocationList => {
                storelocation::list::update(app, ui, frame);
            }
            Page::EntityList => {
                entity::list::update(app, ui, frame);
            }
            Page::Pubchem => {
                pubchem::search::update(app, ui, frame);
            }
        });
}
