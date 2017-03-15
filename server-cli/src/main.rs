extern crate clap;
extern crate sda_server;
extern crate sda_server_http;
extern crate slog;
extern crate slog_term;
extern crate slog_scope;

use std::sync;
use slog::*;

fn main() {
    let root = Logger::root(slog_term::streamer().stderr().use_utc_timestamp().build().fuse(),
                            o!());
    slog_scope::set_global_logger(root);
    let server = sda_server::new_jfs_server("tmp").unwrap();
    sda_server_http::listen("0.0.0.0:8888", sync::Arc::new(server))
}
