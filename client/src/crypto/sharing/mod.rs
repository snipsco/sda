//! Code for secret sharing.

mod batched;
mod combiner;
mod additive;
mod packed_shamir;

use super::*;

pub trait ShareGeneratorConstruction<S> {
    fn new_share_generator(&self, scheme: &S) -> SdaClientResult<Box<ShareGenerator>>;
}

pub trait ShareGenerator {
    /// Generate shares for secrets.
    fn generate(&mut self, secrets: &[Secret]) -> SdaClientResult<Vec<Vec<Share>>>;
}

pub trait ShareCombinerConstruction<S> {
    fn new_share_combiner(&self, scheme: &S) -> SdaClientResult<Box<ShareCombiner>>;
}

pub trait ShareCombiner {
    fn combine(&self, shares: &Vec<Vec<Share>>) -> SdaClientResult<Vec<Share>>;
}

pub trait SecretReconstructorConstruction<S> {
    fn new_secret_reconstructor(&self, scheme: &S, dimension: usize) -> SdaClientResult<Box<SecretReconstructor>>;
}

pub trait SecretReconstructor {
    fn reconstruct(&self, indexed_shares: &Vec<(usize, Vec<Share>)>) -> SdaClientResult<Vec<Secret>>;
}

impl ShareGeneratorConstruction<LinearSecretSharingScheme> for CryptoModule {
    fn new_share_generator(&self, scheme: &LinearSecretSharingScheme) -> SdaClientResult<Box<ShareGenerator>> {
        match *scheme {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let generator = additive::AdditiveSecretSharing::new(share_count, modulus);
                Ok(Box::new(generator))
            },

            LinearSecretSharingScheme::PackedShamir { prime_modulus, omega_secrets, omega_shares, .. } => {
                let generator = packed_shamir::Generator::new(
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

            LinearSecretSharingScheme::Additive { modulus, .. } => {
                let combiner = combiner::Combiner::new(modulus);
                Ok(Box::new(combiner))
            },

            LinearSecretSharingScheme::PackedShamir { prime_modulus, .. } => {
                let combiner = combiner::Combiner::new(prime_modulus);
                Ok(Box::new(combiner))
            },

        }
    }
}

impl SecretReconstructorConstruction<LinearSecretSharingScheme> for CryptoModule {
    fn new_secret_reconstructor(&self, scheme: &LinearSecretSharingScheme, dimension: usize) -> SdaClientResult<Box<SecretReconstructor>> {
        match *scheme {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let reconstructor = additive::AdditiveSecretSharing::new(share_count, modulus);
                Ok(Box::new(reconstructor))
            },
            
            LinearSecretSharingScheme::PackedShamir { prime_modulus, omega_secrets, omega_shares, .. } => {
                let reconstructor = packed_shamir::Reconstructor::new(
                    dimension,
                    scheme.privacy_threshold(),
                    scheme.output_size(),
                    scheme.input_size(),
                    prime_modulus, omega_secrets, omega_shares);
                Ok(Box::new(reconstructor))
            },

        }
    }
}
