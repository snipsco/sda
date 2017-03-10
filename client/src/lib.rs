//! This crate provides the basic functionality needed by clerks and participants for interacting with an SDA service.

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
mod trust;
mod profile;
mod participate;
mod clerk;
mod receive;
// mod cache;

pub use errors::{SdaClientResult, SdaClientError};
pub use crypto::{Keystore, KeyStorage, EncryptionKeypair, SignatureKeypair};
pub use profile::{Maintenance};
pub use participate::{Participating, ParticipantInput};
pub use clerk::Clerking;
pub use receive::Receive;

// pub use cache::CachedService;

use sda_protocol::*;
use crypto::CryptoModule;

use std::sync::Arc;

/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient {
    pub agent: Agent,
    crypto: CryptoModule,
    service: Arc<SdaService>,
    trust: trust::Pessimistic,
}

impl SdaClient {
    pub fn new(agent: Agent, keystore: Arc<Keystore>, service: Arc<SdaService>) -> SdaClient
     {
        SdaClient {
            agent: agent,
            crypto: CryptoModule::new(keystore),
            service: service,
            trust: trust::Pessimistic,
        }
    }
}
