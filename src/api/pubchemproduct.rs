use crate::{
    error::apperror::AppError, keycloak::get_token, types::SharedPubchemProduct,
    ui::app::ChannelMessage,
};
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
    channel_sender: Option<Sender<ChannelMessage>>,
) {
    let request = build_request(name);

    let Some(channel_sender) = channel_sender else {
        log::error!("channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&ChannelMessage::Loading(true));
    let _ = channel_sender.send(&ChannelMessage::Info(format!(
        "getting pubchem product with {name}"
    )));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current pubchem product.
                let mut maybe_current_pubchem_product = match shared_maybe_pubchem_product.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        log::error!("{e}");
                        let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
                        return;
                    }
                };

                match parse_response(&response) {
                    Ok(response_pubchem_product) => {
                        *maybe_current_pubchem_product = Some(response_pubchem_product);
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

fn parse_response(response: &ehttp::Response) -> Result<PubchemProduct, AppError> {
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
