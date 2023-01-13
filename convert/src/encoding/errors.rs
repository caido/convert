use base64::DecodeError as DecodeB64Error;
use core::str::Utf8Error;
use hex::FromHexError;
use std::fmt::Debug;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error("InvalidInput `{0}`")]
    InvalidInput(String),
    #[error("Error while transforming bytes to utf8 `{0}`")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("Error while transforming bytes to utf8 `{0}`")]
    Utf8Error(#[from] Utf8Error),
    #[error("Could not read as base64 value. `{0}`")]
    Base64DecodeError(#[from] DecodeB64Error),
    #[error("Could not read as hex value. `{0}`")]
    HexDecodeError(#[from] FromHexError),
}
