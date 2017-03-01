
//! Code for additive encryption.

mod sodium;

use super::*;

pub trait GenerateKeypair {
    fn new_keypair(&self) -> SdaClientResult<(EncryptionKey, DecryptionKey)>;
}

impl GenerateKeypair for AdditiveEncryptionScheme {
    fn new_keypair(&self) -> SdaClientResult<(EncryptionKey, DecryptionKey)> {
        // TODO
        unimplemented!()
    }
}


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

        }
    }
}


pub trait ShareDecryptor {
    /// Decrypt shares.
    fn decrypt(&self, encryption: &Encryption) -> SdaClientResult<Vec<Share>>;
}

pub trait DecryptorConstruction<ID, KS> {
    fn new_share_decryptor(&self, id: &ID, keystore: &KS) -> SdaClientResult<Box<ShareDecryptor>>;
}

impl<KS> DecryptorConstruction<EncryptionKeyId, KS> for AdditiveEncryptionScheme
    where KS: ExportDecryptionKey<EncryptionKeyId, (EncryptionKey, DecryptionKey)>
{
    fn new_share_decryptor(&self, id: &EncryptionKeyId, keystore: &KS) -> SdaClientResult<Box<ShareDecryptor>> {
        match self {

            &AdditiveEncryptionScheme::Sodium => {
                let decryptor = sodium::Decryptor::new(id, keystore)?;
                Ok(Box::new(decryptor))
            },

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
