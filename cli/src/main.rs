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
        (@subcommand ping => (about: "check service availablity"))
        (@subcommand agent =>
            (about: "identity management")
            (@subcommand show =>)
            (@subcommand create =>
                (@arg force: -f --force "Overwrite any existing identity")
            )
            (@subcommand keys =>
                (@subcommand show =>)
                (@subcommand create =>)
            )
        )
        (@subcommand clerk =>
            (about: "run a clerk in a loop")
            (@arg once: -o --once "Run just once and leave")
        )
        (@subcommand aggregations =>
            (about: "aggregations command")
            (visible_aliases: &["agg", "aggs", "aggregation"])
            (about: "manage aggregations")
            (@subcommand create =>
                (@arg title: +required "aggregation title")
                (@arg dimension: +required "number of coefficient in the vector to be summed")
                (@arg modulus: +required "modulus all cryptographic operation will operate on")
                (@arg key: +required "key to use for recipient encryption")
                (@arg share_count: +required "number of shares (and clerks)")
                (@arg id: --id +takes_value "aggregation id")
                (@arg mask: --mask possible_value[none full chacha] default_value[none] "mask scheme")
                (@arg sharing: --sharing possible_value[add shamir] default_value[add] "sharing scheme")
            )
            (@subcommand begin =>
                (about: "autoselect a committee for the aggregation")
                (@arg id: +required "aggregation id")
            )
            (@subcommand end =>
                (about: "create an aggregation snapshot and clerking jobs")
                (@arg aggregation_id: +required "aggregation id")
            )
            (@subcommand reveal =>
                (about: "reveal an aggregation result")
                (@arg aggregation_id: +required "aggregation id")
            )
        )
        (@subcommand participate =>
            (about: "contribute a participation vector to an aggregation")
            (@arg id: "aggregation id")
            (@arg values: +multiple "values")
        )
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

    let identity_path = PathBuf::from(matches.value_of("identity")
        .unwrap_or(".sda"));

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
                Pong { running } if running => {
                    info!("Service appears to be running");
                    Ok(())
                }
                _ => Err("Service may not be running")?,
            }
        }

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
                }

                ("show", Some(_)) => {
                    match agent {
                        None => {
                            warn!("No local agent found");
                            Ok(())
                        }
                        Some(agent) => {
                            info!("Local agent is {:?}", agent);
                            Ok(())
                        }
                    }
                }

                ("keys", Some(matches)) => {
                    let agent = agent.ok_or("Agent missing")?;
                    let client = SdaClient::new(agent, keystore, Arc::new(service));

                    match matches.subcommand() {
                        ("create", Some(_)) => {
                            let key = client.new_encryption_key()?;
                            client.upload_encryption_key(&key)?;
                            info!("Created and uploaded key: {}", &key.to_string());
                            Ok(())
                        }

                        (cmd, _) => Err(format!("Unknown subcommand {}", cmd))?,
                    }
                }

                (cmd, _) => Err(format!("Unknown subcommand {}", cmd))?,

            }
        }

        ("clerk", Some(matches)) => {
            let agent = agent.ok_or("Agent is needed. Maybe run \"sda agent create\" ?")?;
            service.ping()?;
            let client = SdaClient::new(agent, keystore, Arc::new(service));
            loop {
                debug!("Polling for clerking job");
                client.run_chores(-1)?;
                if matches.is_present("once") {
                    return Ok(())
                }
                ::std::thread::sleep(::std::time::Duration::from_secs(5 * 60));
            }
        }

        ("aggregations", Some(matches)) => {
            let agent = agent.ok_or("Agent is needed. Maybe run \"sda agent create\" ?")?;
            service.ping()?;
            let client = SdaClient::new(agent.clone(), keystore, Arc::new(service));
            match matches.subcommand() {
                ("create", Some(matches)) => {
                    use std::str::FromStr;
                    let modulus = value_t!(matches.value_of("modulus"), i64)
                        .unwrap_or_else(|e| e.exit());
                    let share_count = value_t!(matches.value_of("share_count"), usize)
                        .unwrap_or_else(|e| e.exit());
                    let sharing = match matches.value_of("sharing").unwrap() {
                        "add" => {
                            LinearSecretSharingScheme::Additive {
                                modulus: modulus,
                                share_count: share_count,
                            }
                        }
                        "shamir" => unimplemented!(),
                        _ => panic!(),
                    };
                    let masking = match matches.value_of("mask").unwrap() {
                        "none" => LinearMaskingScheme::None,
                        "full" => LinearMaskingScheme::Full { modulus: modulus },
                        "chacha" => {
                            LinearMaskingScheme::ChaCha {
                                modulus: modulus,
                                dimension: share_count,
                                seed_bitsize: 128,
                            }
                        }
                        _ => panic!(),
                    };
                    let id = match matches.value_of("id") {
                        Some(value) => AggregationId::from_str(value)?,
                        None => AggregationId::random(),
                    };
                    let agg = Aggregation {
                        id: id,
                        title: matches.value_of("title").unwrap().to_string(),
                        vector_dimension: value_t!(matches.value_of("dimension"), usize)
                            .unwrap_or_else(|e| e.exit()),
                        modulus: modulus,
                        recipient: agent.id,
                        recipient_key: value_t!(matches.value_of("key"), EncryptionKeyId)
                            .unwrap_or_else(|e| e.exit()),
                        committee_sharing_scheme: sharing,
                        masking_scheme: masking,
                        recipient_encryption_scheme: AdditiveEncryptionScheme::Sodium,
                        committee_encryption_scheme: AdditiveEncryptionScheme::Sodium,
                    };
                    client.upload_aggregation(&agg)?;
                    info!("aggregation created. id: {}", agg.id().to_string());
                    Ok(())
                }
                ("begin", Some(matches)) => {
                    client.begin_aggregation(&value_t!(matches.value_of("id"), AggregationId)
                            .unwrap_or_else(|e| e.exit()))?;
                    Ok(())
                }
                ("end", Some(matches)) => {
                    client.end_aggregation(&value_t!(matches.value_of("aggregation_id"), AggregationId)
                            .unwrap_or_else(|e| e.exit()))?;
                    Ok(())
                }
                ("reveal", Some(matches)) => {
                    let result = client.reveal_aggregation(&value_t!(matches.value_of("aggregation_id"), AggregationId)
                            .unwrap_or_else(|e| e.exit()))?;
                    println!("result: {:?}", result.positive());
                    Ok(())
                }
                (cmd, _) => Err(format!("Unknown command {}", cmd))?,
            }
        }

        ("participate", Some(matches)) => {
            let agent = agent.ok_or("Agent is needed. Maybe run \"sda agent create\" ?")?;
            let client = SdaClient::new(agent, keystore, Arc::new(service));
            client.participate(
                values_t!(matches.values_of("values"), i64).unwrap_or_else(|e| e.exit()),
                &value_t!(matches.value_of("id"), AggregationId).unwrap_or_else(|e| e.exit())
            )?;
            Ok(())
        }

        (cmd, _) => Err(format!("Unknown command {}", cmd))?,
    }

}
