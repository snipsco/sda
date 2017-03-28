//! Full masker, using full masks.

use super::*;

use rand::{Rng, OsRng};

pub struct Masker {
    modulus: i64,
    rng: OsRng,
}

impl Masker {
    pub fn new(modulus: i64) -> Masker {
        Masker {
            modulus: modulus,
            rng: OsRng::new().expect("Unable to get randomness source"),
        }
    }
}

impl SecretMasker for Masker {
    fn mask(&mut self, secrets: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>) {

        let masks: Vec<Mask> = (0..secrets.len())
            .map(|_| self.rng.gen_range(0_i64, self.modulus))
            .collect();

        let masked_secrets = secrets.iter()
            .zip(&masks)
            .map(|(secret, mask)| (secret + mask) % self.modulus)
            .collect();

        (masks, masked_secrets)
    }
}

impl MaskCombiner for Masker {
    fn combine(&self, masks: &Vec<Vec<Mask>>) -> Vec<Mask> {
        let dimension: usize = masks.get(0).map_or(0, Vec::len);

        let mut result: Vec<Share> = vec![0; dimension];
        for mask in masks {
            assert_eq!(mask.len(), dimension);
            for (ix, value) in mask.iter().enumerate() {
                result[ix] += *value;
                result[ix] %= self.modulus;
            }
        }

        result
    }
}

impl SecretUnmasker for Masker {
    fn unmask(&self, values: &(Vec<Mask>, Vec<MaskedSecret>)) -> Vec<Secret> {
        let ref masks = values.0;
        let ref masked_secrets = values.1;
        assert_eq!(masks.len(), masked_secrets.len());

        let secrets = masked_secrets.iter()
            .zip(masks)
            .map(|(masked_secret, mask)| (masked_secret - mask) % self.modulus)
            .collect();

        secrets
    }
}
