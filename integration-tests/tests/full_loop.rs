extern crate sda_protocol;
extern crate sda_server;
extern crate sda_client;
extern crate sda_client_store;
extern crate sda_tests;
extern crate tempdir;
use sda_protocol::*;
use sda_client::*;
use sda_tests::*;

fn agg_default() -> Aggregation {
    Aggregation {
        id: AggregationId::random(),
        title: "foo".into(),
        vector_dimension: 4,
        modulus: 433,
        recipient: AgentId::random(),
        recipient_key: EncryptionKeyId::random(),
        masking_scheme: LinearMaskingScheme::None,
        committee_sharing_scheme: LinearSecretSharingScheme::Additive {
            share_count: 3,
            modulus: 433,
        },
        recipient_encryption_scheme: AdditiveEncryptionScheme::Sodium,
        committee_encryption_scheme: AdditiveEncryptionScheme::Sodium,
    }
}

#[test]
pub fn simple() {
    check_full_aggregation(Aggregation { ..agg_default() });
}

#[test]
pub fn with_fullmask() {
    check_full_aggregation(Aggregation {
        masking_scheme: LinearMaskingScheme::Full { modulus: 433 },
        ..agg_default()
    });
}

#[test]
pub fn with_chachamask() {
    check_full_aggregation(Aggregation {
        masking_scheme: LinearMaskingScheme::ChaCha {
            modulus: 433,
            dimension: 4,
            seed_bitsize: 128,
        },
        ..agg_default()
    });
}

#[test]
pub fn with_packedshamir() {
    check_full_aggregation(Aggregation {
        committee_sharing_scheme: LinearSecretSharingScheme::PackedShamir {
            secret_count: 3,
            share_count: 8,
            privacy_threshold: 4,
            prime_modulus: 433,
            omega_secrets: 354,
            omega_shares: 150,
        },
        ..agg_default()
    });
}


pub fn check_full_aggregation(aggregation: Aggregation) {
    with_service(move |ctx| {

        // prepare recipient
        let recipient_store = ::tempdir::TempDir::new("sda-tests-clients-keystores").unwrap();
        let recipient = new_client(&recipient_store, &ctx.service);
        let recipient_key = recipient.new_encryption_key().unwrap();
        recipient.upload_agent().unwrap();
        recipient.upload_encryption_key(&recipient_key).unwrap();

        let aggregation = Aggregation {
            recipient: recipient.agent.id().clone(),
            recipient_key: recipient_key.clone(),
            ..aggregation
        };

        recipient.upload_aggregation(&aggregation).unwrap();

        // prepare clerks
        let clerks_store: Vec<::tempdir::TempDir> = (0..8)
            .map(|_| ::tempdir::TempDir::new("sda-tests-clients-keystores").unwrap())
            .collect();
        let clerks: Vec<SdaClient> =
            clerks_store.iter().map(|store| new_client(store, &ctx.service)).collect();
        for clerk in clerks.iter() {
            let clerk_key = clerk.new_encryption_key().unwrap();
            clerk.upload_agent().unwrap();
            clerk.upload_encryption_key(&clerk_key).unwrap();
        }

        // assign committee
        recipient.begin_aggregation(&aggregation.id).unwrap();

        // prepare participants
        let participants_store: Vec<::tempdir::TempDir> = (0..2)
            .map(|_| ::tempdir::TempDir::new("sda-tests-clients-keystores").unwrap())
            .collect();
        let participants: Vec<SdaClient> =
            participants_store.iter().map(|store| new_client(store, &ctx.service)).collect();

        // participate
        for participant in &participants {
            participant.upload_agent().unwrap();
            participant.participate(vec![1, 2, 3, 4], &aggregation.id).unwrap();
        }

        // close aggregation (by creating snapshot)
        recipient.end_aggregation(&aggregation.id).unwrap();

        // .. and check status
        let status =
            ctx.service.get_aggregation_status(&recipient.agent, &aggregation.id).unwrap().unwrap();
        assert_eq!(aggregation.id, status.aggregation);
        assert_eq!(&participants.len(), &status.number_of_participations);
        assert_eq!(1, status.snapshots.len());
        let snapshot_status = &status.snapshots[0];
        assert_eq!(0, snapshot_status.number_of_clerking_results);
        assert_eq!(false, snapshot_status.result_ready);

        // perform clerking
        recipient.run_chores(-1).unwrap();
        for clerk in clerks {
            clerk.run_chores(-1).unwrap();
        }

        // .. and recheck status
        let status =
            ctx.service.get_aggregation_status(&recipient.agent, &aggregation.id).unwrap().unwrap();
        assert_eq!(aggregation.id, status.aggregation);
        assert_eq!(&participants.len(), &status.number_of_participations);
        assert_eq!(1, status.snapshots.len());
        let snapshot_status = &status.snapshots[0];
        assert_eq!(aggregation.committee_sharing_scheme.output_size(),
            snapshot_status.number_of_clerking_results);
        assert_eq!(true, snapshot_status.result_ready);

        // reveal aggregation
        let output = recipient.reveal_aggregation(&aggregation.id).unwrap();
        assert_eq!(vec![2, 4, 6, 8], output.positive().values);
    });
}
