//! All crypto-related code.

use errors::SdaClientResult;

use sda_protocol::*;

pub type Secret = i64;
pub type Mask = i64;
pub type MaskedSecret = i64;
pub type Share = i64;

pub struct CryptoModule<K> {
    keystore: K
}

impl<K> CryptoModule<K> {
    pub fn new(keystore: K) -> CryptoModule<K> {
        CryptoModule { keystore: keystore }
    }
}

pub trait KeyGeneration<T> {
    fn new_key(&self) -> SdaClientResult<T>;
}

mod signing;
mod masking;
mod sharing;
mod encryption;

pub use self::signing::{SignExport, SignatureVerification};
pub use self::masking::{SecretMaskerConstruction};
pub use self::sharing::{ShareGeneratorConstruction, ShareCombinerConstruction};
pub use self::encryption::{EncryptorConstruction, DecryptorConstruction};