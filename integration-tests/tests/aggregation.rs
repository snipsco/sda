extern crate sda_protocol;
extern crate sda_server;
extern crate sda_client;
extern crate sda_client_store;
extern crate sda_tests;
use sda_protocol::*;
use sda_client::*;
use sda_client_store::*;
use sda_tests::*;

fn small_aggregation(recipient: &AgentId, recipient_key: &EncryptionKeyId) -> Aggregation {
    Aggregation {
        id: AggregationId::default(),
        title: "foo".into(),
        vector_dimension: 4,
        recipient: *recipient,
        recipient_key: *recipient_key,
        masking_scheme: LinearMaskingScheme::None,
        committee_sharing_scheme: LinearSecretSharingScheme::Additive {
            share_count: 3,
            modulus: 13,
        },
        recipient_encryption_scheme: AdditiveEncryptionScheme::Sodium,
        committee_encryption_scheme: AdditiveEncryptionScheme::Sodium,
    }
}

#[test]
pub fn committee() {
    with_service(|ctx| {
        let agents: Vec<(Agent,SignedEncryptionKey)> = (0..10).map(|_| new_full_agent(&ctx.service)).collect();
        let (alice, alice_key) = new_full_agent(&ctx.service);
        let agg = small_aggregation(&alice.id(), &alice_key.body.id());
        ctx.service.create_aggregation(&alice, &agg).unwrap();
        let candidates = ctx.service.suggest_committee(&alice, &agg.id).unwrap();
        assert_eq!(agents.len() + 1, candidates.len());

        let clerks = &candidates[0..agg.committee_sharing_scheme.output_size()];

        let committee = Committee {
            aggregation: agg.id,
            clerks_and_keys: clerks.iter().map(|cc| (cc.id, cc.keys[0])).collect(),
        };
        ctx.service.create_committee(&alice, &committee).unwrap();
        let committee_again = ctx.service.get_committee(&alice, &agg.id).unwrap();
        assert_eq!(Some(&committee), committee_again.as_ref());
    });
}

// #[test]
// pub fn clients() {
//     with_service(|ctx| {

//         let dir = ".tests/";
//         use std::sync::Arc;
//         use sda_server::SdaServer;

//         // server
//         let agents = sda_server::jfs_stores::JfsAgentStore::new(dir.join("agents")).unwrap();
//         let auth = sda_server::jfs_stores::JfsAuthStore::new(dir.join("auths")).unwrap();
//         let agg = sda_server::jfs_stores::JfsAggregationsStore::new(dir.join("service")).unwrap();
//         let server = Arc::new(Box::new(SdaServer {
//             agent_store: Box::new(agents),
//             auth_token_store: Box::new(auth),
//             aggregation_store: Box::new(agg),
//         }));

//         // client
//         let agent = sda_tests::new_agent();
//         let keystore = sda_client_store::Filebased::new(&".sda").unwrap();
//         let client = SdaClient::new(agent, Box::new(keystore), server.clone());
//     });
// }