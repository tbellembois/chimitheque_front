use crate::ui::{
    app::App,
    pages::{product, storelocation},
    state::{Action, Page},
    widgets::searchform::render_search_form,
};
use egui::{Margin, RichText};
use rust_i18n::t;
use std::sync::Arc;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let visuals = ui.visuals();
    let bg_color = visuals.window_fill;

    egui::Panel::top("menu_panel")
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
                        // Display possible error.
                        if let Some(error) = &app.current_error {
                            ui.label(RichText::new(format!(
                                "{} {error}",
                                egui_phosphor::fill::WARNING,
                            )));
                        }

                        // Display possible message.
                        if let Some(info) = &app.current_info {
                            ui.label(RichText::new(format!(
                                "{} {info}",
                                egui_phosphor::fill::INFO,
                            )));
                        }
                    },
                );

                // Switch locale, theme and user info.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Switch locale.
                    if let Some(fr_locale_icon) = app.textures.get("flag_fr") {
                        if ui
                            .add(egui::Button::image_and_text(fr_locale_icon, "Fr"))
                            .clicked()
                        {
                            rust_i18n::set_locale("fr-FR");
                        }
                    }
                    if let Some(en_locale_icon) = app.textures.get("flag_gb") {
                        if ui
                            .add(egui::Button::image_and_text(en_locale_icon, "En"))
                            .clicked()
                        {
                            rust_i18n::set_locale("en-GB");
                        }
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
                    let connected_user = Arc::clone(&app.connected_user);
                    let connected_user_locked = connected_user.lock().unwrap();
                    let email = connected_user_locked
                        .as_ref()
                        .map(|u| u.person_email.clone())
                        .unwrap_or_default();
                    ui.label(egui::RichText::new(format!(
                        "{} {}",
                        egui_phosphor::regular::USER,
                        email
                    )));
                });
            });

            // Render logo and menu.
            ui.horizontal(|ui| {
                // Logo.
                if let Some(chimitheque_logo) = app.textures.get("chimitheque_logo") {
                    ui.image(chimitheque_logo);
                }

                ui.add_space(20.0);

                // Menu.
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button(
                        egui::RichText::new(format!(
                            "{} {}",
                            egui_phosphor::fill::BOOKMARK,
                            t!("menu_bookmarks")
                        )),
                        |ui| {
                            if ui.button(t!("list")).clicked() {
                                //functionality
                            }
                        },
                    );

                    ui.menu_button(
                        egui::RichText::new(format!(
                            "{} {}",
                            egui_phosphor::fill::TAG,
                            t!("menu_products")
                        )),
                        |ui| {
                            if ui.button(t!("list")).clicked() {
                                app.state.action = Action::GetProducts;
                                app.state.active_page = Page::ProductList;
                            }
                        },
                    );

                    ui.menu_button(
                        egui::RichText::new(format!(
                            "{} {}",
                            egui_phosphor::fill::WAREHOUSE,
                            t!("menu_storelocations")
                        )),
                        |ui| {
                            if ui.button(t!("list")).clicked() {
                                app.state.action = Action::GetStorelocations;
                                app.state.active_page = Page::StorelocationList;
                            }
                        },
                    );
                });
            });
        });

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
        });
}
