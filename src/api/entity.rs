use crate::{
    error::apperror::AppError, keycloak::get_token, types::SharedEntityAndCountList,
    ui::app::ChannelMessage,
};
use chimitheque_types::{entity::Entity, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};
use wasm_rs_shared_channel::spsc::Sender;

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

pub fn get_entities(
    request_filter: &RequestFilter,
    shared_maybe_entities_and_count: SharedEntityAndCountList,
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
        "getting entities (offset: {offset})"
    )));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current entities.
                let mut maybe_current_entities_and_count =
                    match shared_maybe_entities_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            log::error!("{e}");
                            let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
                            return;
                        }
                    };

                match parse_response(&response) {
                    Ok(mut response_entities_and_count) => {
                        if append {
                            if let Some(current_entities_and_count) =
                                maybe_current_entities_and_count.as_mut()
                            {
                                current_entities_and_count
                                    .0
                                    .append(&mut response_entities_and_count.0);
                            } else {
                                *maybe_current_entities_and_count =
                                    Some(response_entities_and_count);
                            }
                        } else {
                            *maybe_current_entities_and_count = Some(response_entities_and_count);
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                        let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
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
            let mut maybe_current_suggestions = match shared_suggestions.lock() {
                Ok(locked) => locked,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            // We don't need to log the errors here as they are already logged in `get_store_locations_from_response`.
            match parse_response(&response) {
                Ok(response_entities_and_count) => {
                    let items: Vec<SelectItem> = response_entities_and_count
                        .0
                        .into_iter()
                        .map(|entity| SelectItem {
                            id: entity.entity_id,
                            label: entity.entity_name,
                        })
                        .collect();
                    let total = match usize::try_from(response_entities_and_count.1) {
                        Ok(total) => total,
                        Err(e) => {
                            log::error!("{e}");
                            return;
                        }
                    };
                    *maybe_current_suggestions = Some(SelectItems { items, total });
                }
                Err(e) => {
                    log::error!("{e}");
                }
            }

            // if let Ok(maybe_entities) = parse_response(&response)
            //     && let Some(entities) = maybe_entities
            // {
            //     let items: Vec<SelectItem> = entities
            //         .0
            //         .into_iter()
            //         .map(|entity| SelectItem {
            //             id: entity.entity_id,
            //             label: entity.entity_name,
            //         })
            //         .collect();
            //     let total = match usize::try_from(entities.1) {
            //         Ok(total) => total,
            //         Err(e) => {
            //             log::error!("{e}");
            //             return;
            //         }
            //     };

            //     *current_suggestions = Some(SelectItems { items, total });
            // }
        }
        Err(e) => {
            log::error!("{e}");
        }
    });
}

fn parse_response(response: &ehttp::Response) -> Result<(Vec<Entity>, u64), AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_response: InternalError: {e}");
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!("parse_response: NotOkHTTPResponse: {text_response}");
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!("parse_response: NotOkHTTPResponse: {}", response.status);
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
