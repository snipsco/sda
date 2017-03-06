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
extern crate sda_client_store;

mod errors;
mod crypto;
mod keystore;
mod trust;
mod service;
mod profile;
mod participate;
mod clerk;

pub use participate::{Participating, ParticipantInput};
pub use clerk::Clerking;
pub use profile::{load_agent, new_agent, Maintenance};
pub use errors::SdaClientError;

use sda_protocol::Agent;
use service::NoCache;

/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient<K, C, S> {
    agent: Agent,
    keystore: K,
    trust: trust::Pessimistic,
    cache: C,
    service: S,
}

impl<K, S> SdaClient<K, NoCache, S> {
    pub fn new(agent: Agent, keystore: K, service: S) -> SdaClient<K, NoCache, S> {
        SdaClient {
            agent: agent,
            keystore: keystore,
            trust: trust::Pessimistic,
            cache: service::NoCache,
            service: service,
        }
    }
}