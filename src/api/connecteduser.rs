use crate::{elog, error::apperror::AppError, keycloak::get_token, types::SharedPerson};
use chimitheque_types::person::Person;
use wasm_rs_shared_channel::spsc::Sender;

pub fn get_connected_user(
    shared_maybe_person: SharedPerson,
    is_loading_channel_sender: Option<Sender<bool>>,
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

    let Some(channel_sender) = is_loading_channel_sender else {
        elog!(error, "is_loading_channel_sender is None");
        return;
    };

    let _ = channel_sender.send(&true);

    elog!(info, "getting connected user");

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current person.
                let mut maybe_current_person = match shared_maybe_person.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        elog!(error, format!("{e}"));
                        return;
                    }
                };

                match parse_response(&response) {
                    Ok(response_person) => {
                        *maybe_current_person = Some(response_person);
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
                "getting connected user {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}

fn parse_response(response: &ehttp::Response) -> Result<Person, AppError> {
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
