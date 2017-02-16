
//! This crate describes the common interface of SDA, including the operations
//! exposed by an SDA service and the message format used.
//!
//! As such it is lightweight crate referenced by most of the other (Rust) crates.
//!
//! It takes a REST approach whenever possible.

#[macro_use]
extern crate error_chain;
extern crate uuid;

mod errors {
    error_chain! {
        types {
            SdaError, SdaErrorKind, SdaResultExt, SdaResult;
        }
    }
}

pub use uuid::Uuid;
pub use errors::*;

mod crypto;
mod protocol;
mod helpers;

pub use crypto::*;
pub use protocol::*;
pub use helpers::*;
