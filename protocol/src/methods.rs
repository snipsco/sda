
//! Methods of the SDA services.

use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pong { pub running: bool }

/// Basic methods for SDA services.
pub trait SdaService : Sync + Send {
    /// Send a ping to the service, expecting a pong in return if everything appears to be running.
    fn ping(&self) -> SdaResult<Pong>;
}

/// Methods used mainly for discovering and maintaining agents and their
/// identities.
pub trait SdaAgentService : SdaService {

    /// Create an agent.
    fn create_agent(&self, caller: &Agent, agent: &Agent) -> SdaResult<()>;

    /// Retrieve the agent description.
    fn get_agent(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Agent>>;

    /// Register the given public profile; updates any existing profile.
    fn upsert_profile(&self, caller: &Agent, profile: &Profile) -> SdaResult<()>;

    /// Retrieve the associated public profile.
    fn get_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>>;

    /// Register new encryption key for agent.
    fn create_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()>;

    /// Retrieve agent encryption key.
    fn get_encryption_key(&self, caller: &Agent, key: &EncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>>;
}

/// Methods used mainly for discovering aggregation objects.
pub trait SdaAggregationService : SdaService {

    /// Search for aggregations with titles matching the filter.
    fn list_aggregations_by_title(&self, caller: &Agent, filter: &str) -> SdaResult<Vec<AggregationId>>;

    /// Search for aggregations with specific recipient.
    fn list_aggregations_by_recipient(&self, caller: &Agent, recipient: &AgentId) -> SdaResult<Vec<AggregationId>>;

    /// Retrieve an aggregation and its description.
    fn get_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>>;

    /// Retrieve the associated committee.
    fn get_committee(&self, caller: &Agent, owner: &AggregationId) -> SdaResult<Option<Committee>>;
}


/// Methods used for participation in particular.
pub trait SdaParticipationService : SdaService {

    /// Provide user input to an aggregation.
    fn create_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()>;

}

/// Methods used for clerking in particular.
pub trait SdaClerkingService : SdaService {

    /// Pull any job waiting to be performed by the speficied clerk.
    fn get_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>>;

    /// Push the result of a finished job.
    fn create_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()>;

}

/// Methods used by the recipient in particular.
pub trait SdaRecipientService : SdaService {

    /// Poll status of an aggregation.
    fn get_aggregation_status(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<AggregationStatus>>;

    /// Retrieve results of an aggregation.
    fn get_aggregation_results(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<AggregationResult>>;

}

/// Methods used only for administration, including creating and deleting aggregations.
pub trait SdaAdministrationService : SdaService {

    /// Create a new aggregation on the service (without any associated result).
    /// If successful, the original id has been replaced by the returned id.
    fn create_aggregation(&self, caller: &Agent, aggregation: &Aggregation) -> SdaResult<()>;

    /// Delete all information (including results) regarding an aggregation.
    fn delete_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<()>;

    fn create_committee(&self, caller: &Agent, committee: &Committee) -> SdaResult<()>;

}
