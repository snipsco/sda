//! Code for masking.

use super::*;

pub trait SecretMaskerConstruction<S> {
    fn new_secret_masker(&self, scheme: &S) -> SdaClientResult<Box<SecretMasker>>;
}

pub trait SecretMasker {
    fn mask_secrets(&mut self, values: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>);
}

mod none;

impl SecretMaskerConstruction<LinearMaskingScheme> for CryptoModule {
    fn new_secret_masker(&self, scheme: &LinearMaskingScheme) -> SdaClientResult<Box<SecretMasker>> {
        match *scheme {
            LinearMaskingScheme::None => {
                let masker = none::Masker::new();
                Ok(Box::new(masker))
            }
        }
    }
}