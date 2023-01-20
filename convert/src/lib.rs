mod encoding;
mod errors;
mod hash;
pub use encoding::base64::*;
pub use encoding::hex::*;
pub use encoding::html::*;
pub use encoding::url::*;
pub use errors::OperationError;
pub use hash::*;

pub trait Operation {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError>;
}
