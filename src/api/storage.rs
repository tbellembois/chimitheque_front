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
        "{}/back/storages{request_filter}",
        crate::config::BASE_URL.as_str(),
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
        "{}/back/exportstorages{request_filter}",
        crate::config::BASE_URL.as_str(),
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
    channel_sender: Sender<i32>,
) {
    let mut request_filter = request_filter.clone();
    request_filter.offset = None;
    request_filter.limit = None;
    let request = build_export_request(&request_filter);

    let _ = channel_sender.send(&1_i32);

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

                match match response.status {
                    200 => response
                        .text()
                        .map(std::string::ToString::to_string)
                        .ok_or(AppError::UnexpectedEmptyResponse),
                    _ => Err(AppError::NotOkHTTPResponse(
                        response
                            .text().map_or_else(|| response.status.to_string(), std::string::ToString::to_string),
                    )),
                } {
                    Ok(response_string) => {
                        *maybe_string = Some(response_string);
                    }
                    Err(e) => {
                        elog!(error, format!("{e}"));
                    }
                }
            }
            Err(e) => {
                elog!(error, e.clone());
            }
        }

        let _ = channel_sender.send(&(-1_i32));

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
    channel_sender: Sender<i32>,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let _ = channel_sender.send(&1_i32);

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

                match crate::api::parse_json_response::<(Vec<Storage>, u64)>(&response) {
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
                elog!(error, e.clone());
            }
        }

        let _ = channel_sender.send(&(-1_i32));

        elog!(
            info,
            format!(
                "getting storages (offset: {offset}) {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}
