#![allow(dead_code)]
#[cfg(test)]
use super::*;
#[allow(unused_imports)]
use sda_protocol::*;

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
        let agents: Vec<(Agent,SignedEncryptionKey)> = (0..10).map(|_| new_full_agent(ctx.agents)).collect();
        let (alice, alice_key) = new_full_agent(ctx.agents);
        let agg = small_aggregation(&alice.id(), &alice_key.body.id());
        ctx.admin.create_aggregation(&alice, &agg).unwrap();
        let candidates = ctx.admin.suggest_committee(&alice, &agg.id).unwrap();
        assert_eq!(agents.len() + 1, candidates.len());

        let clerks = &candidates[0..agg.committee_sharing_scheme.output_size()];

        let committee = Committee {
            aggregation: agg.id,
            clerks_and_keys: clerks.iter().map(|cc| (cc.id, cc.keys[0])).collect(),
        };
        ctx.admin.create_committee(&alice, &committee).unwrap();
        let committee_again = ctx.aggregation.get_committee(&alice, &agg.id).unwrap();
        assert_eq!(Some(&committee), committee_again.as_ref());
    });
}
