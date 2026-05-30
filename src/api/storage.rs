use crate::{
    elog,
    error::apperror::AppError,
    keycloak::get_token,
    types::{SharedStorageAndCountList, SharedString},
};
use chimitheque_types::{requestfilter::RequestFilter, storage::Storage};
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/storages{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

fn build_export_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/exportstorages{request_filter}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

pub fn export_storages(
    request_filter: &RequestFilter,
    shared_maybe_string: SharedString,
    is_loading_channel_sender: Option<Sender<bool>>,
) {
    let mut request_filter = request_filter.clone();
    request_filter.offset = None;
    request_filter.limit = None;
    let request = build_export_request(&request_filter);

    let Some(channel_sender) = is_loading_channel_sender else {
        elog!(error, "is_loading_channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&true);

    elog!(info, "exporting storages");

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current storages.
                let mut maybe_string = match shared_maybe_string.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        elog!(error, format!("{e}"));
                        return;
                    }
                };

                match parse_export_response(&response) {
                    Ok(response_string) => {
                        *maybe_string = Some(response_string);
                    }
                    Err(e) => {
                        elog!(error, format!("{e}"));
                    }
                }
            }
            Err(e) => {
                elog!(error, format!("{e}"));
            }
        }

        let _ = channel_sender.send(&false);

        elog!(
            info,
            format!(
                "exporting storages {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}

pub fn get_storages(
    request_filter: &RequestFilter,
    shared_maybe_storages_and_count: SharedStorageAndCountList,
    append: bool,
    is_loading_channel_sender: Option<Sender<bool>>,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let Some(channel_sender) = is_loading_channel_sender else {
        elog!(error, "is_loading_channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&true);

    elog!(info, format!("getting storages (offset: {offset})"));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current storages.
                let mut maybe_current_storages_and_count =
                    match shared_maybe_storages_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            elog!(error, format!("{e}"));
                            return;
                        }
                    };

                match parse_response(&response) {
                    Ok(mut response_storages_and_count) => {
                        if append {
                            if let Some(current_storages_and_count) =
                                maybe_current_storages_and_count.as_mut()
                            {
                                current_storages_and_count
                                    .0
                                    .append(&mut response_storages_and_count.0);
                            } else {
                                *maybe_current_storages_and_count =
                                    Some(response_storages_and_count);
                            }
                        } else {
                            *maybe_current_storages_and_count = Some(response_storages_and_count);
                        }
                    }
                    Err(e) => {
                        elog!(error, format!("{e}"));
                    }
                }
            }
            Err(e) => {
                elog!(error, format!("{e}"));
            }
        }

        let _ = channel_sender.send(&false);

        elog!(
            info,
            format!(
                "getting storages (offset: {offset}) {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}

fn parse_response(response: &ehttp::Response) -> Result<(Vec<Storage>, u64), AppError> {
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

fn parse_export_response(response: &ehttp::Response) -> Result<String, AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                Ok(text_response.to_string())
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
