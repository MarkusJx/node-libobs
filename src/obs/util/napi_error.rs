use crate::obs::util::types::ResultType;
use std::error::Error;

pub fn to_napi_error(err: Box<dyn Error>) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
}

pub fn to_napi_error_str(err: &str) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
}

pub fn to_napi_error_string(err: String) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, err)
}

pub trait MapToNapiError<T> {
    fn map_napi_err(self) -> napi::Result<T>;
}

impl<T> MapToNapiError<T> for ResultType<T> {
    fn map_napi_err(self) -> napi::Result<T> {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(to_napi_error(err)),
        }
    }
}
