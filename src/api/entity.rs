use crate::{elog, keycloak::get_token, types::SharedEntityAndCountList};
use chimitheque_types::{entity::Entity, requestfilter::RequestFilter};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "{}/back/entities{request_filter}",
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

pub fn get_entities(
    request_filter: &RequestFilter,
    shared_maybe_entities_and_count: SharedEntityAndCountList,
    append: bool,
    channel_sender: Sender<i32>,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let _ = channel_sender.send(&1_i32);

    elog!(info, format!("getting entities (offset: {offset})"));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current entities.
                let mut maybe_current_entities_and_count =
                    match shared_maybe_entities_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            elog!(error, format!("{e}"));
                            return;
                        }
                    };

                match crate::api::parse_json_response::<(Vec<Entity>, u64)>(&response) {
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
                        elog!(error, format!("{e}"));
                        return;
                    }
                }
            }
            Err(e) => {
                elog!(error, e.clone());
                return;
            }
        }

        let _ = channel_sender.send(&(-1_i32));

        elog!(
            info,
            format!(
                "getting entities (offset: {offset}) {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}

pub fn load_suggestions(
    shared_suggestions: &SharedSelect2Items,
    limit: usize,
    offset: usize,
    query: &str,
) {
    let request = build_request(&RequestFilter {
        search: Some(query.to_string()),
        limit: Some(limit),
        offset: Some(offset),
        ..Default::default()
    });

    let shared_suggestions = shared_suggestions.clone();

    ehttp::fetch(request, move |mayerr_response| match mayerr_response {
        Ok(response) => {
            // Acquire lock on current suggestions.
            let mut maybe_current_suggestions = match shared_suggestions.lock() {
                Ok(locked) => locked,
                Err(e) => {
                    elog!(error, format!("{e}"));
                    return;
                }
            };

            match crate::api::parse_json_response::<(Vec<Entity>, u64)>(&response) {
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
            elog!(error, e.clone());
        }
    });
}
