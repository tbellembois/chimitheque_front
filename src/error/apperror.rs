use std::fmt;

#[allow(dead_code)]
pub enum AppError {
    TestError,
    InternalError(String),
    UnexpectedEmptyResponse,
    NotOkHTTPResponse(String),
    GetUserInfoError(String),
}

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TestError => write!(f, "just a test error, nothing anormal"),
            AppError::InternalError(s) => write!(f, "internal error: {}", s),
            AppError::UnexpectedEmptyResponse => write!(f, "unexpected empty response"),
            AppError::NotOkHTTPResponse(s) => write!(f, "HTTP response not ok: {}", s),
            AppError::GetUserInfoError(s) => write!(f, "error retrieving user informations: {}", s),
        }
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
