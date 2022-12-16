pub trait Operation {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError>;
}

pub mod errors;
pub mod url_encode;
use errors::OperationError;

/*#[cfg!(platform = "wasm")]*/
/*impl UrlEncode {*/
/*pub fn apply(&self, input: SharedArrayBuffer) -> Result<SharedArrayBuffer> {*/
/*}*/
/*}*/
