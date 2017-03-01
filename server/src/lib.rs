//! This crates contains the aggregation workflow implementation, regardless
//! transport and storage. It is the real implementation of the SDAService
//! trait, the other ones are proxies.
//!
//! It defines a set of Store interfaces that abstract the database.
//!
//! * simple JFS-based storage for integration test

#[macro_use]
extern crate error_chain;
extern crate jfs;
extern crate serde;
extern crate uuid;

extern crate sda_protocol;

mod errors;
mod server;
mod stores;

pub mod jfs_stores;

pub use server::SdaServer;
use errors::*;
