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

use sda_protocol::*;
use sda_client::*;
use sda_client_http::*;
use sda_client_store::Filebased;

use slog::*;
use std::sync::Arc;
use std::path::PathBuf;

use errors::*;

fn main() {
    let matches = clap_app!(sda =>
        (@arg server: -s --server +takes_value "Server root")
        (@arg verbose: -v --verbose +multiple "verbose logging")
        (@arg identity: -i --identity +takes_value "Storage directory for identity, including keys (defaults to .sda)")
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
        (@subcommand clerk =>)
    ).get_matches();

    let root = slog_term::streamer().stderr().use_utc_timestamp().build().fuse();
    let root = level_filter(Level::from_usize(4 + matches.occurrences_of("verbose") as usize)
                                .unwrap_or(Level::Warning),
                            root);
    let root = Logger::root(root, o!());
    slog_scope::set_global_logger(root);

    if let Err(e) = run(&matches) {
        debug!("{:?}", e);
        error!("{}", e);
        std::process::exit(1);
    }
}

fn run(matches: &clap::ArgMatches) -> SdaCliResult<()> {

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

    let identity_path = PathBuf::from(matches
        .value_of("identity")
        .unwrap_or(".sda")
    );

    let identitystore = {
        debug!("Using identity at {:?}", &identity_path);
        Filebased::new(&identity_path)?
    };

    let keystore = {
        let keystore_path = identity_path.join("keys");
        debug!("Using keystore at {:?}", &keystore_path);
        Arc::new(Filebased::new(&keystore_path)?)
    };

    use sda_client_store::Store;
    let agent = identitystore.get_aliased("agent")?;

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
                        let agent = SdaClient::new_agent(keystore.clone())?;
                        identitystore.put_aliased("agent", &agent)?;
                        info!("Created new agent with id {:?}", &agent.id);
                        agent
                    };
                    let client = SdaClient::new(agent, keystore, Arc::new(service));
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
                    let client = SdaClient::new(agent, keystore, Arc::new(service));

                    match matches.subcommand() {
                        ("create", Some(_)) => {
                            let key = client.new_encryption_key()?;
                            client.upload_encryption_key(&key)?;
                            Ok(())
                        },

                        (cmd, _) => Err(format!("Unknown subcommand {}", cmd))?
                    }
                },

                (cmd, _) => Err(format!("Unknown subcommand {}",  cmd))?

            }

        },

        ("clerk", Some(_matches)) => {
            let agent = agent.ok_or("Agent is needed. Maybe run \"sda agent create\" ?")?;
            service.ping()?;
            let client = SdaClient::new(agent, keystore, Arc::new(service));
            loop {
                debug!("Polling for clerking job");
                client.run_chores(-1)?;
                ::std::thread::sleep(::std::time::Duration::from_secs(5*60));
            }
        }

        (cmd, _) => Err(format!("Unknown command {}", cmd))?
    }

}
