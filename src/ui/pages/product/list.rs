use egui::{RichText, TextBuffer};
use egui_phosphor_icons::icons;
use elegance::{Accent, Button, ButtonSize};
use rust_i18n::t;

use crate::{
    config::APP_SIDE_MARGIN,
    types::ProductsOrderBy,
    ui::{
        app::App,
        components::productlabel::render_product_label,
        state::{Action, Page},
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
                ) && ui
                    .add(
                        Button::icon_text(
                            icons::MAGIC_WAND
                                .fill()
                                .text()
                                .chars()
                                .next()
                                .unwrap_or_default(),
                            t!("product_create").to_string(),
                        )
                        .size(ButtonSize::Medium),
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
                ) && ui
                    .add(
                        Button::icon_text(
                            icons::PACKAGE
                                .fill()
                                .text()
                                .chars()
                                .next()
                                .unwrap_or_default(),
                            t!("switch_to_storage_view").to_string(),
                        )
                        .size(ButtonSize::Medium),
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::GetStorages(false));
                    app.state.active_page = Page::StorageList;
                }

                if ui
                    .add(
                        Button::icon_text(
                            icons::EXPORT
                                .fill()
                                .text()
                                .chars()
                                .next()
                                .unwrap_or_default(),
                            t!("export").to_string(),
                        )
                        .size(ButtonSize::Medium),
                    )
                    .clicked()
                {
                    app.state.action.push_back(Action::ExportProducts);
                }

                ui.add_space(20.0);

                if !app.search_form.is_expanded
                    && ui
                        .add(
                            Button::icon_text(
                                icons::ARROWS_OUT
                                    .fill()
                                    .text()
                                    .chars()
                                    .next()
                                    .unwrap_or_default(),
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
            let showing_products = app.search_form.search_limit + app.search_form.search_offset;
            let total_products = count;

            ui.label(t!(
                "search_results_showing",
                showing = showing_products,
                total = total_products,
            ));

            ui.add_space(20.0);

            ui.label(RichText::new(t!("order_by")).underline());

            ui.add_space(20.0);

            if ui
                .selectable_value(
                    &mut app.product_state.products_order_by,
                    ProductsOrderBy::Name,
                    t!("product_card_name"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.product_state.products_order_by,
                    ProductsOrderBy::CasNumber,
                    t!("product_card_cas_number"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.product_state.products_order_by,
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
                    &mut app.product_state.products_order,
                    crate::types::GenericOrder::Asc,
                    t!("order_asc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }

            if ui
                .selectable_value(
                    &mut app.product_state.products_order,
                    crate::types::GenericOrder::Desc,
                    t!("order_desc"),
                )
                .clicked()
            {
                app.state.action.push_back(Action::GetProducts(false));
            }
        });

        ui.add_space(20.0);

        let central_panel_width = app.state.central_panel_rect.width();
        let col_width =
            (central_panel_width / 2.0) - 10.0 - f32::from(*APP_SIDE_MARGIN.get().unwrap());

        let output = egui::ScrollArea::vertical()
            .id_salt("products_scrollarea")
            .show(ui, |ui| {
                egui::Grid::new("products_grid")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .max_col_width(col_width)
                    .min_col_width(col_width)
                    .striped(false)
                    .show(ui, |ui| {
                        let mut count: usize = 0;
                        for product in products {
                            render_product_label(app, ui, frame, product.clone());
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

            app.state.action.push_back(Action::GetProducts(true));
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
