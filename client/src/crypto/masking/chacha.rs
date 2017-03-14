//! Full masker, using full masks.

use super::*;

use rand::{Rng, OsRng, SeedableRng, ChaChaRng};
use ::std::iter::repeat;

pub struct Masker {
    modulus: i64,
    dimension: usize,

    /// Note: the PRG will use at most 256 bits according to the standard documentation.
    seed_bitsize: usize,
}

impl Masker {
    pub fn new(modulus: i64, dimension: usize, seed_bitsize: usize) -> Masker {
        Masker {
            modulus: modulus,
            dimension: dimension,
            seed_bitsize: seed_bitsize,
        }
    }
}

impl SecretMasker for Masker {
    fn mask(&mut self, secrets: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>) {
        assert_eq!(self.dimension, secrets.len());

        // generate new seed using OsRng
        let mut seed_generator = OsRng::new().unwrap(); // TODO not nice
        let seed_wordsize: usize = (self.seed_bitsize + 31) / 32; // ceil(x/32)
        let seed: Vec<u32> = (0..seed_wordsize)
            .map(|_| seed_generator.next_u32())
            .collect();

        // generate new mask from seed
        let mut mask_generator = ChaChaRng::from_seed(&seed);
        let mask: Vec<Mask> = (0..secrets.len())
            .map(|_| mask_generator.gen_range(0_i64, self.modulus))
            .collect();

        // add mask to secrets
        let masked_secrets = secrets.iter()
            .zip(&mask)
            .map(|(s, m)| (s + m) % self.modulus)
            .collect();

        // convert seed to match expected `Vec<Mask>`
        let seed_as_i64 = seed.iter()
            .map(|&s| s as Mask)
            .collect();

        (seed_as_i64, masked_secrets)
    }
}

impl MaskCombiner for Masker {
    fn combine(&self, seeds_as_i64: &Vec<Vec<Mask>>) -> Vec<Mask> {
        let mut result: Vec<Share> = repeat(0).take(self.dimension).collect();

        for seed_as_i64 in seeds_as_i64 {
            // convert seed from `Vec<Mask>`
            let seed: Vec<u32> = seed_as_i64.iter()
                .map(|&s| s as u32)
                .collect();

            // re-generate mask based on seed and add to result
            let mut mask_generator = ChaChaRng::from_seed(&seed);
            for i in 0..self.dimension {
                let m = mask_generator.gen_range(0_i64, self.modulus);
                result[i] += m;
                result[i] %= self.modulus;
            }
        }

        result
    }
}

impl SecretUnmasker for Masker {
    fn unmask(&self, values: &(Vec<Mask>, Vec<MaskedSecret>)) -> Vec<Secret> {
        let ref mask = values.0;
        let ref masked_secrets = values.1;
        assert_eq!(mask.len(), masked_secrets.len());

        // sustract mask from masked secrets
        let secrets = masked_secrets.iter()
            .zip(mask)
            .map(|(ms, m)| (ms - m) % self.modulus)
            .collect();

        secrets
    }
}
