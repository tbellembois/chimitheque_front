use crate::error::apperror::AppError;
use chimitheque_types::person::Person;
use std::sync::{Arc, Mutex};

pub fn retrieve_connected_user(products: Arc<Mutex<Option<Person>>>) -> Result<(), AppError> {
    let request = ehttp::Request::get("https://localhost:8443/back/connecteduser");

    ehttp::fetch(request, move |response| {
        let mut locked_mutex = products.lock().unwrap();

        let response_products = parse_retrieve_connected_user_response(response.unwrap()).unwrap();

        *locked_mutex = Some(response_products);
    });

    Ok(())
}

fn parse_retrieve_connected_user_response(response: ehttp::Response) -> Result<Person, AppError> {
    match response.status {
        200 => match response.text() {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => {
                    log::error!(
                        "parse_retrieve_connected_user_response: InternalError: {}",
                        e,
                    );
                    Err(AppError::InternalError(e.to_string()))
                }
            },
            None => {
                log::error!("parse_retrieve_connected_user_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        },
        _ => match response.text() {
            Some(text_response) => {
                log::error!(
                    "parse_retrieve_connected_user_response: NotOkHTTPResponse: {}",
                    text_response
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            }
            None => {
                log::error!(
                    "parse_retrieve_connected_user_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        },
    }
}
