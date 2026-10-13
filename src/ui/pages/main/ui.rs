use crate::{
    config::{
        APP_LEFT_MARGIN, APP_RIGHT_MARGIN, APP_TOP_MARGIN, ERROR_COLOR,
        NORMAL_BG_COLOR,
    },
    logger::{LOGS, LogMessage},
    ui::{
        app::App,
        pages::{entity, person, product, pubchem, storage, storelocation},
        state::{Action, Page},
        widgets::{
            buttonwithimage::button_with_image, clickablelabelwithicon::clickable_label_with_icon,
            clickablelabelwithiconandtext::clickable_label_with_icon_and_text, icon::icon,
            size::Size,
        },
    },
};
use egui::{Margin, Popup, Rect, RichText, TextBuffer};
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    ui.label(format!(
        "central_panel_rect: {:?}",
        app.state.central_panel_rect
    ));
    ui.label(format!("top_panel_rect: {:?}", app.state.top_panel_rect));
    ui.label(format!(
        "bottom_panel_rect: {:?}",
        app.state.bottom_panel_rect
    ));

    let app_top_margin = *APP_TOP_MARGIN.get().unwrap();
    let app_left_margin = *APP_LEFT_MARGIN.get().unwrap();
    let app_right_margin = *APP_RIGHT_MARGIN.get().unwrap();
    let normal_bg_color = *NORMAL_BG_COLOR.get().unwrap();
    let error_color = *ERROR_COLOR.get().unwrap();

    let top_panel_response = egui::Panel::top("top_panel")
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    top: app_top_margin,
                    bottom: 0,
                    left: app_left_margin,
                    right: app_right_margin,
                })
                .fill(normal_bg_color),
        )
        .show_separator_line(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Switch locale, theme.
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Switch locale.
                    if let Some(fr_locale_icon) = app.textures.get("flag_fr")
                        && button_with_image(ui, fr_locale_icon).clicked()
                    {
                        rust_i18n::set_locale("fr-FR");
                    }
                    if let Some(en_locale_icon) = app.textures.get("flag_gb")
                        && button_with_image(ui, en_locale_icon).clicked()
                    {
                        rust_i18n::set_locale("en-GB");
                    }

                    ui.add_space(30.0);

                    // Font size.

                    if clickable_label_with_icon(
                        ui,
                        egui_phosphor::regular::MINUS_CIRCLE,
                        &Size::Medium,
                    )
                    .clicked()
                        && app.state.font_size > 13.0 {
                            app.state.font_size -= 1.0;
                        }
                    if clickable_label_with_icon(
                        ui,
                        egui_phosphor::regular::PLUS_CIRCLE,
                        &Size::Medium,
                    )
                    .clicked()
                        && app.state.font_size < 23.0 {
                            app.state.font_size += 1.0;
                        }

                    // This scales fonts, spacing and widgets.✅
                    let ctx = ui.ctx();
                    ctx.set_pixels_per_point(app.state.font_size / 16.0);

                    ui.add_space(10.0);

                    // Theme switch.
                    if app.state.darkmode {
                        icon(ui, egui_phosphor::regular::SUN, &Size::Medium);
                    } else {
                        icon(ui, egui_phosphor::regular::MOON, &Size::Medium);
                    }
                    if ui.checkbox(&mut app.state.darkmode, "").changed() {
                        if app.state.darkmode {
                            ui.ctx().set_theme(egui::Theme::Dark);
                        } else {
                            ui.ctx().set_theme(egui::Theme::Light);
                        }
                    }
                });

                // User info and version info.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Version info.
                    if let Ok(maybe_version_info) = &app.get_version_info()
                        && let Some(version_info) = maybe_version_info
                    {
                        let button_response = clickable_label_with_icon_and_text(
                            ui,
                            t!("version").as_str(),
                            egui_phosphor::regular::INFO,
                            &Size::Medium,
                        );
                        let popup = Popup::menu(&button_response);
                        popup.show(|ui| {
                            ui.label(version_info.clone().build_time);
                            ui.label(version_info.clone().git_commit.unwrap_or_default());
                            ui.label(version_info.clone().git_commit_hash.unwrap_or_default());
                            ui.label(version_info.clone().rustc);
                            ui.label(version_info.clone().target);
                            ui.label(version_info.clone().version);
                        });
                    }

                    // User info.
                    if let Ok(maybe_connected_user) = app.get_connected_user()
                        && let Some(connected_user) = maybe_connected_user
                    {
                        ui.label(connected_user.person_email.clone());
                        icon(ui, egui_phosphor::regular::USER, &Size::Medium);
                    }
                });
            });

            ui.add_space(5.0);

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
                    app.search_form.is_expanded = false;
                    app.state.action.push_back(Action::GetProducts(false));
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
                    app.state.action.push_back(Action::GetStorelocations);
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
                    app.state.action.push_back(Action::GetEntities);
                    app.state.active_page = Page::EntityList;
                }

                if clickable_label_with_icon_and_text(
                    ui,
                    t!("menu_people").as_str(),
                    egui_phosphor::regular::PERSON,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action.push_back(Action::GetPeople);
                    app.state.active_page = Page::PeopleList;
                }
            });
        });

    // Update top panel rect.
    app.state.top_panel_rect = top_panel_response.response.rect;

    //
    // Footer bar
    //
    let bottom_panel_response = egui::Panel::bottom("bottom_panel").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label("© 2026 Chimithèque, released under the GPL-3.0 license.");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let logs = LOGS
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);

                if let Some(msg) = logs.back() {
                    match msg {
                        LogMessage::Info(text) => {
                            ui.label(RichText::new(text));
                        }
                        LogMessage::Error(text) => {
                            ui.label(RichText::new(text).color(error_color));
                        }
                        LogMessage::Debug(_) => (),
                    }
                }
            });
        });
    });

    // Update bottom panel rect.
    app.state.bottom_panel_rect = bottom_panel_response.response.rect;

    //
    // Render active page.
    //
    egui::CentralPanel::default()
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    // margins are managed by inner content
                    top: 0,
                    bottom: 0,
                    left: 0,
                    right: 0,
                })
                .fill(normal_bg_color),
        )
        .show(ui, |ui| match app.state.active_page {
            Page::ProductList => {
                let state = &mut app.state;
                app.search_form.render(ui, state);

                product::list::update(app, ui, frame);
            }
            Page::StorageList => {
                let state = &mut app.state;
                app.search_form.render(ui, state);

                storage::list::update(app, ui, frame);
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
            Page::PeopleList => {
                person::list::update(app, ui, frame);
            }
            Page::ProductCreate => {
                let state = &mut app.state;
                app.create_product_form.render(ui, state);
            }
        });

    // Update central panel rect.
    app.state.central_panel_rect = Rect::from_points(&[
        app.state.top_panel_rect.left_bottom(),
        app.state.bottom_panel_rect.right_top(),
    ]);
}
