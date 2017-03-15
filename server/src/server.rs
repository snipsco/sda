use sda_protocol::*;
use errors::*;
use stores::*; // FIXME make all store names plural

pub struct SdaServer {
    pub agents_store: Box<AgentsStore>,
    pub auth_tokens_store: Box<AuthTokensStore>,
    pub aggregation_store: Box<AggregationsStore>,
    pub clerking_job_store: Box<ClerkingJobStore>,
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
    pub fn ping(&self) -> SdaServerResult<Pong> {
        self.agents_store.ping()?;
        Ok(Pong { running: true })
    }

    pub fn create_agent(&self, agent: &Agent) -> SdaServerResult<()> {
        self.agents_store.create_agent(&agent)
    }

    pub fn get_agent(&self, id: &AgentId) -> SdaServerResult<Option<Agent>> {
        self.agents_store.get_agent(&id)
    }

    pub fn upsert_profile(&self, profile: &Profile) -> SdaServerResult<()> {
        self.agents_store.upsert_profile(profile)
    }

    pub fn get_profile(&self, agent: &AgentId) -> SdaServerResult<Option<Profile>> {
        self.agents_store.get_profile(agent)
    }

    pub fn create_encryption_key(&self, key: &SignedEncryptionKey) -> SdaServerResult<()> {
        self.agents_store.create_encryption_key(key)
    }

    pub fn get_encryption_key(&self,
                              key: &EncryptionKeyId)
                              -> SdaServerResult<Option<SignedEncryptionKey>> {
        self.agents_store.get_encryption_key(key)
    }

    pub fn list_aggregations(&self,
                             filter: Option<&str>,
                             recipient: Option<&AgentId>)
                             -> SdaServerResult<Vec<AggregationId>> {
        self.aggregation_store.list_aggregations(filter, recipient)
    }

    pub fn get_aggregation(&self,
                           aggregation: &AggregationId)
                           -> SdaServerResult<Option<Aggregation>> {
        self.aggregation_store.get_aggregation(aggregation)
    }

    pub fn get_committee(&self, aggregation: &AggregationId) -> SdaServerResult<Option<Committee>> {
        self.aggregation_store.get_committee(aggregation)
    }

    pub fn create_aggregation(&self, aggregation: &Aggregation) -> SdaServerResult<()> {
        self.aggregation_store.create_aggregation(aggregation)
    }

    pub fn delete_aggregation(&self, aggregation: &AggregationId) -> SdaServerResult<()> {
        self.aggregation_store.delete_aggregation(aggregation)
    }

    pub fn suggest_committee(&self,
                             aggregation: &AggregationId)
                             -> SdaServerResult<Vec<ClerkCandidate>> {
        let _aggregation =
            self.aggregation_store.get_aggregation(aggregation)?.ok_or("deleted aggregation")?;
        self.agents_store.suggest_committee()
    }

    pub fn create_committee(&self, committee: &Committee) -> SdaServerResult<()> {
        self.aggregation_store.create_committee(committee)
    }

    pub fn create_participation(&self, participation: &Participation) -> SdaServerResult<()> {
        self.aggregation_store.create_participation(participation)
    }

    pub fn get_aggregation_status(&self,
                                  aggregation: &AggregationId)
                                  -> SdaServerResult<Option<AggregationStatus>> {
        let agg = match self.aggregation_store.get_aggregation(aggregation)? {
            None => return Ok(None),
            Some(agg) => agg,
        };
        let snapshots = self.aggregation_store
            .list_snapshots(aggregation)?
            .iter()
            .map(|id| {
                let results_count = self.clerking_job_store.list_results(id)?.len();
                Ok(SnapshotStatus {
                    id: id.clone(),
                    number_of_clerking_results: results_count,
                    result_ready: results_count >=
                                  agg.committee_sharing_scheme.reconstruction_threshold(),
                })
            })
            .collect::<SdaServerResult<Vec<SnapshotStatus>>>()?;
        Ok(Some(AggregationStatus {
            aggregation: aggregation.clone(),
            number_of_participations: self.aggregation_store.count_participations(aggregation)?,
            snapshots: snapshots,
        }))
    }

    pub fn create_snapshot(&self, snapshot: &Snapshot) -> SdaServerResult<()> {
        ::snapshot::snapshot(self, snapshot)
    }

