use crate::{elog, keycloak::get_token, types::SharedProductAndCountList};
use chimitheque_types::{product::Product, requestfilter::RequestFilter};
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(request_filter: &RequestFilter) -> ehttp::Request {
    ehttp::Request::get(format!(
        "{}/back/products{request_filter}",
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

pub fn get_products(
    request_filter: &RequestFilter,
    shared_maybe_products_and_count: SharedProductAndCountList,
    append: bool,
    channel_sender: Sender<i32>,
) {
    let offset = request_filter.offset.unwrap_or_default();
    let request = build_request(request_filter);

    let _ = channel_sender.send(&1_i32);

    elog!(info, format!("getting products (offset: {offset})"));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current products.
                let mut maybe_current_products_and_count =
                    match shared_maybe_products_and_count.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            elog!(error, format!("{e}"));
                            return;
                        }
                    };

                match crate::api::parse_json_response::<(Vec<Product>, u64)>(&response) {
                    Ok(mut response_products_and_count) => {
                        if append {
                            if let Some(current_products_and_count) =
                                maybe_current_products_and_count.as_mut()
                            {
                                current_products_and_count
                                    .0
                                    .append(&mut response_products_and_count.0);
                            } else {
                                *maybe_current_products_and_count =
                                    Some(response_products_and_count);
                            }
                        } else {
                            *maybe_current_products_and_count = Some(response_products_and_count);
                        }
                    }
                    Err(e) => {
                        elog!(error, format!("{e}"));
                    }
                }
            }
            Err(e) => {
                elog!(error, e.to_string());
            }
        }

        let _ = channel_sender.send(&(-1_i32));

        elog!(
            info,
            format!(
                "getting products (offset: {offset}) {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}
