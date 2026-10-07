use crate::{
    logger::{LOGS, LogMessage},
    ui::{
        app::App,
        pages::{entity, person, product, pubchem, storage, storelocation},
        state::{Action, Page},
        widgets::{
            buttonwithicon::button_with_icon, buttonwithiconandtext::button_with_icon_and_text,
            buttonwithimage::button_with_image, clickablelabelwithicon::clickable_label_with_icon,
            clickablelabelwithiconandtext::clickable_label_with_icon_and_text, icon::icon,
            size::Size,
        },
    },
};
use egui::{Margin, Popup, Pos2, Rect, RichText, TextBuffer};
use egui_phosphor::fill::PLUS_CIRCLE;
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let panel_response = egui::Panel::top("top_panel")
        .frame(
            egui::Frame::NONE
                .inner_margin(Margin {
                    top: app.visual.app_top_margin,
                    bottom: app.visual.app_bottom_margin,
                    left: app.visual.app_left_margin,
                    right: app.visual.app_right_margin,
                })
                .fill(app.visual.normal_bg_color),
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
                    {
                        if app.visual.app_font_size > 13.0 {
                            app.visual.app_font_size -= 1.0;
                        }
                    }
                    if clickable_label_with_icon(
                        ui,
                        egui_phosphor::regular::PLUS_CIRCLE,
                        &Size::Medium,
                    )
                    .clicked()
                    {
                        if app.visual.app_font_size < 23.0 {
                            app.visual.app_font_size += 1.0;
                        }
                    }

                    // This scales fonts, spacing and widgets.✅
                    let ctx = ui.ctx();
                    ctx.set_pixels_per_point(app.visual.app_font_size / 16.0);

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
                            app.visual_needs_init = true;
                        } else {
                            ui.ctx().set_theme(egui::Theme::Light);
                            app.visual_needs_init = true;
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

            ui.add_space(10.0);

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

    // Update app rects.
    app.state.top_panel_rect = panel_response.response.rect;
    app.state.window_available_rect = Rect {
        min: Pos2 {
            x: app.state.top_panel_rect.min.x + f32::from(app.visual.app_left_margin),
            y: app.state.top_panel_rect.min.y + f32::from(app.visual.app_top_margin),
        },
        max: Pos2 {
            x: app.state.top_panel_rect.max.x - f32::from(app.visual.app_right_margin),
            y: app.state.top_panel_rect.max.y - f32::from(app.visual.app_bottom_margin),
        },
    };

    //
    // Footer bar
    //
    egui::Panel::bottom("footer").show(ui, |ui| {
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
                            ui.label(RichText::new(text).color(app.visual.error_color));
                        }
                        LogMessage::Debug(_) => (),
                    }
                }
            });
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
                    left: 25,
                    right: 25,
                })
                .fill(app.visual.normal_bg_color),
        )
        .show(ui, |ui| match app.state.active_page {
            Page::ProductList => {
                let state = &mut app.state;
                app.search_form.render(ui, state, &app.visual);

                ui.add_space(20.0);

                product::list::update(app, ui, frame);
            }
            Page::StorageList => {
                let state = &mut app.state;
                app.search_form.render(ui, state, &app.visual);

                ui.add_space(20.0);

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
                app.create_product_form.render(ui, state, &app.visual);
            }
        });
}
