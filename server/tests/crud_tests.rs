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

    let myself = proto::Agent::default();

    service.create_agent(&myself, &myself).unwrap();
    let clone = service.get_agent(&myself, &myself.id).unwrap();
    assert_eq!(Some(&myself), clone.as_ref());

    let nobody = service.get_agent(&myself, &proto::AgentId::default()).unwrap();
    assert!(nobody.is_none());
}
