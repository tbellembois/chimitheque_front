use crate::{elog, keycloak::get_token, types::SharedPerson};
use chimitheque_types::person::Person;
use egui_phosphor_icons::icons;
use wasm_rs_shared_channel::spsc::Sender;

pub fn get_connected_user(shared_maybe_person: SharedPerson, channel_sender: Sender<i32>) {
    let request = ehttp::Request::get(format!(
        "{}/back/connecteduser",
        crate::config::BASE_URL.as_str()
    ))
    .with_headers(ehttp::Headers::new(&[
        (
            "Authorization",
            format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
        ),
        ("Content-Type", "application/json; charset=UTF-8;"),
    ]));

    let _ = channel_sender.send(&1_i32);

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

                match crate::api::parse_json_response::<Person>(&response) {
                    Ok(response_person) => {
                        *maybe_current_person = Some(response_person);
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

        elog!(info, "getting connected user \u{E06C} done".to_string());
    });
}
