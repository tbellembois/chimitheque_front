use egui::{RichText, TextBuffer};
use egui_phosphor_icons::icons;
use elegance::{Accent, Button, ButtonSize};
use rust_i18n::t;

use crate::{
    config::APP_SIDE_MARGIN,
    types::StoragesOrderBy,
    ui::{
        app::App,
        components::storagelabel::render_storage_label,
        state::{Action, Page},
        widgets::{
            buttonwithiconandtext::button_with_icon_and_text,
            clickablelabelwithiconandtext::clickable_label_with_icon_and_text, size::Size,
        },
    },
    utils::first_char,
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    if let Ok(maybe_storages_and_count) = app.get_storages_and_count()
        && let Some((storages, count)) = maybe_storages_and_count
    {
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 32.0),
            egui::Layout::right_to_left(egui::Align::TOP),
            |ui| {
                // if app.has_permission(
                //     &chimitheque_types::permission::PermissionItem::Storages,
                //     None,
                //     &ehttp::Method::POST,
                //     &app.permissions.clone(),
                // ) && button_with_icon_and_text(
                //     ui,
                //     t!("storage_create").to_string(),
                //     egui_phosphor::fill::MAGIC_WAND,
                //     &Size::Medium,
                // )
                // .clicked()
                // {
                //     todo!()
                // }

                if app.storage_state.storages_show_archives
                    && ui
                        .add(Button::icon_text(
                            first_char(&icons::ARCHIVE),
                            t!("storages_hide_archives").to_string(),
                        ))
                        .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.storage_state.storages_show_archives = false;
                }

                if !app.storage_state.storages_show_archives
                    && ui
                        .add(Button::icon_text(
                            first_char(&icons::ARCHIVE),
                            t!("storages_show_archives").to_string(),
                        ))
                        .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.storage_state.storages_show_archives = true;
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Products,
                    None,
                    &ehttp::Method::GET,
                    &app.permissions.clone(),
                ) && ui
                    .add(
                        Button::icon_text(
                            first_char(&icons::TAG),
                            t!("switch_to_product_view").to_string(),
                        )
                        .size(ButtonSize::Medium),
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::GetProducts(false));
                    app.state.active_page = Page::ProductList;
                }

                if ui
                    .add(
                        Button::icon_text(first_char(&icons::EXPORT), t!("export").to_string())
                            .size(ButtonSize::Medium),
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::ExportStorages);
                }

                ui.add_space(20.0);

                if !app.search_form.is_expanded
                    && ui
                        .add(
                            Button::icon_text(
                                first_char(&icons::ARROWS_OUT),
                                t!("search_form_expand").to_string(),
                            )
                            .accent(Accent::Green)
                            .size(ButtonSize::Large),
                        )
                        .clicked()
                {
                    app.search_form.is_expanded = true;
                }
            },
        );

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            let showing_storages = app.search_form.search_limit + app.search_form.search_offset;
            let total_storages = count;

            ui.label(t!(
                "search_results_showing",
                showing = showing_storages,
                total = total_storages,
            ));

            ui.add_space(20.0);

            ui.label(RichText::new(t!("order_by")).underline());

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order_by,
                    StoragesOrderBy::Product,
                    t!("storage_card_product"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order_by,
                    StoragesOrderBy::StoreLocation,
                    t!("storage_card_store_location"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order_by,
                    StoragesOrderBy::BatchNumber,
                    t!("storage_card_batch_number"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order_by,
                    StoragesOrderBy::ModificationDate,
                    t!("storage_card_modification_date"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order,
                    crate::types::GenericOrder::Asc,
                    t!("order_asc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storage_state.storages_order,
                    crate::types::GenericOrder::Desc,
                    t!("order_desc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }
        });

        ui.add_space(20.0);

        let central_panel_width = app.state.central_panel_rect.width();
        let col_width =
            (central_panel_width / 2.0) - 10.0 - f32::from(*APP_SIDE_MARGIN.get().unwrap());

        let output = egui::ScrollArea::vertical()
            .id_salt("storages_scrollarea")
            .show(ui, |ui| {
                egui::Grid::new("products_grid")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .max_col_width(col_width)
                    .min_col_width(col_width)
                    .striped(false)
                    .show(ui, |ui| {
                        let mut count: usize = 0;
                        for storage in storages {
                            render_storage_label(app, ui, frame, storage.clone());
                            count += 1;
                            if count.is_multiple_of(2) {
                                ui.end_row();
                            }
                        }
                    });
            });

        let offset = output.state.offset().y;
        let max_offset = output.content_size.y - output.inner_rect.height();
        let near_bottom = offset >= max_offset - 50.0;

        if near_bottom && !app.state.scrollarea_was_near_bottom && count > 0 {
            app.search_form.search_offset += app.search_form.search_limit;

            app.state.action.push_back(Action::GetStorages(true));
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
