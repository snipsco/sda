
//! TODO

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
mod store;
mod trust;
mod identity;
mod fetch;
// mod profile;
mod discover;
mod participate;
mod clerk;


pub use sda_protocol::*;
pub use errors::*;
use crypto::*;
use store::*;
use trust::*;
use identity::*;
use fetch::*;
// pub use profile::*;
pub use discover::{Discover};
pub use participate::{Participate};
pub use clerk::{Clerk};


pub struct SdaClient<L, I, S> {
    agent: Agent,
    local_store: L,
    identity: I,
    sda_service: S,
}

impl<L, I, S> SdaClient<L, I, S> {
    pub fn new(agent: Agent, local_store: L, identity: I, sda_service: S) -> SdaClient<L, I, S> {
        SdaClient {
            agent: agent,
            local_store: local_store,
            identity: identity,
            sda_service: sda_service,
        }
    }
}