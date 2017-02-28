extern crate sda_client;
#[macro_use]
extern crate clap;
// #[macro_use]
// extern crate slog;
// #[macro_use]
// extern crate slog_scope;
// extern crate slog_envlogger;

use sda_client::*;
// use sda_httpproxy::{SdaHttpProxy};

fn main() {
    // slog_envlogger::init().unwrap();
    // println!("cdscs");
    // error!("main");
    // if let Err(e) = run() {
    //     debug!("{:?}", e);
    //     error!("{}", e);
    //     std::process::exit(1);
    // }
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> sda_client::SdaClientResult<()> {
    
    let matches = clap_app!(sda =>
        (@arg server: -s --server +takes_value "Server URI")
        (@arg keystore: -k --keystore +takes_value "Keystore directory")
        (@subcommand init =>)
        (@subcommand clerk =>)
        (@subcommand keystore =>
            // (@subcommand list =>)
        )
        ).get_matches();

    let keystore: Keystore = Keystore::new(
        matches.value_of("keystore").unwrap_or(".sda")
    )?;
    let agent: Agent = match matches.subcommand() {
        
        ("init", _) => {
            let id = AgentId::new();
            let vk: VerificationKey = keystore.new_keypair()?;
            Ok(Agent {
                id: id,
                verification_key: Some(vk)
            })      
        },

        _ => {
            keystore.get("agent")?
                .ok_or("Please run 'sda init' first")
        }

    }?;

    match matches.subcommand() {
        ("keystore", Some(matches)) => {
            match matches.subcommand() {
                ("init", Some(matches)) => {
                    keystore.init();
                    println!("init");
                },
                (_, _) => panic!("Unknown subcommand for keystore"),
            }
        },

        (_, _) => panic!("Unknown command"),
    }

    Ok(())

}
