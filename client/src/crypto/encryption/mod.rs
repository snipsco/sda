//! Code for encryption.

use sda_protocol::*;
use errors::*;
use crypto::*;

use sda_client_store::Store;


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
    fn new_share_encryptor(&self, ek: &EncryptionKey, scheme: &S) -> SdaClientResult<Box<ShareEncryptor>>;
}

pub trait ShareEncryptor {
    /// Encrypt shares.
    fn encrypt(&self, shares: &[Share]) -> SdaClientResult<Encryption>;
}

pub trait DecryptorConstruction<ID, S> {
    fn new_share_decryptor(&self, id: &ID, scheme: &S) -> SdaClientResult<Box<ShareDecryptor>>;
}

pub trait ShareDecryptor {
    /// Decrypt shares.
    fn decrypt(&self, encryption: &Encryption) -> SdaClientResult<Vec<Share>>;
}







impl<K> ExportDecryptionKey<EncryptionKeyId, (EncryptionKey, DecryptionKey)> for CryptoModule<K> {
    fn export_decryption_key(&self, id: &EncryptionKeyId) -> SdaClientResult<Option<(EncryptionKey, DecryptionKey)>> {
        unimplemented!()
    }
}

impl GenerateKeypair for AdditiveEncryptionScheme {
    fn new_keypair(&self) -> SdaClientResult<(EncryptionKey, DecryptionKey)> {
        // TODO
        unimplemented!()
    }
}

mod sodium;

impl<K> EncryptorConstruction<AdditiveEncryptionScheme> for CryptoModule<K> {
    fn new_share_encryptor(&self, ek: &EncryptionKey, scheme: &AdditiveEncryptionScheme) -> SdaClientResult<Box<ShareEncryptor>> {
        match *scheme {

            AdditiveEncryptionScheme::Sodium => {
                let encryptor = sodium::Encryptor::new(ek)?;
                Ok(Box::new(encryptor))
            },

        }
    }
}

impl<K> DecryptorConstruction<EncryptionKeyId, AdditiveEncryptionScheme> for CryptoModule<K>
    // where KS: ExportDecryptionKey<EncryptionKeyId, (EncryptionKey, DecryptionKey)>
{
    fn new_share_decryptor(&self, id: &EncryptionKeyId, scheme: &AdditiveEncryptionScheme) -> SdaClientResult<Box<ShareDecryptor>> {
        unimplemented!()

        // TODO

        // match self {

        //     &AdditiveEncryptionScheme::Sodium => {
        //         let decryptor = sodium::Decryptor::new(id, keystore)?;
        //         Ok(Box::new(decryptor))
        //     },

        // }
    }
}