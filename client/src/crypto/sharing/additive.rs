
use super::*;


pub struct AdditiveSecretSharing {
    pub share_count: usize,
    pub modulus: i64,
    pub rng: OsRng,
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

        // NOTE
        // we assume that the values are really i32 to prevent overflow -- make this explicit!!
        // specifically, that self.modulus fits within i32

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

impl ShareCombiner for AdditiveSecretSharing {
    fn combine(&self, shares: &Vec<Vec<Share>>) -> Vec<Share> {
        let dimension: usize = shares.get(0).map_or(0, Vec::len);

        let mut result: Vec<Share> = repeat(0).take(dimension).collect();
        for share in shares {
            assert!(share.len() == dimension);
            for (ix, value) in share.iter().enumerate() {
                result[ix] += *value;
                result[ix] %= self.modulus;
            }
        }

        result
    }
}