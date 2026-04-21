use std::sync::Arc;

use crate::{
    error::apperror::AppError,
    keycloak::get_token,
    types::{SharedProductList, SharedString},
};
use chimitheque_types::{product::Product, requestfilter::RequestFilter};

pub fn retrieve_products(
    request_filter: RequestFilter,
    products: SharedProductList,
    current_info: SharedString,
    current_error: SharedString,
) -> Result<(), AppError> {
    let request = ehttp::Request::get(format!(
        "https://localhost:8443/back/products{}",
        request_filter
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]));

    let mut locked_current_error = current_error.lock().unwrap();
    let mut locked_current_info = current_info.lock().unwrap();
    *locked_current_info = Some("getting products".to_string());

    let current_error_clone = Arc::clone(&current_error);
    let current_info_clone = Arc::clone(&current_info);

    ehttp::fetch(request, move |response| {
        let mut locked_current_error = current_error_clone.lock().unwrap();
        let mut locked_current_info = current_info_clone.lock().unwrap();

        let mut locked_mutex = match products.lock() {
            Ok(locked_mutex) => locked_mutex,
            Err(e) => {
                log::error!("{}", e);
                *locked_current_error = Some(e.to_string());
                return;
            }
        };

        let response_products = parse_retrieve_products_response(response.unwrap()).unwrap();

        *locked_mutex = Some(response_products);
        *locked_current_info = None;
    });

    Ok(())
}

fn parse_retrieve_products_response(
    response: ehttp::Response,
) -> Result<(Vec<Product>, u64), AppError> {
    match response.status {
        200 => match response.text() {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => {
                    log::error!("parse_retrieve_products_response: InternalError: {}", e);
                    Err(AppError::InternalError(e.to_string()))
                }
            },
            None => {
                log::error!("parse_retrieve_products_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        },
        _ => match response.text() {
            Some(text_response) => {
                log::error!(
                    "parse_retrieve_products_response: NotOkHTTPResponse: {}",
                    text_response
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            }
            None => {
                log::error!(
                    "parse_retrieve_products_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        },
    }
}
