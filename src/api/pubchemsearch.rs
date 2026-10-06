use crate::{elog, keycloak::get_token, types::SharedPubchemAutocomplete};
use chimitheque_types::pubchem::Autocomplete;
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(name: &String) -> ehttp::Request {
    ehttp::Request::get(format!(
        "{}/back/products/pubchemautocomplete/{name}",
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

pub fn get_pubchem_autocomplete(
    name: &String,
    shared_maybe_pubchem_autocomplete: SharedPubchemAutocomplete,
    channel_sender: Sender<i32>,
) {
    let request = build_request(name);
    let name_clone = name.clone();

    let _ = channel_sender.send(&1_i32);

    elog!(info, format!("getting pubchem autocomplete for {name}"));

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current autocomplete.
                let mut maybe_current_pubchem_autocomplete =
                    match shared_maybe_pubchem_autocomplete.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            elog!(error, format!("{e}"));
                            return;
                        }
                    };

                match crate::api::parse_json_response::<Autocomplete>(&response) {
                    Ok(response_pubchem_autocomplete) => {
                        *maybe_current_pubchem_autocomplete = Some(response_pubchem_autocomplete);
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
                "getting pubchem autocomplete for {name_clone} {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}
