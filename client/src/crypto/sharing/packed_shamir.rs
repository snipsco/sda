use super::*;
use super::batched::{BatchShareGenerator, BatchSecretReconstructor};

use tss;

pub struct Generator {
    batch_input_size: usize,
    batch_output_size: usize,
    pss_instance: tss::packed::PackedSecretSharing,
}

impl Generator {
    pub fn new(threshold: usize, share_count: usize, secret_count: usize, prime_modulus: i64, omega_secrets: i64, omega_shares: i64) -> Generator {
        let pss = tss::packed::PackedSecretSharing {
            threshold: threshold,
            share_count: share_count,
            secret_count: secret_count,
            prime: prime_modulus,
            omega_secrets: omega_secrets,
            omega_shares: omega_shares,
        };
        Generator { 
            batch_input_size: secret_count,
            batch_output_size: share_count,
            pss_instance: pss,
        }
    }
}

impl BatchShareGenerator for Generator {

    fn batch_input_size(&self) -> usize { 
        self.batch_input_size
    }

    fn batch_output_size(&self) -> usize {
        self.batch_output_size
    }

    fn generate_for_batch(&mut self, batch_input: &[Secret]) -> Vec<Share> {
        self.pss_instance.share(batch_input)
    }

}

pub struct Reconstructor {
    batch_output_size: usize,
    output_size: usize,
    pss_instance: tss::packed::PackedSecretSharing,
}

impl Reconstructor {
    pub fn new(dimension: usize, threshold: usize, share_count: usize, secret_count: usize, prime_modulus: i64, omega_secrets: i64, omega_shares: i64) -> Reconstructor {
        let pss = tss::packed::PackedSecretSharing {
            threshold: threshold,
            share_count: share_count,
            secret_count: secret_count,
            prime: prime_modulus,
            omega_secrets: omega_secrets,
            omega_shares: omega_shares,
        };
        Reconstructor {
            batch_output_size: secret_count,
            output_size: dimension,
            pss_instance: pss,
        }
    }
}

impl BatchSecretReconstructor for Reconstructor {
    
    fn reconstruct_for_batch(&self, indices: &[usize], batch_shares: &[Secret]) -> Vec<Share> {
        self.pss_instance.reconstruct(indices, batch_shares)
    }

    fn batch_output_size(&self) -> usize {
        self.batch_output_size
    }
    
    fn output_size(&self) -> usize {
        self.output_size
    }
    
}
