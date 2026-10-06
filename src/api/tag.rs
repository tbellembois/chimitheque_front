use crate::{elog, keycloak::get_token};
use chimitheque_types::{requestfilter::RequestFilter, tag::Tag};
use egui_select2::select2::{SelectItem, SelectItems, SharedSelect2Items};

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "{}/back/products/tags{request_filter}",
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
                    log::error!("{e}");
                    return;
                }
            };

            match crate::api::parse_json_response::<(Vec<Tag>, u64)>(&response) {
                Ok(response_tags_and_count) => {
                    let items: Vec<SelectItem> = response_tags_and_count
                        .0
                        .into_iter()
                        .map(|tag| SelectItem {
                            id: tag.tag_id,
                            label: tag.tag_label,
                        })
                        .collect();
                    let total = match usize::try_from(response_tags_and_count.1) {
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