    pub fn poll_clerking_job(&self, clerk: &AgentId) -> SdaServerResult<Option<ClerkingJob>> {
        self.clerking_job_store.poll_clerking_job(clerk)
    }

    pub fn get_clerking_job(&self,
                            clerk: &AgentId,
                            job: &ClerkingJobId)
                            -> SdaServerResult<Option<ClerkingJob>> {
        self.clerking_job_store.get_clerking_job(clerk, job)
    }

    pub fn create_clerking_result(&self, result: &ClerkingResult) -> SdaServerResult<()> {
        self.clerking_job_store.create_clerking_result(&result)
    }

    pub fn get_snapshot_result(&self,
                               aggregation: &AggregationId,
                               snapshot: &SnapshotId)
                               -> SdaServerResult<Option<SnapshotResult>> {
        let results = self.clerking_job_store
            .list_results(snapshot)?
            .iter()
            .map(|id| Ok(self.clerking_job_store.get_result(snapshot, id)?.ok_or("inconsistent storage")?))
            .collect::<SdaServerResult<Vec<ClerkingResult>>>()?;
        Ok(Some(SnapshotResult {
            snapshot: snapshot.clone(),
            number_of_participations: self.aggregation_store
                .count_participations_snapshot(aggregation, snapshot)?,
            clerk_encryptions: results,
            recipient_encryptions: self.aggregation_store.get_snapshot_mask(snapshot)?,
        }))
    }

    pub fn upsert_auth_token(&self, token: &AuthToken) -> SdaResult<()> {
        wrap! { self.auth_tokens_store.upsert_auth_token(token) }
    }

    pub fn check_auth_token(&self, token: &AuthToken) -> SdaResult<Agent> {
        let db = self.auth_tokens_store
            .get_auth_token(token.id())
            .map_err(|e| format!("error in server: {}", e))?;
        if db.as_ref() == Some(token) {
            Ok(self.agents_store
                .get_agent(&token.id)
                .map_err(|e| format!("error in server: {}", e))?
                .ok_or("Agent not found")?)
        } else {
            Err(SdaErrorKind::InvalidCredentials)?
        }
    }

    pub fn delete_auth_token(&self, agent: &AgentId) -> SdaResult<()> {
        wrap!(self.auth_tokens_store.delete_auth_token(agent))
    }
}

pub struct SdaServerService(pub SdaServer);

impl SdaService for SdaServerService {}

impl SdaServerService {
    pub fn new_jfs_server(dir: &::std::path::Path) -> SdaResult<SdaServerService> {
        let agents = ::jfs_stores::JfsAgentsStore::new(dir.join("agents")).unwrap();
        let auth = ::jfs_stores::JfsAuthTokensStore::new(dir.join("auths")).unwrap();
        let agg = ::jfs_stores::JfsAggregationsStore::new(dir.join("agg")).unwrap();
        let jobs = ::jfs_stores::JfsClerkingJobStore::new(dir.join("jobs")).unwrap();
        Ok(SdaServerService(SdaServer {
            agents_store: Box::new(agents),
            auth_tokens_store: Box::new(auth),
            aggregation_store: Box::new(agg),
            clerking_job_store: Box::new(jobs),
        }))
    }
}

impl SdaBaseService for SdaServerService {
    fn ping(&self) -> SdaResult<Pong> {
        wrap!(self.0.ping())
    }
}

fn acl_agent_is(agent: &Agent, agent_id: AgentId) -> SdaResult<()> {
    if agent.id != agent_id {
        Err(SdaErrorKind::PermissionDenied.into())
    } else {
        Ok(())
    }
}

impl SdaAgentService for SdaServerService {
    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()> {
        acl_agent_is(caller, agent.id)?;
        wrap!(self.0.create_agent(&agent))
    }

