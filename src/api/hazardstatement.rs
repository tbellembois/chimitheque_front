use crate::{elog, error::apperror::AppError, keycloak::get_token};
use chimitheque_types::{hazardstatement::HazardStatement, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/hazardstatements{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
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
            let mut maybe_current_suggestions = match shared_suggestions.lock() {
                Ok(locked) => locked,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            match parse_response(&response) {
                Ok(response_hazard_statements_and_count) => {
                    let items: Vec<SelectItem> = response_hazard_statements_and_count
                        .0
                        .into_iter()
                        .map(|hazard_statement| SelectItem {
                            id: hazard_statement.hazard_statement_id,
                            label: hazard_statement.hazard_statement_label,
                        })
                        .collect();
                    let total = match usize::try_from(response_hazard_statements_and_count.1) {
                        Ok(total) => total,
                        Err(e) => {
                            elog!(error, format!("{e}"));
                            return;
                        }
                    };
                    *maybe_current_suggestions = Some(SelectItems { items, total });
                }
                Err(e) => {
                    elog!(error, format!("{e}"));
                }
            }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_response(response: &ehttp::Response) -> Result<(Vec<HazardStatement>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => Err(AppError::InternalError(e.to_string())),
                }
            } else {
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
