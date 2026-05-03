use crate::{error::apperror::AppError, keycloak::get_token};
use chimitheque_types::{category::Category, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/categories{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_categorys_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<Category>, u64)>, String> {
    match parse_retrieve_categorys_response(response) {
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

            // We don't need to log the errors here as they are already logged in `get_categorys_from_response`.
            if let Ok(maybe_categorys) = get_categorys_from_response(&response)
                && let Some(categorys) = maybe_categorys
            {
                let items: Vec<SelectItem> = categorys
                    .0
                    .into_iter()
                    .map(|category| SelectItem {
                        id: category.category_id,
                        label: category.category_label,
                    })
                    .collect();
                let total = categorys.1 as usize;

                *current_suggestions = Some(SelectItems { items, total });
            }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_retrieve_categorys_response(
    response: &ehttp::Response,
) -> Result<(Vec<Category>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_categorys_response: InternalError: {e}",);
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_categorys_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!(
                    "parse_retrieve_categorys_response: NotOkHTTPResponse: {text_response}",
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_categorys_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
