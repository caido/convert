use serde::{Deserialize, Serialize};

use crate::Operation;
use crate::OperationError;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct HtmlDecode {}

// Inspired by this https://doc.rust-lang.org/stable/nightly-rustc/src/rustdoc/html/escape.rs.html#1-40
// and this http://stackoverflow.com/questions/7381974
impl Operation for HtmlDecode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut decoded = vec![];
        let mut i = 0;
        while i < input.len() {
            if let Some(window) = input.get(i..i + 4) {
                match window {
                    b"&gt;" => {
                        decoded.push(b'>');
                        i += 4;
                        continue;
                    }
                    b"&lt;" => {
                        decoded.push(b'<');
                        i += 4;
                        continue;
                    }
                    _ => (),
                };
            }
            if let Some(window) = input.get(i..i + 5) {
                match window {
                    b"&amp;" => {
                        decoded.push(b'&');
                        i += 5;
                        continue;
                    }
                    b"&#39;" => {
                        decoded.push(b'\\');
                        i += 5;
                        continue;
                    }

                    _ => (),
                }
            }
            if let Some(window) = input.get(i..i + 6) {
                if window == b"&quot;" {
                    decoded.push(b'"');
                    i += 6;
                    continue;
                }
            }
            decoded.push(input[i]);
            i += 1;
        }
        Ok(decoded)
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
        let mut encoded = vec![];
        for ch in input {
            match ch {
                b'>' => encoded.extend_from_slice(b"&gt;"),
                b'<' => encoded.extend_from_slice(b"&lt;"),
                b'&' => encoded.extend_from_slice(b"&amp;"),
                b'\\' => encoded.extend_from_slice(b"&#39;"),
                b'"' => encoded.extend_from_slice(b"&quot;"),
                byte => encoded.push(*byte),
            };
        }
        Ok(encoded)
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
            .execute(b"&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;")
            .unwrap();
        let expected = b"\\&<script>alert(1)</script>a\"".to_vec();
        assert_eq!(actual, expected);
    }

    #[test]
    fn url_encode() {
        let encoder = HtmlEncode::new();
        let actual = encoder.execute(b"\\&<script>alert(1)</script>a\"").unwrap();
        let expected = b"&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;".to_vec();
        assert_eq!(actual, expected);
    }
}
