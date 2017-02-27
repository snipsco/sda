
mod jfs;

use super::*;
pub use jfs::{Store};


pub trait GenerateEncryptionKeypair<S, I> {
    fn new_keypair(&self, scheme: &S) -> SdaClientResult<I>;
}

pub trait ExportEncryptionKey<I, EK> {
    fn export_encryption_key(&self, id: &I) -> SdaClientResult<EK>;
}

// TODO should not be allowed; keep decryption keys in IdentityModule instead and ask it to do the decryption
pub trait ExportDecryptionKey<I, DK> {
    fn export_decryption_key(&self, id: &I) -> SdaClientResult<DK>;
}

// impl<S, I> GenerateEncryptionKeypair for ... 
//     where 
//         S: GenerateKeypair,
//         I: New,
// {
//     fn new_keypair(&self, scheme: &S) -> SdaClientResult<I> {
//         let (ek, dk) = scheme.new_keypair()?;
//         let id = I::new();
//         // TODO store keypair under I; fail if exists already
//         Ok(id)
//     }
// }




pub trait IdentityModule {
    fn replace_identity_keypair(&mut self) -> SdaClientResult<()>;
    fn export_verification_key(&self) -> SdaClientResult<VerificationKey>;
    fn sign(&self, message: Vec<u8>) -> SdaClientResult<Signature>;
}

