//! All crypto-related code.

use errors::SdaClientResult;
use sda_protocol::*;
use self::encryption::DecryptionKey; // TODO
use sda_client_store::Store;

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

pub trait GenerateEncryptionKeypair<S> {
    fn new_keypair<I>(&self, scheme: &S) -> SdaClientResult<I>;
}

pub trait GenerateKeypair {
    fn new_keypair(&self) -> SdaClientResult<(EncryptionKey, DecryptionKey)>;
}

pub trait Export<I, K> {
    fn export(&self, id: &I) -> SdaClientResult<Option<K>>;
}

pub trait SignExport<I, O>
    where O: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    fn sign_export(&self, signer: &Agent, id: &I) -> SdaClientResult<Option<Signed<O>>>;
}

// TODO should not be allowed; keep decryption keys in IdentityModule instead and ask it to do the decryption
pub trait ExportDecryptionKey<I, DK> {
    fn export_decryption_key(&self, id: &I) -> SdaClientResult<Option<DK>>;
}

pub trait SignatureVerification<O> {
    fn signature_is_valid(&self, object: &O) -> SdaClientResult<bool>;
}

pub mod signing;
pub mod masking;
pub mod sharing;
pub mod encryption;

pub use self::signing::*;
pub use self::masking::*;
pub use self::sharing::*;
pub use self::encryption::*;

// pub use self::masking::*;
// pub use self::sharing::*;
// pub use self::encryption::*;