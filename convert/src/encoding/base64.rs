use caido_convert::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Base64Encode {
    base64_encode: caido_convert::Base64Encode,
}

#[wasm_bindgen]
impl Base64Encode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Base64Encode {
        Base64Encode {
            base64_encode: caido_convert::Base64Encode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.base64_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct Base64Decode {
    base64_decode: caido_convert::Base64Decode,
}

#[wasm_bindgen]
impl Base64Decode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Base64Decode {
        Base64Decode {
            base64_decode: caido_convert::Base64Decode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.base64_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
