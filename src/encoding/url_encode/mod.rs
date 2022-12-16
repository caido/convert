use super::Operation;
use super::OperationError;
use serde::{Deserialize, Serialize};
use urlencoding;
use wasm_bindgen::prelude::*;
pub use web_api::WUrlEncode;

pub mod web_api;

#[derive(Serialize, Deserialize, Clone, Copy)]
#[wasm_bindgen(js_name = Foo1)]
pub struct UrlEncode {
    special_chars: bool,
}

impl Operation for UrlEncode {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let encoded: &str = &urlencoding::encode_binary(input).to_owned();
        Ok(Vec::from(encoded))
    }
}

impl UrlEncode {
    pub fn new(special_chars: bool) -> Self {
        UrlEncode { special_chars }
    }
}
