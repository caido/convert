use data_encoding;
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
#[cfg_attr(target_family = "wasm", serde(rename_all = "snake_case"))]
pub enum Base64Format {
    Standard,
    Url,
    Mime,
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base64Decode {
    format: Base64Format,
    pad: bool,
}

impl Operation for Base64Decode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        match (&self.format, self.pad) {
            (Base64Format::Standard, false) => data_encoding::BASE64_NOPAD.decode(input),
            (Base64Format::Standard, true) => data_encoding::BASE64.decode(input),
            (Base64Format::Url, false) => data_encoding::BASE64URL_NOPAD.decode(input),
            (Base64Format::Url, true) => data_encoding::BASE64URL.decode(input),
            (Base64Format::Mime, _) => data_encoding::BASE64_MIME.decode(input),
        }
        .map_err(|_| OperationError::DecodeError("Invalid base64 input".to_string()))
    }
}

impl Base64Decode {
    pub fn new(format: Base64Format, pad: bool) -> Self {
        Base64Decode { format, pad }
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Base64Encode {
    format: Base64Format,
    pad: bool,
}

impl Operation for Base64Encode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let encoded = match (&self.format, self.pad) {
            (Base64Format::Standard, false) => data_encoding::BASE64_NOPAD.encode(input),
            (Base64Format::Standard, true) => data_encoding::BASE64.encode(input),
            (Base64Format::Url, false) => data_encoding::BASE64URL_NOPAD.encode(input),
            (Base64Format::Url, true) => data_encoding::BASE64URL.encode(input),
            (Base64Format::Mime, _) => data_encoding::BASE64_MIME.encode(input),
        };
        Ok(encoded.into())
    }
}

impl Base64Encode {
    pub fn new(format: Base64Format, pad: bool) -> Self {
        Base64Encode { format, pad }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_decode() {
        let encoder = Base64Decode::new(Base64Format::Standard, true);
        let actual = encoder.execute("Y2FpZG8=".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn base64_encode() {
        let encoder = Base64Encode::new(Base64Format::Standard, true);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "Y2FpZG8=".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
