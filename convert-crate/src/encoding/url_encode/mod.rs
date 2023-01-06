use super::Operation;
use super::OperationError;
use serde::{Deserialize, Serialize};
use urlencoding;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct UrlEncode {
    special_chars: bool,
}

impl Operation for UrlEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let encoded: &str = &urlencoding::encode_binary(input);
        Ok(Vec::from(encoded))
    }
}

impl UrlEncode {
    pub fn new(special_chars: bool) -> Self {
        UrlEncode { special_chars }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encode() {
        let encoder = UrlEncode::new(false);
        let actual = encoder.execute("a@ test".as_bytes()).unwrap();
        let expected = "a%40%20test".as_bytes().to_vec();
        assert_eq!(actual, expected);
    }
}
