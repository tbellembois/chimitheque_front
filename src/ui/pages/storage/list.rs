use egui::RichText;
use rust_i18n::t;

use crate::{
    types::StoragesOrderBy,
    ui::{
        app::App,
        components::storagelabel::render_storage_label,
        state::{Action, Page},
        widgets::{buttonwithiconandtext::button_with_icon_and_text, size::Size},
    },
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

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Products,
                    None,
                    &ehttp::Method::GET,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("switch_to_product_view").to_string(),
                    egui_phosphor::fill::TAG,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action.push_back(Action::GetProducts(false));
                    app.state.active_page = Page::ProductList;
                }

                if app.storages_show_archives
                    && button_with_icon_and_text(
                        ui,
                        t!("storages_hide_archives").to_string(),
                        egui_phosphor::fill::ARCHIVE,
                        &Size::Medium,
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.storages_show_archives = false;
                }

                if !app.storages_show_archives
                    && button_with_icon_and_text(
                        ui,
                        t!("storages_show_archives").to_string(),
                        egui_phosphor::fill::ARCHIVE,
                        &Size::Medium,
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.storages_show_archives = true;
                }

                if button_with_icon_and_text(
                    ui,
                    t!("export").to_string(),
                    egui_phosphor::fill::EXPORT,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action.push_back(Action::ExportStorages);
                }
            },
        );

        let showing_storages = app.search_limit + app.current_search_offset;
        let total_storages = count;

        ui.label(t!(
            "search_results_showing",
            showing = showing_storages,
            total = total_storages,
        ));

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(t!("order_by")).underline());

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.storages_order_by,
                    StoragesOrderBy::Product,
                    t!("storage_card_product"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storages_order_by,
                    StoragesOrderBy::StoreLocation,
                    t!("storage_card_store_location"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storages_order_by,
                    StoragesOrderBy::BatchNumber,
                    t!("storage_card_batch_number"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storages_order_by,
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
                    &mut app.storages_order,
                    crate::types::GenericOrder::Asc,
                    t!("order_asc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }

            if ui
                .selectable_value(
                    &mut app.storages_order,
                    crate::types::GenericOrder::Desc,
                    t!("order_desc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetStorages(false));
            }
        });

        ui.add_space(20.0);

        let output = egui::ScrollArea::vertical()
            .id_salt("storages_scrollarea")
            .show(ui, |ui| {
                for storage in storages {
                    render_storage_label(app, ui, frame, storage.clone());
                }
            });

        let offset = output.state.offset.y;
        let max_offset = output.content_size.y - output.inner_rect.height();
        let near_bottom = offset >= max_offset - 50.0;

        if near_bottom && !app.state.scrollarea_was_near_bottom && count > 0 {
            app.current_search_offset += app.search_limit;

            app.state.action.push_back(Action::GetStorages(true));
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
