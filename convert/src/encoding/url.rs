use super::Operation;
use super::OperationError;
use bstr::ByteSlice;
use percent_encoding::{self, percent_encode_byte};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct UrlEncode {
    non_ascii: bool,
    charset: Vec<u8>,
}

fn extend_with_grapheme_encode(output: &mut Vec<u8>, grapheme: &str) {
    grapheme
        .as_bytes()
        .iter()
        .for_each(|b| output.extend_from_slice(percent_encode_byte(*b).as_bytes()));
}

impl Operation for UrlEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let charset = self.charset.graphemes();
        let input_graphemes = input.graphemes();
        let mut output: Vec<u8> = vec![];

        for grapheme in input_graphemes {
            let mut charset_graphemes = charset.clone();
            if charset_graphemes.any(|charset_grapheme| charset_grapheme == grapheme)
                || (self.non_ascii && !grapheme.is_ascii())
            {
                extend_with_grapheme_encode(&mut output, grapheme);
            } else {
                output.extend_from_slice(grapheme.as_bytes());
            }
        }
        Ok(output)
    }
}

impl UrlEncode {
    pub fn new(non_ascii: bool, charset: &[u8]) -> Self {
        UrlEncode {
            non_ascii,
            charset: charset.to_vec(),
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
        let encoder = UrlEncode::new(false, b"\xF0\x9F\xA5\x96");
        let actual = encoder.execute("a🥖🥖st".as_bytes()).unwrap();
        let expected = "a%F0%9F%A5%96%F0%9F%A5%96st".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_non_ascii() {
        let encoder = UrlEncode::new(true, b"");
        let actual = encoder.execute("caido @éé".as_bytes()).unwrap();
        let expected = "caido @%C3%A9%C3%A9".as_bytes().to_vec();
        println!("{:?}", String::from_utf8(expected.clone()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode_non_ascii_and_charset() {
        let encoder = UrlEncode::new(true, b"");
        let actual = encoder.execute("caido @éé🥖".as_bytes()).unwrap();
        let expected = "caido @%C3%A9%C3%A9%F0%9F%A5%96".as_bytes().to_vec();
        println!("{:?}", String::from_utf8(expected.clone()));
        assert_eq!(actual, expected);
    }
}
