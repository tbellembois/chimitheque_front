use rust_i18n::t;
use std::sync::Arc;

use crate::{
    api::product::retrieve_products,
    defines::SEARCH_LIMIT,
    ui::{app::App, widgets::productlabel::render_product_label},
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    if let Ok(may_err_products_and_count) = app.GetProductsAndCount()
        && let Some((products, count)) = may_err_products_and_count
    {
        let showing_products = SEARCH_LIMIT + (app.current_search_offset as u64);
        let total_products = count;

        ui.label(t!(
            "search_results_showing",
            showing = showing_products,
            total = total_products,
        ));

        ui.add_space(20.0);

        let output = egui::ScrollArea::vertical()
            .id_salt("products_scrollarea")
            .show(ui, |ui| {
                for product in products {
                    // let ui_id =
                    //     egui::Id::new(("product", product.product_id, &product.name.name_label));

                    // ui.push_id(ui_id, |ui| {
                    render_product_label(app, ui, frame, product.clone());
                    // });
                }
            });

        let offset = output.state.offset.y;
        let max_offset = output.content_size.y - output.inner_rect.height();
        let near_bottom = offset >= max_offset - 50.0;

        if near_bottom && !app.state.scrollarea_was_near_bottom && count > 0 {
            app.current_search_offset += SEARCH_LIMIT as usize;

            retrieve_products(
                &app.GetRequestFilter(),
                Arc::clone(&app.products),
                true,
                app.info_sender.clone(),
                app.error_sender.clone(),
                app.loading_sender.clone(),
            );
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
