use crate::error::apperror::AppError;
use chimitheque_types::storelocation::Storelocation;
use log::debug;
use poll_promise::Promise;

pub fn retrieve_storelocations(
    ctx: &egui::Context,
) -> Promise<Result<(Vec<Storelocation>, u64), AppError>> {
    debug!("retrieve_storelocations");

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get("http://localhost:8081/storelocations");

    ehttp::fetch(request, move |response| {
        let user_info = match response {
            Ok(r) => parse_retrieve_storelocations_response(r),
            Err(e) => Err(AppError::InternalError(e)),
        };

        sender.send(user_info);
        ctx.request_repaint();
    });

    promise
}

fn parse_retrieve_storelocations_response(
    response: ehttp::Response,
) -> Result<(Vec<Storelocation>, u64), AppError> {
    debug!("{:?}", response.text());

    match response.status {
        200 => match response.text() {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => Err(AppError::InternalError(e.to_string())),
            },
            None => Err(AppError::UnexpectedEmptyResponse),
        },
        _ => match response.text() {
            Some(text_response) => Err(AppError::NotOkHTTPResponse(text_response.to_string())),
            None => Err(AppError::NotOkHTTPResponse(response.status.to_string())),
        },
    }
}
