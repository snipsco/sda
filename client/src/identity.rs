
use super::*;


pub trait IdentityModule {
    fn replace_identity_keypair(&mut self) -> SdaClientResult<()>;
    fn export_verification_key(&self) -> SdaClientResult<VerificationKey>;
    fn sign(&self, message: Vec<u8>) -> SdaClientResult<Signature>;
}

// TODO should not be allowed; keep decryption keys in IdentityModule instead and ask it to do the decryption
pub trait ExportDecryptionKey {
    fn export_decryption_key(&self, ek: &EncryptionKey) -> SdaClientResult<DecryptionKey>;
}