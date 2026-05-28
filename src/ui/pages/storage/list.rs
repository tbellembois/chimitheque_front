use rust_i18n::t;
use std::sync::Arc;

use crate::{
    api::storage::get_storages,
    defines::SEARCH_LIMIT,
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
                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Storages,
                    None,
                    &ehttp::Method::POST,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("storage_create").to_string(),
                    egui_phosphor::fill::MAGIC_WAND,
                    &Size::Medium,
                )
                .clicked()
                {
                    todo!()
                }

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
                    app.state.action.push_back(Action::GetProducts);
                    app.state.active_page = Page::ProductList;
                }
            },
        );

        let showing_storages = SEARCH_LIMIT + app.current_search_offset;
        let total_storages = count;

        ui.label(t!(
            "search_results_showing",
            showing = showing_storages,
            total = total_storages,
        ));

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
            app.current_search_offset += SEARCH_LIMIT;

            get_storages(
                &app.get_request_filter(),
                Arc::clone(&app.storages),
                true,
                app.channel_sender.clone(),
            );
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
