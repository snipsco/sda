
use super::*;

pub trait IdentityModule {
    fn replace_identity_keypair(&mut self) -> SdaClientResult<()>;
    fn export_verification_key(&self) -> SdaClientResult<VerificationKey>;
    fn sign_message(&self, message: Vec<u8>) -> SdaClientResult<Signature>;
}