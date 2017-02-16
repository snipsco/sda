
use super::*;
use std::collections::HashMap;

pub struct UserInput(pub [i64]);

pub type RawData = Vec<u8>;

pub struct AuthToken(pub String);
pub struct EncryptionKey(pub RawData);
pub struct VerificationKey(pub RawData);
pub struct Signature(pub RawData);

#[derive(Clone, PartialEq, Eq, Hash)] // TODO could we use Copy instead?
pub struct AgentId(pub Uuid);

/// Identifies any agent accessing the SDA service (including users, clerks, and admins).
pub struct Agent {
    pub id: AgentId,
    pub auth_token: Option<AuthToken>,
}

pub struct CommitteeId(pub Uuid);

pub struct Committee {
    pub id: CommitteeId,
    pub name: Option<String>,
    pub clerks: Vec<AgentId>,
}

pub struct AssociatedEncryptionKey {
    pub key: EncryptionKey,
    pub signature: Signature,
}

#[derive(Clone)] // TODO could we use Copy instead?
pub struct AggregationId(pub Uuid);

pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    pub vector_dimension: usize,
    pub secret_sharing_scheme: LinearSecretSharingScheme,
    pub encryption_scheme: AdditiveEncryptionScheme,
    pub committee: CommitteeId,
    pub keyset: KeysetId,
}

pub struct KeysetId(pub Uuid);

pub struct Keyset {
    pub id: KeysetId,
    pub keys: HashMap<AgentId, AssociatedEncryptionKey>,
}

pub struct AggregationStatus {
    pub aggregation: AggregationId,
    /// Current number of participations received from the users.
    pub number_of_participations: usize,
    /// Current number of clerking results received from the clerks.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current data.
    pub result_ready: bool,
}

pub struct AggregationResult {
    pub aggregation: AggregationId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Number of clerking results used in this result.
    pub number_of_clerking_results: usize,
    /// Result of the aggregation.
    pub output: Vec<i64>,
}

/// Public profile of a clerk, including identity, cryptographic keys, name, social handles, etc.
pub struct ClerkProfile {
    pub owner: AgentId,
    pub name: Option<String>,
    pub verification_key: VerificationKey,
    pub twitter_id: Option<String>,
    pub keybase_id: Option<String>,
    pub website: Option<String>,
}

/// Partial aggregation job to be performed by a clerk, including all inputs needed.
pub struct ClerkingJob {
    pub aggregation: AggregationId,
}

/// Result of a partial aggregation job performed by a clerk.
pub struct ClerkingResult {
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    // TODO result
}

pub struct ParticipationId(pub Uuid);

/// Description of an user's input to an aggregation.
pub struct Participation {
    pub id: ParticipationId,
    pub user: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: Vec<Vec<RawData>>,
}

/// Basic operations for all SDA services.
pub trait SdaService {
    /// Send a ping to the service, expecting a pong in return if everything appears to be running.
    fn ping(&self) -> SdaResult<()>;
}

/// Common operations used by all parties.
pub trait SdaAggregationService : SdaService {

    /// Search for aggregations matching the filter.
    fn find_aggregations(&self, caller: &Agent, filter: Option<&str>) -> SdaResult<Vec<AggregationId>>;

    /// Retrieve an aggregation and its description.
    fn pull_aggregation(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Aggregation>>;

    /// Retrieve the committee associated with an aggregation.
    fn pull_committee(&self, caller: &Agent, aggregation: &AggregationId) -> SdaResult<Option<Committee>>;

    /// Retrieve the associated public clerk profile.
    fn pull_clerk_profile(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkProfile>>;

}

/// Operations used only by users.
pub trait UserSdaAggregationService : SdaAggregationService {

    /// Provide user input to an aggregation.
    fn push_user_participation(&self, caller: &Agent, participation: &Participation) -> SdaResult<()>;

}

/// Operations used only by clerks.
pub trait ClerkSdaAggregationService : SdaAggregationService {

    /// Register clerk with the given public profile and identity; updates any existing profile.
    fn update_clerk_profile(&self, caller: &Agent, profile: &ClerkProfile) -> SdaResult<()>;

    /// Pull any job waiting to be performed by the speficied clerk.
    fn pull_clerking_job(&self, caller: &Agent, clerk: &AgentId) -> SdaResult<Option<ClerkingJob>>;

    /// Push the result of a finished job.
    fn push_clerking_result(&self, caller: &Agent, result: &ClerkingResult) -> SdaResult<()>;

}

/// Extra operations used only for admins, including opening aggregations and getting their results.
pub trait AdminSdaAggregationService : SdaAggregationService {

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
