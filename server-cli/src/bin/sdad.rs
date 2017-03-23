extern crate clap;
extern crate sda_protocol;
extern crate sda_server_cli;
extern crate sda_server_http;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate tempdir;

use std::sync;
use sda_protocol::SdaResult;

fn main() {
    let app = clap::App::new("sdad");
    let app = sda_server_cli::add_verbose_arg(app);
    let app = sda_server_cli::add_store_args(app);
    let app = app.subcommand(clap::SubCommand::with_name("httpd")
                   .about("Run a http server")
                   .arg_from_usage("-b, --bind [ip_and_port] 'defaults to 127.0.0.1:8888'"));

    if let Err(e) = run(&app.get_matches()) {
        error!("{}", e);
        ::std::process::exit(1);
    }
}

fn run(matches: &clap::ArgMatches) -> SdaResult<()> {
    sda_server_cli::setup_slog(&matches);
    let server_service = sda_server_cli::build_backend_server(&matches).unwrap();

    match matches.subcommand() {
        ("httpd", Some(m)) => {
            let port = m.value_of("ip_and_port").unwrap_or("127.0.0.1:8888");
            info!("Starting server on {}", port);
            sda_server_http::listen(port, sync::Arc::new(server_service))
        },
        (_, _) => Err("Unknown subcommand")?
    }
}
