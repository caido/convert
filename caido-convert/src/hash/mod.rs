#[cfg(target_family = "wasm")]
use serde::{Deserialize, Serialize};
use sha1::{Digest as Sha1Digest, Sha1};

use crate::Operation;
use crate::OperationError;

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Md5Hash {}

impl Operation for Md5Hash {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        Ok(md5::compute(input).to_vec())
    }
}

impl Md5Hash {
    pub fn new() -> Self {
        Md5Hash {}
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Sha1Hash {}

impl Operation for Sha1Hash {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let mut hasher = Sha1::new();
        hasher.update(input);
        Ok(hasher.finalize().to_vec())
    }
}

impl Sha1Hash {
    pub fn new() -> Sha1Hash {
        Sha1Hash {}
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub struct Sha2Hash {
    version: Sha2Version,
}

impl Sha2Hash {
    pub fn new(version: Sha2Version) -> Sha2Hash {
        Sha2Hash { version }
    }
}

#[derive(Clone)]
#[cfg_attr(target_family = "wasm", derive(Serialize, Deserialize))]
pub enum Sha2Version {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl Operation for Sha2Hash {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let hash_vec = match self.version {
            Sha2Version::Sha224 => {
                let mut hasher = sha2::Sha224::new();
                hasher.update(input);
                hasher.finalize().to_vec()
            }
            Sha2Version::Sha256 => {
                let mut hasher = sha2::Sha256::new();
                hasher.update(input);
                hasher.finalize().to_vec()
            }
            Sha2Version::Sha384 => {
                let mut hasher = sha2::Sha384::new();
                hasher.update(input);
                hasher.finalize().to_vec()
            }
            Sha2Version::Sha512 => {
                let mut hasher = sha2::Sha512::new();
                hasher.update(input);
                hasher.finalize().to_vec()
            }
        };
        Ok(hash_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HexEncode;

    #[test]
    fn hash_md5() {
        let hasher = Md5Hash::new();
        let res = hasher.execute(b"caido").unwrap();
        let encode = HexEncode::new(crate::HexFormat::Lower, None, 0);
        let hex_result = encode.execute(&res).unwrap();
        assert_eq!(hex_result, b"7542bf4fd4500c58ac741ae2e05a1521")
    }
}
