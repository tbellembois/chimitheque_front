// use std::sync::Arc;

use std::{
    ops::Deref,
    sync::{
        Arc, Mutex,
        atomic::{AtomicIsize, Ordering},
    },
};

use crate::{
    error::apperror::AppError,
    keycloak::get_token,
    types::{SharedProductAndCountList, SharedString},
    ui::app::LoadingState,
};
use chimitheque_types::{product::Product, requestfilter::RequestFilter};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_products_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<Product>, u64)>, String> {
    match parse_retrieve_products_response(response) {
        Ok(response) => Ok(Some(response)),
        Err(e) => {
            log::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn retrieve_products(
    request_filter: &RequestFilter,
    shared_maybe_products_and_count: SharedProductAndCountList,
    append: bool,
    current_info: &SharedString,
    current_error: &SharedString,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let mut locked_current_info = current_info.lock().unwrap_or_else(|e| {
        log::error!("{e}");
        e.into_inner()
    });
    *locked_current_info = Some(format!("getting products (offset: {offset})"));

    let current_error_clone = Arc::clone(current_error);

    ehttp::fetch(request, move |mayerr_response| {
        let mut locked_current_error = current_error_clone.lock().unwrap_or_else(|e| {
            log::error!("{e}");
            e.into_inner()
        });

        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current products.
                let mut maybe_current_products_and_count =
                    match shared_maybe_products_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            log::error!("{e}");
                            *locked_current_error = Some(e.to_string());
                            return;
                        }
                    };

                match get_products_from_response(&response) {
                    Ok(maybe_response_products_and_count) => {
                        if let Some(mut response_products_and_count) =
                            maybe_response_products_and_count
                        {
                            if append {
                                if let Some(current_products_and_count) =
                                    maybe_current_products_and_count.as_mut()
                                {
                                    current_products_and_count
                                        .0
                                        .append(&mut response_products_and_count.0);
                                } else {
                                    *maybe_current_products_and_count =
                                        Some(response_products_and_count);
                                }
                            } else {
                                *maybe_current_products_and_count =
                                    Some(response_products_and_count);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                        *locked_current_error = Some(e);
                    }
                }
            }
            Err(e) => {
                log::error!("{e}");
                *locked_current_error = Some(e);
            }
        }
    });
}

fn parse_retrieve_products_response(
    response: &ehttp::Response,
) -> Result<(Vec<Product>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_products_response: InternalError: {e}");
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_products_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!("parse_retrieve_products_response: NotOkHTTPResponse: {text_response}");
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_products_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
