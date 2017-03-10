//! None masker, effectively masking using the zero vector.

use super::*;

pub struct Masker;

impl Masker {
    pub fn new() -> Masker {
        Masker
    }
}

impl SecretMasker for Masker {
    fn mask_secrets(&mut self, values: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>) {
        let mask = vec![];
        let masked_values = values.to_vec();
        (mask, masked_values)
    }
}
