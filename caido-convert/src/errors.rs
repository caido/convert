use core::str::Utf8Error;
use std::fmt::Debug;
use std::string::FromUtf8Error;
use thiserror::Error;

use base64::DecodeError as DecodeB64Error;
use hex::FromHexError;

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Utf8 decode error")]
    DecodeUtf8Error,

    #[error("Decode error")]
    DecodeError(String),
}

impl From<Utf8Error> for OperationError {
    fn from(_value: Utf8Error) -> Self {
        OperationError::DecodeUtf8Error
    }
}

impl From<FromUtf8Error> for OperationError {
    fn from(_value: FromUtf8Error) -> Self {
        OperationError::DecodeUtf8Error
    }
}

impl From<DecodeB64Error> for OperationError {
    fn from(_value: DecodeB64Error) -> Self {
        OperationError::DecodeError("Invalid Base64 input".to_string())
    }
}

impl From<FromHexError> for OperationError {
    fn from(_value: FromHexError) -> Self {
        OperationError::DecodeError("Invalid Hex input".to_string())
    }
}
