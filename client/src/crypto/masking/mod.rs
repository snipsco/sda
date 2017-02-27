
//! Code for masking.

mod none;

use super::*;


pub trait SecretMasker {
    fn mask_secrets(&mut self, values: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>);
}

pub trait SecretMaskerConstruction {
    fn new_secret_masker(&self) -> SdaClientResult<Box<SecretMasker>>;
}

impl SecretMaskerConstruction for LinearMaskingScheme {
    fn new_secret_masker(&self) -> SdaClientResult<Box<SecretMasker>> {
        match *self {

            LinearMaskingScheme::None => {
                let masker = none::Masker::new();
                Ok(Box::new(masker))
            },

        }
    }
}
