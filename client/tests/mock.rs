
extern crate sda_client;

use sda_client::*;


pub trait Mock {
    fn mock() -> Self;
}


pub struct MockSdaService {}

impl Mock for MockSdaService {
    fn mock() -> Self {
        MockSdaService {}
    }
}

impl SdaService for MockSdaService {
    fn ping(&self) -> SdaResult<()> {
        unimplemented!()
    }
} 

impl SdaAggregationService for MockSdaService {

    fn find_aggregations(&self, caller: &Agent, filter: Option<&str>) -> SdaResult<Vec<AggregationId>> {
        unimplemented!()
    }

    fn pull_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>> {
        unimplemented!()
    }

    fn pull_committee(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Committee>> {
        unimplemented!()
    }

    fn pull_clerk_profile(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkProfile>> {
        unimplemented!()
    }

}

impl UserSdaAggregationService for MockSdaService {

    fn push_user_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()> {
        println!("push_user_participation");
        unimplemented!()
    }

}


pub struct MockTrustStore {}

impl Mock for MockTrustStore {
    fn mock() -> Self {
        MockTrustStore {}
    }
}

impl TrustedCommitteeStore for MockTrustStore {
    fn load_trusted_committee(&self, committee: &CommitteeId) -> SdaClientResult<Committee> {
        let mock_committee = Committee::mock();
        if *committee == mock_committee.id {
            Ok(mock_committee)
        } else {
            Err("Committee not found")?
        }
    }
}

impl TrustedKeysetStore for MockTrustStore {
    fn load_trusted_keyset(&self, keyset: &KeysetId) -> SdaClientResult<Keyset> {
        let mock_keyset = Keyset::mock();
        if *keyset == mock_keyset.id {
            Ok(mock_keyset)
        } else {
            Err("Keyset not found")?
        }
    }
}


impl Mock for SdaClient<MockTrustStore, MockSdaService> {
    fn mock() -> Self {
        let agent = Agent::mock();
        let store = MockTrustStore::mock();
        let service = MockSdaService::mock();
        SdaClient::new(agent, store, service)
    }
}

impl Mock for Agent {
    fn mock() -> Self {
        Agent {
            id: AgentId::mock(),
            auth_token: None,
        }
    }
}

impl Mock for AgentId {
    fn mock() -> Self {
        AgentId(Uuid::nil())
    }
}

impl Mock for UserInput {
    fn mock() -> Self {
        UserInput(vec![1,2,3,4,5])
    }
}

impl Mock for AggregationId {
    fn mock() -> Self {
        AggregationId(Uuid::new_v4())
    }
}

impl Mock for CommitteeId {
    fn mock() -> Self {
        CommitteeId(Uuid::nil())
    }
}

impl Mock for Committee {
    fn mock() -> Self {
        Committee {
            id: CommitteeId::mock(),
            name: Some("Mock Committee".to_string()),
            clerks: vec![AgentId::mock()],
        }
    }
}

impl Mock for KeysetId {
    fn mock() -> Self {
        KeysetId(Uuid::nil())
    }
}

impl Mock for Keyset {
    fn mock() -> Self {
        use std::collections::HashMap;
        let mut keys = HashMap::new();
        keys.insert(AgentId::mock(), AssociatedEncryptionKey::mock());
        Keyset {
            id: KeysetId::mock(),
            keys: keys,
        }
    }
}

impl Mock for EncryptionKey {
    fn mock() -> Self {
        EncryptionKey(vec![0, 0, 0])
    }
}

impl Mock for Signature {
    fn mock() -> Self {
        Signature(vec![0, 0, 0])
    }
}

impl Mock for AssociatedEncryptionKey {
    fn mock() -> Self {
        AssociatedEncryptionKey {
            key: EncryptionKey::mock(),
            signature: Signature::mock(),
        }
    }
}

impl Mock for LinearSecretSharingScheme {
    fn mock() -> Self {
        LinearSecretSharingScheme::Additive {
            share_count: 5,
            modulus: 100,
        }
    }
}

impl Mock for AdditiveEncryptionScheme {
    fn mock() -> Self {
        AdditiveEncryptionScheme::Sodium {}
    }
}

impl Mock for Aggregation {
    fn mock() -> Self {
        Aggregation {
            id: AggregationId::mock(),
            title: "Mock".to_string(),
            vector_dimension: 5,
            secret_sharing_scheme: LinearSecretSharingScheme::mock(),
            encryption_scheme: AdditiveEncryptionScheme::mock(),
            committee: CommitteeId::mock(),
            keyset: KeysetId::mock(),
        }
    }
}