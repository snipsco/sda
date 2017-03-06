//! This crate provides HTTP access to the SDA services for clients.

extern crate rand;
#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

extern crate sda_protocol;
extern crate sda_client_store;

mod errors;
mod authtoken;
mod client;

pub use client::{SdaHttpClient};
pub use errors::{SdaHttpClientError};