//! This crate provides the basic functionality and abstract logic typically needed by participants,
//! clerks, and recipients for interacting with an SDA service.

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate threshold_secret_sharing as tss;
extern crate sodiumoxide;
extern crate integer_encoding;
extern crate rand;

extern crate sda_protocol;

mod errors;
mod crypto;
mod profile;
mod participate;
mod clerk;
mod receive;

pub use errors::{SdaClientResult, SdaClientError};
pub use crypto::{Keystore, KeyStorage, EncryptionKeypair, SignatureKeypair};
pub use profile::{Maintenance};
pub use participate::{Participating, ParticipantInput};
pub use clerk::Clerking;
pub use receive::Receiving;

use sda_protocol::*;
use crypto::CryptoModule;

use std::sync::Arc;

/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient {
    /// Agent to be used when e.g. identifying with the service.
    pub agent: Agent,
    crypto: CryptoModule,
    service: Arc<SdaService>,
}

impl SdaClient {
    /// Create a new client.
    pub fn new(agent: Agent, keystore: Arc<Keystore>, service: Arc<SdaService>) -> SdaClient
    {
        SdaClient {
            agent: agent,
            crypto: CryptoModule::new(keystore),
            service: service,
        }
    }
}
