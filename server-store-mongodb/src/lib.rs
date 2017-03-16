extern crate mongodb;
extern crate sda_protocol;
extern crate sda_server;

use sda_protocol::*;
use sda_server::{ SdaServer, SdaServerService };

pub fn new_mongodb_server<P: AsRef<::std::path::Path>>(client: &mongodb::Client, db:&str, dir:P) -> SdaResult<SdaServerService> {
    let dir = dir.as_ref();
    let agents = sda_server::jfs_stores::JfsAgentsStore::new(dir.join("agents")).unwrap();
    let auth = sda_server::jfs_stores::JfsAuthTokensStore::new(dir.join("auths")).unwrap();
    let agg = sda_server::jfs_stores::JfsAggregationsStore::new(dir.join("agg")).unwrap();
    let jobs = sda_server::jfs_stores::JfsClerkingJobStore::new(dir.join("jobs")).unwrap();
    Ok(SdaServerService(SdaServer {
        agents_store: Box::new(agents),
        auth_tokens_store: Box::new(auth),
        aggregation_store: Box::new(agg),
        clerking_job_store: Box::new(jobs),
    }))
}
