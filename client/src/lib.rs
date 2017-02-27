
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
extern crate jfs;

extern crate sda_protocol;


mod errors;
mod crypto;
mod keystore;
mod trust;
mod service;
// mod profile;
mod participate;
mod clerk;


use sda_protocol::*;
use crypto::*;
use keystore::*;
use trust::*;
use service::*;
// pub use profile::*;
pub use errors::*;
pub use participate::{Participating};
pub use clerk::{Clerking};


/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient<C, K, S> {
    agent: Agent,
    cache: C,
    key_store: K,
    sda_service: S,
}

impl<C, K, S> SdaClient<C, K, S> {
    pub fn new(agent: Agent, cache: C, key_store: K, sda_service: S) -> SdaClient<C, K, S> {
        SdaClient {
            agent: agent,
            cache: cache,
            key_store: key_store,
            sda_service: sda_service,
        }
    }
}