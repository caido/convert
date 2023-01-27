use caido_convert::{HexFormat, Operation};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HexEncode {
    hex_encode: caido_convert::HexEncode,
}

#[wasm_bindgen(typescript_custom_section)]
const IHexEncode: &'static str = r#"
interface IHexEncode {
    format: "Upper" | "Lower";
    delimiter?: string;
    bytes_per_line?: number; 
}
"#;

#[derive(Serialize, Deserialize)]
struct JsHexEncode {
    format: String, // Wasm bindgen doesnt support enums yet
    delimiter: Option<String>,
    bytes_per_line: Option<usize>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IHexEncode")]
    pub type IHexEncode;
}

fn convert_hex_format(format: String) -> Result<HexFormat, JsValue> {
    match format.as_str() {
        "Upper" => Ok(HexFormat::Upper),
        "Lower" => Ok(HexFormat::Lower),
        _ => Err(JsValue::from_str("Invalid format")),
    }
}

#[wasm_bindgen]
impl HexEncode {
    #[wasm_bindgen(constructor)]
    pub fn new(params: IHexEncode) -> Result<HexEncode, JsValue> {
        let js_value: JsValue = params.into();
        let js_hex_encode: JsHexEncode =
            from_value(js_value).map_err(|_err| JsValue::from_str("Invalid argument"))?;
        let hex_encode = caido_convert::HexEncode::new(
            convert_hex_format(js_hex_encode.format)?,
            js_hex_encode.delimiter,
            js_hex_encode.bytes_per_line.unwrap_or(0),
        );
        Ok(HexEncode { hex_encode })
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.hex_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct HexDecode {
    hex_decode: caido_convert::HexDecode,
}

#[wasm_bindgen(typescript_custom_section)]
const IHexDecode: &'static str = r#"
interface IHexDecode {
    delimiter?: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IHexDecode")]
    pub type IHexDecode;
}

#[wasm_bindgen]
impl HexDecode {
    #[wasm_bindgen(constructor)]
    pub fn new(params: IHexDecode) -> Result<HexDecode, JsValue> {
        let js_value: JsValue = params.into();
        let hex_decode: caido_convert::HexDecode =
            from_value(js_value).map_err(|_err| JsValue::from_str("Invalid argument"))?;
        Ok(HexDecode { hex_decode })
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.hex_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
