extern crate sda_client;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_envlogger;

use sda_client::*;

fn main() {
    slog_envlogger::init().unwrap();
    if let Err(e) = run() {
        // TODO the below doesn't show anything!
        debug!("{:?}", e);
        error!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> sda_client::SdaClientResult<()> {
    
    let matches = clap_app!(sda =>
        (@arg server: -s --server +takes_value "Server URI")
        (@arg keystore: -k --keystore +takes_value "Keystore directory")
        (@subcommand identity => 
            (@subcommand create =>
                (@arg force: -f --force "Overwrite any existing identity")
            )
            (@subcommand show =>)
        )
        (@subcommand clerk =>)
        (@subcommand keystore =>
            // (@subcommand list =>)
        )
        ).get_matches();

    let keystore: keystore::Filebased = keystore::Filebased::new(matches.value_of("keystore").unwrap_or(".sda"))?;
    let identity = keystore.resolve_alias("identity")?;
    
    match matches.subcommand() {

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


