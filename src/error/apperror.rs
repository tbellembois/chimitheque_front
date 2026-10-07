use std::fmt;

pub enum AppError {
    InternalError(String),
    UnexpectedEmptyResponse,
    NotOkHTTPResponse(String),
}

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InternalError(s) => write!(f, "internal error: {s}"),
            AppError::UnexpectedEmptyResponse => write!(f, "unexpected empty response"),
            AppError::NotOkHTTPResponse(s) => write!(f, "HTTP response not ok: {s}"),
        }
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
