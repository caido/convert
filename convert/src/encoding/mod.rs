pub trait Operation {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError>;
}

pub mod base64;
pub mod errors;
pub mod hex;
pub mod html;
pub mod url;
use errors::OperationError;
