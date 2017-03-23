extern crate clap;
extern crate sda_protocol;
extern crate sda_server;
extern crate sda_server_http;
#[cfg(feature="mongodb")]
extern crate sda_server_store_mongodb;
extern crate slog;
extern crate slog_term;
extern crate slog_scope;
extern crate tempdir;

use sda_protocol::*;
use slog::*;

pub fn add_verbose_arg<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    app.arg_from_usage("-v, --verbose... 'verbose mode'")
}

pub fn add_store_args<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    let mut app = app.arg_from_usage("--jfs [jfs_root] 'use a JFS store'");
    if cfg!(feature = "mongodb") {
        app = app.arg_from_usage("--mongo [mongo_url] 'use a mongodb store'")
            .arg_from_usage("--mongo_dbname [mongo_dbname] 'mongodb database name (default is \
                             sda)'")
    }
    app
}

pub fn setup_slog(matches: &clap::ArgMatches) {
    let root = slog_term::streamer().stderr().use_utc_timestamp().build().fuse();
    let root = level_filter(Level::from_usize(2 + matches.occurrences_of("verbose") as usize)
                                .unwrap_or(Level::Warning),
                            root);
    let root = Logger::root(root, o!());
    slog_scope::set_global_logger(root);
}

pub fn build_backend_server(matches: &clap::ArgMatches) -> SdaResult<sda_server::SdaServerService> {
    if let Some(server) = build_mongo_server(matches)? {
        return Ok(server)
    }
    if let Some(root) = matches.value_of("jfs") {
        return sda_server::new_jfs_server(root)
    }
    Err("need a store configuration")?
}

#[cfg(feature="mongodb")]
pub fn build_mongo_server(matches: &clap::ArgMatches)
                          -> SdaResult<Option<sda_server::SdaServerService>> {
    if let Some(url) = matches.value_of("mongo") {
        let db_name = matches.value_of("mongo_dbname").unwrap_or("sda");
        let it = sda_server_store_mongodb::new_mongodb_server_for_url(url, db_name)?;
        Ok(Some(it))
    } else {
        Ok(None)
    }

}

#[cfg(not(feature="mongodb"))]
pub fn build_mongo_server(_matches: &clap::ArgMatches)
                          -> SdaResult<Option<sda_server::SdaServerService>> {
    Ok(None)
}
