use crate::{
    error::apperror::AppError, keycloak::get_token, types::SharedPerson, ui::app::ChannelMessage,
};
use chimitheque_types::person::Person;
use wasm_rs_shared_channel::spsc::Sender;

pub fn get_connected_user(
    shared_maybe_person: SharedPerson,
    channel_sender: Option<Sender<ChannelMessage>>,
) {
    let request = ehttp::Request::get("https://localhost:8443/back/connecteduser").with_headers(
        ehttp::Headers::new(&[
            (
                "Authorization",
                format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
            ),
            ("Content-Type", "application/json; charset=UTF-8;"),
        ]),
    );

    let Some(channel_sender) = channel_sender else {
        log::error!("channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&ChannelMessage::Loading(true));
    let _ = channel_sender.send(&ChannelMessage::Info("getting connected user".to_string()));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current person.
                let mut maybe_current_person = match shared_maybe_person.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        log::error!("{e}");
                        let _ = channel_sender.send(&ChannelMessage::Error(e.to_string()));
                        return;
                    }
                };

                match parse_response(&response) {
                    Ok(response_person) => {
                        *maybe_current_person = Some(response_person);
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

fn parse_response(response: &ehttp::Response) -> Result<Person, AppError> {
    match response.status {
        200 => {
            if let Some(text_response) = response.text() {
                match serde_json::from_str(text_response) {
                    Ok(json_response) => Ok(json_response),
                    Err(e) => {
                        log::error!("parse_retrieve_connected_user_response: InternalError: {e}");
                        Err(AppError::InternalError(e.to_string()))
                    }
                }
            } else {
                log::error!("parse_retrieve_connected_user_response: UnexpectedEmptyResponse");
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text_response) = response.text() {
                log::error!(
                    "parse_retrieve_connected_user_response: NotOkHTTPResponse: {text_response}"
                );
                Err(AppError::NotOkHTTPResponse(text_response.to_string()))
            } else {
                log::error!(
                    "parse_retrieve_connected_user_response: NotOkHTTPResponse: {}",
                    response.status
                );
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}
