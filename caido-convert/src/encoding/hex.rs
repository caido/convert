use bstr::ByteSlice;
use hex;
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct HexDecode {
    prefix: Option<String>,
    delimiter: Option<String>,
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub enum HexFormat {
    Upper,
    Lower,
}

impl Operation for HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut input = input.to_vec();
        if let Some(p) = &self.prefix {
            input = input.replace(p, "");
        };
        if let Some(d) = &self.delimiter {
            input = input.replace(d, "");
        };
        Ok(hex::decode(input)?)
    }
}

impl HexDecode {
    pub fn new(prefix: Option<String>, delimiter: Option<String>) -> Self {
        HexDecode { prefix, delimiter }
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct HexEncode {
    format: HexFormat,
    prefix: Option<String>,
    delimiter: Option<String>,
}

impl Operation for HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let hex_string = match self.format {
            HexFormat::Lower => hex::encode(input),
            HexFormat::Upper => hex::encode_upper(input),
        };
        let mut output = vec![];
        let delimiter = self
            .delimiter
            .clone()
            .and_then(|d| if d.is_empty() { None } else { Some(d) });
        let prefix = self
            .prefix
            .clone()
            .and_then(|p| if p.is_empty() { None } else { Some(p) });
        let mut chunks = hex_string.as_bytes().chunks(2).peekable();
        while let Some(chunk) = chunks.next() {
            if let Some(p) = &prefix {
                output.extend_from_slice(p.as_bytes());
            }
            output.extend_from_slice(chunk);
            if let Some(d) = &delimiter {
                if chunks.peek().is_some() {
                    output.extend_from_slice(d.as_bytes());
                }
            }
        }
        Ok(output)
    }
}

impl HexEncode {
    pub fn new(format: HexFormat, prefix: Option<String>, delimiter: Option<String>) -> Self {
        HexEncode {
            format,
            prefix,
            delimiter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_decode_no_prefix_no_delimiter() {
        let encoder = HexDecode::new(None, None);
        let actual = encoder.execute("636169646f".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_prefix() {
        let encoder = HexDecode::new(Some("\\x".to_string()), None);
        let actual = encoder
            .execute("\\x63\\x61\\x69\\x64\\x6f".as_bytes())
            .unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_delimiter() {
        let encoder = HexDecode::new(None, Some(",".to_string()));
        let actual = encoder.execute("63,61,69,64,6f".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, Some("\\x".to_string()), None);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "\\x63\\x61\\x69\\x64\\x6F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, Some("0x".to_string()), None);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "0x630x610x690x640x6f".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_delimiter_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, None, Some("\n".to_string()));
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "63\n61\n69\n64\n6f".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_delimiter_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, None, Some("\n".to_string()));
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "63\n61\n69\n64\n6F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
