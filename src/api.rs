use crate::error::apperror::AppError;

/// Generic response parser: deserializes a 200 JSON body into `T`,
/// or maps HTTP errors to `AppError`.
pub(crate) fn parse_json_response<T>(response: &ehttp::Response) -> Result<T, AppError>
where
    T: serde::de::DeserializeOwned,
{
    match response.status {
        200 => {
            if let Some(text) = response.text() {
                serde_json::from_str(text).map_err(|e| AppError::InternalError(e.to_string()))
            } else {
                Err(AppError::UnexpectedEmptyResponse)
            }
        }
        _ => {
            if let Some(text) = response.text() {
                Err(AppError::NotOkHTTPResponse(text.to_string()))
            } else {
                Err(AppError::NotOkHTTPResponse(response.status.to_string()))
            }
        }
    }
}

pub(crate) mod casnumber;
pub(crate) mod category;
pub(crate) mod cenumber;
pub(crate) mod classofcompound;
pub(crate) mod connecteduser;
pub(crate) mod empiricalformula;
pub(crate) mod entity;
pub(crate) mod hazardstatement;
pub(crate) mod linearformula;
pub(crate) mod name;
pub(crate) mod people;
pub(crate) mod permission;
pub(crate) mod physicalstate;
pub(crate) mod precautionarystatement;
pub(crate) mod producerref;
pub(crate) mod product;
pub(crate) mod pubchemproduct;
pub(crate) mod pubchemsearch;
pub(crate) mod signalword;
pub(crate) mod storage;
pub(crate) mod storelocation;
pub(crate) mod symbol;
pub(crate) mod tag;
pub(crate) mod unitmolecularweight;
pub(crate) mod unittemperature;
pub(crate) mod versioninfo;
