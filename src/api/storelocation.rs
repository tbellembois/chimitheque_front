use crate::error::apperror::AppError;
use chimitheque_types::{requestfilter::RequestFilter, storelocation::StoreLocation};
use std::sync::{Arc, Mutex};

pub fn retrieve_store_locations(
    request_filter: RequestFilter,
    store_locations: Arc<Mutex<Option<(Vec<StoreLocation>, u64)>>>,
) -> Result<(), AppError> {
    let request = ehttp::Request::get(format!(
        "https://localhost:8443/back/store_locations{}",
        request_filter
    ));

    ehttp::fetch(request, move |response| {
        let mut locked_mutex = store_locations.lock().unwrap();

        let response_store_locations =
            parse_retrieve_store_locations_response(response.unwrap()).unwrap();

        *locked_mutex = Some(response_store_locations);
    });

    Ok(())
}

fn parse_retrieve_store_locations_response(
    response: ehttp::Response,
) -> Result<(Vec<StoreLocation>, u64), AppError> {
    match response.status {
        200 => match response.text() {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => {
                    log::error!(
                        "parse_retrieve_store_locations_response: InternalError: {}",
                        e
                    );
                    Err(AppError::InternalError(e.to_string()))
                }
            },
            None => {
                log::error!("parse_retrieve_store_locations_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        },
        _ => match response.text() {
            Some(text_response) => {
                log::error!(
                    "parse_retrieve_store_locations_response: NotOkHTTPResponse: {}",
                    text_response
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            }
            None => {
                log::error!(
                    "parse_retrieve_store_locations_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        },
    }
}
