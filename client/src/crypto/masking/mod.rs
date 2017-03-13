//! Code for masking.

mod none;
mod full;
mod chacha;

use super::*;

pub trait SecretMaskerConstruction<S> {
    fn new_secret_masker(&self, scheme: &S) -> SdaClientResult<Box<SecretMasker>>;
}

pub trait SecretMasker {
    fn mask(&mut self, secrets: &[Secret]) -> (Vec<Mask>, Vec<MaskedSecret>);
}

pub trait MaskCombinerConstruction<S> {
    fn new_mask_combiner(&self, scheme: &S) -> SdaClientResult<Box<MaskCombiner>>;
}

pub trait MaskCombiner {
    fn combine(&self, masks: &Vec<Vec<Mask>>) -> Vec<Mask>;
}

pub trait SecretUnmaskerConstruction<S> {
    fn new_secret_unmasker(&self, scheme: &S) -> SdaClientResult<Box<SecretUnmasker>>;
}

pub trait SecretUnmasker {
    fn unmask(&self, values: &(Vec<Mask>, Vec<MaskedSecret>)) -> Vec<Secret>;
}

impl SecretMaskerConstruction<LinearMaskingScheme> for CryptoModule {
    fn new_secret_masker(&self, scheme: &LinearMaskingScheme) -> SdaClientResult<Box<SecretMasker>> {
        match *scheme {
            LinearMaskingScheme::None => {
                let masker = none::Masker::new();
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::Full { modulus } => {
                let masker = full::Masker::new(modulus);
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::ChaCha { modulus, dimension, seed_bitsize } => {
                let masker = chacha::Masker::new(modulus, dimension, seed_bitsize);
                Ok(Box::new(masker))
            },
        }
    }
}

impl MaskCombinerConstruction<LinearMaskingScheme> for CryptoModule {
    fn new_mask_combiner(&self, scheme: &LinearMaskingScheme) -> SdaClientResult<Box<MaskCombiner>> {
        match *scheme {
            LinearMaskingScheme::None => {
                let masker = none::Masker::new();
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::Full { modulus } => {
                let masker = full::Masker::new(modulus);
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::ChaCha { modulus, dimension, seed_bitsize } => {
                let masker = chacha::Masker::new(modulus, dimension, seed_bitsize);
                Ok(Box::new(masker))
            },
        }
    }
}

impl SecretUnmaskerConstruction<LinearMaskingScheme> for CryptoModule {
    fn new_secret_unmasker(&self, scheme: &LinearMaskingScheme) -> SdaClientResult<Box<SecretUnmasker>> {
        match *scheme {
            LinearMaskingScheme::None => {
                let masker = none::Masker::new();
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::Full { modulus } => {
                let masker = full::Masker::new(modulus);
                Ok(Box::new(masker))
            },

            LinearMaskingScheme::ChaCha { modulus, dimension, seed_bitsize } => {
                let masker = chacha::Masker::new(modulus, dimension, seed_bitsize);
                Ok(Box::new(masker))
            },
        }
    }
}
