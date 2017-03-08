//! This crate provides storage for clients.

#[macro_use]
extern crate error_chain;
extern crate jfs;
extern crate serde;
extern crate serde_json;

extern crate sda_protocol;
extern crate sda_client;

mod errors;
mod store;
mod file;

use sda_protocol::*;
use sda_client::*;

pub use errors::{SdaClientStoreError, SdaClientStoreResult};
pub use store::Store;
pub use file::Filebased;