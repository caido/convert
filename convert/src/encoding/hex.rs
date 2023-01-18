use hex;
use serde::{Deserialize, Serialize};

use super::Operation;
use super::OperationError;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct HexDecode {}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum HexFormat {
    Upper,
    Lower,
    PrefixLower,
    PrefixUpper,
}

impl Operation for HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        if input.starts_with("0x".as_bytes()) {
            Ok(hex::decode(input[2..].to_ascii_uppercase())?)
        } else {
            Ok(hex::decode(input.to_ascii_uppercase())?)
        }
    }
}

impl HexDecode {
    pub fn new() -> Self {
        HexDecode {}
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct HexEncode {
    format: HexFormat,
}

impl Operation for HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let hex_string_bytes = match self.format {
            HexFormat::Lower => hex::encode(input).as_bytes().to_vec(),
            HexFormat::Upper => hex::encode_upper(input).as_bytes().to_vec(),
            HexFormat::PrefixLower => {
                let mut hex = "0x".as_bytes().to_vec();
                hex.append(&mut hex::encode(input).as_bytes().to_vec());
                hex
            }
            HexFormat::PrefixUpper => {
                let mut hex = "0x".as_bytes().to_vec();
                hex.append(&mut hex::encode_upper(input).as_bytes().to_vec());
                hex
            }
        };
        Ok(hex_string_bytes)
    }
}

impl HexEncode {
    pub fn new(format: HexFormat) -> Self {
        HexEncode { format }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_decode_0x() {
        let encoder = HexDecode::new();
        let actual = encoder.execute("0x636169646f".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_no_prefix() {
        let encoder = HexDecode::new();
        let actual = encoder.execute("636169646f".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_upper() {
        let encoder = HexEncode::new(HexFormat::Upper);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "636169646F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_lower() {
        let encoder = HexEncode::new(HexFormat::Lower);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "636169646f".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_lower() {
        let encoder = HexEncode::new(HexFormat::PrefixLower);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "0x636169646f".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_upper() {
        let encoder = HexEncode::new(HexFormat::PrefixUpper);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "0x636169646F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
