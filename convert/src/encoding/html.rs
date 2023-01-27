use caido_convert::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HtmlEncode {
    html_encode: caido_convert::HtmlEncode,
}

#[wasm_bindgen]
impl HtmlEncode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmlEncode {
        HtmlEncode {
            html_encode: caido_convert::HtmlEncode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.html_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct HtmlDecode {
    html_decode: caido_convert::HtmlDecode,
}

#[wasm_bindgen]
impl HtmlDecode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmlDecode {
        HtmlDecode {
            html_decode: caido_convert::HtmlDecode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.html_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
