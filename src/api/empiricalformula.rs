use crate::{error::apperror::AppError, keycloak::get_token};
use chimitheque_types::{empiricalformula::EmpiricalFormula, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/empiricalformulas{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_empirical_formulas_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<EmpiricalFormula>, u64)>, String> {
    match parse_retrieve_empirical_formulas_response(response) {
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

            // We don't need to log the errors here as they are already logged in `get_empirical_formulas_from_response`.
            if let Ok(maybe_empirical_formulas) = get_empirical_formulas_from_response(&response)
                && let Some(empirical_formulas) = maybe_empirical_formulas
            {
                let items: Vec<SelectItem> = empirical_formulas
                    .0
                    .into_iter()
                    .map(|empirical_formula| SelectItem {
                        id: empirical_formula.empirical_formula_id,
                        label: empirical_formula.empirical_formula_label,
                    })
                    .collect();
                let total = match usize::try_from(empirical_formulas.1) {
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

fn parse_retrieve_empirical_formulas_response(
    response: &ehttp::Response,
) -> Result<(Vec<EmpiricalFormula>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!(
                            "parse_retrieve_empirical_formulas_response: InternalError: {e}",
                        );
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_empirical_formulas_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!(
                    "parse_retrieve_empirical_formulas_response: NotOkHTTPResponse: {text_response}",
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_empirical_formulas_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
