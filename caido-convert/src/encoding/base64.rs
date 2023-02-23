use data_encoding;
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base64Decode {}

impl Operation for Base64Decode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        data_encoding::BASE64
            .decode(input)
            .map_err(|_| OperationError::DecodeError("Invalid base64 input".to_string()))
    }
}

impl Base64Decode {
    pub fn new() -> Self {
        Base64Decode {}
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base64Encode {}

impl Operation for Base64Encode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(data_encoding::BASE64.encode(input).into())
    }
}

impl Base64Encode {
    pub fn new() -> Self {
        Base64Encode {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_decode() {
        let encoder = Base64Decode::new();
        let actual = encoder.execute("Y2FpZG8=".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn base64_encode() {
        let encoder = Base64Encode::new();
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "Y2FpZG8=".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
