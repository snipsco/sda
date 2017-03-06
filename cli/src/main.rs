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
            (@subcommand show =>)
            (@subcommand create =>
                (@arg force: -f --force "Overwrite any existing identity")
            )
            (@subcommand keys =>
                (@subcommand show =>)
                (@subcommand create =>)
            )
        )
    ).get_matches();

    let service = {
        let server_root = {
            let root = matches.value_of("server").unwrap_or("http://localhost:8888");
            debug!("Using service at {}", root);
            root
        };
        let authstore = {
            let path = matches.value_of("identity").unwrap_or(".sda");
            debug!("Using authorisation at {}", path);
            Filebased::new(path)?
        };
        SdaHttpClient::new(server_root, authstore)?
    };

    let identitystore = {
        let path = matches.value_of("identity").unwrap_or(".sda");
        debug!("Using identity at {}", path);
        Filebased::new(path)?
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
                            warn!("No local agent found");
                            Ok(())
                        },
                        Some(agent) => {
                            info!("Local agent is {:?}", agent);
                            Ok(())
                        }
                    }
                },

                ("keys", Some(matches)) => {
                    let agent = agent.ok_or("Agent missing")?;
                    match matches.subcommand() {
                        ("create", Some(_)) => {
                            

                            Ok(())
                        },

                        (cmd, _) => Err(format!("Unknown subcommand {}", cmd))?
                    }
                },

                (cmd, _) => Err(format!("Unknown subcommand {}",  cmd))?

            }
            
        },

        (cmd, _) => Err(format!("Unknown command {}", cmd))?
    }

}