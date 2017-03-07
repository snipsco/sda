use super::*;
use super::helpers::BatchShareGenerator;

use tss;

pub struct Wrapper {
    pub batch_input_size: usize,
    pub batch_output_size: usize,
    pub pss_instance: tss::packed::PackedSecretSharing,
}

impl Wrapper {
    pub fn new(threshold: usize, share_count: usize, secret_count: usize, prime_modulus: i64, omega_secrets: i64, omega_shares: i64) -> Wrapper {
        let pss = tss::packed::PackedSecretSharing {
            threshold: threshold,
            share_count: share_count,
            secret_count: secret_count,
            prime: prime_modulus,
            omega_secrets: omega_secrets,
            omega_shares: omega_shares,
        };
        packed_shamir::Wrapper { 
            batch_input_size: secret_count,
            batch_output_size: share_count,
            pss_instance: pss,
        }
    }
}

impl BatchShareGenerator for Wrapper {

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