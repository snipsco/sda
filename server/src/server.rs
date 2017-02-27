use sda_protocol::*;

use errors::*;

use stores::AgentStore;

pub struct SdaServer {
    pub agent_store: Box<AgentStore>,
}

#[allow(unused_variables)]
// FIXME
#[allow(dead_code)] // FIXME
impl SdaServer {
    fn ping(&self) -> SdaServerResult<()> {
        self.agent_store.ping()
    }

    fn list_aggregations_by_title(&self, filter: &str) -> SdaServerResult<Vec<AggregationId>> {
        unimplemented!();
    }

    fn list_aggregations_by_recipient(&self,
                                      recipient: &AgentId)
                                      -> SdaServerResult<Vec<AggregationId>> {
        unimplemented!();
    }

    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>> {
        unimplemented!();
    }

    fn get_committee(&self, committee: &CommitteeId) -> SdaServerResult<Option<Committee>> {
        unimplemented!();
    }

    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.agent_store.create_agent(&agent)
    }

    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        self.agent_store.get_agent(&id)
    }

    fn upsert_profile(&mut self, caller: &Agent, profile: &Profile) -> SdaServerResult<Profile> {
        unimplemented!();
    }

    fn get_profile(&self, agent: &AgentId) -> SdaServerResult<Option<Profile>> {
        unimplemented!();
    }

    fn create_encryption_key(&mut self,
                             caller: &Agent,
                             key: &SignedEncryptionKey)
                             -> SdaServerResult<()> {
        unimplemented!();
    }

    fn get_encryption_key(&self,
                          caller: &Agent,
                          key: &SignedEncryptionKeyId)
                          -> SdaServerResult<Option<SignedEncryptionKey>> {
        unimplemented!();
    }
}

macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("error in server: {}", err).into()),
        }
    }
}

impl SdaService for SdaServer {
    fn ping(&self) -> SdaResult<()> {
        wrap!(SdaServer::ping(self))
    }
}

#[allow(unused_variables)] // FIXME
impl SdaDiscoveryService for SdaServer {
    fn list_aggregations_by_title(&self,
                                  caller: &Agent,
                                  filter: &str)
                                  -> SdaResult<Vec<AggregationId>> {
        wrap! { SdaServer::list_aggregations_by_title(self, filter) }
    }

    fn list_aggregations_by_recipient(&self,
                                      caller: &Agent,
                                      recipient: &AgentId)
                                      -> SdaResult<Vec<AggregationId>> {
        wrap!(Self::list_aggregations_by_recipient(self, recipient))
    }

    fn get_aggregation(&self,
                       caller: &Agent,
                       aggregation: &AggregationId)
                       -> SdaResult<Option<Aggregation>> {
        wrap!(Self::get_aggregation(self, aggregation))
    }

    fn get_committee(&self,
                     caller: &Agent,
                     committee: &CommitteeId)
                     -> SdaResult<Option<Committee>> {
        unimplemented!();
    }

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        wrap!(Self::create_agent(self, &caller))
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        wrap! { Self::get_agent(self, owner) }
    }

    fn upsert_profile(&mut self, caller: &Agent, profile: &Profile) -> SdaResult<Profile> {
        unimplemented!();
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        unimplemented!();
    }

    fn create_encryption_key(&mut self,
                             caller: &Agent,
                             key: &SignedEncryptionKey)
                             -> SdaResult<()> {
        unimplemented!();
    }

    fn get_encryption_key(&self,
                          caller: &Agent,
                          key: &SignedEncryptionKeyId)
                          -> SdaResult<Option<SignedEncryptionKey>> {
        unimplemented!();
    }
}
