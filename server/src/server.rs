use sda_protocol::*;

use errors::*;

use stores::{ AgentStore, AuthenticationStore, AuthenticationToken };

pub struct SdaServer {
    pub agent_store: Box<AgentStore>,
    pub auth_token_store: Box<AuthenticationStore>,
}

#[allow(unused_variables)]
// FIXME
#[allow(dead_code)] // FIXME
impl SdaServer {
    fn ping(&self) -> SdaServerResult<Pong> {
        self.agent_store.ping()?;
        Ok(Pong { running: true })
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

    fn get_committee(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Committee>> {
        unimplemented!();
    }

    fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.agent_store.create_agent(&agent)
    }

    fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        self.agent_store.get_agent(&id)
    }

    fn upsert_profile(&self, profile: &Profile) -> SdaServerResult<()> {
        self.agent_store.upsert_profile(profile)
    }

    fn get_profile(&self, agent: &AgentId) -> SdaServerResult<Option<Profile>> {
        self.agent_store.get_profile(agent)
    }

    fn create_encryption_key(&self, key: &SignedEncryptionKey) -> SdaServerResult<()> {
        self.agent_store.create_encryption_key(key)
    }

    fn get_encryption_key(&self,
                          key: &EncryptionKeyId)
                          -> SdaServerResult<Option<SignedEncryptionKey>> {
        self.agent_store.get_encryption_key(key)
    }

    // TODO put these 3 auth_token in a separate trait ?

    pub fn upsert_auth_token(&self, token:&AuthenticationToken) -> SdaServerResult<()> {
        self.auth_token_store.upsert_auth_token(token)
    }

    pub fn check_auth_token(&self, token:&AuthenticationToken) -> SdaServerResult<()> {
        let db = self.auth_token_store.get_auth_token(token.id())?;
        if db.as_ref() == Some(token) {
            Ok(())
        } else {
            Err(SdaServerErrorKind::InvalidCredentials)?
        }
    }

    pub fn delete_auth_token(&self, agent:&AgentId) -> SdaServerResult<()> {
        self.auth_token_store.delete_auth_token(agent)
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
    fn ping(&self) -> SdaResult<Pong> {
        wrap!(SdaServer::ping(self))
    }
}

fn acl_agent_is(agent: &Agent, agent_id: AgentId) -> SdaResult<()> {
    if agent.id != agent_id {
        Err(SdaErrorKind::PermissionDenied.into())
    } else {
        Ok(())
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
                     aggregation: &AggregationId)
                     -> SdaResult<Option<Committee>> {
        unimplemented!();
    }

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        acl_agent_is(caller, agent.id)?;
        wrap!(Self::create_agent(self, &agent))
    }

    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        // everything here is public, no acl
        wrap! { Self::get_agent(self, owner) }
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        acl_agent_is(caller, profile.owner)?;
        wrap! { Self::upsert_profile(self, profile) }
    }

    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        // everything here is public, no acl
        wrap! { Self::get_profile(self, owner) }
    }

    fn create_encryption_key(&self,
                             caller: &Agent,
                             key: &SignedEncryptionKey)
                             -> SdaResult<()> {
        acl_agent_is(caller, key.signer)?;
        wrap! { Self::create_encryption_key(self, key) }
    }

    fn get_encryption_key(&self,
                          caller: &Agent,
                          key: &EncryptionKeyId)
                          -> SdaResult<Option<SignedEncryptionKey>> {
        // everything here is public, no acl
        wrap! { Self::get_encryption_key(self, key) }
    }
}
