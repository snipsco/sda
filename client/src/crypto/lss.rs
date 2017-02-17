
//! Code for linear secret sharing.

use super::*;
use ::std::iter::repeat;
use rand::{Rng, OsRng};

pub trait ShareGenerator {

    /// Generate shares for values.
    ///
    /// If the number of values is larger than the input size of the underlying scheme then several
    /// shares are generated for each recipient. Zeroes are appended to inputs that are not a multiple
    /// of the input size of the underlying scheme.
    fn generate_shares(&mut self, values: &[Secret]) -> Vec<Vec<Share>>;

}

pub trait ShareGeneratorConstruction {
    fn new_share_generator(&self) -> SdaClientResult<Box<ShareGenerator>>;
}

impl ShareGeneratorConstruction for LinearSecretSharingScheme {

    fn new_share_generator<'a>(&self) -> SdaClientResult<Box<ShareGenerator>> {

        match *self {

            LinearSecretSharingScheme::Additive { share_count, modulus } => {
                let generator = AdditiveSecretSharing {
                    share_count: share_count,
                    modulus: modulus,
                    rng: OsRng::new()?,
                };
                Ok(Box::new(generator))
            }

            LinearSecretSharingScheme::PackedShamir { prime_modulus, omega_secrets, omega_shares, .. } => {
                let pss = tss::packed::PackedSecretSharing {
                    threshold: self.privacy_threshold(),
                    share_count: self.output_size(),
                    secret_count: self.input_size(),
                    prime: prime_modulus,
                    omega_secrets: omega_secrets,
                    omega_shares: omega_shares,
                };
                let generator = PackedShamirWrapper { 
                    batch_input_size: self.input_size(),
                    batch_output_size: self.output_size(),
                    pss_instance: pss,
                };
                Ok(Box::new(generator))
            },

            // TODO implement
            _ => unimplemented!(),
        }

    }

}

pub trait BatchShareGenerator {

    /// Generate shares for a single batch: the number of inputs is assumed to match the input size of the underlying scheme.
    fn generate_shares_for_batch(&mut self, batch_values: &[Secret]) -> Vec<Share>;

    /// Input size of each batch; expected to be constant.
    fn batch_input_size(&self) -> usize;

    /// Output size of each batch; expected to be constant.
    fn batch_output_size(&self) -> usize;

}

impl<G: BatchShareGenerator> ShareGenerator for G {

    fn generate_shares(&mut self, values: &[Secret]) -> Vec<Vec<Share>> {

        let values_per_batch = self.batch_input_size();
        let number_of_shares = self.batch_output_size();
        let number_of_batches = (values.len() + values_per_batch - 1) / values_per_batch;

        let mut shares_grouped_per_recipient: Vec<Vec<Share>> =
            repeat(Vec::with_capacity(number_of_batches))
            .take(number_of_shares)
            .collect();

        for batch_index in 0..number_of_batches {

            // generate shares for each batch, first making sure the length of each is values_per_batch
            let shares = if (batch_index + 1) * values_per_batch <= values.len() {
                // haven't reached the end yet so no need to patch
                let batch_values = &values[batch_index * values_per_batch .. (batch_index + 1) * values_per_batch];
                self.generate_shares_for_batch(batch_values)
            } else {
                // reached the end so may need to pad
                let mut padded = Vec::with_capacity(values_per_batch);
                padded.extend(&values[batch_index * values_per_batch .. ]);
                while padded.len() < values_per_batch { padded.push(0) }
                self.generate_shares_for_batch(&padded)
            };

            // distribute the shares across the clerks
            for (recipient, share) in shares.iter().enumerate() {
                shares_grouped_per_recipient[recipient].push(*share);
            }
        }

        shares_grouped_per_recipient
    }

}

struct AdditiveSecretSharing {
    share_count: usize,
    modulus: i64,
    rng: OsRng,
}

impl BatchShareGenerator for AdditiveSecretSharing {

    fn batch_input_size(&self) -> usize { 
        1
    }

    fn batch_output_size(&self) -> usize {
        self.share_count
    }

    fn generate_shares_for_batch(&mut self, batch_input: &[Secret]) -> Vec<Share> {
        assert_eq!(batch_input.len(), 1);
        let secret = batch_input[0];

        // TODO we assume that the values are really i32 to prevent overflow -- make this explicit!!
        // TODO specifically, that self.modulus fits within i32

        // pick share_count - 1 random values from group
        let mut shares: Vec<Share> = repeat(self.rng.gen_range(0_i64, self.modulus))
            .take(self.share_count - 1)
            .collect();

        // compute the last share as the secret minus the sum of all other shares
        let last_share = shares.iter().fold(-secret, |sum, &x| { (sum + x) % self.modulus });
        shares.push(last_share);

        shares
    }

}

struct PackedShamirWrapper {
    batch_input_size: usize,
    batch_output_size: usize,
    pss_instance: tss::packed::PackedSecretSharing,
}

impl BatchShareGenerator for PackedShamirWrapper {

    fn batch_input_size(&self) -> usize { 
        self.batch_input_size
    }

    fn batch_output_size(&self) -> usize {
        self.batch_output_size
    }

    fn generate_shares_for_batch(&mut self, batch_input: &[Secret]) -> Vec<Share> {
        self.pss_instance.share(batch_input)
    }

}
