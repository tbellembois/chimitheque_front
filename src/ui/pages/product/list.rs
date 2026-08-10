use egui::RichText;
use rust_i18n::t;

use crate::{
    types::ProductsOrderBy,
    ui::{
        app::App,
        components::productlabel::render_product_label,
        state::{Action, Page},
        widgets::{buttonwithiconandtext::button_with_icon_and_text, size::Size},
    },
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    if let Ok(maybe_products_and_count) = app.get_products_and_count()
        && let Some((products, count)) = maybe_products_and_count
    {
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 32.0),
            egui::Layout::right_to_left(egui::Align::TOP),
            |ui| {
                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Products,
                    None,
                    &ehttp::Method::POST,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("product_create").to_string(),
                    egui_phosphor::fill::MAGIC_WAND,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.active_page = Page::ProductCreate;
                }

                if app.has_permission(
                    &chimitheque_types::permission::PermissionItem::Storages,
                    None,
                    &ehttp::Method::GET,
                    &app.permissions.clone(),
                ) && button_with_icon_and_text(
                    ui,
                    t!("switch_to_storage_view").to_string(),
                    egui_phosphor::fill::PACKAGE,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.state.active_page = Page::StorageList;
                }

                if button_with_icon_and_text(
                    ui,
                    t!("export").to_string(),
                    egui_phosphor::fill::EXPORT,
                    &Size::Medium,
                )
                .clicked()
                {
                    app.state.action.push_back(Action::ExportProducts);
                }
            },
        );

        let showing_products = app.search_limit + app.current_search_offset;
        let total_products = count;

        ui.label(t!(
            "search_results_showing",
            showing = showing_products,
            total = total_products,
        ));

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(t!("order_by")).underline());

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.products_order_by,
                    ProductsOrderBy::Name,
                    t!("product_card_name"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.products_order_by,
                    ProductsOrderBy::CasNumber,
                    t!("product_card_cas_number"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.products_order_by,
                    ProductsOrderBy::EmpiricalFormula,
                    t!("product_card_empirical_formula"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.products_order,
                    crate::types::GenericOrder::Asc,
                    t!("order_asc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.products_order,
                    crate::types::GenericOrder::Desc,
                    t!("order_desc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }
        });

        ui.add_space(20.0);

        let output = egui::ScrollArea::vertical()
            .id_salt("products_scrollarea")
            .show(ui, |ui| {
                for product in products {
                    render_product_label(app, ui, frame, product.clone());
                }
            });

        let offset = output.state.offset.y;
        let max_offset = output.content_size.y - output.inner_rect.height();
        let near_bottom = offset >= max_offset - 50.0;

        if near_bottom && !app.state.scrollarea_was_near_bottom && count > 0 {
            app.current_search_offset += app.search_limit;

            app.state.action.push_back(Action::GetProducts(true));
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
