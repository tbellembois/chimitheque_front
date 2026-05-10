use std::sync::Arc;

use chimitheque_types::requestfilter::RequestFilter;
use egui::ScrollArea;

use crate::{
    api::product::retrieve_products,
    ui::{
        app::{App, LoadingState},
        widgets::productlabel::render_product_label,
    },
};

pub fn update(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let output = egui::ScrollArea::vertical()
        .id_salt("products_scrollarea")
        .show(ui, |ui| {
            if let Some((products, _)) = app.products.clone().lock().unwrap().as_ref() {
                for product in products {
                    render_product_label(app, ui, frame, product.clone());
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
        app.current_search_offset += 10;

        // if *locked_loading_state != LoadingState::LoadingForOffset(app.current_search_offset as u64)
        //     && *locked_loading_state
        //         != LoadingState::LoadedForOffset(app.current_search_offset as u64)
        // {
        retrieve_products(
            &RequestFilter {
                offset: Some(app.current_search_offset as u64),
                limit: Some(10),
                ..Default::default()
            },
            Arc::clone(&app.products),
            true,
            // Arc::clone(&app.loading_state),
            &Arc::clone(&app.current_info),
            &Arc::clone(&app.current_error),
        );
        // }
    }

    app.state.scrollarea_was_near_bottom = near_bottom;
}
