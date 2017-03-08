//! All crypto-related code.

mod signing;
mod masking;
mod sharing;
mod encryption;

use sda_protocol::*;
use errors::SdaClientResult;

pub use self::signing::{SignatureKeypair, SignExport, SignatureVerification};
pub use self::masking::{SecretMaskerConstruction};
pub use self::sharing::{ShareGeneratorConstruction, ShareCombinerConstruction};
pub use self::encryption::{EncryptionKeypair, EncryptorConstruction, DecryptorConstruction};

type Secret = i64;
type Mask = i64;
type MaskedSecret = i64;
pub type Share = i64;

pub trait KeyGeneration<K> {
    fn new_key(&self) -> SdaClientResult<K>;
}

pub trait KeyStorage<ID, K> {
    fn put(&self, id: &ID, key: &K) -> SdaClientResult<()>;
    fn get(&self, id: &ID) -> SdaClientResult<Option<K>>;
}

pub trait Keystore :
    KeyStorage<EncryptionKeyId, EncryptionKeypair>
    + KeyStorage<VerificationKeyId, SignatureKeypair>
{}

pub struct CryptoModule {
    keystore: Box<Keystore>
}

impl CryptoModule {
    pub fn new(keystore: Box<Keystore>) -> CryptoModule {
        CryptoModule { keystore: keystore }
    }
}