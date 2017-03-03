extern crate sda_protocol as proto;
extern crate sda_server;

fn tmp_server() -> sda_server::SdaServer {
    let agents = sda_server::jfs_stores::JfsAgentStore::new("tmp/agents").unwrap();
    let auth = sda_server::jfs_stores::JfsAuthStore::new("tmp/auths").unwrap();
    sda_server::SdaServer { agent_store: Box::new(agents), auth_token_store: Box::new(auth)  }
}

fn new_agent() -> proto::Agent {
    proto::Agent {
        id: proto::AgentId::default(),
        verification_key: proto::Labeled {
            id: proto::VerificationKeyId::default(),
            body: proto::VerificationKey::Sodium(proto::byte_arrays::B32::default()),
        }
    }
}

#[test]
pub fn ping() {
    let server = tmp_server();
    let service: &proto::SdaDiscoveryService = &server;

    service.ping().unwrap();
}

#[test]
pub fn agent_crud() {
    let server = tmp_server();

    let service: &proto::SdaDiscoveryService = &server;

    let alice = new_agent();

    service.create_agent(&alice, &alice).unwrap();
    let clone = service.get_agent(&alice, &alice.id).unwrap();
    assert_eq!(Some(&alice), clone.as_ref());

    let bob = service.get_agent(&alice, &proto::AgentId::default()).unwrap();
    assert!(bob.is_none());
}

#[test]
pub fn profile_crud() {
    let server = tmp_server();
    let service: &proto::SdaDiscoveryService = &server;

    let alice = new_agent();

    service.create_agent(&alice, &alice).unwrap();
    let no_profile = service.get_profile(&alice, &alice.id).unwrap();
    assert!(no_profile.is_none());

    let alice_profile = proto::Profile {
        owner: alice.id,
        name: Some("alice".into()),
        ..proto::Profile::default()
    };
    service.upsert_profile(&alice, &alice_profile).unwrap();

    let clone = service.get_profile(&alice, &alice.id).unwrap();
    assert_eq!(Some(&alice_profile), clone.as_ref());

    let still_alice_profile = proto::Profile {
        owner: alice.id,
        name: Some("still alice".into()),
        ..proto::Profile::default()
    };
    service.upsert_profile(&alice, &still_alice_profile).unwrap();

    let clone = service.get_profile(&alice, &alice.id).unwrap();
    assert_eq!(Some(&still_alice_profile), clone.as_ref());
}

#[test]
pub fn profile_crud_acl() {
    let server = tmp_server();
    let service: &proto::SdaDiscoveryService = &server;

    let alice = new_agent();

    let bob = new_agent();
    let alice_fake_profile = proto::Profile {
        owner: alice.id,
        name: Some("bob".into()),
        ..proto::Profile::default()
    };

    let denied = service.upsert_profile(&bob, &alice_fake_profile);
    match denied {
        Err(proto::SdaError(proto::SdaErrorKind::PermissionDenied, _)) => {},
        e => panic!("unexpected result: {:?}", e)
    }
}

#[test]
pub fn encryption_key_crud() {
    use proto::byte_arrays::*;
    let server = tmp_server();
    let service: &proto::SdaDiscoveryService = &server;

    let alice = new_agent();
    let bob = new_agent();
    service.create_agent(&alice, &alice).unwrap();

    let alice_key = proto::SignedEncryptionKey {
        body: proto::Labeled {
            id: proto::EncryptionKeyId::default(),
            body: proto::EncryptionKey::Sodium(B8::default())
        },
        signer: alice.id,
        signature: proto::Signature::Sodium(B64::default())
    };

    service.create_encryption_key(&alice, &alice_key).unwrap();
    let still_alice = service.get_encryption_key(&bob, &alice_key.body.id).unwrap();
    assert_eq!(Some(&alice_key), still_alice.as_ref());
}

#[test]
pub fn auth_tokens_crud() {
    use sda_server::stores::AuthToken;
    let server = tmp_server();
    let service: &proto::SdaDiscoveryService = &server;
    let alice = new_agent();
    let alice_token = AuthToken {
        id: alice.id,
        body: "tok".into()
    };
    assert!(server.check_auth_token(&alice_token).is_err());
    // TODO check error kind is InvalidCredentials
    service.create_agent(&alice, &alice).unwrap();
    server.upsert_auth_token(&alice_token).unwrap();
    assert!(server.check_auth_token(&alice_token).is_ok());
    let alice_token_new = AuthToken {
        id: alice.id,
        body: "token".into()
    };
    assert!(server.check_auth_token(&alice_token_new).is_err());
    server.upsert_auth_token(&alice_token_new).unwrap();
    assert!(server.check_auth_token(&alice_token_new).is_ok());
    assert!(server.check_auth_token(&alice_token).is_err());
    server.delete_auth_token(&alice.id).unwrap();
    assert!(server.check_auth_token(&alice_token_new).is_err());
    assert!(server.check_auth_token(&alice_token).is_err());
}
