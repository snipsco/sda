use super::*;

use ::std::iter::repeat;

pub trait BatchShareGenerator {

    /// Generate shares for a single batch: the number of inputs is assumed to match the input size of the underlying scheme.
    fn generate_for_batch(&mut self, batch_values: &[Secret]) -> Vec<Share>;

    /// Input size of each batch; expected to be constant.
    fn batch_input_size(&self) -> usize;

    /// Output size of each batch; expected to be constant.
    fn batch_output_size(&self) -> usize;

}

impl<G: BatchShareGenerator> ShareGenerator for G {
    fn generate(&mut self, secrets: &[Secret]) -> Vec<Vec<Share>> {

        let secrets_per_batch = self.batch_input_size();
        let number_of_shares = self.batch_output_size();
        let number_of_batches = (secrets.len() + secrets_per_batch - 1) / secrets_per_batch;

        let mut shares_grouped_per_recipient: Vec<Vec<Share>> =
            repeat(Vec::with_capacity(number_of_batches))
            .take(number_of_shares)
            .collect();

        for batch_index in 0..number_of_batches {

            // generate shares for each batch, first making sure the length of each is secrets_per_batch
            let shares = if (batch_index + 1) * secrets_per_batch <= secrets.len() {
                // haven't reached the end yet so no need to patch
                let batch_secrets = &secrets[batch_index * secrets_per_batch .. (batch_index + 1) * secrets_per_batch];
                self.generate_for_batch(batch_secrets)
            } else {
                // reached the end so may need to pad
                let mut padded = Vec::with_capacity(secrets_per_batch);
                padded.extend(&secrets[batch_index * secrets_per_batch .. ]);
                while padded.len() < secrets_per_batch { padded.push(0) }
                self.generate_for_batch(&padded)
            };

            // distribute the shares across the clerks
            for (recipient, share) in shares.iter().enumerate() {
                shares_grouped_per_recipient[recipient].push(*share);
            }
        }
        
        shares_grouped_per_recipient
    }
}

pub trait BatchSecretReconstructor {

    /// Reconstruct secrets for a single batch: the number of inputs is assumed to match the input size of the underlying scheme.
    fn reconstruct_for_batch(&self, indices: &[usize], batch_shares: &[Secret]) -> Vec<Share>;

    /// Output size of batch; expected to be constant.
    fn batch_output_size(&self) -> usize;

    /// Output size of final result; expected to be constant.
    fn output_size(&self) -> usize;

}

impl<G: BatchSecretReconstructor> SecretReconstructor for G {
    fn reconstruct(&self, indexed_shares: &Vec<(usize, Vec<Share>)>) -> Vec<Secret> {
        
        let indices: Vec<usize> = indexed_shares.iter()
            .map(|&(index, _)| index)
            .collect();
        
        let batch_input_size = indexed_shares.len();
        let batch_output_size = self.batch_output_size();
        let number_of_batches = (self.output_size() + batch_output_size - 1) / batch_output_size; // ceiling
        
        let mut batch_shares: Vec<Share> = vec![0; batch_input_size];
        let mut secrets: Vec<Secret> = Vec::with_capacity(number_of_batches * batch_output_size);
        
        for batch_index in 0..number_of_batches {
            for share_index in 0..batch_input_size {
                batch_shares[share_index] = indexed_shares[share_index].1[batch_index];
            }
            
            let batch_secrets = self.reconstruct_for_batch(&indices, &batch_shares);
            for secret in batch_secrets {
                secrets.push(secret);
            }
        }
        
        // drop any extra values that were appended during share generation
        secrets.truncate(self.output_size());
        
        secrets
    }
    
}
