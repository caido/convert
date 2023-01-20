use bstr::ByteSlice;
use hex;
use serde::{Deserialize, Serialize};

use super::Operation;
use super::OperationError;

#[derive(Serialize, Deserialize, Clone)]
pub struct HexDecode {
    delimiter: Option<String>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum HexFormat {
    Upper,
    Lower,
}

impl Operation for HexDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut hex_byte_str = input.to_vec();
        if let Some(del) = &self.delimiter {
            hex_byte_str = input
                .split_str(&del)
                .flatten()
                .cloned()
                .filter(|c| !(*c == b'\n'))
                .collect();
        };
        Ok(hex::decode(hex_byte_str.to_ascii_uppercase())?)
    }
}

impl HexDecode {
    pub fn new(delimiter: Option<String>) -> Self {
        HexDecode { delimiter }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HexEncode {
    format: HexFormat,
    delimiter: Option<String>,
    bytes_per_line: usize,
}

impl Operation for HexEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let hex_string = match self.format {
            HexFormat::Lower => hex::encode(input),
            HexFormat::Upper => hex::encode_upper(input),
        };
        let mut output = vec![];
        let mut i = if self.bytes_per_line == 0 {
            hex_string.len()
        } else {
            1
        };
        let delimiter = self
            .delimiter
            .clone()
            .and_then(|d| if d.is_empty() { None } else { Some(d) });
        for hex in hex_string.as_bytes().chunks(2) {
            if let Some(del) = &delimiter {
                output.extend_from_slice(&del.as_bytes());
            }
            output.extend_from_slice(hex);
            if i == self.bytes_per_line {
                output.push(b'\n');
                i = i % self.bytes_per_line;
            }
            i = i + 1
        }
        if let Some(char) = output.last() {
            if *char == b'\n' {
                output.pop();
            }
        }
        Ok(output)
    }
}

impl HexEncode {
    pub fn new(format: HexFormat, delimiter: Option<String>, bytes_per_line: usize) -> Self {
        HexEncode {
            format,
            bytes_per_line,
            delimiter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_decode_0x() {
        let encoder = HexDecode::new(None);
        let actual = encoder.execute("636169646f".as_bytes()).unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_decode_no_prefix() {
        let encoder = HexDecode::new(Some("\\x".to_string()));
        let actual = encoder
            .execute("\\x63\\x61\n\\x69\\x64\n\\x6f".as_bytes())
            .unwrap();
        let expected = "caido".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, Some("\\x".to_string()), 0);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "\\x63\\x61\\x69\\x64\\x6F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, Some("0x".to_string()), 0);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "0x630x610x690x640x6f".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_lower() {
        let encoder = HexEncode::new(HexFormat::Lower, None, 2);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "6361\n6964\n6f".as_bytes().to_vec();
        println!("{}", String::from_utf8_lossy(&actual));
        println!("{}", String::from_utf8_lossy(&expected));
        assert_eq!(actual, expected);
    }

    #[test]
    fn hex_encode_prefix_upper() {
        let encoder = HexEncode::new(HexFormat::Upper, None, 0);
        let actual = encoder.execute("caido".as_bytes()).unwrap();
        let expected = "636169646F".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
