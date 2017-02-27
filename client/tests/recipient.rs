#![cfg(test)]

extern crate sda_client;
use sda_client::*;

mod mock;
use mock::*;

#[test]
fn new_recipient() {
    let mut client = SdaClient::mock();

    let mut profile = client.new_profile().unwrap();
    assert!(profile.owner.0 == Uuid::nil());

    profile = client.upload_profile(&profile).unwrap();
    assert!(profile.owner.0 != Uuid::nil());

    client.new_associated_encryption_key().unwrap();

}