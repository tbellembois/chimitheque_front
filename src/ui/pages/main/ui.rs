use crate::{
    config::{
        APP_LEFT_MARGIN, APP_RIGHT_MARGIN, APP_TOP_MARGIN, ERROR_COLOR, HEADER_BOTTOM_MARGIN,
    },
    logger::{LOGS, LogMessage},
    ui::{
        app::App,
        pages::{entity, person, product, pubchem, storage, storelocation},
        state::{Action, Page},
    },
};
use egui::{Margin, Rect, RichText, TextBuffer};
use egui_phosphor_icons::icons;
use elegance::{
    Badge, BadgeTone, Button, Card, Popover, PopoverSide, TabBar, Theme, ThemeSwitcher,
};
use rust_i18n::t;

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    // ui.label(format!(
    //     "central_panel_rect: {:?}",
    //     app.state.central_panel_rect
    // ));
    // ui.label(format!("top_panel_rect: {:?}", app.state.top_panel_rect));
    // ui.label(format!(
    //     "bottom_panel_rect: {:?}",
    //     app.state.bottom_panel_rect
    // ));
    // let viewport_rect = ui.viewport_rect();
    // ui.label(format!("viewport_rect: {:?}", viewport_rect));

    let app_top_margin = *APP_TOP_MARGIN.get().unwrap();
    let app_left_margin = *APP_LEFT_MARGIN.get().unwrap();
    let app_right_margin = *APP_RIGHT_MARGIN.get().unwrap();
    let header_bottom_margin = *HEADER_BOTTOM_MARGIN.get().unwrap();
    let error_color = *ERROR_COLOR.get().unwrap();

    let top_panel_response = egui::Panel::top("top_panel")
        .frame(
            egui::Frame::new()
                .fill(Theme::current(ui.ctx()).palette.bg)
                .inner_margin(Margin {
                    left: app_left_margin,
                    right: app_right_margin,
                    top: app_top_margin,
                    bottom: header_bottom_margin,
                }),
        )
        .show_separator_line(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Switch locale.
                Card::new().heading(t!("locale")).show(ui, |ui| {
                    // Switch locale.
                    if ui.add(Button::new("Fr")).clicked() {
                        rust_i18n::set_locale("fr-FR");
                    };
                    if ui.add(Button::new("En")).clicked() {
                        rust_i18n::set_locale("en-GB");
                    }
                });

                // Interface size.
                Card::new().heading(t!("interface_size")).show(ui, |ui| {
                    if ui.add(Button::new(icons::MINUS.regular())).clicked() {
                        app.state.zoom_factor -= 0.1;
                    }
                    if ui.add(Button::new(icons::PLUS.regular())).clicked() {
                        app.state.zoom_factor += 0.1;
                    }
                });

                Card::new().heading(t!("theme")).show(ui, |ui| {
                    // This scales fonts, spacing and widgets.✅
                    let ctx = ui.ctx();
                    // ctx.set_pixels_per_point(app.state.font_size / 16.0);

                    ui.add_space(10.0);

                    // Theme switch.
                    ui.add(ThemeSwitcher::new(&mut app.state.active_theme));
                });

                // User info and version info.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Version info.
                    if let Ok(maybe_version_info) = &app.get_version_info()
                        && let Some(version_info) = maybe_version_info
                    {
                        let trigger = ui.add(Button::new(icons::INFO.regular()));

                        Popover::new(t!("version"))
                            .side(PopoverSide::Left)
                            .title("Chimithèque")
                            .show(&trigger, |ui| {
                                ui.label(format!("version: {}", version_info.clone().version));
                                ui.label(format!(
                                    "build time: {}",
                                    version_info.clone().build_time
                                ));
                                ui.label(format!(
                                    "commit: {}",
                                    version_info.clone().git_commit.unwrap_or_default()
                                ));
                                ui.label(format!("rustc: {}", version_info.clone().rustc));
                                ui.label(format!("target: {}", version_info.clone().target));
                            });
                    }

                    // User info.
                    if let Ok(maybe_connected_user) = app.get_connected_user()
                        && let Some(connected_user) = maybe_connected_user
                    {
                        ui.label(connected_user.person_email.clone());
                        // icon(ui, egui_phosphor::regular::USER, &Size::Medium);
                    }
                });
            });

            ui.add_space(20.0);

            // Render logo and menu.
            ui.horizontal(|ui| {
                // Logo.
                ui.add(egui::Label::new(
                    icons::COPYRIGHT
                        .regular()
                        .color(app.state.active_theme.theme().palette.focus)
                        .size(26.0)
                        .strong(),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("Chimithèque")
                        .color(app.state.active_theme.theme().palette.focus)
                        .size(26.0)
                        .strong(),
                ));

                // let chimitheque_logo = match app.state.active_theme {
                //     BuiltInTheme::Slate | BuiltInTheme::Charcoal => {
                //         app.textures.get("chimitheque_logo_light")
                //     }
                //     _ => app.textures.get("chimitheque_logo_dark"),
                // };
                // if let Some(chimitheque_logo) = chimitheque_logo {
                //     ui.image(chimitheque_logo);
                // }

                ui.add_space(40.0);

                // Menu.
                ui.add(TabBar::new(
                    &mut app.state.active_tab,
                    [
                        t!("menu_bookmarks").as_str(),
                        t!("menu_products").as_str(),
                        t!("menu_pubchem").as_str(),
                        t!("menu_storelocations").as_str(),
                        t!("menu_entities").as_str(),
                        t!("menu_people").as_str(),
                    ],
                ));

                if app.state.active_tab != app.state.previous_tab {
                    match app.state.active_tab {
                        0 => {
                            todo!()
                        }
                        1 => {
                            app.search_form.is_expanded = false;
                            app.state.action.push_back(Action::GetProducts(false));
                            app.state.active_page = Page::ProductList;
                        }
                        2 => {
                            app.state.active_page = Page::Pubchem;
                        }
                        3 => {
                            app.state.action.push_back(Action::GetStorelocations);
                            app.state.active_page = Page::StorelocationList;
                        }
                        4 => {
                            app.state.action.push_back(Action::GetEntities);
                            app.state.active_page = Page::EntityList;
                        }
                        5 => {
                            app.state.action.push_back(Action::GetPeople);
                            app.state.active_page = Page::PeopleList;
                        }
                        _ => {
                            unreachable!()
                        }
                    }

                    app.state.previous_tab = app.state.active_tab;
                }

                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_bookmarks").as_str(),
                //                     icons::BOOKMARK.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     //functionality
                //                 }
                //
                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_products").as_str(),
                //                     icons::TAG.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     app.search_form.is_expanded = false;
                //                     app.state.action.push_back(Action::GetProducts(false));
                //                     app.state.active_page = Page::ProductList;
                //                 }
                //
                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_pubchem").as_str(),
                //                     icons::LETTER_CIRCLE_P.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     app.state.active_page = Page::Pubchem;
                //                 }
                //
                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_storelocations").as_str(),
                //                     icons::DRESSER.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     app.state.action.push_back(Action::GetStorelocations);
                //                     app.state.active_page = Page::StorelocationList;
                //                 }
                //
                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_entities").as_str(),
                //                     icons::WAREHOUSE.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     app.state.action.push_back(Action::GetEntities);
                //                     app.state.active_page = Page::EntityList;
                //                 }
                //
                //                 if clickable_label_with_icon_and_text(
                //                     ui,
                //                     t!("menu_people").as_str(),
                //                     icons::PERSON.regular().text(),
                //                     &Size::Medium,
                //                 )
                //                 .clicked()
                //                 {
                //                     app.state.action.push_back(Action::GetPeople);
                //                     app.state.active_page = Page::PeopleList;
                //                 }
            });
        });

    // Update top panel rect.
    app.state.top_panel_rect = top_panel_response.response.rect;

    //
    // Footer bar
    //
    let bottom_panel_response = egui::Panel::bottom("bottom_panel")
        .frame(
            egui::Frame::new()
                .fill(Theme::current(ui.ctx()).palette.bg)
                .inner_margin(egui::Margin::symmetric(0, 0)),
        )
        .show(ui, |ui| {
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
            egui::Frame::new()
                .fill(Theme::current(ui.ctx()).palette.bg)
                .inner_margin(Margin {
                    left: app_left_margin,
                    right: app_right_margin,
                    top: 0,
                    bottom: header_bottom_margin,
                }),
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
