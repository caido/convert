use caido_convert::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Base32HexEncode {
    base32hex_encode: caido_convert::Base32HexEncode,
}

#[wasm_bindgen]
impl Base32HexEncode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Base32HexEncode {
        Base32HexEncode {
            base32hex_encode: caido_convert::Base32HexEncode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.base32hex_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct Base32HexDecode {
    base32hex_decode: caido_convert::Base32HexDecode,
}

#[wasm_bindgen]
impl Base32HexDecode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Base32HexDecode {
        Base32HexDecode {
            base32hex_decode: caido_convert::Base32HexDecode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.base32hex_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
