#[cfg(test)]

extern crate sda_client;
use sda_client::*;

mod mock;
use mock::*;

#[test]
fn client_creation() {
    let _ = SdaClient::mock();
}

// #[test]
// fn client_foo() {
//     let client = SdaClient::mock();
//     let input = UserInput::mock();
//     let aggregation = Aggregation::mock();
//     let participation = client.create_participation(&input, &aggregation);
//     println!("{:?}", participation);
// }
