extern crate sda_protocol as proto;
extern crate sda_server;

#[test]
pub fn ping() {
    let store = sda_server::jfs_stores::JfsAgentStore::new("tmp").unwrap();
    let server = sda_server::SdaServer { agent_store: Box::new(store) };
    let service: &proto::SdaDiscoveryService = &server;

    service.ping().unwrap();
}

#[test]
pub fn agent_crud() {
    let store = sda_server::jfs_stores::JfsAgentStore::new("tmp").unwrap();
    let server = sda_server::SdaServer { agent_store: Box::new(store) };
    let service: &proto::SdaDiscoveryService = &server;

    let alice = proto::Agent::default();

    service.create_agent(&alice, &alice).unwrap();
    let clone = service.get_agent(&alice, &alice.id).unwrap();
    assert_eq!(Some(&alice), clone.as_ref());

    let bob = service.get_agent(&alice, &proto::AgentId::default()).unwrap();
    assert!(bob.is_none());
}

#[test]
pub fn profile_crud() {
    let store = sda_server::jfs_stores::JfsAgentStore::new("tmp").unwrap();
    let server = sda_server::SdaServer { agent_store: Box::new(store) };
    let service: &proto::SdaDiscoveryService = &server;

    let alice = proto::Agent::default();

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
}
