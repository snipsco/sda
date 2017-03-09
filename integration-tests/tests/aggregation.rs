extern crate sda_protocol;
extern crate sda_server;
extern crate sda_client;
extern crate sda_tests;
extern crate tempdir;
use sda_protocol::*;
use sda_client::*;
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
pub fn full_mocked_loop() {
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

        let participants: Vec<(Agent,SignedEncryptionKey)> = (0..100).map(|_| new_full_agent(&ctx.service)).collect();
        for p in participants.iter() {
            let participation = Participation {
                id: ParticipationId::random(),
                participant: p.0.id().clone(),
                aggregation: agg.id,
                encryptions: vec!(),
            };
            ctx.service.create_participation(&p.0, &participation).unwrap();
        };
        let status = ctx.service.get_aggregation_status(&alice, &agg.id).unwrap().unwrap();
        assert_eq!(agg.id, status.aggregation);
        assert_eq!(participants.len(), status.number_of_participations);
        assert_eq!(0, status.number_of_clerking_results);
        assert_eq!(false, status.result_ready);
        let snapshot = Snapshot {
            id: SnapshotId::random(),
            aggregation: agg.id.clone()
        };
        ctx.service.create_snapshot(&alice, &snapshot).unwrap();
    });
}

#[test]
pub fn participation() {
    with_service(|ctx| {
        let stores: Vec<::tempdir::TempDir> = (0..10).map(|_| ::tempdir::TempDir::new("sda-tests-clients-keystores").unwrap()).collect();
        let clients: Vec<SdaClient> = stores.iter().map(|store| new_client(store, &ctx.service)).collect();

        for client in clients {
            client.upload_agent().unwrap();

            let key = client.new_encryption_key().unwrap();
            client.upload_encryption_key(&key).unwrap();
        }

        // let recipient = clients.as_slice()[0];
        // let participants = &clients[1..10];

        assert!(true);
    });
}
