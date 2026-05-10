use std::sync::Arc;

use crate::{
    error::apperror::AppError,
    keycloak::get_token,
    types::{SharedEntityAndCountList, SharedString},
};
use chimitheque_types::{entity::Entity, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/entities{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_entities_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<Entity>, u64)>, String> {
    match parse_retrieve_entities_response(response) {
        Ok(response) => Ok(Some(response)),
        Err(e) => {
            log::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn retrieve_entities(
    request_filter: &RequestFilter,
    shared_entities: SharedEntityAndCountList,
    current_info: &SharedString,
    current_error: &SharedString,
) {
    let request = build_request(request_filter);

    let mut locked_current_info = current_info.lock().unwrap_or_else(|e| {
        log::error!("{e}");
        e.into_inner()
    });
    *locked_current_info = Some("getting entities".to_string());

    let current_error_clone = Arc::clone(current_error);

    ehttp::fetch(request, move |mayerr_response| {
        let mut locked_current_error = current_error_clone.lock().unwrap_or_else(|e| {
            log::error!("{e}");
            e.into_inner()
        });

        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current entities.
                let mut current_entities = match shared_entities.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        log::error!("{e}");
                        *locked_current_error = Some(e.to_string());
                        return;
                    }
                };

                match get_entities_from_response(&response) {
                    Ok(maybe_entities) => {
                        if let Some(entities) = maybe_entities {
                            *current_entities = Some(entities);
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

pub fn load_suggestions(
    shared_suggestions: SharedSelect2Items,
    limit: usize,
    offset: usize,
    query: String,
) {
    let request = build_request(&RequestFilter {
        search: Some(query),
        limit: Some(limit as u64),
        offset: Some(offset as u64),
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

            // We don't need to log the errors here as they are already logged in `get_entities_from_response`.
            if let Ok(maybe_entities) = get_entities_from_response(&response)
                && let Some(entities) = maybe_entities
            {
                let items: Vec<SelectItem> = entities
                    .0
                    .into_iter()
                    .map(|entity| SelectItem {
                        id: entity.entity_id,
                        label: entity.entity_name,
                    })
                    .collect();
                let total = entities.1 as usize;

                *current_suggestions = Some(SelectItems { items, total });
            }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_retrieve_entities_response(
    response: &ehttp::Response,
) -> Result<(Vec<Entity>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_entities_response: InternalError: {e}",);
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_entities_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!("parse_retrieve_entities_response: NotOkHTTPResponse: {text_response}",);
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_entities_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
