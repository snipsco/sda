
extern crate sda_protocol;

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate threshold_secret_sharing as tss;

mod keys;
mod crypto;
mod errors;
mod user;
// mod clerk;

use sda_protocol::*;
use keys::*;
use crypto::*;
pub use errors::*;
pub use user::*;
// pub use clerk::*;

pub struct SdaClient<'l, S: 'l, K: 'l> {
    agent: &'l Agent,
    key_service: &'l K,
    sda_service: &'l S,
}

impl<'l, S: 'l, K: 'l> SdaClient<'l, S, K> {
    pub fn new(agent: &'l Agent, key_service: &'l K, sda_service: &'l S) -> SdaClient<'l, S, K> {
        SdaClient {
            agent: agent,
            key_service: key_service,
            sda_service: sda_service,
        }
    }
}
