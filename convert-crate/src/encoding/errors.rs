use std::fmt::Debug;
use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error("InvalidInput `{0}`")]
    InvalidInput(String),
    #[error("Error while transforming bytes to utf8 `{0}`")]
    Utf8Error(#[from] Utf8Error),
}

/*impl From<OperationError> for JsValue {*/
/*fn from(value: OperationError) -> Self {*/
/*match value {*/
/*OperationError::InvalidInput(error) => JsValue::from_str(&error),*/
/*OperationError::Utf8Error(error) => JsValue::from_str(&error.to_string()),*/
/*}*/
/*}*/
/*}*/
