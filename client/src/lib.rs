
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
mod keystore;
mod trust;
mod service;
// mod profile;
mod participate;
mod clerk;


pub use sda_protocol::*;
pub use errors::*;
use crypto::*;
use keystore::*;
use trust::*;
use service::*;
// pub use profile::*;
pub use participate::{Participate};
pub use clerk::{Clerk};


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