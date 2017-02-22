
//! Code for additive encryption.

mod sodium;

use super::*;

pub trait ShareEncryptor {
    /// Encrypt shares.
    fn encrypt(&self, shares: &[Share]) -> SdaClientResult<Encryption>;
}

pub trait EncryptorConstruction {
    fn new_share_encryptor(&self, ek: &EncryptionKey) -> SdaClientResult<Box<ShareEncryptor>>;
}

impl EncryptorConstruction for AdditiveEncryptionScheme {
    fn new_share_encryptor(&self, ek: &EncryptionKey) -> SdaClientResult<Box<ShareEncryptor>> {
        match self {

            &AdditiveEncryptionScheme::Sodium => {
                let encryptor = sodium::Encryptor::new(ek)?;
                Ok(Box::new(encryptor))
            },

            _ => unimplemented!(),

        }
    }
}


pub trait ShareDecryptor {
    /// Decrypt shares.
    fn decrypt(&self, encryption: &Encryption) -> SdaClientResult<Vec<Share>>;
}

pub trait DecryptorConstruction {
    fn new_share_decryptor<I: ExportDecryptionKey>(&self, ek: &EncryptionKey, identity: &I) -> SdaClientResult<Box<ShareDecryptor>>;
}

impl DecryptorConstruction for AdditiveEncryptionScheme {
    fn new_share_decryptor<I: ExportDecryptionKey>(&self, ek: &EncryptionKey, identity: &I) -> SdaClientResult<Box<ShareDecryptor>> {
        match self {

            &AdditiveEncryptionScheme::Sodium => {
                let decryptor = sodium::Decryptor::new(ek, identity)?;
                Ok(Box::new(decryptor))
            },

            _ => unimplemented!(),

        }
    }
}



// struct PackedPaillierWrapper {
//     eek: paillier::coding::EncodingEncryptionKey<_>,
//     component_count: usize,
//     component_bitsize: usize,

// }

// impl Encryptor for PackedPaillierWrapper {

//     fn batch_input_size(&self) -> usize {
//         self.batch_input_size
//     }

//     fn batch_output_size(&self) -> usize {
//         self.batch_output_size
//     }

//     fn generate_shares_for_batch(&self, batch_input: &[i64]) -> Vec<i64> {
//         self.pss_instance.share(batch_input)
//     }

// }
