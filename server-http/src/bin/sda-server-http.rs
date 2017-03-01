extern crate clap;
extern crate sda_server;
extern crate sda_server_http;
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;

use slog::*;

fn main() {
    let root = Logger::root(slog_term::streamer().stderr().use_utc_timestamp().build().fuse(),
                            o!());
    slog_scope::set_global_logger(root);

    let store = sda_server::jfs_stores::JfsAgentStore::new("tmp").unwrap();
    let server = sda_server::SdaServer { agent_store: Box::new(store) };

    sda_server_http::listen("0.0.0.0:8888", server)
}
