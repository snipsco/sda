
//! SDA services.
///
/// These follow the resource-oriented API design philosophy.

// TODO use Google naming convension? 
// - List, Get, Create, Update, and Delete;
//   see https://cloud.google.com/apis/design/standard_methods
//   and https://cloud.google.com/apis/design/custom_methods

use super::*;

/// Basic operations for all SDA services.
pub trait SdaService {
    /// Send a ping to the service, expecting a pong in return if everything appears to be running.
    fn ping(&self) -> SdaResult<()>;
}

/// Common operations used by all parties.
pub trait SdaDiscoveryService : SdaService {

    /// Search for aggregations with titles matching the filter.
    fn list_aggregations_by_title(&self, caller: &Agent, filter: &str) -> SdaResult<Vec<AggregationId>>;

    /// Search for aggregations with specific recipient.
    fn list_aggregations_by_recipient(&self, caller: &Agent, recipient: &AgentId) -> SdaResult<Vec<AggregationId>>;

    /// Retrieve an aggregation and its description.
    fn pull_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>>;

    /// Retrieve the associated committee.
    fn pull_committee(&self, caller: &Agent, committee: &CommitteeId) -> SdaResult<Option<Committee>>;

    /// Retrieve the associated public profile.
    fn pull_profile(&self, caller: &Agent, owner: &AgentId) -> SdaResult<Option<Profile>>;

    /// Register the given public profile; updates any existing profile.
    fn update_profile(&mut self, caller: &Agent, profile: &Profile) -> SdaResult<Profile>;

    /// Register new encryption key for agent.
    fn push_encryption_key(&mut self, caller: &Agent, key: &SignedEncryptionKey) -> SdaResult<()>;

    fn pull_encryption_key(&self, caller: &Agent, key: &SignedEncryptionKeyId) -> SdaResult<Option<SignedEncryptionKey>>;

}

/// Operations used only by participants.
pub trait SdaParticipationService : SdaService {

    /// Provide user input to an aggregation.
    fn push_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()>;

}

/// Operations used only by clerks.
pub trait SdaClerkingService : SdaService {

    /// Pull any job waiting to be performed by the speficied clerk.
    fn pull_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>>;

    /// Push the result of a finished job.
    fn push_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()>;

}

/// Extra operations used only for admins, including opening aggregations and getting their results.
pub trait SdaAdministrationService : SdaService {

    /// Create a new aggregation on the service (without any associated result).
    /// If successful, the original id has been replaced by the returned id.
    fn create_aggregation(&self, caller: &Agent, aggregation: &Aggregation) -> SdaResult<AggregationId>;

    /// Poll status of an aggregation.
    fn poll_aggregation_status(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<AggregationStatus>>;

    /// Retrieve results of an aggregation.
    fn pull_aggregation_results(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Vec<AggregationResult>>;
    
    /// Delete all information (including results) regarding an aggregation.
    fn delete_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<bool>;

}
