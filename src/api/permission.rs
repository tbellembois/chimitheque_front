use crate::{
    elog,
    error::apperror::AppError,
    keycloak::get_token,
    types::{PermissionStatus, SharedPermissionList},
};

pub fn get_permissions(maybe_permissions: &SharedPermissionList) {
    let mut permissions_lock = match maybe_permissions.lock() {
        Ok(locked) => locked,
        Err(e) => {
            elog!(error, e.to_string());
            return;
        }
    };

    let mut ehttp_requests = Vec::new();

    {
        for permission in permissions_lock.iter_mut() {
            if permission.status == PermissionStatus::ToRetrieve {
                permission.status = PermissionStatus::InProgress;

                let mut url = "https://localhost:8443/back/f".to_string();
                url.push_str(format!("/{}", permission.item).as_str());

                if let Some(entity) = permission.entity {
                    url.push_str(format!("/{entity}").as_str());
                }

                ehttp_requests.push((
                    permission.unique_id,
                    ehttp::Request::new(
                        permission.http_method.clone(),
                        url,
                        ehttp::Headers::new(&[
                            (
                                "Authorization",
                                format!("Bearer {}", get_token().unwrap_or_default()).as_str(),
                            ),
                            ("Content-Type", "application/json; charset=UTF-8;"),
                        ]),
                    ),
                ));
            }
        }
    }

    for request in ehttp_requests {
        send_request(request.0, &request.1, maybe_permissions.clone());
    }
}

fn send_request(
    unique_id: usize,
    request: &ehttp::Request,
    maybe_permissions: SharedPermissionList,
) {
    ehttp::fetch(
        request.clone(),
        move |mayerr_response| match mayerr_response {
            Ok(response) => match parse_response(&response) {
                Ok(granted) => {
                    let mut permissions_lock = match maybe_permissions.lock() {
                        Ok(locked) => locked,
                        Err(e) => {
                            elog!(error, e.to_string());
                            return;
                        }
                    };

                    if let Some(permission) = permissions_lock
                        .iter_mut()
                        .find(|p| p.unique_id == unique_id)
                    {
                        permission.granted = granted;
                        permission.status = PermissionStatus::Done;
                    }
                }
                Err(e) => {
                    elog!(error, e.to_string());
                }
            },
            Err(e) => {
                elog!(error, e);
            }
        },
    );
}

fn parse_response(response: &ehttp::Response) -> Result<bool, AppError> {
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
