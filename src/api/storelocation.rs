use crate::{
    error::apperror::AppError, keycloak::get_token, types::SharedStoreLocationAndCountList,
    ui::app::ChannelMessage,
};
use chimitheque_types::{requestfilter::RequestFilter, storelocation::StoreLocation};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/store_locations{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn get_store_locations_from_response(
    response: &ehttp::Response,
) -> Result<Option<(Vec<StoreLocation>, u64)>, String> {
    match parse_retrieve_store_locations_response(response) {
        Ok(response) => Ok(Some(response)),
        Err(e) => {
            log::error!("{e}");
            Err(e.to_string())
        }
    }
}

pub fn retrieve_store_locations(
    request_filter: &RequestFilter,
    shared_maybe_store_locations_and_count: SharedStoreLocationAndCountList,
    append: bool,
    channel_sender: Option<Sender<ChannelMessage>>,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let Some(channel_sender) = channel_sender else {
        log::error!("channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&ChannelMessage::Loading(true));
    let _ = channel_sender.send(&ChannelMessage::Info(format!(
        "getting store locations (offset: {offset})"
    )));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current store locations.
                let mut maybe_current_store_locations_and_count =
                    match shared_maybe_store_locations_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            log::error!("{e}");
                            let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
                            return;
                        }
                    };

                match get_store_locations_from_response(&response) {
                    Ok(maybe_response_store_locations_and_count) => {
                        if let Some(mut response_store_locations_and_count) =
                            maybe_response_store_locations_and_count
                        {
                            if append {
                                if let Some(current_store_locations_and_count) =
                                    maybe_current_store_locations_and_count.as_mut()
                                {
                                    current_store_locations_and_count
                                        .0
                                        .append(&mut response_store_locations_and_count.0);
                                } else {
                                    *maybe_current_store_locations_and_count =
                                        Some(response_store_locations_and_count);
                                }
                            } else {
                                *maybe_current_store_locations_and_count =
                                    Some(response_store_locations_and_count);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                        let _ = channel_sender.send(&ChannelMessage::Error(e));
                    }
                }
            }
            Err(e) => {
                log::error!("{e}");
                let _ = channel_sender.send(&ChannelMessage::Error(e));
            }
        }

        let _ = channel_sender.send(&ChannelMessage::Loading(false));
        let _ = channel_sender.send(&ChannelMessage::Info("done".to_string()));
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

            // We don't need to log the errors here as they are already logged in `get_store_locations_from_response`.
            if let Ok(maybe_store_locations) = get_store_locations_from_response(&response)
                && let Some(store_locations) = maybe_store_locations
            {
                let items: Vec<SelectItem> = store_locations
                    .0
                    .into_iter()
                    .map(|store_location| SelectItem {
                        id: store_location.store_location_id,
                        label: store_location.store_location_name,
                    })
                    .collect();
                let total = match usize::try_from(store_locations.1) {
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

fn parse_retrieve_store_locations_response(
    response: &ehttp::Response,
) -> Result<(Vec<StoreLocation>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_store_locations_response: InternalError: {e}");
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_store_locations_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!(
                    "parse_retrieve_store_locations_response: NotOkHTTPResponse: {text_response}",
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_store_locations_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
