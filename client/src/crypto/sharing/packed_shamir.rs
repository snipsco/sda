
use super::*;


pub struct Wrapper {
    pub batch_input_size: usize,
    pub batch_output_size: usize,
    pub pss_instance: tss::packed::PackedSecretSharing,
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