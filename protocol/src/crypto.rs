
//! Parameters for the cryptographic primitives supported by the system.


#[derive(Debug)]
pub enum Encryption {
    Sodium(Vec<u8>)
}

pub enum Signature {
    Sodium([u8; 64])
}

pub enum EncryptionKey {
    Sodium([u8; 0]) // TODO what is right size?
}

pub enum SigningKey {
    Sodium([u8; 64])
}

#[derive(Eq, PartialEq)]
pub enum VerificationKey {
    Sodium([u8; 32])
}

/// Supported masking schemes and their parameters.
pub enum LinearMaskingScheme {
    
    None,

    // Full { 
    //     modulus: i64 
    // },

}

/// Supported secret sharing schemes and their parameters.
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
        /// Number of shares needed to reconstruct.
        privacy_threshold: usize,
        /// Prime number specifying the prime field in which to operate.
        prime_modulus: i64,
    },

    PackedShamir {
        /// Dimension of secrets, i.e. number of components in vector.
        secret_count: usize,
        /// Number of shares to generate for each vector of secrets.
        share_count: usize,
        /// Number of shares needed to reconstruct.
        privacy_threshold: usize,
        /// Prime number specifying the prime field in which to operate.
        prime_modulus: i64,
        /// TODO
        omega_secrets: i64,
        /// TODO
        omega_shares: i64,
    },

}

/// Derived properties of the secret sharing schemes.
impl LinearSecretSharingScheme {

    pub fn input_size(&self) -> usize {
        match *self {
            LinearSecretSharingScheme::Additive {..} => 1,
            LinearSecretSharingScheme::BasicShamir {..} => 1,
            LinearSecretSharingScheme::PackedShamir { secret_count, .. } => secret_count,
        }
    }

    pub fn output_size(&self) -> usize {
        match *self {
            LinearSecretSharingScheme::Additive { share_count, .. } => share_count,
            LinearSecretSharingScheme::BasicShamir { share_count, .. } => share_count,
            LinearSecretSharingScheme::PackedShamir { share_count, .. } => share_count,
        }
    }

    pub fn privacy_threshold(&self) -> usize {
        match *self {
            LinearSecretSharingScheme::Additive { share_count, .. } => share_count - 1,
            LinearSecretSharingScheme::BasicShamir { privacy_threshold, .. } => privacy_threshold,
            LinearSecretSharingScheme::PackedShamir { privacy_threshold, .. } => privacy_threshold,
        }
    }

    pub fn reconstruction_threshold(&self) -> usize {
        match *self {
            LinearSecretSharingScheme::Additive { share_count, .. } => share_count,
            LinearSecretSharingScheme::BasicShamir { privacy_threshold, .. } => privacy_threshold + 1,
            LinearSecretSharingScheme::PackedShamir { privacy_threshold, secret_count, .. } => privacy_threshold + secret_count,
        }
    }

}

/// Supported additive encryption schemes and their parameters.
pub enum AdditiveEncryptionScheme {

    Sodium,

    // PackedPaillier {
    //     /// Number of components in a plaintext/ciphertext.
    //     component_count: usize,
    //     /// Number of bits allocated to each component in a ciphertext.
    //     component_bitsize: usize,
    //     /// Maximum number of bits each component value may occupy in a fresh ciphertext, i.e.
    //     /// in a fresh ciphertext each value must be strictly upper bounded by 2^value_max_bitsize.
    //     max_value_bitsize: usize,
    //     /// Minimum size of the (plaintext) modulus in bits.
    //     min_modulus_bitsize: usize,
    // }

}

/// Derived properties of the additive encryption schemes.
impl AdditiveEncryptionScheme {

    pub fn batch_size(&self) -> usize {
        match self {
            &AdditiveEncryptionScheme::Sodium {..} => 1,
            // &AdditiveEncryptionScheme::PackedPaillier { component_count, .. } => component_count
        }
    }
    
}
