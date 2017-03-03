extern crate sda_protocol;
extern crate sda_client;
extern crate sda_client_store;
extern crate sda_client_http;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;

mod errors;

use sda_client::*;
use sda_client_http::*;
use sda_client_store::*;
use slog::*;

use errors::*;

fn main() {
    let root = slog::Logger::root(slog_term::streamer().stderr().use_utc_timestamp().build().fuse(), o!());
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
        (@arg identity: -i --identity +takes_value "Storage directory for identity, including keys")
        (@subcommand ping =>)
        (@subcommand agent => 
            (@subcommand create =>
                (@arg force: -f --force "Overwrite any existing identity")
            )
            (@subcommand show =>)
        )
    ).get_matches();

    let identitystore = {
        let path = matches.value_of("identity").unwrap_or(".sda");
        debug!("Using identity at {}", path);
        Filebased::new(path)?
    };

    let service = {
        let server_root = matches.value_of("server").unwrap_or("http://localhost:8888");
        debug!("Using server {}", server_root);
        SdaHttpClient::new(server_root)?
    };

    let agent = sda_client::load_agent(&identitystore)?;
    
    match matches.subcommand() {

        ("ping", Some(_)) => {
            let pong = service.ping()?;
            match pong {
                Pong {running} if running => {
                    info!("Service appears to be running");
                    Ok(())
                },
                _ => Err("Service may not be running")?
            }
        },

        ("agent", Some(matches)) => {

            match matches.subcommand() {
                ("create", Some(matches)) => {
                    let agent = if agent.is_some() && !matches.is_present("force") {
                        warn!("Using existing agent; use --force to create new");
                        agent.unwrap()
                    } else {
                        let agent = sda_client::new_agent(&identitystore)?;
                        info!("Created new agent with id {:?}", &agent.id);
                        agent
                    };
                    let client = SdaClient::new(agent, identitystore, service);
                    Ok(client.upload_agent()?)
                },
                ("show", Some(_)) => {
                    match agent {
                        None => { 
                            warn!("No agent found");
                            Ok(())
                        },
                        Some(agent) => {
                            println!("Agent is {:?}", agent); // TODO formatting
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