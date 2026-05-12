use crate::{error::apperror::AppError, keycloak::get_token};
use chimitheque_types::{requestfilter::RequestFilter, symbol::Symbol};
use egui::{Image, Response, Ui, Vec2};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/symbols{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_symbols_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<Symbol>, u64)>, String> {
    match parse_retrieve_symbols_response(response) {
        Ok(response) => Ok(Some(response)),
        Err(e) => {
            log::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn format_suggestion(ui: &mut Ui, selected: bool, select_item: &SelectItem) -> Response {
    let image = match select_item.label.as_str() {
        "GHS01" => egui::Image::new(egui::include_image!("../assets/GHS01.svg")).corner_radius(5.0),
        "GHS02" => egui::Image::new(egui::include_image!("../assets/GHS02.svg")).corner_radius(5.0),
        "GHS03" => egui::Image::new(egui::include_image!("../assets/GHS03.svg")).corner_radius(5.0),
        "GHS04" => egui::Image::new(egui::include_image!("../assets/GHS04.svg")).corner_radius(5.0),
        "GHS05" => egui::Image::new(egui::include_image!("../assets/GHS05.svg")).corner_radius(5.0),
        "GHS06" => egui::Image::new(egui::include_image!("../assets/GHS06.svg")).corner_radius(5.0),
        "GHS07" => egui::Image::new(egui::include_image!("../assets/GHS07.svg")).corner_radius(5.0),
        "GHS08" => egui::Image::new(egui::include_image!("../assets/GHS08.svg")).corner_radius(5.0),
        "GHS09" => egui::Image::new(egui::include_image!("../assets/GHS09.svg")).corner_radius(5.0),
        _ => egui::Image::new(egui::include_image!("../assets/wrong.svg")).corner_radius(5.0),
    };

    let image = image.fit_to_exact_size(Vec2::new(40.0, 40.0));

    ui.add(egui::Button::image_and_text(image, select_item.label.clone()).selected(selected))
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

            // We don't need to log the errors here as they are already logged in `get_symbols_from_response`.
            if let Ok(maybe_symbols) = get_symbols_from_response(&response)
                && let Some(symbols) = maybe_symbols
            {
                let items: Vec<SelectItem> = symbols
                    .0
                    .into_iter()
                    .map(|symbol| SelectItem {
                        id: symbol.symbol_id,
                        label: symbol.symbol_label,
                    })
                    .collect();
                let total = symbols.1 as usize;

                *current_suggestions = Some(SelectItems { items, total });
            }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_retrieve_symbols_response(
    response: &ehttp::Response,
) -> Result<(Vec<Symbol>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_symbols_response: InternalError: {e}",);
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_symbols_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!("parse_retrieve_symbols_response: NotOkHTTPResponse: {text_response}",);
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_symbols_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
