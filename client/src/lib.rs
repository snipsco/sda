
extern crate sda_protocol;

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


mod errors;
mod crypto;
mod trust;
// mod identity;
mod discover;
mod participate;
// mod clerk;

pub use errors::*;
pub use sda_protocol::*;
use crypto::*;
pub use trust::*;
// pub use identity::*;
pub use discover::*;
pub use participate::*;
// pub use clerk::*;


pub struct SdaClient<T, S> {
    agent: Agent,
    trust_store: T,
    sda_service: S,
}

impl<T, S> SdaClient<T, S> {
    pub fn new(agent: Agent, trust_store: T, sda_service: S) -> SdaClient<T, S> {
        SdaClient {
            agent: agent,
            trust_store: trust_store,
            sda_service: sda_service,
        }
    }
}