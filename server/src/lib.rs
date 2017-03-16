//! This crates contains the aggregation workflow implementation, regardless
//! transport and storage. It is the real implementation of the SDAService
//! trait, the other ones are proxies.
//!
//! It defines a set of Store interfaces that abstract the database.
//!
//! * simple JFS-based storage for integration test

#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate jfs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

extern crate sda_protocol;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;

pub mod errors;
mod server;
mod snapshot;

pub mod stores;
pub mod jfs_stores;

pub use server::{ SdaServer, SdaServerService };
use errors::*;

pub fn new_jfs_server<P: AsRef<::std::path::Path>>(dir: P) -> sda_protocol::SdaResult<SdaServerService> {
    let agents = ::jfs_stores::JfsAgentsStore::new(dir.as_ref().join("agents")).unwrap();
    let auth = ::jfs_stores::JfsAuthTokensStore::new(dir.as_ref().join("auths")).unwrap();
    let agg = ::jfs_stores::JfsAggregationsStore::new(dir.as_ref().join("agg")).unwrap();
    let jobs = ::jfs_stores::JfsClerkingJobStore::new(dir.as_ref().join("jobs")).unwrap();
    Ok(SdaServerService(SdaServer {
        agents_store: Box::new(agents),
        auth_tokens_store: Box::new(auth),
        aggregation_store: Box::new(agg),
        clerking_job_store: Box::new(jobs),
    }))
}
