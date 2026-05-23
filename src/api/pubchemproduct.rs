use crate::{elog, error::apperror::AppError, keycloak::get_token, types::SharedPubchemProduct};
use chimitheque_types::pubchemproduct::PubchemProduct;
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(name: &String) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/pubchemgetproductbyname/{name}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

pub fn get_pubchem_product(
    name: &String,
    shared_maybe_pubchem_product: SharedPubchemProduct,
    is_loading_channel_sender: Option<Sender<bool>>,
) {
    let request = build_request(name);
    let name_clone = name.clone();

    let Some(channel_sender) = is_loading_channel_sender else {
        elog!(error, "is_loading_channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&true);

    elog!(info, format!("getting pubchem product with {name}"));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current pubchem product.
                let mut maybe_current_pubchem_product = match shared_maybe_pubchem_product.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        elog!(error, format!("{e}"));
                        return;
                    }
                };

                match parse_response(&response) {
                    Ok(response_pubchem_product) => {
                        *maybe_current_pubchem_product = Some(response_pubchem_product);
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
                "getting pubchem product with {name_clone} {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}

fn parse_response(response: &ehttp::Response) -> Result<PubchemProduct, AppError> {
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
