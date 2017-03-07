
// struct PackedPaillierWrapper {
//     eek: paillier::coding::EncodingEncryptionKey<_>,
//     component_count: usize,
//     component_bitsize: usize,

// }

// impl Encryptor for PackedPaillierWrapper {

//     fn batch_input_size(&self) -> usize {
//         self.batch_input_size
//     }

//     fn batch_output_size(&self) -> usize {
//         self.batch_output_size
//     }

//     fn generate_shares_for_batch(&self, batch_input: &[i64]) -> Vec<i64> {
//         self.pss_instance.share(batch_input)
//     }

// }