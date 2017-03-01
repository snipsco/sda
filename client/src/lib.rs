
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
pub mod keystore;
mod trust;
mod cache;
mod service;
mod profile;
mod participate;
mod clerk;

pub use sda_protocol::*;
use crypto::*;
// use keystore::*;
use trust::{Policy};
use cache::{NoCache};
use service::*;
use profile::*;

pub use errors::*;
pub use participate::{Participating};
pub use clerk::{Clerking};
pub use keystore::*;
pub use profile::{Maintenance};

/// Primary object for interacting with the SDA service.
///
/// For instance used by participants to provide input to aggregations and by clerks to perform their clerking tasks.
pub struct SdaClient<C, S> {
    agent: Agent,
    keystore: keystore::Filebased,
    trust: trust::Permissistic,
    cache: C,
    sda_service: S,
}

impl<S> SdaClient<NoCache, S> {
    pub fn new_from_prefix<P: AsRef<std::path::Path>>(prefix: P, sda_service: S) -> SdaClient<NoCache, S> {
        SdaClient {
            agent: Agent::default(),
            keystore: keystore::Filebased::new(prefix.as_ref().join("user").join("keys")).unwrap(),
            trust: trust::Permissistic,
            cache:       NoCache {},
            sda_service: sda_service,
        }
    }
}