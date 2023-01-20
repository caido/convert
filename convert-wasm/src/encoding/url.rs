use convert::Operation;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UrlEncode {
    url_encode: convert::UrlEncode,
}

#[wasm_bindgen(typescript_custom_section)]
const IUrlEncode: &'static str = r#"
interface IUrlEncode {
    non_ascii: boolean;
    charset: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IUrlEncode")]
    pub type IUrlEncode;
}

#[wasm_bindgen]
impl UrlEncode {
    #[wasm_bindgen(constructor)]
    pub fn new(params: IUrlEncode) -> Result<UrlEncode, JsValue> {
        let js_value: JsValue = params.into();
        let url_encode: convert::UrlEncode =
            from_value(js_value).map_err(|_err| JsValue::from_str("Invalid argument"))?;
        Ok(UrlEncode { url_encode })
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.url_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{:?}", err)))
    }
}

#[wasm_bindgen]
pub struct UrlDecode {
    url_decode: convert::UrlDecode,
}

#[wasm_bindgen]
impl UrlDecode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UrlDecode {
        UrlDecode {
            url_decode: convert::UrlDecode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.url_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{:?}", err)))
    }
}
