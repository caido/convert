use base64;
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Base64Decode {}

impl Operation for Base64Decode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(base64::decode(input)?)
    }
}

impl Base64Decode {
    pub fn new() -> Self {
        Base64Decode {}
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Base64Encode {}

impl Operation for Base64Encode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(base64::encode(input).into())
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
