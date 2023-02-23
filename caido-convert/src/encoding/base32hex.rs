use data_encoding::BASE32HEX;
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base32HexDecode {}

impl Operation for Base32HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        BASE32HEX
            .decode(input)
            .map_err(|_| OperationError::DecodeError("Invalid base64 input".to_string()))
    }
}

impl Base32HexDecode {
    pub fn new() -> Self {
        Base32HexDecode {}
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base32HexEncode {}

impl Operation for Base32HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(BASE32HEX.encode(input).into())
    }
}

impl Base32HexEncode {
    pub fn new() -> Self {
        Base32HexEncode {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_decode() {
        let encoder = Base32HexDecode::new();
        let actual = encoder.execute("CDGMIP3F".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn base64_encode() {
        let encoder = Base32HexEncode::new();
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "CDGMIP3F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
