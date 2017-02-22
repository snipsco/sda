
//! All crypto-related code.

use super::*;

pub type Secret = i64;
pub type Mask = i64;
pub type MaskedSecret = i64;
pub type Share = i64;

mod masking;
mod lss;
mod ae;

pub use self::masking::*;
pub use self::lss::*;
pub use self::ae::*;


// TODO which module should the below belong to?

pub trait SignatureVerification<O> {
    fn signature_is_valid(&self, object: &O) -> SdaClientResult<bool>;
}

impl SignatureVerification<AssociatedEncryptionKey> for Profile {
    fn signature_is_valid(&self, signed_encryption_key: &AssociatedEncryptionKey) -> SdaClientResult<bool> {

        let raw_msg = match &signed_encryption_key.key {
            &EncryptionKey::Sodium(raw_ek) => raw_ek
        };

        let wrapped_vk = &self.verification_key;
        let wrapped_sig = &signed_encryption_key.signature;

        match (wrapped_vk, wrapped_sig) {

            (&VerificationKey::Sodium(raw_vk), &Signature::Sodium(raw_sig)) => {
                let sig = sodiumoxide::crypto::sign::Signature(raw_sig);
                let vk = sodiumoxide::crypto::sign::PublicKey(raw_vk);
                let is_valid = sodiumoxide::crypto::sign::verify_detached(&sig, &raw_msg, &vk);
                Ok(is_valid)
            },

            _ => unimplemented!()
        }

    }
}