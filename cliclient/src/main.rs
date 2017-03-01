extern crate sda_protocol;
extern crate sda_client;
extern crate sda_client_http;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;


use sda_client::*;
use sda_client_http::*;

use slog::*;


error_chain!{
    types {
        SdaCliError, SdaCliErrorKind, SdaCliResultExt, SdaCliResult;
    }
    foreign_links {
        Protocol(::sda_protocol::SdaError);
        Http(::sda_client_http::SdaHttpClientError);
        Client(::sda_client::SdaClientError);
    }
}

fn main() {
    let root = Logger::root(slog_term::streamer().stderr().use_utc_timestamp().build().fuse(), o!());
    slog_scope::set_global_logger(root);

    if let Err(e) = run() {
        debug!("{:?}", e);
        error!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> SdaCliResult<()> {
    
    let matches = clap_app!(sda =>
        (@arg server: -s --server +takes_value "Server root")
        (@arg keystore: -k --keystore +takes_value "Keystore directory")
        (@subcommand ping =>)
        (@subcommand identity => 
            (@subcommand create =>
                (@arg force: -f --force "Overwrite any existing identity")
            )
            (@subcommand show =>)
        )
        (@subcommand clerk =>)
    ).get_matches();

    let service = {
        let server_root = matches.value_of("server").unwrap_or("http://localhost:8888");
        debug!("Using server {}", server_root);
        SdaHttpClient::new(server_root)?
    };

    let keystore = {
        let keystore_path = matches.value_of("keystore").unwrap_or(".sda");
        debug!("Using keystore {}", keystore_path);
        keystore::Filebased::new(keystore_path)?
    };

    let identity = {
        let identity = keystore.resolve_alias("identity")?;
        debug!("Using identity {:?}", identity);
        identity
    };
    
    
    match matches.subcommand() {

        ("ping", Some(matches)) => {
            let pong = service.ping()?;
            match pong {
                Pong{running} if running => {
                    info!("Service appears to be running");
                    Ok(())
                },
                _ => Err("Service may not be running")?
            }
        },

        ("identity", Some(matches)) => {

            match matches.subcommand() {
                ("create", Some(matches)) => {
                    match identity {
                        Some(_) if !matches.is_present("force") => {
                            Err("Already created; use --force to create new")?
                        },
                        _ => {
                            let identity: LabelledVerificationKeypairId = keystore.new_keypair()?;
                            info!("Created identity with id {}", identity.to_string());
                            keystore.define_alias("identity", &identity.to_string())?;
                            Ok(())
                        }
                    }
                },
                ("show", Some(matches)) => {
                    match identity {
                        None => { 
                            warn!("No identity found");
                            Ok(())
                        },
                        Some(identity) => {
                            println!("Identity id is {}", identity);
                            Ok(())
                        }
                    }
                },
                (cmd, _) => Err(format!("Unknown subcommand {}",  cmd))?
            }
            
        },

        (cmd, _) => Err(format!("Unknown command {}", cmd))?
    }

}


