
//! All crypto-related code.

use super::*;

pub type Secret = i64;
pub type Mask = i64;
pub type MaskedSecret = i64;
pub type Share = i64;

mod masking;
mod sharing;
mod encryption;

pub use self::masking::*;
pub use self::sharing::*;
pub use self::encryption::*;


// TODO which module should the below belong to?

pub enum DecryptionKey {
    Sodium([u8; 0]) // TODO what is the right size?
}


// pub trait Signee {
//     fn signer(&self) -> AgentId;
//     fn signed(&self) -> Vec<u8>;
//     fn signature(&self) -> Signature;
// }

// impl Signee for SignedEncryptionKey {
//     fn signer(&self) -> AgentId {
//         self.owner
//     }
//     fn signed(&self) -> {
//         self.
//     }
//     fn signature(&self) -> Signature {
//         self.signature
//     }
// }

// impl<T: Signer, U: Signee> SignatureVerification<U> for T {
//     fn signature_is_valid(&self, object: U) -> SdaClientResult<bool> {

//     }
// }


pub trait SignatureVerification<O> {
    fn signature_is_valid(&self, object: &O) -> SdaClientResult<bool>;
}

impl SignatureVerification<SignedEncryptionKey> for Agent {
    fn signature_is_valid(&self, signed_encryption_key: &SignedEncryptionKey) -> SdaClientResult<bool> {

        // TODO remember result to avoid running verification more than once

        let raw_msg = match &signed_encryption_key.key {
            &EncryptionKey::Sodium(raw_ek) => raw_ek
        };

        let wrapped_vk = &self.verification_key;
        let wrapped_sig = &signed_encryption_key.signature;

        match (wrapped_vk, wrapped_sig) {

            (&None, _) => {
                Err("No verification key found")?
            },

            (&Some(VerificationKey::Sodium(raw_vk)), &Signature::Sodium(raw_sig)) => {
                let sig = sodiumoxide::crypto::sign::Signature(raw_sig);
                let vk = sodiumoxide::crypto::sign::PublicKey(raw_vk);
                let is_valid = sodiumoxide::crypto::sign::verify_detached(&sig, &raw_msg, &vk);
                Ok(is_valid)
            },

        }
    }
}
