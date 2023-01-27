use caido_convert::{Operation, Sha2Version};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Md5Hash {
    md5_hash: caido_convert::Md5Hash,
}

#[wasm_bindgen]
impl Md5Hash {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Md5Hash {
        Md5Hash {
            md5_hash: caido_convert::Md5Hash::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.md5_hash
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct Sha1Hash {
    sha1_hash: caido_convert::Sha1Hash,
}

#[wasm_bindgen]
impl Sha1Hash {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sha1Hash {
        Sha1Hash {
            sha1_hash: caido_convert::Sha1Hash::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.sha1_hash
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct Sha2Hash {
    sha2_hash: caido_convert::Sha2Hash,
}

#[wasm_bindgen(typescript_custom_section)]
const ISha2Hash: &'static str = r#"
interface ISha2Hash {
    version?: "Sha224" | "Sha256" | "Sha384" | "Sha512";
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ISha2Hash")]
    pub type ISha2Hash;
}

#[derive(Serialize, Deserialize)]
struct JsSha2Hash {
    version: String,
}

#[wasm_bindgen]
impl Sha2Hash {
    #[wasm_bindgen(constructor)]
    pub fn new(params: ISha2Hash) -> Result<Sha2Hash, JsValue> {
        let js_value: JsValue = params.into();
        let js_params: JsSha2Hash =
            from_value(js_value).map_err(|_err| JsValue::from_str("Invalid argument"))?;
        let version = match js_params.version.as_str() {
            "Sha224" => Sha2Version::Sha224,
            "Sha256" => Sha2Version::Sha256,
            "Sha384" => Sha2Version::Sha384,
            "Sha512" => Sha2Version::Sha512,
            _ => Sha2Version::Sha256,
        };
        Ok(Sha2Hash {
            sha2_hash: caido_convert::Sha2Hash::new(version),
        })
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.sha2_hash
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
