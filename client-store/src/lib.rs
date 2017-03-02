//! This crate provides storage for clients.

#[macro_use]
extern crate error_chain;
extern crate jfs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod errors;
mod file;

pub use file::{Filebased};
pub use errors::{SdaClientStoreError};