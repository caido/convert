use super::Operation;
use super::OperationError;
use html_escape;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct HtmlDecode {}

impl Operation for HtmlDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(
            html_escape::decode_html_entities(&String::from_utf8(input.to_vec())?)
                .as_bytes()
                .to_vec(),
        )
    }
}

impl HtmlDecode {
    pub fn new() -> Self {
        HtmlDecode {}
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct HtmlEncode {}

impl Operation for HtmlEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(
            html_escape::encode_text(&String::from_utf8(input.to_vec())?)
                .as_bytes()
                .to_vec(),
        )
    }
}

impl HtmlEncode {
    pub fn new() -> Self {
        HtmlEncode {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_decode() {
        let encoder = HtmlDecode::new();
        let actual = encoder
            .execute("%3Cdiv%3Etest%3Cdiv%3E".as_bytes())
            .unwrap();
        println!("{}", html_escape::decode_script("%3Cdiv%3Etest%3Cdiv%3E"));
        let expected = "<div>test<div>".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode() {
        let encoder = HtmlEncode::new();
        let actual = encoder
            .execute("<script>alert(1)</script>".as_bytes())
            .unwrap();
        let expected = "&lt;script&gt;alert(1)&lt;/script&gt;".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
