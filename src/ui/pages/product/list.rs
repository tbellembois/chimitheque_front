use rust_i18n::t;
use std::sync::Arc;

use crate::{
    api::product::get_products,
    defines::SEARCH_LIMIT,
    ui::{app::App, components::productlabel::render_product_label},
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    if let Ok(maybe_products_and_count) = app.get_products_and_count()
        && let Some((products, count)) = maybe_products_and_count
    {
        let showing_products = SEARCH_LIMIT + app.current_search_offset;
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
                    render_product_label(app, ui, frame, product.clone());
                }
            });

        let offset = output.state.offset.y;
        let max_offset = output.content_size.y - output.inner_rect.height();
        let near_bottom = offset >= max_offset - 50.0;

        if near_bottom && !app.state.scrollarea_was_near_bottom && count > 0 {
            app.current_search_offset += SEARCH_LIMIT;

            get_products(
                &app.get_request_filter(),
                Arc::clone(&app.products),
                true,
                app.channel_sender.clone(),
            );
        }

        app.state.scrollarea_was_near_bottom = near_bottom;
    }
}