    fn get_agent(&self, _caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>> {
        // everything here is public, no acl
        wrap! { self.0.get_agent(owner) }
    }

    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()> {
        acl_agent_is(caller, profile.owner)?;
        wrap! { self.0.upsert_profile(profile) }
    }

    fn get_profile(&self, _caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>> {
        // everything here is public, no acl
        wrap! { self.0.get_profile(owner) }
    }

    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()> {
        acl_agent_is(caller, key.signer)?;
        wrap! { self.0.create_encryption_key(key) }
    }

    fn get_encryption_key(&self,
                          _caller: &Agent,
                          key: &EncryptionKeyId)
                          -> SdaResult<Option<SignedEncryptionKey>> {
        // everything here is public, no acl
        wrap! { self.0.get_encryption_key(key) }
    }
}

impl SdaAggregationService for SdaServerService {
    fn list_aggregations(&self,
                         _caller: &Agent,
                         filter: Option<&str>,
                         recipient: Option<&AgentId>)
                         -> SdaResult<Vec<AggregationId>> {
        wrap! { self.0.list_aggregations(filter, recipient) }
    }

    fn get_aggregation(&self,
                       _caller: &Agent,
                       aggregation: &AggregationId)
                       -> SdaResult<Option<Aggregation>> {
        wrap!(self.0.get_aggregation(aggregation))
    }

    fn get_committee(&self,
                     _caller: &Agent,
                     aggregation: &AggregationId)
                     -> SdaResult<Option<Committee>> {
        wrap!(self.0.get_committee(aggregation))
    }
}

impl SdaRecipientService for SdaServerService {
    fn create_aggregation(&self, caller: &Agent, aggregation: &Aggregation) -> SdaResult<()> {
        acl_agent_is(caller, aggregation.recipient)?;
        wrap! { self.0.create_aggregation(&aggregation) }
    }

    fn delete_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<()> {
        let agg: SdaResult<Option<Aggregation>> = wrap! { self.0.get_aggregation(aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { self.0.delete_aggregation(&aggregation) }
    }

    fn suggest_committee(&self,
                         caller: &Agent,
                         aggregation: &AggregationId)
                         -> SdaResult<Vec<ClerkCandidate>> {
        let agg: SdaResult<Option<Aggregation>> = wrap! { self.0.get_aggregation(aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { self.0.suggest_committee(aggregation) }
    }

    fn create_committee(&self, caller: &Agent, committee: &Committee) -> SdaResult<()> {
        let agg: SdaResult<Option<Aggregation>> =
            wrap! { self.0.get_aggregation(&committee.aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { self.0.create_committee(committee) }
    }

    fn get_aggregation_status(&self,
                              caller: &Agent,
                              aggregation: &AggregationId)
                              -> SdaResult<Option<AggregationStatus>> {
        let agg: SdaResult<Option<Aggregation>> = wrap! { self.0.get_aggregation(&aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap!(self.0.get_aggregation_status(aggregation))
    }

    fn create_snapshot(&self, caller: &Agent, snapshot: &Snapshot) -> SdaResult<()> {
        let agg: SdaResult<Option<Aggregation>> =
            wrap! { self.0.get_aggregation(&snapshot.aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { self.0.create_snapshot(snapshot) }
    }

    fn get_snapshot_result(&self,
                           caller: &Agent,
                           aggregation: &AggregationId,
                           snapshot: &SnapshotId)
                           -> SdaResult<Option<SnapshotResult>> {
        // FIXME no aggregation/snapshot spoofing
        let agg: SdaResult<Option<Aggregation>> = wrap! { self.0.get_aggregation(&aggregation) };
        let agg = agg?;
        let agg = agg.ok_or("No aggregation found")?;
        acl_agent_is(caller, agg.recipient)?;
        wrap! { self.0.get_snapshot_result(aggregation, snapshot) }
    }
}

impl SdaParticipationService for SdaServerService {
    fn create_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()> {
        acl_agent_is(caller, participation.participant)?;
        wrap!(self.0.create_participation(participation))
    }
}

impl SdaClerkingService for SdaServerService {
    fn get_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>> {
        acl_agent_is(caller, *clerk)?;
        wrap!(self.0.poll_clerking_job(clerk))
    }

    fn create_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()> {
        // double check the job really belongs to the caller (could be spoofed
        // if the store do a find_by_job_id without filtering on clerk id)
        let job: SdaResult<Option<ClerkingJob>> =
            wrap! { self.0.get_clerking_job(&result.clerk, &result.job) };
        let job = job?;
        let job = job.ok_or("Job not found")?;
        acl_agent_is(caller, job.clerk)?;
        wrap!(self.0.create_clerking_result(result))
    }
}
