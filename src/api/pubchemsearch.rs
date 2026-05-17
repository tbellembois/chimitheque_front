use crate::{
    error::apperror::AppError, keycloak::get_token, types::SharedPubchemAutocomplete,
    ui::app::ChannelMessage,
};
use chimitheque_types::pubchem::Autocomplete;
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(name: &String) -> ehttp::Request {
    ehttp::Request::get(format!(
        "https://localhost:8443/back/products/pubchemautocomplete/{name}",
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]))
}

pub fn get_pubchem_autocomplete(
    name: &String,
    shared_maybe_pubchem_autocomplete: SharedPubchemAutocomplete,
    channel_sender: Option<Sender<ChannelMessage>>,
) {
    let request = build_request(name);

    let Some(channel_sender) = channel_sender else {
        log::error!("channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&ChannelMessage::Loading(true));
    let _ = channel_sender.send(&ChannelMessage::Info(format!(
        "getting pubchem autocomplete for {name}"
    )));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current autocomplete.
                let mut maybe_current_pubchem_autocomplete =
                    match shared_maybe_pubchem_autocomplete.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            log::error!("{e}");
                            let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
                            return;
                        }
                    };

                match parse_response(&response) {
                    Ok(response_pubchem_autocomplete) => {
                        *maybe_current_pubchem_autocomplete = Some(response_pubchem_autocomplete);
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

fn parse_response(response: &ehttp::Response) -> Result<Autocomplete, AppError> {
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
