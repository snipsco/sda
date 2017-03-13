//! None masker, effectively masking using the zero vector.

use super::*;

pub struct Masker;

impl Masker {
    pub fn new() -> Masker {
        Masker
    }
}

impl SecretMasker for Masker {
    fn mask(&mut self, secrets: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>) {
        let mask = vec![];
        let masked_secrets = secrets.to_vec();
        (mask, masked_secrets)
    }
}

impl MaskCombiner for Masker {
    fn combine(&self, masks: &Vec<Vec<Mask>>) -> Vec<Mask> {
        for mask in masks {
            assert!(mask.len() == 0);
        }
        vec![]
    }
}

impl SecretUnmasker for Masker {
    fn unmask(&self, masked_secrets: &(Vec<Mask>, Vec<MaskedSecret>)) -> Vec<Secret> {
        assert!(masked_secrets.0.len() == 0);
        masked_secrets.1.to_vec()
    }
}
