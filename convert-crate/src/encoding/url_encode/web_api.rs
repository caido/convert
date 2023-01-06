use super::UrlEncode;
use crate::encoding::Operation;

use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = UrlEncode)]
pub struct WUrlEncode {
    pub url_encode: UrlEncode,
}

#[wasm_bindgen(typescript_custom_section)]
const IUrlEncode: &'static str = r#"
interface IUrlEncode {
    special_chars: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IUrlEncode")]
    pub type IUrlEncode;
}

#[wasm_bindgen]
impl WUrlEncode {
    #[wasm_bindgen(constructor)]
    pub fn new(params: IUrlEncode) -> WUrlEncode {
        let js_value: JsValue = params.into();
        let url_encode: UrlEncode = from_value(js_value).unwrap();
        WUrlEncode { url_encode }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.url_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&err.to_string()))
    }
}
