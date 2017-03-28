//! Code for encryption.

mod sodium;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum DecryptionKey {
    Sodium(::sda_protocol::byte_arrays::B32)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionKeypair {
    pub ek: EncryptionKey,
    pub dk: DecryptionKey,
}

pub trait EncryptorConstruction<S> {
    /// Create a new encryptor for the specificed encryption key and scheme.
    fn new_share_encryptor(&self, ek: &EncryptionKey, scheme: &S) -> SdaClientResult<Box<ShareEncryptor>>;
}

pub trait ShareEncryptor {
    /// Encrypt shares.
    fn encrypt(&self, shares: &[Share]) -> SdaClientResult<Encryption>;
}

pub trait DecryptorConstruction<ID, S> {
    /// Create a new decryptor for the specified keypair and scheme.
    fn new_share_decryptor(&self, id: &ID, scheme: &S) -> SdaClientResult<Box<ShareDecryptor>>;
}

pub trait ShareDecryptor {
    /// Decrypt shares.
    fn decrypt(&self, encryption: &Encryption) -> SdaClientResult<Vec<Share>>;
}

impl EncryptorConstruction<AdditiveEncryptionScheme> for CryptoModule {
    fn new_share_encryptor(&self, ek: &EncryptionKey, scheme: &AdditiveEncryptionScheme) -> SdaClientResult<Box<ShareEncryptor>> {
        match *scheme {
            AdditiveEncryptionScheme::Sodium => {
                let encryptor = sodium::Encryptor::new(ek)?;
                Ok(Box::new(encryptor))
            }
        }
    }
}

impl DecryptorConstruction<EncryptionKeyId, AdditiveEncryptionScheme> for CryptoModule {
    fn new_share_decryptor(&self, id: &EncryptionKeyId, scheme: &AdditiveEncryptionScheme) -> SdaClientResult<Box<ShareDecryptor>> {
        match *scheme {
            AdditiveEncryptionScheme::Sodium => {
                let decryptor = sodium::Decryptor::new(id, &self.keystore)?;
                Ok(Box::new(decryptor))
            }
        }
    }
}