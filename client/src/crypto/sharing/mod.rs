//! Code for secret sharing.

use super::*;

pub trait ShareGeneratorConstruction<S> {
    fn new_share_generator(&self, scheme: &S) -> SdaClientResult<Box<ShareGenerator>>;
}

pub trait ShareGenerator {
    /// Generate shares for secrets.
    fn generate_shares(&mut self, secrets: &[Secret]) -> Vec<Vec<Share>>;
}


pub trait ShareCombinerConstruction<S> {
    fn new_share_combiner(&self, scheme: &S) -> SdaClientResult<Box<ShareCombiner>>;
}

pub trait ShareCombiner {
    fn combine(&self, shares: &Vec<Vec<Share>>) -> Vec<Share>;
}

mod helpers;
mod additive;
mod packed_shamir;

impl ShareGeneratorConstruction<LinearSecretSharingScheme> for CryptoModule {
    fn new_share_generator(&self, scheme: &LinearSecretSharingScheme) -> SdaClientResult<Box<ShareGenerator>> {
        match *scheme {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let generator = additive::AdditiveSecretSharing::new(share_count, modulus);
                Ok(Box::new(generator))
            },

            LinearSecretSharingScheme::PackedShamir { prime_modulus, omega_secrets, omega_shares, .. } => {
                let generator = packed_shamir::Wrapper::new(
                    scheme.privacy_threshold(),
                    scheme.output_size(),
                    scheme.input_size(),
                    prime_modulus, omega_secrets, omega_shares);
                Ok(Box::new(generator))
            },

        }
    }
}

impl ShareCombinerConstruction<LinearSecretSharingScheme> for CryptoModule {
    fn new_share_combiner(&self, scheme: &LinearSecretSharingScheme) -> SdaClientResult<Box<ShareCombiner>> {
        match *scheme {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let combiner = additive::AdditiveSecretSharing::new(share_count, modulus);
                Ok(Box::new(combiner))
            },

            // TODO
            _ => unimplemented!(),

        }
    }
}