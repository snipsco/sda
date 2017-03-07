use super::*;

use ::std::iter::repeat;

pub trait BatchShareGenerator {

    /// Generate shares for a single batch: the number of inputs is assumed to match the input size of the underlying scheme.
    fn generate_shares_for_batch(&mut self, batch_values: &[Secret]) -> Vec<Share>;

    /// Input size of each batch; expected to be constant.
    fn batch_input_size(&self) -> usize;

    /// Output size of each batch; expected to be constant.
    fn batch_output_size(&self) -> usize;

}

impl<G: BatchShareGenerator> ShareGenerator for G {
    fn generate_shares(&mut self, secrets: &[Secret]) -> Vec<Vec<Share>> {

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
                self.generate_shares_for_batch(batch_secrets)
            } else {
                // reached the end so may need to pad
                let mut padded = Vec::with_capacity(secrets_per_batch);
                padded.extend(&secrets[batch_index * secrets_per_batch .. ]);
                while padded.len() < secrets_per_batch { padded.push(0) }
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