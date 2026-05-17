use crate::{error::apperror::AppError, keycloak::get_token};
use chimitheque_types::{producerref::ProducerRef, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/producerrefs{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_producer_refs_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<ProducerRef>, u64)>, String> {
    match parse_retrieve_producer_refs_response(response) {
        Ok(response) => Ok(Some(response)),
        Err(e) => {
            log::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn load_suggestions(
    shared_suggestions: SharedSelect2Items,
    limit: usize,
    offset: usize,
    query: String,
) {
    let request = build_request(&RequestFilter {
        search: Some(query),
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    });

    ehttp::fetch(request, move |mayerr_response| match mayerr_response {
        Ok(response) => {
            // Acquire lock on current suggestions.
            let mut current_suggestions = match shared_suggestions.lock() {
                Ok(locked) => locked,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            // We don't need to log the errors here as they are already logged in `get_producer_refs_from_response`.
            if let Ok(maybe_producer_refs) = get_producer_refs_from_response(&response)
                && let Some(producer_refs) = maybe_producer_refs
            {
                let items: Vec<SelectItem> = producer_refs
                    .0
                    .into_iter()
                    .map(|producer_ref| SelectItem {
                        id: producer_ref.producer_ref_id,
                        label: format!(
                            "{} ({})",
                            producer_ref.producer_ref_label, producer_ref.producer.producer_label
                        ),
                    })
                    .collect();
                let total = match usize::try_from(producer_refs.1) {
                    Ok(total) => total,
                    Err(e) => {
                        log::error!("{e}");
                        return;
                    }
                };

                *current_suggestions = Some(SelectItems { items, total });
            }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_retrieve_producer_refs_response(
    response: &ehttp::Response,
) -> Result<(Vec<ProducerRef>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_producer_refs_response: InternalError: {e}");
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_producer_refs_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!(
                    "parse_retrieve_producer_refs_response: NotOkHTTPResponse: {text_response}",
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_producer_refs_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
