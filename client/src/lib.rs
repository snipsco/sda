
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

mod crypto;
mod trust;
mod user;
// mod clerk;

use crypto::*;
pub use sda_protocol::*;
pub use trust::*;
pub use user::*;
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

mod errors {
    error_chain!{
        types {
            SdaClientError, SdaClientErrorKind, SdaClientResultExt, SdaClientResult;
        }
        foreign_links {
            Protocol(::sda_protocol::SdaError);
            Io(::std::io::Error);
            SerdeJson(::serde_json::Error);
            NumParseInt(::std::num::ParseIntError);
            TimeSystemTime(::std::time::SystemTimeError);
        }

    }
}
pub use errors::*;
