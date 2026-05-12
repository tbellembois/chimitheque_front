use std::sync::Arc;

use chimitheque_types::requestfilter::RequestFilter;
use egui::ScrollArea;

use crate::{
    api::product::retrieve_products,
    defines::SEARCH_LIMIT,
    ui::{app::App, widgets::productlabel::render_product_label},
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let output = egui::ScrollArea::vertical()
        .id_salt("products_scrollarea")
        .show(ui, |ui| {
            if let Some((products, _)) = app.products.clone().lock().unwrap().as_ref() {
                for product in products {
                    let ui_id =
                        egui::Id::new(("product", product.product_id, &product.name.name_label));

                    ui.push_id(ui_id, |ui| {
                        render_product_label(app, ui, frame, product.clone());
                    });
                }
            }
        });

    // let scroll_y = output.state.offset.y;
    // let content_height = output.content_size.y;
    // let viewport_height = output.inner_rect.height();
    // let at_bottom = scroll_y + viewport_height >= content_height - 1.0;

    let offset = output.state.offset.y;
    let max_offset = output.content_size.y - output.inner_rect.height();
    let near_bottom = offset >= max_offset - 50.0;

    let count = app
        .products
        .clone()
        .lock()
        .unwrap()
        .as_ref()
        .map(|(products, _)| products.len())
        .unwrap_or(0);

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
