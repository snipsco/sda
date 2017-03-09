extern crate sda_protocol;
extern crate sda_server;
extern crate sda_tests;
use sda_protocol::*;
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
        let agents: Vec<(Agent, SignedEncryptionKey)> =
            (0..20).map(|_| new_full_agent(&ctx.service)).collect();
        let (ref alice, ref alice_key) = agents[0];
        let agg = small_aggregation(&alice.id(), &alice_key.body.id());
        ctx.service.create_aggregation(&alice, &agg).unwrap();
        let candidates = ctx.service.suggest_committee(&alice, &agg.id).unwrap();
        assert_eq!(agents.len(), candidates.len());

        let clerks = &candidates[0..agg.committee_sharing_scheme.output_size()];

        let committee = Committee {
            aggregation: agg.id,
            clerks_and_keys: clerks.iter().map(|cc| (cc.id, cc.keys[0])).collect(),
        };
        ctx.service.create_committee(&alice, &committee).unwrap();
        let committee_again = ctx.service.get_committee(&alice, &agg.id).unwrap();
        assert_eq!(Some(&committee), committee_again.as_ref());

        let participants: Vec<(Agent, SignedEncryptionKey)> =
            (0..100).map(|_| new_full_agent(&ctx.service)).collect();
        for (pi, p) in participants.iter().enumerate() {
            let participation = Participation {
                id: ParticipationId::random(),
                participant: p.0.id().clone(),
                aggregation: agg.id,
                encryptions: clerks.iter()
                    .enumerate()
                    .map(|(ci, c)| (c.id, Encryption::Sodium(vec![ci as u8, pi as u8])))
                    .collect(),
            };
            ctx.service.create_participation(&p.0, &participation).unwrap();
        }
        let status = ctx.service.get_aggregation_status(&alice, &agg.id).unwrap().unwrap();
        assert_eq!(agg.id, status.aggregation);
        assert_eq!(participants.len(), status.number_of_participations);
        assert_eq!(0, status.number_of_clerking_results);
        assert_eq!(false, status.result_ready);
        let snapshot = Snapshot {
            id: SnapshotId::random(),
            aggregation: agg.id.clone(),
        };
        ctx.service.create_snapshot(&alice, &snapshot).unwrap();

        for (ci, c) in clerks.iter().enumerate() {
            let agent = agents.iter().find(|a| a.0.id == c.id).unwrap();
            let job = ctx.service.get_clerking_job(&agent.0, &c.id).unwrap().unwrap();
            assert_eq!(snapshot.id, job.snapshot);
            for enc in job.encryptions.iter() {
                let &Encryption::Sodium(ref data) = enc;
                assert_eq!(ci as u8, data[0]);
            }
        }
    });
}
