use crate::{
    api::{product::retrieve_products, storelocation::retrieve_store_locations},
    ui::{
        app::App,
        pages::{product, storelocation},
        state::Page,
    },
};
use chimitheque_types::requestfilter::RequestFilter;
use egui::RichText;
use rust_i18n::t;
use std::sync::Arc;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
        // .frame(egui::Frame::default().inner_margin(egui::Margin::symmetric(16, 16)))
        .show_inside(ui, |ui| {
            egui::Panel::top("menu_panel")
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Info and error messages.
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            // Display possible error.
                            if let Some(error) = &app.current_error.lock().unwrap().as_ref() {
                                ui.label(RichText::new(format!(" {error}")));
                            }

                            // Display possible message.
                            if let Some(info) = &app.current_info.lock().unwrap().as_ref() {
                                ui.label(RichText::new(format!(" {info}")));
                            }
                        });

                        // Switch locale, theme and user info.
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Switch locale.
                            let fr_locale_icon = egui::include_image!("../../media/fr.svg");
                            let en_locale_icon = egui::include_image!("../../media/gb.svg");
                            if ui
                                .add(egui::Button::image_and_text(fr_locale_icon, ""))
                                .clicked()
                            {
                                rust_i18n::set_locale("fr-FR");
                            }
                            if ui
                                .add(egui::Button::image_and_text(en_locale_icon, ""))
                                .clicked()
                            {
                                rust_i18n::set_locale("en-GB");
                            }

                            // User info.
                            let connected_user = Arc::clone(&app.connected_user);
                            let connected_user_locked = connected_user.lock().unwrap();
                            let email = connected_user_locked
                                .as_ref()
                                .map(|u| u.person_email.clone())
                                .unwrap_or_default();
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} {}",
                                    egui_phosphor::regular::USER,
                                    email
                                ))
                                .size(16.0),
                            );
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
                                    egui_phosphor::regular::BOOKMARK,
                                    t!("menu_bookmarks")
                                ))
                                .size(16.0),
                                |ui| {
                                    if ui.button(t!("list")).clicked() {
                                        //functionality
                                    }
                                },
                            );

                            ui.menu_button(
                                egui::RichText::new(format!(
                                    "{} {}",
                                    egui_phosphor::regular::TAG,
                                    t!("menu_products")
                                ))
                                .size(16.0),
                                |ui| {
                                    if ui.button(t!("list")).clicked() {
                                        retrieve_products(
                                            RequestFilter {
                                                limit: Some(10),
                                                ..Default::default()
                                            },
                                            Arc::clone(&app.products),
                                            Arc::clone(&app.current_info),
                                            Arc::clone(&app.current_error),
                                        );

                                        app.state.active_page = Page::ProductList;
                                    }
                                },
                            );

                            ui.menu_button(
                                egui::RichText::new(format!(
                                    "{} {}",
                                    egui_phosphor::regular::WAREHOUSE,
                                    t!("menu_storelocations")
                                ))
                                .size(16.0),
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

            // Footer bar
            egui::Panel::bottom("footer").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("© 2026 Chimithèque, released under the GPL-3.0 license.");
                });
            });

            //
            // Render active page.
            //
            egui::CentralPanel::default()
                // .frame(Frame {
                //     ..Default::default()
                // })
                .show_inside(ui, |ui| match app.state.active_page {
                    Page::ProductList => product::list::update(app, ui, frame),
                    Page::StorelocationList => storelocation::list::update(app, ui, frame),
                });
        });
}
