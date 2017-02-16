
//! Code for additive encryption.

use super::*;

pub trait ShareEncryptor {

    fn encrypt(&self, values: &[Share]) -> Vec<Encryption> {
        vec![]
    }

    /// Generate shares for a single batch: the number of inputs is assumed to match the input size of the underlying scheme.
    fn encrypt_for_batch(&self, batch_values: &[Share]) -> Encryption;

    /// Input size of each batch; expected to be constant.
    fn batch_input_size(&self) -> usize;

    /// Output size of each batch; expected to be constant.
    fn batch_output_size(&self) -> usize;

}


pub trait EncryptorConstruction {
    fn new_encryptor(&self, ek: &EncryptionKey) -> Box<ShareEncryptor>;
}

impl EncryptorConstruction for AdditiveEncryptionScheme {

    fn new_encryptor(&self, ek: &EncryptionKey) -> Box<ShareEncryptor> {

        match *self {
            // TODO
            _ => unimplemented!(),
        }

    }

}

// struct PackedPaillierWrapper {
//     batch_input_size: usize,
//     batch_output_size: usize,
//     instance: tss::packed::PackedSecretSharing,
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
