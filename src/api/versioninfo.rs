use crate::{elog, keycloak::get_token, types::SharedVersionInfo};
use chimitheque_types::versioninfo::VersionInfo;
use wasm_rs_shared_channel::spsc::Sender;

pub fn get_version_info(shared_maybe_version_info: SharedVersionInfo, channel_sender: Sender<i32>) {
    let request = ehttp::Request::get(format!(
        "{}/back/versioninfo",
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

    elog!(info, "getting version info");

    ehttp::fetch(request, move |mayerr_response| {
        match mayerr_response {
            Ok(response) => {
                // Acquire lock on current version info.
                let mut maybe_current_version_info = match shared_maybe_version_info.lock() {
                    Ok(locked) => locked,
                    Err(e) => {
                        elog!(error, format!("{e}"));
                        return;
                    }
                };

                match crate::api::parse_json_response::<VersionInfo>(&response) {
                    Ok(response_version_info) => {
                        *maybe_current_version_info = Some(response_version_info);
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

        let _ = channel_sender.send(&(-1_i32));

        elog!(
            info,
            format!(
                "getting version info {} done",
                egui_phosphor::fill::ARROW_RIGHT
            )
        );
    });
}
