use bstr::ByteSlice;
use percent_encoding::{self, percent_encode_byte};
#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct UrlDecode {}

impl Operation for UrlDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let decoded: Vec<u8> = percent_encoding::percent_decode(input).collect();
        Ok(decoded)
    }
}

impl UrlDecode {
    pub fn new() -> Self {
        UrlDecode {}
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct UrlEncode {
    non_ascii: bool,
    charset: String,
}

fn extend_with_grapheme_encode(output: &mut Vec<u8>, grapheme: &[u8]) {
    grapheme
        .iter()
        .for_each(|b| output.extend_from_slice(percent_encode_byte(*b).as_bytes()));
}

impl Operation for UrlEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let charset = self.charset.as_bytes().graphemes();
        let input_graphemes = input.grapheme_indices();
        let mut output: Vec<u8> = vec![];

        for (start, stop, grapheme) in input_graphemes {
            let mut charset_graphemes = charset.clone();
            if charset_graphemes.any(|charset_grapheme| charset_grapheme == grapheme)
                || (self.non_ascii && !grapheme.is_ascii())
            {
                extend_with_grapheme_encode(&mut output, &input[start..stop]);
            } else {
                output.extend_from_slice(&input[start..stop]);
            }
        }
        Ok(output)
    }
}

impl UrlEncode {
    pub fn new(non_ascii: bool, charset: Option<String>) -> Self {
        UrlEncode {
            non_ascii,
            charset: charset.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_decode() {
        let encoder = UrlDecode::new();
        let actual = encoder
            .execute("caido @%C3%A9%C3%A9%F0%9F%A5%96".as_bytes())
            .unwrap();
        let expected = "caido @éé🥖".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_unicode_char() {
        let encoder = UrlEncode::new(false, Some("🥖".to_string()));
        let actual = encoder.execute("a🥖🥖st".as_bytes()).unwrap();
        let expected = "a%F0%9F%A5%96%F0%9F%A5%96st".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_non_ascii() {
        let encoder = UrlEncode::new(true, None);
        let actual = encoder.execute("caido @éé".as_bytes()).unwrap();
        let expected = "caido @%C3%A9%C3%A9".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_non_ascii_and_charset() {
        let encoder = UrlEncode::new(true, Some("c".to_string()));
        let actual = encoder.execute("caido @éé🥖".as_bytes()).unwrap();
        let expected = "%63aido @%C3%A9%C3%A9%F0%9F%A5%96".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_charset() {
        let encoder = UrlEncode::new(true, Some("@t".to_string()));
        let actual = encoder.execute("a@ test".as_bytes()).unwrap();
        let expected = "a%40 %74es%74".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_invalid_utf_8() {
        let encoder = UrlEncode::new(false, None);
        let actual = encoder
            .execute(&[0x98, 0xfd, 0xe0, 0xbf, 0xb8, 0xa7, 0xd6, 0xe1, 0x74, 0xa0])
            .unwrap();
        let expected = &[0x98, 0xfd, 0xe0, 0xbf, 0xb8, 0xa7, 0xd6, 0xe1, 0x74, 0xa0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_non_ascii_invalid_utf_8() {
        let encoder = UrlEncode::new(true, None);
        let actual = encoder
            .execute(&[0x98, 0xfd, 0xe0, 0xbf, 0xb8, 0xa7, 0xd6, 0xe1, 0x74, 0xa0])
            .unwrap();
        let expected = "%98%FD%E0%BF%B8%A7%D6%E1t%A0".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
