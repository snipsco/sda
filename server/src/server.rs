use sda_protocol::*;
use errors::*;
use stores::*; // FIXME make all store names plural

pub struct SdaServer {
    pub agent_store: Box<AgentStore>,
    pub auth_token_store: Box<AuthStore>,
    pub aggregation_store: Box<AggregationsStore>,
}

macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("error in server: {}", err).into()),
        }
    }
}

impl SdaServer {
    fn ping(&self) -> SdaServerResult<Pong> {
        self.agent_store.ping()?;
        Ok(Pong { running: true })
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

    fn list_aggregations(&self, filter: Option<&str>, recipient: Option<&AgentId>)
        -> SdaServerResult<Vec<AggregationId>> {
        self.aggregation_store.list_aggregations(filter, recipient)
    }

    fn get_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Aggregation>> {
        self.aggregation_store.get_aggregation(aggregation)
    }

    fn get_committee(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Committee>> {
        self.aggregation_store.get_committee(aggregation)
    }

    fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()> {
        self.aggregation_store.create_aggregation(aggregation)
    }

    fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()> {
        self.aggregation_store.delete_aggregation(aggregation)
    }

    fn suggest_committee(&self, aggregation:&AggregationId) -> SdaServerResult<Vec<ClerkCandidate>> {
        let _aggregation = self.aggregation_store.get_aggregation(aggregation)?.ok_or("deleted aggregation")?;
        self.agent_store.suggest_committee()
    }

    fn create_committee(&self, committee: &Committee) -> SdaServerResult<()> {
        self.aggregation_store.create_committee(committee)
    }

    fn create_participation(&self, participation: &Participation) -> SdaServerResult<()> {
        wrap!(self.aggregation_store.create_participation(participation))
    }

    pub fn upsert_auth_token(&self, token: &AuthToken) -> SdaResult<()> {
        wrap! { self.auth_token_store.upsert_auth_token(token) }
    }

    pub fn check_auth_token(&self, token: &AuthToken) -> SdaResult<Agent> {
        let db = self.auth_token_store
            .get_auth_token(token.id())
            .map_err(|e| format!("error in server: {}", e))?;
        if db.as_ref() == Some(token) {
            Ok(self.agent_store.get_agent(&token.id)
            .map_err(|e| format!("error in server: {}", e))?
            .ok_or("Agent not found")?)
        } else {
            Err(SdaErrorKind::InvalidCredentials)?
        }
    }

    pub fn delete_auth_token(&self, agent: &AgentId) -> SdaResult<()> {
        wrap!(self.auth_token_store.delete_auth_token(agent))
    }

}

impl SdaService for SdaServer {}

impl SdaBaseService for SdaServer {
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

impl SdaAgentService for SdaServer {

    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        acl_agent_is(caller, agent.id)?;
        wrap!(Self::create_agent(self, &agent))
    }

    fn get_agent(&self, _caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        // everything here is public, no acl
        wrap! { Self::get_agent(self, owner) }
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        acl_agent_is(caller, profile.owner)?;
        wrap! { Self::upsert_profile(self, profile) }
    }

    fn get_profile(&self, _caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        // everything here is public, no acl
        wrap! { Self::get_profile(self, owner) }
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        acl_agent_is(caller, key.signer)?;
        wrap! { Self::create_encryption_key(self, key) }
    }

    fn get_encryption_key(&self,
                          _caller: &Agent,
                          key: &EncryptionKeyId)
                          -> SdaResult<Option<SignedEncryptionKey>> {
        // everything here is public, no acl
        wrap! { Self::get_encryption_key(self, key) }
    }
}

impl SdaAggregationService for SdaServer {
    fn list_aggregations(&self, _caller: &Agent, filter: Option<&str>, recipient: Option<&AgentId>) -> SdaResult<Vec<AggregationId>> {
        wrap! { SdaServer::list_aggregations(self, filter, recipient) }
    }

    fn get_aggregation(&self,
                       _caller: &Agent,
                       aggregation: &AggregationId)
                       -> SdaResult<Option<Aggregation>> {
        wrap!(Self::get_aggregation(self, aggregation))
    }

    fn get_committee(&self,
                     _caller: &Agent,
                     aggregation: &AggregationId)
                     -> SdaResult<Option<Committee>> {
        wrap!(Self::get_committee(self, aggregation))
    }
}

impl SdaAdministrationService for SdaServer {
    fn create_aggregation(&self, caller: &Agent, aggregation: &Aggregation) -> SdaResult<()> {
        acl_agent_is(caller, aggregation.recipient)?;
        wrap! { SdaServer::create_aggregation(self, &aggregation) }
    }

    fn delete_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<()> {
        let agg:SdaResult<Option<Aggregation>> = wrap! { SdaServer::get_aggregation(self, aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { SdaServer::delete_aggregation(self, &aggregation) }
    }

    fn suggest_committee(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<ClerkCandidate>> {
        let agg:SdaResult<Option<Aggregation>> = wrap! { SdaServer::get_aggregation(self, aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { SdaServer::suggest_committee(self, aggregation) }
    }

    fn create_committee(&self, caller: &Agent, committee: &Committee) -> SdaResult<()> {
        let agg:SdaResult<Option<Aggregation>> = wrap! { SdaServer::get_aggregation(self, &committee.aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { SdaServer::create_committee(self, committee) }
    }
}

impl SdaParticipationService for SdaServer {

    fn create_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()> {
        acl_agent_is(caller, participation.participant)?;
        wrap!( SdaServer::create_participation(self, participation) )
    }

}

impl SdaClerkingService for SdaServer {
    fn get_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>> {
        unimplemented!()
    }

    fn create_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()> {
        unimplemented!()
    }
}

