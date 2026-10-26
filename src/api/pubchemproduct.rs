use crate::{elog, keycloak::get_token, types::SharedPubchemProduct};
use chimitheque_types::pubchemproduct::PubchemProduct;
use wasm_rs_shared_channel::spsc::Sender;

fn build_request(name: &String) -> ehttp::Request {
    ehttp::Request::get(format!(
        "{}/back/products/pubchemgetproductbyname/{name}",
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

pub fn get_pubchem_product(
    name: &String,
    shared_maybe_pubchem_product: SharedPubchemProduct,
    channel_sender: Sender<i32>,
) {
    let request = build_request(name);
    let name_clone = name.clone();

    let _ = channel_sender.send(&1_i32);

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

                match crate::api::parse_json_response::<PubchemProduct>(&response) {
                    Ok(response_pubchem_product) => {
                        *maybe_current_pubchem_product = Some(response_pubchem_product);
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

        elog!(
            info,
            format!("getting pubchem product with {name_clone} \u{E06C} done")
        );
    });
}
