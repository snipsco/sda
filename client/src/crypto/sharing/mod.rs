//! Code for secret sharing.

use crypto::*;
use errors::SdaClientResult;
use sda_protocol::LinearSecretSharingScheme;

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

impl<K> ShareGeneratorConstruction<LinearSecretSharingScheme> for CryptoModule<K> {
    fn new_share_generator(&self, scheme: &LinearSecretSharingScheme) -> SdaClientResult<Box<ShareGenerator>> {
        match *scheme {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let generator = additive::AdditiveSecretSharing::new(share_count, modulus);
                Ok(Box::new(generator))
            },

            _ => unimplemented!() // TODO

            // LinearSecretSharingScheme::PackedShamir { prime_modulus, omega_secrets, omega_shares, .. } => {
            //     let pss = tss::packed::PackedSecretSharing {
            //         threshold: self.privacy_threshold(),
            //         share_count: self.output_size(),
            //         secret_count: self.input_size(),
            //         prime: prime_modulus,
            //         omega_secrets: omega_secrets,
            //         omega_shares: omega_shares,
            //     };
            //     let generator = packed_shamir::Wrapper { 
            //         batch_input_size: self.input_size(),
            //         batch_output_size: self.output_size(),
            //         pss_instance: pss,
            //     };
            //     Ok(Box::new(generator))
            // },

        }
    }
}

impl<K> ShareCombinerConstruction<LinearSecretSharingScheme> for CryptoModule<K> {
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