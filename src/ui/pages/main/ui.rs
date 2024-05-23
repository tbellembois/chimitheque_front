use crate::ui::{app::App, pages::product, state::Page};
use egui::{Frame, RichText};
use rust_i18n::t;

pub fn update(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame) {
    //
    // Render top panel with user info, logo, info/error message, menu, theme switcher and locale switcher.
    //
    egui::TopBottomPanel::top("info_error_panel")
        .min_height(40.)
        .max_height(40.)
        .show_separator_line(true)
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_secondary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| {
            // Display possible error.
            if let Some(error) = &app.current_error {
                ui.label(
                    RichText::new(format!(" {}", error))
                        .color(app.state.active_theme.fg_error_text_color_visuals()),
                );
            }

            // Display possible message.
            if let Some(info) = &app.current_info {
                ui.label(
                    RichText::new(format!(" {}", info))
                        .color(app.state.active_theme.fg_success_text_color_visuals()),
                );
            }

            // Switch locale, theme and user info.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Switch theme.
                egui::ComboBox::from_id_source("theme_combo_box")
                    .width(200.0)
                    .selected_text(app.state.active_theme.name())
                    .show_ui(ui, |ui_combobox| {
                        for theme in app.themes.iter() {
                            let res: egui::Response = ui_combobox.selectable_value(
                                &mut app.state.active_theme,
                                theme.clone(),
                                theme.name(),
                            );
                            if res.changed() {
                                ui_combobox
                                    .ctx()
                                    .set_style(app.state.active_theme.custom_style());
                            }
                        }
                    });

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
                ui.label(app.user_info.as_ref().unwrap().clone().person_email)
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
                egui::menu::bar(ui, |ui| {
                    ui.menu_button(t!("menu_bookmarks"), |ui| {
                        if ui.button(t!("list")).clicked() {
                            //functionality
                        }
                    });

                    // ui.menu_button("Edit", |ui| {
                    //     if ui.button("Cut").clicked() {
                    //         //functionality
                    //     }
                    //     if ui.button("Copy").clicked() {
                    //         //functionality
                    //     }
                    //     if ui.button("Paste").clicked() {
                    //         //funtionality
                    //     }
                    // })
                });
            });
        });

    //
    // Render active page.
    //
    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_primary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| match app.state.active_page {
            Page::ProductList => product::list::update(app, ctx, frame, ui),
        });
}
