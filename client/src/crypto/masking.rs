
//! Code for masking.

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
                let masker = NoneMasker {};
                Ok(Box::new(masker))
            },

            _ => unimplemented!(),

        }
    }
}

struct NoneMasker {}

impl SecretMasker for NoneMasker {
    fn mask_secrets(&mut self, values: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>) {
        let mask = vec![];
        let masked_values = values.to_vec();
        (mask, masked_values)
    }
}

