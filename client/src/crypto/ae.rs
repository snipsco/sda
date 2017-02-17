
//! Code for additive encryption.

use super::*;

use std::sync::{Once, ONCE_INIT};

static SODIUM_INITIALIZED: Once = ONCE_INIT;

pub trait ShareEncryptor {

    /// Generate shares for values.
    ///
    /// Several encryptions are generated if the number of values does not fit within one. 
    /// If needed, zero-valued inputs are appended to match the underlying scheme.
    fn encrypt(&self, values: &[Share]) -> Vec<Encryption>;

}

pub trait EncryptorConstruction {
    fn new_share_encryptor(&self, ek: &EncryptionKey) -> SdaClientResult<Box<ShareEncryptor>>;
}

impl EncryptorConstruction for AdditiveEncryptionScheme {

    fn new_share_encryptor(&self, ek: &EncryptionKey) -> SdaClientResult<Box<ShareEncryptor>> {

        match *self {

            AdditiveEncryptionScheme::Sodium {} => {

                // initialise Sodium per recommendations; 
                // documentation hints it's okay to do so more than once but we'll play it safe
                SODIUM_INITIALIZED.call_once(|| {
                    sodiumoxide::init();
                });

                let pk = sodiumoxide::crypto::box_::PublicKey::from_slice(&*ek.0)
                    .ok_or("Failed to parse Sodium public key")?;

                let encryptor = SodiumWrapper {
                    pk: pk,
                };
                Ok(Box::new(encryptor))
            }

            // TODO
            _ => unimplemented!(),
        }

    }

}

struct SodiumWrapper {
    pk: sodiumoxide::crypto::box_::PublicKey,
}

impl ShareEncryptor for SodiumWrapper {

    fn encrypt(&self, values: &[Share]) -> Vec<Encryption> {
        use integer_encoding::VarInt;
        // encode values
        let mut encoded_values = vec![];
        let mut buf = [0; 128];
        for &value in values {
            let size = value.encode_var(&mut buf);
            encoded_values.extend(&buf[0..size]);
        }
        // encrypt
        vec![sodiumoxide::crypto::sealedbox::seal(&*encoded_values, &self.pk)]
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
