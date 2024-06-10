use crate::ui::{app::App, state::Page};
use log::error;

pub fn check_storelocations_promise(app: &mut App) {
    if let Some(p) = &app.promise_storelocations {
        if let Some(try_storelocations) = p.ready() {
            match try_storelocations {
                Ok((storelocations, count)) => {
                    app.storelocations = Some((storelocations.to_vec(), *count));
                    app.state.active_page = Page::StorelocationList;
                }
                Err(e) => {
                    app.current_error = Some(crate::error::apperror::AppError::InternalError(
                        e.to_string(),
                    ));
                    error!("promise_storelocations error: {e}");
                }
            }
            app.promise_storelocations = None;
        }
    }
}

pub fn check_products_promise(app: &mut App) {
    if let Some(p) = &app.promise_products {
        if let Some(try_products) = p.ready() {
            match try_products {
                Ok((products, count)) => {
                    app.products = Some((products.to_vec(), *count));
                    app.state.active_page = Page::ProductList;
                }
                Err(e) => {
                    app.current_error = Some(crate::error::apperror::AppError::InternalError(
                        e.to_string(),
                    ));
                    error!("promise_products error: {e}");
                }
            }
            app.promise_products = None;
        }
    }
}
