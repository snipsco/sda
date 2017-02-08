
//! Parameters for the cryptographic primitives supported by the system.

pub enum LinearSecretSharingScheme {

    Additive {
        /// Number of shares to generate for each secret.
        share_count: usize,
        /// Modulus specifying the additive group in which to operate.
        modulus: i64,
    },

    BasicShamir {
        /// Number of shares to generate for each secret.
        share_count: usize,
        /// Prime number specifying the prime field in which to operate.
        prime_modulus: i64,
    },

    PackedShamir {
        /// Dimension of secrets, i.e. number of components in vector.
        secret_count: usize,
        /// Number of shares to generate for each vector of secrets.
        share_count: usize,
        /// Prime number specifying the prime field in which to operate.
        prime_modulus: i64,
        /// TODO
        omega_secrets: i64,
        /// TODO
        omega_shares: i64,
    }

}

pub enum AdditiveEncryptionScheme {

    Sodium,

    PackedPaillier {
        /// Number of components in a plaintext/ciphertext.
        component_count: usize,
        /// Number of bits allocated to each component in a ciphertext.
        component_bitsize: usize,
        /// Maximum number of bits each component value may occupy in a fresh ciphertext, i.e.
        /// in a fresh ciphertext each value must be strictly upper bounded by 2^value_max_bitsize.
        value_max_bitsize: usize,
    }

}

impl AdditiveEncryptionScheme {
    fn additive_capability(&self) -> usize {
        match self {
            Sodium => 1,
            PackedPaillier => 5 // TODO
        }
    }
}
