
//! This crate describes the common interface of SDA, including the resources,
//! methods, and message format exposed by an SDA service.
//!
//! As such it is lightweight crate referenced by most of the other (Rust) crates.
//!
//! It takes a resource-oriented REST approach whenever possible, 
//! influenced by the [Google API Design Guide](https://cloud.google.com/apis/design/).

extern crate data_encoding;
#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_test;
extern crate uuid;

mod errors {
    error_chain! {
        types {
            SdaError, SdaErrorKind, SdaResultExt, SdaResult;
        }
        errors {
            PermissionDenied {
                description("permission denied")
            }
            InvalidCredentials {
                description("invalid credentials")
            }
        }
        foreign_links {
            SerdeJson(::serde_json::Error);
        }
    }
}

use uuid::Uuid;
pub use errors::*;

mod crypto;
mod resources;
mod methods;
mod helpers;
pub mod byte_arrays;

pub use crypto::*;
pub use resources::*;
pub use methods::*;
pub use helpers::*;
