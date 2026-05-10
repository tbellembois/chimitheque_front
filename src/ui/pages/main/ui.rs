use crate::{
    api::{product::retrieve_products, storelocation::retrieve_store_locations},
    ui::{
        app::App,
        pages::{product, storelocation},
        state::Page,
        widgets::searchform::render_search_form,
    },
};
use chimitheque_types::requestfilter::RequestFilter;
use egui::Color32;
use egui::{Margin, RichText};
use rust_i18n::t;
use std::sync::Arc;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    egui::Panel::top("menu_panel")
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    top: 20,
                    bottom: 10,
                    left: 50,
                    right: 50,
                })
                .fill(Color32::WHITE),
        )
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                // Info and error messages.
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Display possible error.
                    if let Some(error) = &app.current_error.lock().unwrap().as_ref() {
                        ui.label(RichText::new(format!(
                            "{} {error}",
                            egui_phosphor::fill::WARNING,
                        )));
                    }

                    // Display possible message.
                    if let Some(info) = &app.current_info.lock().unwrap().as_ref() {
                        ui.label(RichText::new(format!(
                            "{} {info}",
                            egui_phosphor::fill::INFO,
                        )));
                    }
                });

                // Switch locale, theme and user info.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Switch locale.
                    let fr_locale_icon = egui::include_image!("../../media/fr.svg");
                    let en_locale_icon = egui::include_image!("../../media/gb.svg");
                    if ui
                        .add(egui::Button::image_and_text(en_locale_icon, "En"))
                        .clicked()
                    {
                        rust_i18n::set_locale("en-GB");
                    }
                    if ui
                        .add(egui::Button::image_and_text(fr_locale_icon, "Fr"))
                        .clicked()
                    {
                        rust_i18n::set_locale("fr-FR");
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
                ui.add_sized(
                    [50., 50.],
                    egui::Image::new(egui::include_image!(
                        "../../media/chimitheque_logo_simple.svg"
                    )),
                );

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
                                retrieve_products(
                                    &RequestFilter {
                                        limit: Some(10),
                                        ..Default::default()
                                    },
                                    Arc::clone(&app.products),
                                    false,
                                    // Arc::clone(&app.loading_state),
                                    &Arc::clone(&app.current_info),
                                    &Arc::clone(&app.current_error),
                                );

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
                                retrieve_store_locations(
                                    &RequestFilter {
                                        limit: Some(10),
                                        ..Default::default()
                                    },
                                    Arc::clone(&app.storelocations),
                                    &Arc::clone(&app.current_info),
                                    &Arc::clone(&app.current_error),
                                );

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
                .fill(Color32::WHITE),
        )
        .show_inside(ui, |ui| match app.state.active_page {
            Page::ProductList => {
                render_search_form(app, ui, frame);

                ui.add_space(20.0);

                product::list::update(app, ui, frame);
            }
            Page::StorelocationList => storelocation::list::update(app, ui, frame),
        });
}
