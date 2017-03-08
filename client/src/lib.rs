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
// mod cache;

pub use participate::{Participating, ParticipantInput};
pub use clerk::Clerking;
pub use profile::{new_agent, Maintenance};
pub use errors::{SdaClientResult, SdaClientError};
pub use crypto::{Keystore, KeyStorage, EncryptionKeypair, SignatureKeypair};
// pub use cache::CachedService;

use sda_protocol::*;
use crypto::CryptoModule;

pub trait Service : 
    Send
    + Sync
    + SdaService
    + SdaAgentService
    + SdaAggregationService
    + SdaClerkingService
    + SdaParticipationService 
{}

impl<T> Service for T where T: 
    Send
    + Sync
    + SdaService
    + SdaAgentService
    + SdaAggregationService
    + SdaClerkingService
    + SdaParticipationService
{}

/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient {
    agent: Agent,
    crypto: CryptoModule,
    service: Box<Service>,
    trust: trust::Pessimistic,
}

impl SdaClient {
    pub fn new(agent: Agent, keystore: Box<Keystore>, service: Box<Service>) -> SdaClient
     {
        SdaClient {
            agent: agent,
            crypto: CryptoModule::new(keystore),
            service: service,
            trust: trust::Pessimistic,
        }
    }
}